# 组件库和设计系统规范

**文档版本**: 1.0  
**创建日期**: 2025-07-21  
**文档范围**: 微信数据采集工具现代化UI组件库和设计系统规范  

## 设计系统概述

本设计系统为微信数据采集工具的现代化UI重构提供统一的设计语言、组件规范和实现标准，确保整个应用的视觉一致性和用户体验质量。

## 设计原则

### 1. 一致性 (Consistency)
- 视觉元素保持统一风格
- 交互行为符合用户预期
- 信息架构逻辑清晰

### 2. 可用性 (Usability)
- 界面直观易懂
- 操作流程简洁高效
- 错误处理友好

### 3. 可访问性 (Accessibility)
- 支持键盘导航
- 符合WCAG 2.1标准
- 提供屏幕阅读器支持

### 4. 响应性 (Responsiveness)
- 适配不同屏幕尺寸
- 支持触摸和鼠标交互
- 性能优化

## 色彩系统

### 主色调 (Primary Colors)
```css
/* 蓝色系 - 主品牌色 */
--primary-50: #eff6ff;   /* 极浅蓝 */
--primary-100: #dbeafe;  /* 浅蓝 */
--primary-200: #bfdbfe;  /* 较浅蓝 */
--primary-300: #93c5fd;  /* 中浅蓝 */
--primary-400: #60a5fa;  /* 中蓝 */
--primary-500: #3b82f6;  /* 标准蓝 - 主色 */
--primary-600: #2563eb;  /* 较深蓝 */
--primary-700: #1d4ed8;  /* 深蓝 */
--primary-800: #1e40af;  /* 很深蓝 */
--primary-900: #1e3a8a;  /* 极深蓝 */
```

### 功能色彩 (Functional Colors)
```css
/* 成功色 - 绿色系 */
--success-50: #ecfdf5;
--success-500: #10b981;  /* 主成功色 */
--success-700: #047857;

/* 警告色 - 黄色系 */
--warning-50: #fffbeb;
--warning-500: #f59e0b;  /* 主警告色 */
--warning-700: #b45309;

/* 错误色 - 红色系 */
--error-50: #fef2f2;
--error-500: #ef4444;    /* 主错误色 */
--error-700: #b91c1c;

/* 信息色 - 青色系 */
--info-50: #ecfeff;
--info-500: #06b6d4;     /* 主信息色 */
--info-700: #0e7490;
```

### 中性色 (Neutral Colors)
```css
/* 灰色系 */
--gray-50: #f9fafb;      /* 背景色 */
--gray-100: #f3f4f6;     /* 浅灰背景 */
--gray-200: #e5e7eb;     /* 边框色 */
--gray-300: #d1d5db;     /* 分割线 */
--gray-400: #9ca3af;     /* 占位符 */
--gray-500: #6b7280;     /* 辅助文字 */
--gray-600: #4b5563;     /* 次要文字 */
--gray-700: #374151;     /* 主要文字 */
--gray-800: #1f2937;     /* 标题文字 */
--gray-900: #111827;     /* 强调文字 */

/* 纯色 */
--white: #ffffff;
--black: #000000;
```

### 色彩使用规范
- **主色调**: 用于品牌标识、主要操作按钮、链接
- **成功色**: 用于成功状态、确认操作、正向数据
- **警告色**: 用于警告信息、需要注意的状态
- **错误色**: 用于错误状态、危险操作、负向数据
- **信息色**: 用于信息提示、进行中状态
- **中性色**: 用于文字、背景、边框、图标

## 字体系统

### 字体族 (Font Family)
```css
/* 主字体栈 */
font-family: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', 
             'PingFang SC', 'Hiragino Sans GB', 'Microsoft YaHei', 
             '微软雅黑', sans-serif;

/* 等宽字体 */
font-family: 'JetBrains Mono', 'Fira Code', 'Consolas', 
             'Monaco', monospace;
```

### 字体大小 (Font Sizes)
```css
--text-xs: 0.75rem;      /* 12px - 辅助信息 */
--text-sm: 0.875rem;     /* 14px - 小号文字 */
--text-base: 1rem;       /* 16px - 正文 */
--text-lg: 1.125rem;     /* 18px - 大号文字 */
--text-xl: 1.25rem;      /* 20px - 小标题 */
--text-2xl: 1.5rem;      /* 24px - 中标题 */
--text-3xl: 1.875rem;    /* 30px - 大标题 */
--text-4xl: 2.25rem;     /* 36px - 特大标题 */
```

