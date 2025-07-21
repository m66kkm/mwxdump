// 应用常量
export const APP_NAME = '微信数据采集管理系统';
export const APP_VERSION = '1.0.0';
export const APP_DESCRIPTION = '现代化的微信数据采集和管理工具';

// 路由路径常量
export const ROUTES = {
  HOME: '/',
  OVERVIEW: '/',
  DATA_SOURCE: '/data-source',
  DATA_PROCESSING: '/data-processing',
  SYSTEM_MONITOR: '/system-monitor',
  SERVICE_MANAGEMENT: '/service-management',
  SETTINGS: '/settings',
} as const;

// 导航菜单项
export const NAVIGATION_ITEMS = [
  { 
    path: ROUTES.OVERVIEW, 
    icon: 'fas fa-home', 
    label: '概览', 
    key: 'overview' 
  },
  { 
    path: ROUTES.DATA_SOURCE, 
    icon: 'fas fa-key', 
    label: '数据源管理', 
    key: 'data-source' 
  },
  { 
    path: ROUTES.DATA_PROCESSING, 
    icon: 'fas fa-cogs', 
    label: '数据处理', 
    key: 'data-processing' 
  },
  { 
    path: ROUTES.SYSTEM_MONITOR, 
    icon: 'fas fa-chart-line', 
    label: '系统监控', 
    key: 'system-monitor' 
  },
  { 
    path: ROUTES.SERVICE_MANAGEMENT, 
    icon: 'fas fa-server', 
    label: '服务管理', 
    key: 'service-management' 
  },
  { 
    path: ROUTES.SETTINGS, 
    icon: 'fas fa-cog', 
    label: '系统设置', 
    key: 'settings' 
  },
] as const;

// 状态常量
export const STATUS = {
  IDLE: 'idle',
  LOADING: 'loading',
  SUCCESS: 'success',
  ERROR: 'error',
  PENDING: 'pending',
  RUNNING: 'running',
  COMPLETED: 'completed',
  FAILED: 'failed',
} as const;

// 服务状态常量
export const SERVICE_STATUS = {
  STOPPED: 'stopped',
  STARTING: 'starting',
  RUNNING: 'running',
  ERROR: 'error',
} as const;

// 连接状态常量
export const CONNECTION_STATUS = {
  DISCONNECTED: 'disconnected',
  CONNECTING: 'connecting',
  CONNECTED: 'connected',
  ERROR: 'error',
} as const;

// 主题常量
export const THEMES = {
  LIGHT: 'light',
  DARK: 'dark',
  SYSTEM: 'system',
} as const;

// 语言常量
export const LANGUAGES = {
  ZH_CN: 'zh-CN',
  EN_US: 'en-US',
} as const;

// 颜色常量
export const COLORS = {
  PRIMARY: {
    50: '#eff6ff',
    100: '#dbeafe',
    200: '#bfdbfe',
    300: '#93c5fd',
    400: '#60a5fa',
    500: '#3b82f6',
    600: '#2563eb',
    700: '#1d4ed8',
    800: '#1e40af',
    900: '#1e3a8a',
  },
  SUCCESS: {
    50: '#ecfdf5',
    500: '#10b981',
    600: '#059669',
    700: '#047857',
  },
  WARNING: {
    50: '#fffbeb',
    500: '#f59e0b',
    600: '#d97706',
    700: '#b45309',
  },
  ERROR: {
    50: '#fef2f2',
    500: '#ef4444',
    600: '#dc2626',
    700: '#b91c1c',
  },
  INFO: {
    50: '#ecfeff',
    500: '#06b6d4',
    600: '#0891b2',
    700: '#0e7490',
  },
} as const;

// 图标映射
export const ICONS = {
  // 导航图标
  HOME: 'fas fa-home',
  DASHBOARD: 'fas fa-tachometer-alt',
  SETTINGS: 'fas fa-cog',
  
  // 功能图标
  KEY: 'fas fa-key',
  LOCK: 'fas fa-lock',
  UNLOCK: 'fas fa-unlock',
  DATABASE: 'fas fa-database',
  SERVER: 'fas fa-server',
  CHART: 'fas fa-chart-line',
  
  // 状态图标
  SUCCESS: 'fas fa-check-circle',
  WARNING: 'fas fa-exclamation-triangle',
  ERROR: 'fas fa-times-circle',
  INFO: 'fas fa-info-circle',
  LOADING: 'fas fa-spinner fa-spin',
  
  // 操作图标
  EDIT: 'fas fa-edit',
  DELETE: 'fas fa-trash',
  SAVE: 'fas fa-save',
  CANCEL: 'fas fa-times',
  REFRESH: 'fas fa-sync-alt',
  DOWNLOAD: 'fas fa-download',
  UPLOAD: 'fas fa-upload',
  COPY: 'fas fa-copy',
  
  // 方向图标
  UP: 'fas fa-chevron-up',
  DOWN: 'fas fa-chevron-down',
  LEFT: 'fas fa-chevron-left',
  RIGHT: 'fas fa-chevron-right',
  
  // 其他图标
  USERS: 'fas fa-users',
  TASKS: 'fas fa-tasks',
  BELL: 'fas fa-bell',
  SEARCH: 'fas fa-search',
  FILTER: 'fas fa-filter',
  SORT: 'fas fa-sort',
  MENU: 'fas fa-bars',
  CLOSE: 'fas fa-times',
  PLUS: 'fas fa-plus',
  MINUS: 'fas fa-minus',
  PLAY: 'fas fa-play',
  PAUSE: 'fas fa-pause',
  STOP: 'fas fa-stop',
} as const;

