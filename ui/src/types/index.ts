// 系统状态类型定义
export interface SystemStatus {
  keyStatus: 'idle' | 'extracting' | 'success' | 'error';
  decryptStatus: 'idle' | 'processing' | 'success' | 'error';
  serviceStatus: 'stopped' | 'starting' | 'running' | 'error';
  connectionStatus: 'disconnected' | 'connecting' | 'connected' | 'error';
}

// 系统统计数据类型
export interface SystemStats {
  activeConnections: number;
  processedTasks: number;
  serviceUptime: number;
  errorCount: number;
}

// 状态指示器类型
export type StatusType = 'success' | 'warning' | 'error' | 'info' | 'idle';

// 按钮变体类型
export type ButtonVariant = 'primary' | 'secondary' | 'danger' | 'ghost' | 'outline';

// 按钮尺寸类型
export type ButtonSize = 'sm' | 'md' | 'lg';

// 卡片属性类型
export interface CardProps extends React.HTMLAttributes<HTMLDivElement> {
  title?: string;
  subtitle?: string;
  actions?: React.ReactNode;
  padding?: 'sm' | 'md' | 'lg';
  shadow?: 'sm' | 'md' | 'lg';
  children: React.ReactNode;
}

// 统计卡片属性类型
export interface StatCardProps extends React.HTMLAttributes<HTMLDivElement> {
  title: string;
  value: string | number;
  change?: {
    value: number;
    type: 'increase' | 'decrease';
    period: string;
  };
  icon?: string;
  color?: 'blue' | 'green' | 'orange' | 'purple';
  loading?: boolean;
  chart?: React.ReactNode;
}

// 状态指示器属性类型
export interface StatusIndicatorProps extends React.HTMLAttributes<HTMLDivElement> {
  status: StatusType;
  text: string;
  size?: 'sm' | 'md' | 'lg';
  showIcon?: boolean;
  showDot?: boolean;
}

// 导航项类型
export interface NavigationItem {
  path: string;
  icon: string;
  label: string;
  key: string;
}

// 欢迎卡片属性类型
export interface WelcomeCardProps extends React.HTMLAttributes<HTMLDivElement> {
  userName?: string;
  title: string;
  description: string;
  actionText: string;
  onAction: () => void;
  illustration?: React.ReactNode;
}

// 输入框属性类型
export interface InputProps extends Omit<React.InputHTMLAttributes<HTMLInputElement>, 'size' | 'disabled'> {
  type?: 'text' | 'email' | 'password' | 'number';
  size?: 'sm' | 'md' | 'lg';
  state?: 'default' | 'error' | 'success';
  icon?: React.ReactNode;
  label?: string;
  helperText?: string;
  errorMessage?: string;
  isDisabled?: boolean;
}

// 模态框属性类型
export interface ModalProps extends React.HTMLAttributes<HTMLDivElement> {
  isOpen: boolean;
  onClose: () => void;
  title?: string;
  size?: 'sm' | 'md' | 'lg' | 'xl';
  closable?: boolean;
  children: React.ReactNode;
}

// 主题类型
export type Theme = 'light' | 'dark' | 'system';

// 颜色规模类型
export interface ColorScale {
  50: string;
  100: string;
  200: string;
  300: string;
  400: string;
  500: string;
  600: string;
  700: string;
  800: string;
  900: string;
}

// 主题配置类型
export interface ThemeConfig {
  name: string;
  colors: {
    primary: ColorScale;
    gray: ColorScale;
    success: ColorScale;
    warning: ColorScale;
    error: ColorScale;
    info: ColorScale;
  };
  spacing: Record<string, string>;
  typography: Record<string, string>;
  shadows: Record<string, string>;
  borderRadius: Record<string, string>;
}

// API响应类型
export interface ApiResponse<T = any> {
  success: boolean;
  data?: T;
  error?: string;
  message?: string;
}

// 日志条目类型
export interface LogEntry {
  id: string;
  timestamp: string;
  level: 'info' | 'warn' | 'error' | 'debug';
  message: string;
  source?: string;
}

// 微信进程信息类型
export interface WeChatProcessInfo {
  pid: number;
  name: string;
  version: string;
  path: string;
  isRunning: boolean;
}

// 数据源配置类型
export interface DataSourceConfig {
  wechatPath: string;
  databasePath: string;
  outputPath: string;
  keyFile?: string;
}

// 解密任务类型
export interface DecryptTask {
  id: string;
  name: string;
  status: 'pending' | 'running' | 'completed' | 'failed';
  progress: number;
  startTime?: string;
  endTime?: string;
  error?: string;
}

// 服务配置类型
export interface ServiceConfig {
  port: number;
  host: string;
  enableCors: boolean;
  maxConnections: number;
}

// 系统信息类型
export interface SystemInfo {
  os: string;
  arch: string;
  version: string;
  memory: {
    total: number;
    used: number;
    free: number;
  };
  cpu: {
    cores: number;
    usage: number;
  };
}

// 错误类型
export interface AppError {
  code: string;
  message: string;
  details?: any;
  timestamp: string;
}

// 通知类型
export interface Notification {
  id: string;
  type: 'success' | 'warning' | 'error' | 'info';
  title: string;
  message: string;
  timestamp: string;
  read: boolean;
}

// 用户设置类型
export interface UserSettings {
  theme: Theme;
  language: 'zh-CN' | 'en-US';
  autoStart: boolean;
  notifications: boolean;
  logLevel: 'debug' | 'info' | 'warn' | 'error';
}

// 图表数据类型
export interface ChartDataPoint {
  name: string;
  value: number;
  timestamp?: string;
}

// 性能指标类型
export interface PerformanceMetrics {
  responseTime: number;
  throughput: number;
  errorRate: number;
  memoryUsage: number;
  cpuUsage: number;
}