### 字体权重 (Font Weights)
```css
--font-light: 300;       /* 细体 */
--font-normal: 400;      /* 常规 */
--font-medium: 500;      /* 中等 */
--font-semibold: 600;    /* 半粗 */
--font-bold: 700;        /* 粗体 */
```

### 行高 (Line Heights)
```css
--leading-tight: 1.25;   /* 紧凑行高 */
--leading-normal: 1.5;   /* 正常行高 */
--leading-relaxed: 1.75; /* 宽松行高 */
```

## 间距系统

### 间距单位 (Spacing Scale)
基于 4px 网格系统：

```css
--space-0: 0;            /* 0px */
--space-1: 0.25rem;      /* 4px */
--space-2: 0.5rem;       /* 8px */
--space-3: 0.75rem;      /* 12px */
--space-4: 1rem;         /* 16px */
--space-5: 1.25rem;      /* 20px */
--space-6: 1.5rem;       /* 24px */
--space-8: 2rem
;         /* 32px */
--space-10: 2.5rem;      /* 40px */
--space-12: 3rem;        /* 48px */
--space-16: 4rem;        /* 64px */
--space-20: 5rem;        /* 80px */
--space-24: 6rem;        /* 96px */
--space-32: 8rem;        /* 128px */
```

### 间距使用规范
- **组件内间距**: 使用 space-1 到 space-6
- **组件间间距**: 使用 space-4 到 space-8
- **布局间距**: 使用 space-8 到 space-16
- **页面间距**: 使用 space-12 到 space-24

## 圆角系统

### 圆角规格 (Border Radius)
```css
--radius-none: 0;        /* 无圆角 */
--radius-sm: 0.125rem;   /* 2px - 小圆角 */
--radius-md: 0.375rem;   /* 6px - 中圆角 */
--radius-lg: 0.5rem;     /* 8px - 大圆角 */
--radius-xl: 0.75rem;    /* 12px - 特大圆角 */
--radius-2xl: 1rem;      /* 16px - 超大圆角 */
--radius-full: 9999px;   /* 完全圆角 */
```

### 圆角使用规范
- **按钮**: radius-md (6px)
- **卡片**: radius-lg (8px)
- **输入框**: radius-md (6px)
- **头像**: radius-full
- **徽章**: radius-full

## 阴影系统

### 阴影规格 (Box Shadow)
```css
--shadow-sm: 0 1px 2px 0 rgba(0, 0, 0, 0.05);
--shadow-md: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
--shadow-lg: 0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05);
--shadow-xl: 0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04);
--shadow-2xl: 0 25px 50px -12px rgba(0, 0, 0, 0.25);
```

### 阴影使用规范
- **卡片**: shadow-sm (默认) / shadow-md (悬停)
- **模态框**: shadow-xl
- **下拉菜单**: shadow-lg
- **浮动按钮**: shadow-md

## 基础组件规范

### 1. Button 组件

#### 变体 (Variants)
```typescript
interface ButtonProps {
  variant: 'primary' | 'secondary' | 'danger' | 'ghost' | 'outline';
  size: 'sm' | 'md' | 'lg';
  loading?: boolean;
  disabled?: boolean;
  icon?: React.ReactNode;
  children: React.ReactNode;
}
```

#### 样式规范
```css
/* Primary Button */
.btn-primary {
  background-color: var(--primary-500);
  color: var(--white);
  border: 1px solid var(--primary-500);
}

.btn-primary:hover {
  background-color: var(--primary-600);
  border-color: var(--primary-600);
}

.btn-primary:active {
  background-color: var(--primary-700);
  border-color: var(--primary-700);
}

/* Secondary Button */
.btn-secondary {
  background-color: var(--gray-100);
  color: var(--gray-700);
  border: 1px solid var(--gray-200);
}

/* Danger Button */
.btn-danger {
  background-color: var(--error-500);
  color: var(--white);
  border: 1px solid var(--error-500);
}

/* 尺寸规范 */
.btn-sm {
  padding: var(--space-2) var(--space-3);
  font-size: var(--text-sm);
  border-radius: var(--radius-md);
}

.btn-md {
  padding: var(--space-3) var(--space-4);
  font-size: var(--text-base);
  border-radius: var(--radius-md);
}

.btn-lg {
  padding: var(--space-4) var(--space-6);
  font-size: var(--text-lg);
  border-radius: var(--radius-lg);
}
```

