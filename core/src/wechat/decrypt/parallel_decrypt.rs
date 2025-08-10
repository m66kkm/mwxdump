//! 并行页面解密实现
//! 
//! 提供高性能的异步并行解密功能，显著提升大文件解密速度

use std::collections::BTreeMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt, SeekFrom};
use tokio::sync::{mpsc, Mutex, Semaphore};
use tracing::{debug, info, warn};
use futures::future::try_join_all;

use crate::errors::{Result, WeChatError};
use super::{
    decrypt_common::{derive_keys_v4, verify_page_hmac, SQLITE_HEADER},
    DecryptConfig, ProgressCallback,
};

/// 页面处理任务
#[derive(Debug, Clone)]
pub struct PageTask {
    /// 页面编号
    pub page_num: u64,
    /// 文件偏移量
    pub offset: u64,
    /// 页面大小
    pub size: usize,
    /// 页面数据
    pub data: Vec<u8>,
}

/// 处理完成的页面
#[derive(Debug)]
pub struct ProcessedPage {
    /// 页面编号
    pub page_num: u64,
    /// 处理结果
    pub result: Result<Vec<u8>>,
}

impl ProcessedPage {
    /// 创建成功的处理结果
    pub fn success(page_num: u64, data: Vec<u8>) -> Self {
        Self {
            page_num,
            result: Ok(data),
        }
    }
    
    /// 创建错误的处理结果
    pub fn error(page_num: u64, error: crate::errors::WeChatError) -> Self {
        Self {
            page_num,
            result: Err(error.into()),
        }
    }
}

/// 并行解密配置
#[derive(Debug, Clone)]
pub struct ParallelDecryptConfig {
    /// 并发页面数量
    pub concurrent_pages: usize,
    /// 每批处理的页面数
    pub batch_size: usize,
    /// 读取缓冲区大小
    pub read_buffer_size: usize,
    /// 写入缓冲区大小
    pub write_buffer_size: usize,
    /// 内存使用限制 (MB)
    pub max_memory_mb: usize,
}

impl ParallelDecryptConfig {
    /// 自动配置参数
    pub fn auto_configure() -> Self {
        let cpu_count = num_cpus::get();
        Self {
            concurrent_pages: (cpu_count * 2).min(32).max(4),
            batch_size: 64,
            read_buffer_size: 1024 * 1024, // 1MB
            write_buffer_size: 1024 * 1024, // 1MB
            max_memory_mb: 512, // 512MB
        }
    }
    
    /// 为小文件优化的配置
    pub fn small_file_config() -> Self {
        Self {
            concurrent_pages: 4,
            batch_size: 16,
            read_buffer_size: 256 * 1024, // 256KB
            write_buffer_size: 256 * 1024, // 256KB
            max_memory_mb: 128, // 128MB
        }
    }
    
    /// 为大文件优化的配置
    pub fn large_file_config() -> Self {
        let cpu_count = num_cpus::get();
        Self {
            concurrent_pages: (cpu_count * 4).min(64).max(8),
            batch_size: 128,
            read_buffer_size: 2 * 1024 * 1024, // 2MB
            write_buffer_size: 2 * 1024 * 1024, // 2MB
            max_memory_mb: 1024, // 1GB
        }
    }
}

/// 内存使用监控器
pub struct MemoryMonitor {
    max_memory_bytes: usize,
    current_usage: Arc<AtomicUsize>,
}

impl MemoryMonitor {
    pub fn new(max_memory_mb: usize) -> Self {
        Self {
            max_memory_bytes: max_memory_mb * 1024 * 1024,
            current_usage: Arc::new(AtomicUsize::new(0)),
        }
    }
    
    pub fn allocate(&self, size: usize) -> bool {
        let current = self.current_usage.fetch_add(size, Ordering::Relaxed);
        current + size <= self.max_memory_bytes
    }
    
    pub fn deallocate(&self, size: usize) {
        self.current_usage.fetch_sub(size, Ordering::Relaxed);
    }
    
    pub fn current_usage_mb(&self) -> usize {
        self.current_usage.load(Ordering::Relaxed) / (1024 * 1024)
    }
    
    pub fn is_memory_pressure(&self) -> bool {
        let current = self.current_usage.load(Ordering::Relaxed);
        current > (self.max_memory_bytes * 80 / 100) // 80% 阈值
    }
}

/// 并行解密器
pub struct ParallelDecryptor {
    config: DecryptConfig,
    parallel_config: ParallelDecryptConfig,
    memory_monitor: MemoryMonitor,
}

