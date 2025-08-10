//! 缓存密钥验证器
//! 
//! 通过缓存PBKDF2计算结果来显著提升密钥验证性能

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use tokio::sync::RwLock;
use blake3::Hash;
use tracing::{debug, info, warn};

use crate::errors::Result;
use super::{
    DecryptVersion, 
    decrypt_common::{derive_keys_v4, DerivedKeys, SALT_SIZE},
    decrypt_validator::KeyValidator,
};

/// 缓存键，用于唯一标识密钥和Salt的组合
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CacheKey {
    /// 密钥哈希
    key_hash: Hash,
    /// Salt哈希
    salt_hash: Hash,
}

impl CacheKey {
    /// 创建新的缓存键
    pub fn new(key: &[u8], salt: &[u8]) -> Self {
        Self {
            key_hash: blake3::hash(key),
            salt_hash: blake3::hash(salt),
        }
    }
}

/// 验证统计信息
#[derive(Debug, Default)]
pub struct ValidationStats {
    /// 缓存命中次数
    pub cache_hits: AtomicU64,
    /// 缓存未命中次数
    pub cache_misses: AtomicU64,
    /// 总验证次数
    pub total_validations: AtomicU64,
    /// PBKDF2计算次数
    pub pbkdf2_computations: AtomicU64,
}

impl ValidationStats {
    /// 获取缓存命中率
    pub fn cache_hit_rate(&self) -> f64 {
        let hits = self.cache_hits.load(Ordering::Relaxed) as f64;
        let total = self.total_validations.load(Ordering::Relaxed) as f64;
        if total > 0.0 { hits / total * 100.0 } else { 0.0 }
    }
    
    /// 记录缓存命中
    pub fn record_cache_hit(&self) {
        self.cache_hits.fetch_add(1, Ordering::Relaxed);
        self.total_validations.fetch_add(1, Ordering::Relaxed);
    }
    
    /// 记录缓存未命中
    pub fn record_cache_miss(&self) {
        self.cache_misses.fetch_add(1, Ordering::Relaxed);
        self.total_validations.fetch_add(1, Ordering::Relaxed);
    }
    
    /// 记录PBKDF2计算
    pub fn record_pbkdf2_computation(&self) {
        self.pbkdf2_computations.fetch_add(1, Ordering::Relaxed);
    }
}

/// 批量验证结果
#[derive(Debug)]
pub struct BatchValidationResult {
    /// 每个文件的验证结果
    pub results: HashMap<PathBuf, Option<DecryptVersion>>,
    /// 缓存的派生密钥
    pub derived_keys: HashMap<CacheKey, DerivedKeys>,
    /// 统计信息
    pub stats: ValidationStats,
}

/// 缓存配置
#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// 最大内存缓存条目数
    pub max_memory_entries: usize,
    /// 是否启用详细日志
    pub enable_verbose_logging: bool,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            max_memory_entries: 1000,
            enable_verbose_logging: false,
        }
    }
}

/// 缓存的密钥验证器
pub struct CachedKeyValidator {
    /// 密钥缓存：CacheKey -> DerivedKeys
    cache: Arc<RwLock<HashMap<CacheKey, DerivedKeys>>>,
    /// 版本缓存：CacheKey -> DecryptVersion
    version_cache: Arc<RwLock<HashMap<CacheKey, DecryptVersion>>>,
    /// 统计信息
    stats: Arc<ValidationStats>,
    /// 配置
    config: CacheConfig,
    /// 回退验证器
    fallback_validator: KeyValidator,
}