### 2. Card 组件

#### 接口定义
```typescript
interface CardProps {
  title?: string;
  subtitle?: string;
  actions?: React.ReactNode;
  padding?: 'sm' | 'md' | 'lg';
  shadow?: 'sm' | 'md' | 'lg';
  children: React.ReactNode;
}
```

#### 样式规范
```css
.card {
  background-color: var(--white);
  border: 1px solid var(--gray-200);
  border-radius: var(--radius-lg);
  box-shadow: var(--shadow-sm);
  transition: box-shadow 0.2s ease;
}

.card:hover {
  box-shadow: var(--shadow-md);
}

.card-header {
  padding: var(--space-6);
  border-bottom: 1px solid var(--gray-200);
}

.card-body {
  padding: var(--space-6);
}

.card-footer {
  padding: var(--space-6);
  border-top: 1px solid var(--gray-200);
  background-color: var(--gray-50);
}
```

### 3. Input 组件

#### 接口定义
```typescript
interface InputProps {
  type?: 'text' | 'email' | 'password' | 'number';
  size?: 'sm' | 'md' | 'lg';
  state?: 'default' | 'error' | 'success';
  placeholder?: string;
  disabled?: boolean;
  icon?: React.ReactNode;
  label?: string;
  helperText?: string;
  errorMessage?: string;
}
```

#### 样式规范
```css
.input {
  width: 100%;
  border: 1px solid var(--gray-300);
  border-radius: var(--radius-md);
  background-color: var(--white);
  transition: border-color 0.2s ease, box-shadow 0.2s ease;
}

.input:focus {
  outline: none;
  border-color: var(--primary-500);
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

.input-error {
  border-color: var(--error-500);
}

.input-success {
  border-color: var(--success-500);
}

/* 尺寸规范 */
.input-sm {
  padding: var(--space-2) var(--space-3);
  font-size: var(--text-sm);
}

.input-md {
  padding: var(--space-3) var(--space-4);
  font-size: var(--text-base);
}

.input-lg {
  padding: var(--space-4) var(--space-5);
  font-size: var(--text-lg);
}
```

### 4. Modal 组件

#### 接口定义
```typescript
interface ModalProps {
  isOpen: boolean;
  onClose: () => void;
  title?: string;
  size?: 'sm' | 'md' | 'lg' | 'xl';
  closable?: boolean;
  children: React.ReactNode;
}
```

#### 样式规范
```css
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal {
  background-color: var(--white);
  border-radius: var(--radius-lg);
  box-shadow: var(--shadow-2xl);
  max-height: 90vh;
  overflow-y: auto;
}

.modal-sm { max-width: 400px; }
.modal-md { max-width: 500px; }
.modal-lg { max-width: 800px; }
.modal-xl { max-width: 1200px; }
```

## 业务组件规范

### 1. StatCard 统计卡片

#### 设计规范
- **用途**: 展示关键数据指标
- **布局**: 图标 + 数据 + 变化趋势
- **尺寸**: 固定高度，响应式宽度
- **状态**: 支持加载状态

#### 接口定义
```typescript
interface StatCardProps {
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
```

#### 视觉规范
```css
.stat-card {
  background: var(--white);
  border: 1px solid var(--gray-200);
  border-radius: var(--radius-xl);
  padding: var(--space-6);
  box-shadow: var(--shadow-sm);
  transition: all 0.2s ease;
}

.stat-card:hover {
  box-shadow: var(--shadow-md);
  transform: translateY(-1px);
}

.stat-value {
  font-size: var(--text-3xl);
  font-weight: var(--font-bold);
  color: var(--gray-900);
  margin: var(--space-2) 0;
}

.stat-change {
  display: flex;
  align-items: center;
  font-size: var(--text-sm);
  gap: var(--space-1);
}

.stat-change.positive {
  color: var(--success-600);
}

.stat-change.negative {
  color: var(--error-600);
}
```

### 2. StatusIndicator 状态指示器

#### 设计规范
- **用途**: 显示系统或操作状态
- **类型**: 成功、警告、错误、信息
- **形式**: 圆点 + 文字 或 图标 + 文字