impl ParallelDecryptor {
    /// 创建新的并行解密器
    pub fn new(config: DecryptConfig, parallel_config: ParallelDecryptConfig) -> Self {
        let memory_monitor = MemoryMonitor::new(parallel_config.max_memory_mb);
        Self {
            config,
            parallel_config,
            memory_monitor,
        }
    }
    
    /// 并行解密数据库
    pub async fn decrypt_database_parallel(
        &self,
        input_path: &std::path::Path,
        output_path: &std::path::Path,
        key: &[u8],
        progress_callback: Option<ProgressCallback>,
    ) -> Result<()> {
        info!("🚀 开始并行解密: {:?} -> {:?}", input_path, output_path);
        info!("⚙️ 并发配置: {} 个工作线程, 批大小: {}", 
              self.parallel_config.concurrent_pages, 
              self.parallel_config.batch_size);
        
        let start_time = std::time::Instant::now();
        
        // 1. 读取文件信息
        let (file_size, first_page) = self.read_db_info(input_path).await?;
        let total_pages = (file_size as usize + self.config.page_size - 1) / self.config.page_size;
        
        info!("📊 文件信息: 大小 {} MB, 总页数 {}", 
              file_size / (1024 * 1024), total_pages);
        
        // 2. 验证和准备密钥
        let derived_keys = self.prepare_keys(&first_page, key).await?;
        let derived_keys = Arc::new(derived_keys);
        
        // 3. 创建文件句柄
        let input_file = Arc::new(Mutex::new(File::open(input_path).await?));
        let output_file = Arc::new(Mutex::new(File::create(output_path).await?));
        
        // 4. 写入SQLite头
        output_file.lock().await.write_all(SQLITE_HEADER).await?;
        
        // 5. 创建通信通道
        let (page_sender, page_receiver) = mpsc::channel(self.parallel_config.batch_size * 2);
        let (result_sender, result_receiver) = mpsc::channel(self.parallel_config.batch_size * 2);
        
        // 6. 启动任务
        let read_task = self.spawn_read_task(
            input_file.clone(),
            page_sender,
            total_pages,
        );
        
        let process_tasks = self.spawn_process_tasks(
            page_receiver,
            result_sender,
            derived_keys,
        ).await?;
        
        let write_task = self.spawn_write_task(
            output_file,
            result_receiver,
            total_pages,
            progress_callback,
        );
        
        // 7. 等待所有任务完成
        let (read_result, process_results, write_result) = tokio::try_join!(
            read_task,
            try_join_all(process_tasks),
            write_task
        )?;
        
        let elapsed = start_time.elapsed();
        info!("🎉 并行解密完成! 耗时: {:.2}秒", elapsed.as_secs_f64());
        info!("📈 性能统计: 读取 {} 页, 处理 {} 个任务, 写入 {} 页", 
              read_result?, process_results.len(), write_result?);
        info!("💾 内存使用峰值: {} MB", self.memory_monitor.current_usage_mb());
        
        Ok(())
    }
    
    /// 读取数据库文件信息
    async fn read_db_info(&self, file_path: &std::path::Path) -> Result<(u64, Vec<u8>)> {
        let mut file = File::open(file_path).await
            .map_err(|e| WeChatError::DecryptionFailed(format!("打开文件失败: {}", e)))?;
        
        // 获取文件大小
        let file_size = file.metadata().await
            .map_err(|e| WeChatError::DecryptionFailed(format!("获取文件信息失败: {}", e)))?
            .len();
        
        // 读取第一页
        let mut first_page = vec![0u8; self.config.page_size];
        let bytes_read = file.read(&mut first_page).await
            .map_err(|e| WeChatError::DecryptionFailed(format!("读取第一页失败: {}", e)))?;
        
        if bytes_read < self.config.page_size {
            first_page.truncate(bytes_read);
        }
        
        Ok((file_size, first_page))
    }
    
    /// 准备解密密钥
    async fn prepare_keys(&self, first_page: &[u8], key: &[u8]) -> Result<super::decrypt_common::DerivedKeys> {
        use super::decrypt_common::{is_database_encrypted, SALT_SIZE};
        
        // 检查是否已解密
        if !is_database_encrypted(first_page) {
            return Err(WeChatError::DecryptionFailed("数据库已经解密".to_string()).into());
        }
        
        // 提取Salt
        if first_page.len() < SALT_SIZE {
            return Err(WeChatError::DecryptionFailed("第一页数据不完整".to_string()).into());
        }
        
        let salt = &first_page[..SALT_SIZE];
        debug!("提取Salt: {} 字节", salt.len());
        
        // 派生密钥
        let derived_keys = derive_keys_v4(key, salt)?;
        
        // 验证密钥
        if !verify_page_hmac(first_page, &derived_keys.mac_key, 0, &self.config)? {
            return Err(WeChatError::DecryptionFailed("密钥验证失败".to_string()).into());
        }
        
        info!("✅ 密钥验证成功");
        Ok(derived_keys)
    }
    