impl CachedKeyValidator {
    /// 创建新的缓存密钥验证器
    pub fn new(config: CacheConfig) -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            version_cache: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(ValidationStats::default()),
            config,
            fallback_validator: KeyValidator::new(),
        }
    }
    
    /// 使用默认配置创建
    pub fn with_default_config() -> Self {
        Self::new(CacheConfig::default())
    }
    
    /// 获取统计信息
    pub fn stats(&self) -> &ValidationStats {
        &self.stats
    }
    
    /// 清空缓存
    pub async fn clear_cache(&self) {
        let mut cache = self.cache.write().await;
        let mut version_cache = self.version_cache.write().await;
        cache.clear();
        version_cache.clear();
        info!("🧹 缓存已清空");
    }
    
    /// 获取缓存大小
    pub async fn cache_size(&self) -> usize {
        let cache = self.cache.read().await;
        cache.len()
    }
    
    /// 单个文件密钥验证（带缓存）
    pub async fn validate_key_cached(
        &self,
        db_path: &Path,
        key: &[u8],
    ) -> Result<Option<DecryptVersion>> {
        if self.config.enable_verbose_logging {
            debug!("🔍 开始缓存密钥验证: {:?}", db_path);
        }
        
        // 读取Salt
        let salt = match self.read_file_salt(db_path).await {
            Ok(salt) => salt,
            Err(e) => {
                warn!("⚠️ 读取Salt失败: {:?} - {}", db_path, e);
                // 回退到原始验证器
                return self.fallback_validator.validate_key_auto(db_path, key).await;
            }
        };
        
        let cache_key = CacheKey::new(key, &salt);
        
        // 检查版本缓存
        {
            let version_cache = self.version_cache.read().await;
            if let Some(&version) = version_cache.get(&cache_key) {
                self.stats.record_cache_hit();
                if self.config.enable_verbose_logging {
                    debug!("✅ 版本缓存命中: {:?}", version);
                }
                return Ok(Some(version));
            }
        }
        
        // 检查密钥缓存
        let derived_keys = {
            let cache = self.cache.read().await;
            if let Some(keys) = cache.get(&cache_key) {
                self.stats.record_cache_hit();
                if self.config.enable_verbose_logging {
                    debug!("✅ 密钥缓存命中");
                }
                keys.clone()
            } else {
                self.stats.record_cache_miss();
                drop(cache);
                
                // 计算新的派生密钥
                if self.config.enable_verbose_logging {
                    debug!("🔄 计算新的派生密钥");
                }
                
                self.stats.record_pbkdf2_computation();
                let keys = self.compute_derived_keys_async(key, &salt).await?;
                
                // 存入缓存
                self.store_in_cache(cache_key.clone(), keys.clone()).await;
                keys
            }
        };
        
        // 验证HMAC
        let version = match self.verify_hmac_with_keys(db_path, &derived_keys).await {
            Ok(true) => {
                let version = DecryptVersion::V4; // 目前只支持V4
                
                // 存入版本缓存
                {
                    let mut version_cache = self.version_cache.write().await;
                    version_cache.insert(cache_key, version);
                }
                
                if self.config.enable_verbose_logging {
                    debug!("✅ HMAC验证成功: {:?}", version);
                }
                Some(version)
            }
            Ok(false) => {
                if self.config.enable_verbose_logging {
                    debug!("❌ HMAC验证失败");
                }
                None
            }
            Err(e) => {
                warn!("⚠️ HMAC验证出错: {} - 回退到原始验证器", e);
                return self.fallback_validator.validate_key_auto(db_path, key).await;
            }
        };
        
        Ok(version)
    }
    
    /// 批量验证多个文件
    pub async fn validate_files_batch(
        &self,
        files: &[PathBuf],
        key: &[u8],
    ) -> Result<BatchValidationResult> {
        info!("🚀 开始批量密钥验证: {} 个文件", files.len());
        let start_time = std::time::Instant::now();
        
        // 1. 并行读取所有Salt
        let salts = self.read_salts_parallel(files).await?;
        
        // 2. 收集所有唯一的CacheKey
        let mut unique_keys = HashMap::new();
        let mut file_to_cache_key = HashMap::new();
        
        for (file, salt) in &salts {
            let cache_key = CacheKey::new(key, salt);
            unique_keys.insert(cache_key.clone(), salt.clone());
            file_to_cache_key.insert(file.clone(), cache_key);
        }
        
        info!("📊 发现 {} 个唯一的密钥-Salt组合", unique_keys.len());
        
        // 3. 批量计算缺失的派生密钥
        let derived_keys = self.compute_missing_keys_batch(key, &unique_keys).await?;
        
        // 4. 批量验证HMAC
        let mut results = HashMap::new();
        for (file, cache_key) in file_to_cache_key {
            if let Some(keys) = derived_keys.get(&cache_key) {
                match self.verify_hmac_with_keys(&file, keys).await {
                    Ok(true) => {
                        results.insert(file, Some(DecryptVersion::V4));
                    }
                    Ok(false) => {
                        results.insert(file, None);
                    }
                    Err(e) => {
                        warn!("⚠️ 文件 {:?} HMAC验证出错: {}", file, e);
                        results.insert(file, None);
                    }
                }
            } else {
                results.insert(file, None);
            }
        }
        
        let elapsed = start_time.elapsed();
        info!("🎉 批量验证完成! 耗时: {:.2}秒", elapsed.as_secs_f64());
        info!("📈 缓存命中率: {:.1}%", self.stats.cache_hit_rate());
        
        Ok(BatchValidationResult {
            results,
            derived_keys,
            stats: ValidationStats::default(), // 返回当前统计的快照
        })
    }
    
    /// 异步计算派生密钥
    async fn compute_derived_keys_async(&self, key: &[u8], salt: &[u8]) -> Result<DerivedKeys> {
        let key = key.to_vec();
        let salt = salt.to_vec();
        
        tokio::task::spawn_blocking(move || {
            derive_keys_v4(&key, &salt)
        }).await?
    }
    
    /// 存储到缓存
    async fn store_in_cache(&self, cache_key: CacheKey, derived_keys: DerivedKeys) {
        let mut cache = self.cache.write().await;
        
        // 检查缓存大小限制
        if cache.len() >= self.config.max_memory_entries {
            // 简单的LRU策略：清空一半缓存
            let keys_to_remove: Vec<_> = cache.keys().take(cache.len() / 2).cloned().collect();
            for key in keys_to_remove {
                cache.remove(&key);
            }
            debug!("🧹 缓存已清理，当前大小: {}", cache.len());
        }
        
        cache.insert(cache_key, derived_keys);
    }
    
    /// 读取文件的Salt
    async fn read_file_salt(&self, file_path: &Path) -> Result<Vec<u8>> {
        use tokio::fs::File;
        use tokio::io::AsyncReadExt;
        
        let mut file = File::open(file_path).await?;
        let mut salt = vec![0u8; SALT_SIZE];
        file.read_exact(&mut salt).await?;
        Ok(salt)
    }
    
    /// 并行读取多个文件的Salt
    async fn read_salts_parallel(&self, files: &[PathBuf]) -> Result<HashMap<PathBuf, Vec<u8>>> {
        use futures::future::try_join_all;
        
        let tasks = files.iter().map(|file| {
            let file = file.clone();
            async move {
                let salt = self.read_file_salt(&file).await?;
                Ok::<(PathBuf, Vec<u8>), anyhow::Error>((file, salt))
            }
        });
        
        let results = try_join_all(tasks).await?;
        Ok(results.into_iter().collect())
    }
    
    /// 批量计算缺失的派生密钥
    async fn compute_missing_keys_batch(
        &self,
        key: &[u8],
        unique_keys: &HashMap<CacheKey, Vec<u8>>,
    ) -> Result<HashMap<CacheKey, DerivedKeys>> {
        let mut result = HashMap::new();
        let mut missing_keys = Vec::new();
        
        // 检查缓存中已有的密钥
        {
            let cache = self.cache.read().await;
            for (cache_key, _salt) in unique_keys {
                if let Some(keys) = cache.get(cache_key) {
                    result.insert(cache_key.clone(), keys.clone());
                    self.stats.record_cache_hit();
                } else {
                    missing_keys.push(cache_key.clone());
                    self.stats.record_cache_miss();
                }
            }
        }
        
        if !missing_keys.is_empty() {
            info!("🔄 需要计算 {} 个新的派生密钥", missing_keys.len());
            
            // 并行计算缺失的密钥
            let tasks = missing_keys.iter().map(|cache_key| {
                let salt = unique_keys.get(cache_key).unwrap().clone();
                let key = key.to_vec();
                let cache_key = cache_key.clone();
                
                async move {
                    self.stats.record_pbkdf2_computation();
                    let derived_keys = self.compute_derived_keys_async(&key, &salt).await?;
                    Ok::<(CacheKey, DerivedKeys), anyhow::Error>((cache_key, derived_keys))
                }
            });
            
            let computed_results = futures::future::try_join_all(tasks).await?;
            
            // 存储到缓存并添加到结果
            for (cache_key, derived_keys) in computed_results {
                self.store_in_cache(cache_key.clone(), derived_keys.clone()).await;
                result.insert(cache_key, derived_keys);
            }
        }
        
        Ok(result)
    }
    
    /// 使用派生密钥验证HMAC
    async fn verify_hmac_with_keys(&self, db_path: &Path, derived_keys: &DerivedKeys) -> Result<bool> {
        use tokio::fs::File;
        use tokio::io::AsyncReadExt;
        use super::decrypt_common::verify_page_hmac;
        use crate::wechat::decrypt::DecryptConfig;
        
        let mut file = File::open(db_path).await?;
        let config = DecryptConfig::v4();
        let mut first_page = vec![0u8; config.page_size];
        let bytes_read = file.read(&mut first_page).await?;
        
        if bytes_read < config.page_size {
            first_page.truncate(bytes_read);
        }
        
        verify_page_hmac(&first_page, &derived_keys.mac_key, 0, &config)
    }
}