#### 接口定义
```typescript
interface StatusIndicatorProps {
  status: 'success' | 'warning' | 'error' | 'info' | 'idle';
  text: string;
  size?: 'sm' | 'md' | 'lg';
  showIcon?: boolean;
  showDot?: boolean;
}
```

#### 视觉规范
```css
.status-indicator {
  display: inline-flex;
  align-items: center;
  gap: var(--space-2);
  font-size: var(--text-sm);
  font-weight: var(--font-medium);
}

.status-dot {
  width: 8px;
  height: 8px;
  border-radius: var(--radius-full);
  flex-shrink: 0;
}

.status-success .status-dot {
  background-color: var(--success-500);
}

.status-warning .status-dot {
  background-color: var(--warning-500);
}

.status-error .status-dot {
  background-color: var(--error-500);
}

.status-info .status-dot {
  background-color: var(--info-500);
}
```

### 3. WelcomeCard 欢迎卡片

#### 设计规范
- **用途**: 首页欢迎信息和快速操作
- **布局**: 渐变背景 + 插图 + 文字 + 按钮
- **风格**: 现代化卡片设计

#### 接口定义
```typescript
interface WelcomeCardProps {
  userName?: string;
  title: string;
  description: string;
  actionText: string;
  onAction: () => void;
  illustration?: React.ReactNode;
}
```

#### 视觉规范
```css
.welcome-card {
  background: linear-gradient(135deg, var(--primary-500) 0%, var(--primary-700) 100%);
  border-radius: var(--radius-xl);
  padding: var(--space-8);
  color: var(--white);
  position: relative;
  overflow: hidden;
}

.welcome-content {
  position: relative;
  z-index: 2;
}

.welcome-title {
  font-size: var(--text-2xl);
  font-weight: var(--font-bold);
  margin-bottom: var(--space-2);
}

.welcome-description {
  font-size: var(--text-base);
  opacity: 0.9;
  margin-bottom: var(--space-6);
}

.welcome-action {
  background: rgba(255, 255, 255, 0.2);
  border: 1px solid rgba(255, 255, 255, 0.3);
  color: var(--white);
  backdrop-filter: blur(10px);
}
```

## 图标系统

### 图标规范
- **图标库**: FontAwesome 6.5.0
- **风格**: 统一使用 Solid 风格
- **尺寸**: 16px, 20px, 24px, 32px
- **颜色**: 继承父元素颜色

### 常用图标映射
```typescript
const iconMap = {
  // 导航图标
  home: 'fas fa-home',
  dashboard: 'fas fa-tachometer-alt',
  settings: 'fas fa-cog',
  
  // 功能图标
  key: 'fas fa-key',
  lock: 'fas fa-lock',
  unlock: 'fas fa-unlock',
  database: 'fas fa-database',
  server: 'fas fa-server',
  chart: 'fas fa-chart-line',
  
  // 状态图标
  success: 'fas fa-check-circle',
  warning: 'fas fa-exclamation-triangle',
  error: 'fas fa-times-circle
',
  info: 'fas fa-info-circle',
  loading: 'fas fa-spinner fa-spin',
  
  // 操作图标
  edit: 'fas fa-edit',
  delete: 'fas fa-trash',
  save: 'fas fa-save',
  cancel: 'fas fa-times',
  refresh: 'fas fa-sync-alt',
  
  // 方向图标
  up: 'fas fa-chevron-up',
  down: 'fas fa-chevron-down',
  left: 'fas fa-chevron-left',
  right: 'fas fa-chevron-right',
};
```

## 动画系统

### 动画原则
- **有意义**: 动画应该有明确的目的
- **快速**: 动画时长控制在 200-500ms
- **自然**: 使用缓动函数模拟自然运动
- **一致**: 相同类型的动画保持一致

### 缓动函数 (Easing Functions)
```css
/* 标准缓动 */
--ease-in: cubic-bezier(0.4, 0, 1, 1);
--ease-out: cubic-bezier(0, 0, 0.2, 1);
--ease-in-out: cubic-bezier(0.4, 0, 0.2, 1);

/* 自定义缓动 */
--ease-spring: cubic-bezier(0.68, -0.55, 0.265, 1.55);
--ease-smooth: cubic-bezier(0.25, 0.46, 0.45, 0.94);
```

### 动画时长 (Duration)
```css
--duration-fast: 150ms;     /* 快速动画 */
--duration-normal: 250ms;   /* 标准动画 */
--duration-slow: 350ms;     /* 慢速动画 */
--duration-slower: 500ms;   /* 更慢动画 */
```