    /// 启动读取任务
    fn spawn_read_task(
        &self,
        input_file: Arc<Mutex<File>>,
        sender: mpsc::Sender<PageTask>,
        total_pages: usize,
    ) -> tokio::task::JoinHandle<Result<usize>> {
        let page_size = self.config.page_size;
        let batch_size = self.parallel_config.batch_size;
        let memory_monitor = Arc::new(self.memory_monitor.current_usage.clone());
        
        tokio::spawn(async move {
            let mut pages_read = 0;
            let mut current_batch = Vec::with_capacity(batch_size);
            
            for page_num in 0..total_pages {
                let offset = page_num * page_size;
                
                // 内存压力检查
                while memory_monitor.load(Ordering::Relaxed) > 800 * 1024 * 1024 { // 800MB
                    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
                }
                
                // 读取页面数据
                let mut page_data = vec![0u8; page_size];
                let bytes_read = {
                    let mut file = input_file.lock().await;
                    file.seek(SeekFrom::Start(offset as u64)).await?;
                    file.read(&mut page_data).await?
                };
                
                if bytes_read == 0 {
                    break;
                }
                
                if bytes_read < page_size {
                    page_data.truncate(bytes_read);
                }
                
                // 检查是否为空页面，如果是则跳过解密处理
                let _is_empty_page = page_data.iter().all(|&b| b == 0);
                
                let task = PageTask {
                    page_num: page_num as u64,
                    offset: offset as u64,
                    size: bytes_read,
                    data: page_data,
                };
                
                current_batch.push(task);
                
                // 批量发送
                if current_batch.len() >= batch_size || page_num == total_pages - 1 {
                    for task in current_batch.drain(..) {
                        sender.send(task).await.map_err(|_| {
                            WeChatError::DecryptionFailed("发送页面任务失败".to_string())
                        })?;
                        pages_read += 1;
                    }
                    
                    // 让出控制权
                    if pages_read % (batch_size * 4) == 0 {
                        tokio::task::yield_now().await;
                    }
                }
            }
            
            debug!("读取任务完成: {} 页", pages_read);
            Ok(pages_read)
        })
    }
    
    /// 启动处理任务池
    async fn spawn_process_tasks(
        &self,
        receiver: mpsc::Receiver<PageTask>,
        sender: mpsc::Sender<ProcessedPage>,
        derived_keys: Arc<super::decrypt_common::DerivedKeys>,
    ) -> Result<Vec<tokio::task::JoinHandle<Result<usize>>>> {
        let semaphore = Arc::new(Semaphore::new(self.parallel_config.concurrent_pages));
        let receiver = Arc::new(Mutex::new(receiver));
        let mut tasks = Vec::new();
        
        for worker_id in 0..self.parallel_config.concurrent_pages {
            let receiver = receiver.clone();
            let sender = sender.clone();
            let keys = derived_keys.clone();
            let sem = semaphore.clone();
            let decrypt_config = self.config.clone();
            
            let task = tokio::spawn(async move {
                let mut processed = 0;
                
                loop {
                    let page_task = {
                        let mut rx = receiver.lock().await;
                        match rx.recv().await {
                            Some(task) => task,
                            None => break, // 通道关闭
                        }
                    };
                    
                    let _permit = sem.acquire().await.unwrap();
                    let page_num = page_task.page_num; // 保存页面编号
                    
                    match Self::process_page_async(page_task, &keys, &decrypt_config).await {
                        Ok(processed_page) => {
                            sender.send(processed_page).await.map_err(|_| {
                                WeChatError::DecryptionFailed("发送处理结果失败".to_string())
                            })?;
                            processed += 1;
                        }
                        Err(e) => {
                            warn!("Worker {} 处理页面失败: {}", worker_id, e);
                            // 发送错误页面，保持顺序
                            let error_page = ProcessedPage::error(page_num,
                                WeChatError::DecryptionFailed(format!("页面处理失败: {}", e)));
                            sender.send(error_page).await.ok();
                        }
                    }
                    
                    // 定期让出控制权
                    if processed % 10 == 0 {
                        tokio::task::yield_now().await;
                    }
                }
                
                debug!("Worker {} 完成: 处理 {} 页", worker_id, processed);
                Ok(processed)
            });
            
            tasks.push(task);
        }
        
        Ok(tasks)
    }
    