impl Default for CachedKeyValidator {
    fn default() -> Self {
        Self::with_default_config()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;
    
    #[tokio::test]
    async fn test_cache_key_creation() {
        let key = b"test_key_32_bytes_long_for_test!";
        let salt = b"test_salt_16byte";
        
        let cache_key1 = CacheKey::new(key, salt);
        let cache_key2 = CacheKey::new(key, salt);
        
        assert_eq!(cache_key1, cache_key2);
    }
    
    #[tokio::test]
    async fn test_validation_stats() {
        let stats = ValidationStats::default();
        
        stats.record_cache_hit();
        stats.record_cache_miss();
        stats.record_pbkdf2_computation();
        
        assert_eq!(stats.cache_hits.load(Ordering::Relaxed), 1);
        assert_eq!(stats.cache_misses.load(Ordering::Relaxed), 1);
        assert_eq!(stats.total_validations.load(Ordering::Relaxed), 2);
        assert_eq!(stats.pbkdf2_computations.load(Ordering::Relaxed), 1);
        assert_eq!(stats.cache_hit_rate(), 50.0);
    }
    
    #[tokio::test]
    async fn test_cached_validator_creation() {
        let validator = CachedKeyValidator::with_default_config();
        assert_eq!(validator.cache_size().await, 0);
        assert_eq!(validator.stats().cache_hit_rate(), 0.0);
    }
    
    #[tokio::test]
    async fn test_cache_clear() {
        let validator = CachedKeyValidator::with_default_config();
        
        // 模拟添加一些缓存项
        let cache_key = CacheKey::new(b"test", b"salt");
        // 这里我们无法直接测试内部缓存，但可以测试清空操作
        validator.clear_cache().await;
        
        assert_eq!(validator.cache_size().await, 0);
    }
}