### 常用动画类
```css
/* 淡入淡出 */
.fade-enter {
  opacity: 0;
  transform: translateY(10px);
}

.fade-enter-active {
  opacity: 1;
  transform: translateY(0);
  transition: opacity var(--duration-normal) var(--ease-out),
              transform var(--duration-normal) var(--ease-out);
}

/* 滑动 */
.slide-up {
  transform: translateY(100%);
  transition: transform var(--duration-normal) var(--ease-out);
}

.slide-up.active {
  transform: translateY(0);
}

/* 缩放 */
.scale-enter {
  opacity: 0;
  transform: scale(0.95);
}

.scale-enter-active {
  opacity: 1;
  transform: scale(1);
  transition: opacity var(--duration-fast) var(--ease-out),
              transform var(--duration-fast) var(--ease-out);
}

/* 悬停效果 */
.hover-lift {
  transition: transform var(--duration-fast) var(--ease-out),
              box-shadow var(--duration-fast) var(--ease-out);
}

.hover-lift:hover {
  transform: translateY(-2px);
  box-shadow: var(--shadow-lg);
}
```

## 响应式设计

### 断点系统 (Breakpoints)
```css
/* 移动设备优先 */
/* xs: 0px - 默认 */
@media (min-width: 640px) { /* sm */ }
@media (min-width: 768px) { /* md */ }
@media (min-width: 1024px) { /* lg */ }
@media (min-width: 1280px) { /* xl */ }
@media (min-width: 1536px) { /* 2xl */ }
```

### 网格系统
```css
.container {
  width: 100%;
  margin: 0 auto;
  padding: 0 var(--space-4);
}

@media (min-width: 640px) {
  .container { max-width: 640px; }
}

@media (min-width: 768px) {
  .container { max-width: 768px; }
}

@media (min-width: 1024px) {
  .container { max-width: 1024px; }
}

@media (min-width: 1280px) {
  .container { max-width: 1280px; }
}
```

### 响应式工具类
```css
/* 显示/隐藏 */
.hidden { display: none; }
.block { display: block; }
.inline-block { display: inline-block; }
.flex { display: flex; }
.grid { display: grid; }

/* 响应式显示 */
@media (max-width: 639px) {
  .sm\\:hidden { display: none; }
}

@media (min-width: 640px) {
  .sm\\:block { display: block; }
}
```

## 可访问性规范

### 颜色对比度
- **正文文字**: 至少 4.5:1 对比度
- **大号文字**: 至少 3:1 对比度
- **图标和图形**: 至少 3:1 对比度

### 焦点管理
```css
/* 焦点样式 */
.focus-visible {
  outline: 2px solid var(--primary-500);
  outline-offset: 2px;
}

/* 跳过链接 */
.skip-link {
  position: absolute;
  top: -40px;
  left: 6px;
  background: var(--primary-500);
  color: var(--white);
  padding: 8px;
  text-decoration: none;
  border-radius: var(--radius-md);
  z-index: 1000;
}

.skip-link:focus {
  top: 6px;
}
```

### ARIA 标签规范
```typescript
// 常用 ARIA 属性
interface AriaProps {
  'aria-label'?: string;
  'aria-labelledby'?: string;
  'aria-describedby'?: string;
  'aria-expanded'?: boolean;
  'aria-hidden'?: boolean;
  'aria-live'?: 'polite' | 'assertive';
  'aria-current'?: 'page' | 'step' | 'location';
  role?: string;
}
```

## 主题系统

### 主题结构
```typescript
interface Theme {
  name: string;
  colors: {
    primary: ColorScale;
    gray: ColorScale;
    success: ColorScale;
    warning: ColorScale;
    error: ColorScale;
    info: ColorScale;
  };
  spacing: SpacingScale;
  typography: TypographyScale;
  shadows: ShadowScale;
  borderRadius: RadiusScale;
}
```