// 尺寸常量
export const SIZES = {
  SM: 'sm',
  MD: 'md',
  LG: 'lg',
  XL: 'xl',
} as const;

// 按钮变体常量
export const BUTTON_VARIANTS = {
  PRIMARY: 'primary',
  SECONDARY: 'secondary',
  DANGER: 'danger',
  GHOST: 'ghost',
  OUTLINE: 'outline',
} as const;

// 输入框状态常量
export const INPUT_STATES = {
  DEFAULT: 'default',
  ERROR: 'error',
  SUCCESS: 'success',
} as const;

// 通知类型常量
export const NOTIFICATION_TYPES = {
  SUCCESS: 'success',
  WARNING: 'warning',
  ERROR: 'error',
  INFO: 'info',
} as const;

// 日志级别常量
export const LOG_LEVELS = {
  DEBUG: 'debug',
  INFO: 'info',
  WARN: 'warn',
  ERROR: 'error',
} as const;
// 动画持续时间常量
export const ANIMATION_DURATION = {
  FAST: 150,
  NORMAL: 250,
  SLOW: 350,
  SLOWER: 500,
} as const;

// 断点常量
export const BREAKPOINTS = {
  SM: 640,
  MD: 768,
  LG: 1024,
  XL: 1280,
  '2XL': 1536,
} as const;

// 默认配置常量
export const DEFAULT_CONFIG = {
  THEME: THEMES.SYSTEM,
  LANGUAGE: LANGUAGES.ZH_CN,
  AUTO_START: false,
  NOTIFICATIONS: true,
  LOG_LEVEL: LOG_LEVELS.INFO,
  HTTP_PORT: 8080,
  MAX_CONNECTIONS: 100,
  ENABLE_CORS: true,
} as const;

// 文件类型常量
export const FILE_TYPES = {
  IMAGE: ['jpg', 'jpeg', 'png', 'gif', 'bmp', 'webp', 'svg'],
  DOCUMENT: ['pdf', 'doc', 'docx', 'txt', 'rtf'],
  ARCHIVE: ['zip', 'rar', '7z', 'tar', 'gz'],
  DATABASE: ['db', 'sqlite', 'sqlite3'],
} as const;

// API端点常量
export const API_ENDPOINTS = {
  EXTRACT_KEY: 'extract_key',
  DECRYPT_DATA: 'decrypt_data',
  START_SERVICE: 'start_http_service',
  STOP_SERVICE: 'stop_http_service',
  GET_SYSTEM_INFO: 'get_system_info',
  GET_PROCESS_INFO: 'get_process_info',
} as const;

// 错误代码常量
export const ERROR_CODES = {
  NETWORK_ERROR: 'NETWORK_ERROR',
  PERMISSION_DENIED: 'PERMISSION_DENIED',
  FILE_NOT_FOUND: 'FILE_NOT_FOUND',
  INVALID_FORMAT: 'INVALID_FORMAT',
  OPERATION_FAILED: 'OPERATION_FAILED',
  TIMEOUT: 'TIMEOUT',
  UNKNOWN_ERROR: 'UNKNOWN_ERROR',
} as const;

// 存储键常量
export const STORAGE_KEYS = {
  THEME: 'app_theme',
  LANGUAGE: 'app_language',
  USER_SETTINGS: 'user_settings',
  WINDOW_STATE: 'window_state',
  RECENT_FILES: 'recent_files',
  LOGS: 'app_logs',
} as const;

// 事件名称常量
export const EVENTS = {
  THEME_CHANGED: 'theme_changed',
  LANGUAGE_CHANGED: 'language_changed',
  STATUS_UPDATED: 'status_updated',
  LOG_ADDED: 'log_added',
  NOTIFICATION_ADDED: 'notification_added',
} as const;

// 正则表达式常量
export const REGEX = {
  EMAIL: /^[^\s@]+@[^\s@]+\.[^\s@]+$/,
  URL: /^https?:\/\/.+/,
  IPV4: /^(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$/,
  PORT: /^([1-9][0-9]{0,3}|[1-5][0-9]{4}|6[0-4][0-9]{3}|65[0-4][0-9]{2}|655[0-2][0-9]|6553[0-5])$/,
} as const;
  