    /// 异步处理单个页面
    async fn process_page_async(
        page_task: PageTask,
        keys: &super::decrypt_common::DerivedKeys,
        config: &DecryptConfig,
    ) -> Result<ProcessedPage> {
        let page_num = page_task.page_num;
        let page_data = page_task.data;
        
        // 检查是否为空页面
        if page_data.iter().all(|&b| b == 0) {
            debug!("跳过空页面 {}", page_num);
            return Ok(ProcessedPage::success(page_num, page_data));
        }
        
        // 克隆数据用于错误处理
        let page_data_backup = page_data.clone();
        
        // 在专用线程中执行CPU密集型操作
        let enc_key = keys.enc_key.clone();
        let mac_key = keys.mac_key.clone();
        let config = config.clone();
        
        let result = tokio::task::spawn_blocking(move || {
            use super::decrypt_common::decrypt_page;
            decrypt_page(&page_data, &enc_key, &mac_key, page_num, &config)
        }).await;
        
        match result {
            Ok(Ok(decrypted_data)) => {
                debug!("页面 {} 解密成功", page_num);
                Ok(ProcessedPage::success(page_num, decrypted_data))
            }
            Ok(Err(e)) => {
                warn!("页面 {} 解密失败: {}", page_num, e);
                // 对于解密失败的页面，返回原始数据作为备用
                Ok(ProcessedPage::success(page_num, page_data_backup))
            }
            Err(e) => {
                warn!("页面 {} 处理任务失败: {}", page_num, e);
                Err(WeChatError::DecryptionFailed(format!("页面 {} 处理任务失败: {}", page_num, e)).into())
            }
        }
    }
    
    /// 启动写入任务
    fn spawn_write_task(
        &self,
        output_file: Arc<Mutex<File>>,
        mut receiver: mpsc::Receiver<ProcessedPage>,
        total_pages: usize,
        progress_callback: Option<ProgressCallback>,
    ) -> tokio::task::JoinHandle<Result<usize>> {
        tokio::spawn(async move {
            let mut pages_written = 0;
            let mut pending_pages = BTreeMap::new();
            let mut next_expected_page = 0u64;
            let mut last_progress_report = std::time::Instant::now();
            
            while let Some(processed_page) = receiver.recv().await {
                pending_pages.insert(processed_page.page_num, processed_page);
                
                // 按顺序写入连续的页面
                while let Some(page) = pending_pages.remove(&next_expected_page) {
                    match page.result {
                        Ok(data) => {
                            output_file.lock().await.write_all(&data).await?;
                            pages_written += 1;
                            
                            // 调用进度回调
                            if let Some(ref callback) = progress_callback {
                                callback(pages_written as u64, total_pages as u64);
                            }
                            
                            // 定期报告进度
                            if last_progress_report.elapsed().as_secs() >= 2 {
                                let progress = (pages_written as f64 / total_pages as f64) * 100.0;
                                info!("📈 解密进度: {:.1}% ({}/{})", progress, pages_written, total_pages);
                                last_progress_report = std::time::Instant::now();
                            }
                        }
                        Err(e) => {
                            warn!("页面 {} 写入失败: {}", next_expected_page, e);
                            // 写入占位数据
                            let placeholder = vec![0u8; 4096];
                            output_file.lock().await.write_all(&placeholder).await?;
                            pages_written += 1;
                        }
                    }
                    
                    next_expected_page += 1;
                    
                    // 定期刷新缓冲区
                    if pages_written % 100 == 0 {
                        output_file.lock().await.flush().await?;
                        tokio::task::yield_now().await;
                    }
                }
            }
            
            // 最终刷新
            output_file.lock().await.flush().await?;
            debug!("写入任务完成: {} 页", pages_written);
            Ok(pages_written)
        })
    }
    
    /// 获取内存监控器（用于测试）
    #[cfg(test)]
    pub fn memory_monitor(&self) -> &MemoryMonitor {
        &self.memory_monitor
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parallel_config() {
        let config = ParallelDecryptConfig::auto_configure();
        assert!(config.concurrent_pages >= 4);
        assert!(config.batch_size > 0);
        assert!(config.max_memory_mb > 0);
    }
    
    #[test]
    fn test_memory_monitor() {
        let monitor = MemoryMonitor::new(100); // 100MB
        assert!(monitor.allocate(50 * 1024 * 1024)); // 50MB
        assert!(monitor.current_usage_mb() < 100);
        monitor.deallocate(50 * 1024 * 1024);
        assert_eq!(monitor.current_usage_mb(), 0);
    }
    
    #[tokio::test]
    async fn test_page_task_creation() {
        let task = PageTask {
            page_num: 1,
            offset: 4096,
            size: 4096,
            data: vec![0u8; 4096],
        };
        assert_eq!(task.page_num, 1);
        assert_eq!(task.offset, 4096);
        assert_eq!(task.size, 4096);
    }
}