### 主题切换实现
```typescript
// 主题上下文
const ThemeContext = createContext<{
  theme: Theme;
  setTheme: (theme: Theme) => void;
}>({
  theme: lightTheme,
  setTheme: () => {},
});

// 主题提供者
export const ThemeProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [theme, setTheme] = useState<Theme>(lightTheme);

  useEffect(() => {
    // 应用CSS变量
    Object.entries(
theme.colors).forEach(([key, value]) => {
      if (typeof value === 'object') {
        Object.entries(value).forEach(([shade, color]) => {
          document.documentElement.style.setProperty(`--${key}-${shade}`, color);
        });
      } else {
        document.documentElement.style.setProperty(`--${key}`, value);
      }
    });
  }, [theme]);

  return (
    <ThemeContext.Provider value={{ theme, setTheme }}>
      {children}
    </ThemeContext.Provider>
  );
};
```

## 组件开发规范

### 组件文件结构
```
components/
├── Button/
│   ├── Button.tsx          # 主组件
│   ├── Button.stories.tsx  # Storybook 故事
│   ├── Button.test.tsx     # 单元测试
│   ├── Button.module.css   # 样式文件
│   └── index.ts           # 导出文件
```

### 组件开发模板
```typescript
// Button.tsx
import React, { forwardRef } from 'react';
import { clsx } from 'clsx';
import styles from './Button.module.css';

export interface ButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: 'primary' | 'secondary' | 'danger' | 'ghost';
  size?: 'sm' | 'md' | 'lg';
  loading?: boolean;
  icon?: React.ReactNode;
  children: React.ReactNode;
}

export const Button = forwardRef<HTMLButtonElement, ButtonProps>(
  ({ 
    variant = 'primary', 
    size = 'md', 
    loading = false, 
    icon, 
    children, 
    className,
    disabled,
    ...props 
  }, ref) => {
    return (
      <button
        ref={ref}
        className={clsx(
          styles.button,
          styles[variant],
          styles[size],
          {
            [styles.loading]: loading,
            [styles.disabled]: disabled || loading,
          },
          className
        )}
        disabled={disabled || loading}
        {...props}
      >
        {loading && <i className="fas fa-spinner fa-spin" />}
        {!loading && icon && <span className={styles.icon}>{icon}</span>}
        <span className={styles.text}>{children}</span>
      </button>
    );
  }
);

Button.displayName = 'Button';
```

### 组件测试规范
```typescript
// Button.test.tsx
import { render, screen, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import { Button } from './Button';

describe('Button', () => {
  it('renders correctly', () => {
    render(<Button>Click me</Button>);
    expect(screen.getByRole('button')).toBeInTheDocument();
    expect(screen.getByText('Click me')).toBeInTheDocument();
  });

  it('handles click events', () => {
    const handleClick = vi.fn();
    render(<Button onClick={handleClick}>Click me</Button>);
    
    fireEvent.click(screen.getByRole('button'));
    expect(handleClick).toHaveBeenCalledTimes(1);
  });

  it('shows loading state', () => {
    render(<Button loading>Loading</Button>);
    
    expect(screen.getByRole('button')).toBeDisabled();
    expect(screen.getByText('Loading')).toBeInTheDocument();
  });

  it('applies correct variant classes', () => {
    render(<Button variant="danger">Delete</Button>);
    
    expect(screen.getByRole('button')).toHaveClass('danger');
  });
});
```

## 质量保证

### 代码质量检查
```json
// .eslintrc.json
{
  "extends": [
    "eslint:recommended",
    "@typescript-eslint/recommended",
    "plugin:react/recommended",
    "plugin:react-hooks/recommended",
    "plugin:jsx-a11y/recommended"
  ],
  "rules": {
    "react/prop-types": "off",
    "react/react-in-jsx-scope": "off",
    "@typescript-eslint/no-unused-vars": "error",
    "jsx-a11y/no-autofocus": "off"
  }
}
```

### 样式规范检查
```json
// stylelint.config.js
module.exports = {
  extends: [
    'stylelint-config-standard',
    'stylelint-config-css-modules'
  ],
  rules: {
    'selector-class-pattern': '^[a-z][a-zA-Z0-9]*$',
    'custom-property-pattern': '^[a-z][a-zA-Z0-9]*(-[a-zA-Z0-9]+)*$',
    'declaration-block-trailing-semicolon': 'always',
    'string-quotes': 'single'
  }
};
```

### 性能监控
```typescript
// 组件性能监控
const ComponentPerformanceMonitor: React.FC<{ name: string; children: React.ReactNode }> = ({ 
  name, 
  children 
}) => {
  useEffect(() => {
    const startTime = performance.now();
    
    return () => {
      const endTime = performance.now();
      const renderTime = endTime - startTime;
      
      if (renderTime > 16) { // 超过一帧的时间
        console.warn(`Component ${name} render time: ${renderTime}ms`);
      }
    };
  });

  return <>{children}</>;
};
```

## 文档和故事书

### Storybook 配置
```typescript
// Button.stories.tsx
import type { Meta, StoryObj } from '@storybook/react';
import { Button } from './Button';

const meta: Meta<typeof Button> = {
  title: 'Components/Button',
  component: Button,
  parameters: {
    layout: 'centered',
  },
  tags: ['autodocs'],
  argTypes: {
    variant: {
      control: { type: 'select' },
      options: ['primary', 'secondary', 'danger', 'ghost'],
    },
    size: {
      control: { type: 'select' },
      options: ['sm', 'md', 'lg'],
    },
  },
};

export default meta;
type Story = StoryObj<typeof meta>;

export const Primary: Story = {
  args: {
    variant: 'primary',
    children: 'Button',
  },
};

export const Secondary: Story = {
  args: {
    variant: 'secondary',
    children: 'Button',
  },
};

export const Loading: Story = {
  args: {
    loading: true,
    children: 'Loading...',
  },
};

export const WithIcon: Story = {
  args: {
    icon: <i className="fas fa-plus" />,
    children: 'Add Item',
  },
};
```

## 维护和更新

### 版本管理
- 使用语义化版本控制 (Semantic Versioning)
- 主版本号：破坏性变更
- 次版本号：新功能添加
- 修订号：bug 修复

### 变更日志
```markdown
# Changelog

## [1.2.0] - 2025-07-21

### Added
- 新增 StatCard 组件
- 添加深色主题支持
- 增加动画系统

### Changed
- 优化 Button 组件性能
- 更
新 Modal 组件样式

### Fixed
- 修复 Input 组件焦点样式问题
- 解决响应式布局在小屏幕的显示问题

### Deprecated
- Card 组件的 `shadow` 属性将在下个版本移除

## [1.1.0] - 2025-07-15

### Added
- 新增 Card 组件
- 添加响应式网格系统
- 增加可访问性支持

### Changed
- 优化色彩系统
- 更新字体规范

### Fixed
- 修复按钮组件点击事件处理
```

### 迁移指南
当组件库有破坏性变更时，提供详细的迁移指南：

```markdown
# 迁移指南 v1.x 到 v2.x

## Button 组件变更

### 属性变更
- `type` 属性重命名为 `variant`
- 移除 `outline` 属性，使用 `variant="outline"`

### 迁移示例
```typescript
// v1.x
<Button type="primary" outline>Click me</Button>

// v2.x
<Button variant="outline">Click me</Button>
```

## 样式变更
- CSS 变量前缀从 `--color-` 改为 `--`
- 间距单位从 `px` 改为 `rem`

### 自动迁移脚本
```bash
# 运行迁移脚本
npm run migrate:v2
```
```

## 总结

本组件库和设计系统规范文档为微信数据采集工具的现代化UI重构提供了完整的设计指导，包括：

### 核心设计系统
1. **色彩系统**: 完整的色彩规范和使用指南
2. **字体系统**: 统一的字体规范和层次结构
3. **间距系统**: 基于4px网格的间距规范
4. **圆角和阴影**: 统一的视觉效果规范

### 组件规范
1. **基础组件**: Button、Card、Input、Modal等通用组件
2. **业务组件**: StatCard、StatusIndicator、WelcomeCard等专用组件
3. **图标系统**: FontAwesome图标库使用规范
4. **动画系统**: 统一的动画效果和时长规范

### 开发规范
1. **组件开发**: 标准化的组件开发流程和模板
2. **测试规范**: 完整的单元测试和集成测试要求
3. **文档规范**: Storybook和文档维护标准
4. **质量保证**: 代码质量检查和性能监控

### 可访问性和响应式
1. **可访问性**: WCAG 2.1标准支持和实现指南
2. **响应式设计**: 完整的断点系统和适配策略
3. **主题系统**: 支持多主题切换的架构设计

通过遵循本设计系统规范，开发团队可以构建出视觉一致、用户体验优秀、可维护性强的现代化用户界面。

---

**下一步行动**:
1. 基于本规范实现基础组件库
2. 创建 Storybook 文档站点
3. 建立组件测试和质量保证流程
4. 逐步应用到实际页面开发中

**文档维护**: 本文档将随着组件库的发展持续更新和完善。