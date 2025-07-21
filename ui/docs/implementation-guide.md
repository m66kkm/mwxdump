# 详细实施指南

**文档版本**: 1.0  
**创建日期**: 2025-07-21  
**文档范围**: 微信数据采集工具现代化UI重构详细实施指南  

## 指南概述

本文档为开发团队提供微信数据采集工具现代化UI重构的详细实施指南，包括环境搭建、开发流程、代码规范、测试策略等具体操作指导。

## 快速开始

### 环境要求
- **Node.js**: >= 18.0.0
- **Yarn**: >= 1.22.0 (推荐) 或 npm >= 8.0.0
- **Rust**: >= 1.70.0
- **操作系统**: Windows 10+, macOS 10.15+, Linux (Ubuntu 20.04+)

### 项目初始化
```bash
# 1. 克隆项目
git clone <repository-url>
cd mwxdump/ui

# 2. 安装依赖
yarn install

# 3. 启动开发服务器
yarn tauri:dev

# 4. 运行测试
yarn test

# 5. 构建生产版本
yarn tauri:build
```

## 开发环境配置

### 1. IDE配置

#### VSCode推荐配置
```json
// .vscode/settings.json
{
  "typescript.preferences.importModuleSpecifier": "relative",
  "editor.formatOnSave": true,
  "editor.codeActionsOnSave": {
    "source.fixAll.eslint": true
  },
  "emmet.includeLanguages": {
    "typescript": "html",
    "typescriptreact": "html"
  },
  "tailwindCSS.includeLanguages": {
    "typescript": "html",
    "typescriptreact": "html"
  }
}
```

#### 推荐扩展
```json
// .vscode/extensions.json
{
  "recommendations": [
    "bradlc.vscode-tailwindcss",
    "esbenp.prettier-vscode",
    "dbaeumer.vscode-eslint",
    "ms-vscode.vscode-typescript-next",
    "tauri-apps.tauri-vscode",
    "rust-lang.rust-analyzer"
  ]
}
```

### 2. Git配置

#### Git Hooks设置
```bash
# 安装husky
yarn add --dev husky

# 设置pre-commit钩子
npx husky add .husky/pre-commit "yarn lint-staged"

# 设置commit-msg钩子
npx husky add .husky/commit-msg "yarn commitlint --edit $1"
```

#### 提交信息规范
```bash
# 提交信息格式
<type>(<scope>): <subject>

# 示例
feat(components): 添加StatCard组件
fix(layout): 修复侧边栏响应式问题
docs(readme): 更新安装说明
style(button): 统一按钮样式规范
refactor(store): 重构状态管理逻辑
test(utils): 添加工具函数单元测试
```

## 项目结构详解

### 目录结构
```
ui/
├── public/                 # 静态资源
├── src/                    # 源代码
│   ├── components/         # 组件
│   │   ├── ui/            # 基础UI组件
│   │   ├── layout/        # 布局组件
│   │   ├── features/      # 功能组件
│   │   └── charts/        # 图表组件
│   ├── pages/             # 页面组件
│   ├── hooks/             # 自定义Hook
│   ├── store/             # 状态管理
│   ├── utils/             # 工具函数
│   ├── types/             # 类型定义
│   ├── styles/            # 样式文件
│   └── __tests__/         # 测试文件
├── src-tauri/             # Tauri后端
├── docs/                  # 项目文档
├── .storybook/            # Storybook配置
└── dist/                  # 构建输出
```

### 文件命名规范
- **组件文件**: PascalCase (例: `StatCard.tsx`)
- **页面文件**: PascalCase (例: `Overview.tsx`)
- **工具文件**: camelCase (例: `apiClient.ts`)
- **类型文件**: camelCase (例: `userTypes.ts`)
- **样式文件**: kebab-case (例: `global-styles.css`)

## 组件开发指南

### 1. 组件开发流程

#### 步骤1: 创建组件目录
```bash
# 创建组件目录结构
mkdir -p src/components/ui/Button
cd src/components/ui/Button

# 创建必要文件
touch Button.tsx
touch Button.stories.tsx
touch Button.test.tsx
touch Button.module.css
touch index.ts
```

#### 步骤2: 实现组件
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
        {loading && <i className="fas fa-spinner fa-spin mr-2" />}
        {!loading && icon && <span className="mr-2">{icon}</span>}
        {children}
      </button>
    );
  }
);

Button.displayName = 'Button';
```

#### 步骤3: 编写样式
```css
/* Button.module.css */
.button {
  @apply inline-flex items-center justify-center font-medium rounded-md 
         transition-colors focus:outline-none focus:ring-2 focus:ring-offset-2;
}

.primary {
  @apply bg-primary-500 text-white hover:bg-primary-600 
         focus:ring-primary-500;
}

.secondary {
  @apply bg-gray-100 text-gray-700 hover:bg-gray-200 
         focus:ring-gray-500;
}

.danger {
  @apply bg-red-500 text-white hover:bg-red-600 
         focus:ring-red-500;
}

.ghost {
  @apply bg-transparent text-gray-700 hover:bg-gray-100 
         focus:ring-gray-500;
}

.sm {
  @apply px-3 py-2 text-sm;
}

.
md {
  @apply px-4 py-3 text-base;
}

.lg {
  @apply px-6 py-4 text-lg;
}

.loading {
  @apply opacity-75 cursor-not-allowed;
}

.disabled {
  @apply opacity-50 cursor-not-allowed;
}
```

#### 步骤4: 编写测试
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
    expect(screen.getByRole('button')).toHaveClass('loading');
  });

  it('applies correct variant classes', () => {
    render(<Button variant="danger">Delete</Button>);
    expect(screen.getByRole('button')).toHaveClass('danger');
  });

  it('forwards ref correctly', () => {
    const ref = React.createRef<HTMLButtonElement>();
    render(<Button ref={ref}>Button</Button>);
    expect(ref.current).toBeInstanceOf(HTMLButtonElement);
  });
});
```

#### 步骤5: 创建Storybook故事
```typescript
// Button.stories.tsx
import type { Meta, StoryObj } from '@storybook/react';
import { Button } from './Button';

const meta: Meta<typeof Button> = {
  title: 'UI/Button',
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
    loading: {
      control: { type: 'boolean' },
    },
    disabled: {
      control: { type: 'boolean' },
    },
  },
};

export default meta;
type Story = StoryObj<typeof meta>;

export const Primary: Story = {
  args: {
    variant: 'primary',
    children: 'Primary Button',
  },
};

export const Secondary: Story = {
  args: {
    variant: 'secondary',
    children: 'Secondary Button',
  },
};

export const Danger: Story = {
  args: {
    variant: 'danger',
    children: 'Danger Button',
  },
};

export const WithIcon: Story = {
  args: {
    icon: <i className="fas fa-plus" />,
    children: 'Add Item',
  },
};

export const Loading: Story = {
  args: {
    loading: true,
    children: 'Loading...',
  },
};

export const AllSizes: Story = {
  render: () => (
    <div className="flex items-center gap-4">
      <Button size="sm">Small</Button>
      <Button size="md">Medium</Button>
      <Button size="lg">Large</Button>
    </div>
  ),
};
```

#### 步骤6: 导出组件
```typescript
// index.ts
export { Button } from './Button';
export type { ButtonProps } from './Button';
```

### 2. 组件开发最佳实践

#### TypeScript类型定义
```typescript
// 扩展HTML元素属性
interface ButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: 'primary' | 'secondary';
  // 其他自定义属性
}

// 使用泛型约束
interface SelectProps<T> {
  options: T[];
  value: T;
  onChange: (value: T) => void;
  getLabel: (option: T) => string;
  getValue: (option: T) => string;
}

// 条件类型
type ButtonSize = 'sm' | 'md' | 'lg';
type ButtonVariant = 'primary' | 'secondary' | 'danger';

interface ButtonProps {
  size?: ButtonSize;
  variant?: ButtonVariant;
  loading?: boolean;
  // 当loading为true时，disabled自动为true
  disabled?: boolean;
}
```

#### 性能优化
```typescript
// 使用React.memo优化渲染
export const Button = React.memo(
  forwardRef<HTMLButtonElement, ButtonProps>(({ ...props }, ref) => {
    // 组件实现
  })
);

// 使用useMemo缓存计算结果
const StatCard: React.FC<StatCardProps> = ({ data }) => {
  const formattedValue = useMemo(() => {
    return formatNumber(data.value);
  }, [data.value]);

  return <div>{formattedValue}</div>;
};

// 使用useCallback缓存事件处理函数
const DataTable: React.FC<DataTableProps> = ({ data, onRowClick }) => {
  const handleRowClick = useCallback((row: any) => {
    onRowClick?.(row);
  }, [onRowClick]);

  return (
    <table>
      {data.map(row => (
        <tr key={row.id} onClick={() => handleRowClick(row)}>
          {/* 行内容 */}
        </tr>
      ))}
    </table>
  );
};
```

## 状态管理指南

### 1. Zustand状态管理

#### 创建Store
```typescript
// src/store/systemStore.ts
import { create } from 'zustand';
import { devtools } from 'zustand/middleware';

interface SystemState {
  // 状态定义
  status: {
    keyExtraction: 'idle' | 'loading' | 'success' | 'error';
    dataDecryption: 'idle' | 'loading' | 'success' | 'error';
    httpService: 'stopped' | 'starting' | 'running' | 'error';
  };
  
  // 数据定义
  stats: {
    activeConnections: number;
    processedTasks: number;
    errorCount: number;
  };
  
  logs: string[];
  
  // 动作定义
  updateStatus: (key: keyof SystemState['status'], value: string) => void;
  updateStats: (stats: Partial<SystemState['stats']>) => void;
  addLog: (message: string) => void;
  clearLogs: () => void;
  
  // 异步动作
  extractKey: () => Promise<void>;
  decryptData: () => Promise<void>;
  startHttpService: () => Promise<void>;
}

export const useSystemStore = create<SystemState>()(
  devtools(
    (set, get) => ({
      status: {
        keyExtraction: 'idle',
        dataDecryption: 'idle',
        httpService: 'stopped',
      },
      
      stats: {
        activeConnections
: 0,
        processedTasks: 0,
        errorCount: 0,
      },
      
      logs: [],
      
      updateStatus: (key, value) => set((state) => ({
        status: { ...state.status, [key]: value }
      })),
      
      updateStats: (newStats) => set((state) => ({
        stats: { ...state.stats, ...newStats }
      })),
      
      addLog: (message) => set((state) => ({
        logs: [...state.logs, `${new Date().toLocaleTimeString()}: ${message}`].slice(-100)
      })),
      
      clearLogs: () => set({ logs: [] }),
      
      extractKey: async () => {
        const { updateStatus, addLog } = get();
        try {
          updateStatus('keyExtraction', 'loading');
          addLog('开始提取数据密钥...');
          
          // 调用Tauri命令
          const result = await invoke('extract_key');
          
          updateStatus('keyExtraction', 'success');
          addLog('数据密钥提取成功');
          return result;
        } catch (error) {
          updateStatus('keyExtraction', 'error');
          addLog(`密钥提取失败: ${error}`);
          throw error;
        }
      },
      
      decryptData: async () => {
        const { updateStatus, addLog } = get();
        try {
          updateStatus('dataDecryption', 'loading');
          addLog('开始解密数据...');
          
          const result = await invoke('decrypt_data');
          
          updateStatus('dataDecryption', 'success');
          addLog('数据解密完成');
          return result;
        } catch (error) {
          updateStatus('dataDecryption', 'error');
          addLog(`数据解密失败: ${error}`);
          throw error;
        }
      },
      
      startHttpService: async () => {
        const { updateStatus, addLog } = get();
        try {
          updateStatus('httpService', 'starting');
          addLog('正在启动HTTP服务...');
          
          const result = await invoke('start_http_service');
          
          updateStatus('httpService', 'running');
          addLog('HTTP服务启动成功');
          return result;
        } catch (error) {
          updateStatus('httpService', 'error');
          addLog(`服务启动失败: ${error}`);
          throw error;
        }
      },
    }),
    {
      name: 'system-store',
    }
  )
);
```

#### 在组件中使用Store
```typescript
// 在组件中使用状态
const Overview: React.FC = () => {
  const { status, stats, extractKey, updateStats } = useSystemStore();
  
  const handleExtractKey = async () => {
    try {
      await extractKey();
    } catch (error) {
      console.error('Key extraction failed:', error);
    }
  };
  
  return (
    <div>
      <Button 
        onClick={handleExtractKey}
        loading={status.keyExtraction === 'loading'}
      >
        提取密钥
      </Button>
      <StatCard 
        title="活跃连接" 
        value={stats.activeConnections} 
      />
    </div>
  );
};

// 选择性订阅状态
const StatusIndicator: React.FC = () => {
  const keyStatus = useSystemStore(state => state.status.keyExtraction);
  
  return (
    <div className={`status-${keyStatus}`}>
      {keyStatus === 'loading' && '提取中...'}
      {keyStatus === 'success' && '提取成功'}
      {keyStatus === 'error' && '提取失败'}
    </div>
  );
};
```

### 2. 自定义Hook开发

#### 系统状态Hook
```typescript
// src/hooks/useSystemStatus.ts
import { useSystemStore } from '../store/systemStore';
import { useEffect } from 'react';

export const useSystemStatus = () => {
  const { status, updateStats } = useSystemStore();
  
  useEffect(() => {
    // 定期更新系统状态
    const interval = setInterval(async () => {
      try {
        const systemInfo = await invoke('get_system_info');
        updateStats(systemInfo);
      } catch (error) {
        console.error('Failed to get system info:', error);
      }
    }, 5000);
    
    return () => clearInterval(interval);
  }, [updateStats]);
  
  return status;
};
```

#### WebSocket连接Hook
```typescript
// src/hooks/useWebSocket.ts
import { useEffect, useRef, useState } from 'react';

interface UseWebSocketOptions {
  url: string;
  onMessage?: (data: any) => void;
  onError?: (error: Event) => void;
  reconnectInterval?: number;
}

export const useWebSocket = ({
  url,
  onMessage,
  onError,
  reconnectInterval = 3000
}: UseWebSocketOptions) => {
  const [isConnected, setIsConnected] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const wsRef = useRef<WebSocket | null>(null);
  const reconnectTimeoutRef = useRef<NodeJS.Timeout>();
  
  const connect = useCallback(() => {
    try {
      wsRef.current = new WebSocket(url);
      
      wsRef.current.onopen = () => {
        setIsConnected(true);
        setError(null);
      };
      
      wsRef.current.onmessage = (event) => {
        try {
          const data = JSON.parse(event.data);
          onMessage?.(data);
        } catch (err) {
          console.error('Failed to parse WebSocket message:', err);
        }
      };
      
      wsRef.current.onclose = () => {
        setIsConnected(false);
        // 自动重连
        reconnectTimeoutRef.current = setTimeout(connect, reconnectInterval);
      };
      
      wsRef.current.onerror = (event) => {
        setError('WebSocket连接错误');
        onError?.(event);
      };
    } catch (err) {
      setError('无法创建WebSocket连接');
    }
  }, [url, onMessage, onError, reconnectInterval]);
  
  const disconnect = useCallback(() => {
    if (reconnectTimeoutRef.current) {
      clearTimeout(reconnectTimeoutRef.current);
    }
    wsRef.current?.close();
  }, []);
  
  const sendMessage = useCallback((data: any) => {
    if (wsRef.current?.readyState === WebSocket.OPEN) {
      wsRef.current.send(JSON.stringify(data));
    }
  }, []);
  
  useEffect(() => {
    connect();
    return disconnect;
  }, [connect, disconnect]);
  
  return {
    isConnected,
    error,
    sendMessage,
    disconnect,
    reconnect: connect
  };
};
```

## 样式开发指南

### 1. TailwindCSS使用规范

#### 响应式设计
```typescript
// 响应式类名使用
const ResponsiveCard: React.FC = () => {
  return (
    <div className="
      w-full 
      sm:w-1/2 
      md:w-1/3 
      lg:w-1/4 
      xl:w-1/5
      p-4 
      sm:p-6 
      md:p-8
    ">
      <div className="
        text-sm 
sm:text-base 
        md:text-lg 
        lg:text-xl
        text-gray-600 
        dark:text-gray-300
      ">
        响应式文本内容
      </div>
    </div>
  );
};
```

#### 自定义工具类
```css
/* src/styles/utilities.css */
@layer utilities {
  /* 自定义间距 */
  .space-y-18 {
    > * + * {
      margin-top: 4.5rem;
    }
  }
  
  /* 自定义阴影 */
  .shadow-card {
    box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 
                0 2px 4px -1px rgba(0, 0, 0, 0.06);
  }
  
  /* 自定义动画 */
  .animate-fade-in {
    animation: fadeIn 0.3s ease-in-out;
  }
  
  @keyframes fadeIn {
    from { opacity: 0; transform: translateY(10px); }
    to { opacity: 1; transform: translateY(0); }
  }
  
  /* 自定义渐变 */
  .bg-gradient-primary {
    background: linear-gradient(135deg, theme('colors.primary.500'), theme('colors.primary.700'));
  }
}
```

#### 组件样式模块
```css
/* Button.module.css */
.button {
  @apply inline-flex items-center justify-center font-medium rounded-md 
         transition-all duration-200 focus:outline-none focus:ring-2 
         focus:ring-offset-2 disabled:opacity-50 disabled:cursor-not-allowed;
}

.primary {
  @apply bg-primary-500 text-white hover:bg-primary-600 
         focus:ring-primary-500 active:bg-primary-700;
}

.secondary {
  @apply bg-gray-100 text-gray-700 hover:bg-gray-200 
         focus:ring-gray-500 active:bg-gray-300;
}

/* 响应式变体 */
@screen sm {
  .button {
    @apply px-4 py-2;
  }
}

@screen md {
  .button {
    @apply px-6 py-3;
  }
}
```

### 2. 主题系统实现

#### CSS变量定义
```css
/* src/styles/themes.css */
:root {
  /* 亮色主题 */
  --color-primary-50: #eff6ff;
  --color-primary-500: #3b82f6;
  --color-primary-600: #2563eb;
  --color-primary-700: #1d4ed8;
  
  --color-gray-50: #f9fafb;
  --color-gray-100: #f3f4f6;
  --color-gray-500: #6b7280;
  --color-gray-900: #111827;
  
  --color-success: #10b981;
  --color-warning: #f59e0b;
  --color-error: #ef4444;
  --color-info: #06b6d4;
}

[data-theme="dark"] {
  /* 暗色主题 */
  --color-primary-50: #1e3a8a;
  --color-primary-500: #60a5fa;
  --color-primary-600: #3b82f6;
  --color-primary-700: #2563eb;
  
  --color-gray-50: #1f2937;
  --color-gray-100: #374151;
  --color-gray-500: #9ca3af;
  --color-gray-900: #f9fafb;
  
  --color-success: #34d399;
  --color-warning: #fbbf24;
  --color-error: #f87171;
  --color-info: #22d3ee;
}
```

#### 主题切换Hook
```typescript
// src/hooks/useTheme.ts
import { useEffect, useState } from 'react';

type Theme = 'light' | 'dark' | 'system';

export const useTheme = () => {
  const [theme, setTheme] = useState<Theme>(() => {
    const stored = localStorage.getItem('theme') as Theme;
    return stored || 'system';
  });
  
  const [resolvedTheme, setResolvedTheme] = useState<'light' | 'dark'>('light');
  
  useEffect(() => {
    const root = document.documentElement;
    
    const applyTheme = (newTheme: 'light' | 'dark') => {
      root.setAttribute('data-theme', newTheme);
      setResolvedTheme(newTheme);
    };
    
    if (theme === 'system') {
      const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
      applyTheme(mediaQuery.matches ? 'dark' : 'light');
      
      const handleChange = (e: MediaQueryListEvent) => {
        applyTheme(e.matches ? 'dark' : 'light');
      };
      
      mediaQuery.addEventListener('change', handleChange);
      return () => mediaQuery.removeEventListener('change', handleChange);
    } else {
      applyTheme(theme);
    }
  }, [theme]);
  
  const updateTheme = (newTheme: Theme) => {
    setTheme(newTheme);
    localStorage.setItem('theme', newTheme);
  };
  
  return { theme, resolvedTheme, setTheme: updateTheme };
};
```

## 测试开发指南

### 1. 单元测试

#### 测试配置
```typescript
// vitest.config.ts
import { defineConfig } from 'vitest/config';
import react from '@vitejs/plugin-react';
import path from 'path';

export default defineConfig({
  plugins: [react()],
  test: {
    environment: 'jsdom',
    setupFiles: ['./src/test/setup.ts'],
    globals: true,
    css: true,
  },
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
    },
  },
});
```

#### 测试工具设置
```typescript
// src/test/setup.ts
import '@testing-library/jest-dom';
import { vi } from 'vitest';

// Mock Tauri API
global.window = Object.create(window);
global.window.__TAURI__ = {
  invoke: vi.fn(),
  event: {
    listen: vi.fn(),
    emit: vi.fn(),
  },
};

// Mock IntersectionObserver
global.IntersectionObserver = vi.fn().mockImplementation(() => ({
  observe: vi.fn(),
  unobserve: vi.fn(),
  disconnect: vi.fn(),
}));

// Mock ResizeObserver
global.ResizeObserver = vi.fn().mockImplementation(() => ({
  observe: vi.fn(),
  unobserve: vi.fn(),
  disconnect: vi.fn(),
}));
```

#### 组件测试示例
```typescript
// src/components/features/__tests__/StatCard.test.tsx
import { render, screen } from '@testing-library/react';
import { describe, it, expect } from 'vitest';
import { StatCard } from '../StatCard';

describe('StatCard', () => {
  const defaultProps = {
    title: '测试指标',
    value: 1234,
    icon: 'fas fa-chart-line',
    color: 'blue' as const,
  };

  it('renders basic information correctly', () => {
    render(<StatCard {...defaultProps} />);
    
    expect(screen.getByText('测试指标')).toBeInTheDocument();
    expect(screen.getByText('1,234')).toBeInTheDocument();
  });

  it('displays change indicator when provided', () => {
    const props = {
      ...defaultProps,
      change: {
        value: 5.2,
        type: 'increase' as const,
        period: '过去7天'
      }
    };
    
    render(<StatCard {...props} />);
    
    expect(screen.getByText('5.2%')).toBeInTheDocument();
    expect(screen.getByText('过去7天')).toBeInTheDocument();
  });

  it('shows loading state correctly', () => {
    render(<StatCard {...defaultProps} loading />);
    
    expect(screen.getByText('测试指标')).toBeInTheDocument();
    expect(screen.queryByText('1,234')).not.toBeInTheDocument();
    expect(screen.getByTestId('loading-skeleton')).toBeInTheDocument();
  });

  it('applies correct color classes', () => {
    const { container } = render(<StatCard {...defaultProps} color="green" />);
    
    expect(container.firstChild).toHaveClass('stat-card');
    expect(container.querySelector('.stat-icon')).toHaveClass('text-green-600');
  });
});
```

#### Hook测试示例
```typescript
// src/hooks/__tests__/useSystemStatus.test.ts
import { renderHook, act } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import { useSystemStatus } from '../useSystemStatus';

// Mock store
vi.mock('../store/systemStore', () => ({
  useSystemStore: vi.fn(),
}));

describe('useSystemStatus', () => {
  const mockUpdateStats = vi.fn();
  const mockInvoke = vi.fn();

  beforeEach(() => {
    vi.clearAllMocks();
    global.window.__TAURI__.invoke = mockInvoke;
  });

  it('updates stats periodically', async () => {
    const mockSystemInfo = {
      activeConnections: 5,
      processedTasks: 100,
      errorCount: 2,
    };
    
    mockInvoke.mockResolvedValue(mockSystemInfo);
    
    const { result } = renderHook(() => useSystemStatus());
    
    // 等待定时器执行
    await act(async () => {
      vi.advanceTimersByTime(5000);
    });
    
    expect(mockInvoke).toHaveBeenCalledWith('get_system_info');
    expect(mockUpdateStats).toHaveBeenCalledWith(mockSystemInfo);
  });

  it('handles errors gracefully', async () => {
    const consoleSpy = vi.spyOn(console, 'error').mockImplementation(() => {});
    mockInvoke.mockRejectedValue(new Error('API Error'));
    
    renderHook(() => useSystemStatus());
    
    await act(async () => {
      vi.advanceTimersByTime(5000);
    });
    
    expect(consoleSpy).toHaveBeenCalledWith(
      'Failed to get system info:',
      expect.any(Error)
    );
    
    consoleSpy.mockRestore();
  });
});
```

### 2. 集成测试

#### 页面集成测试
```typescript
// src/pages/__tests__/Overview.test.tsx
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { BrowserRouter } from 'react-router-dom';
import { describe, it, expect, vi } from 'vitest';
import { Overview } from '../Overview';

const renderWithRouter = (component: React.ReactElement) => {
  return render(
    <BrowserRouter>
      {component}
    </BrowserRouter>
  );
};

describe('Overview Page', () => {
  it('renders all stat cards', () => {
    renderWithRouter(<Overview />);
    
    expect(screen.getByText('活跃连接')).toBeInTheDocument();
    expect(screen.getByText('处理任务')).toBeInTheDocument();
    expect(screen.getByText('服务状态')).toBeInTheDocument();
    expect(screen.getByText('错误计数')).toBeInTheDocument();
  });

  it('handles key extraction action', async () => {
    const mockExtractKey = vi.fn().mockResolvedValue('success');
    
    renderWithRouter(<Overview />);
    
    const extractButton = screen.getByText('提取密钥');
    fireEvent.click(extractButton);
    
    await waitFor(() => {
      expect(mockExtractKey).toHaveBeenCalled();
    });
  });

  it('displays loading state during operations', async () => {
    const mockExtractKey = vi.fn().mockImplementation(
      () => new Promise(resolve => setTimeout(resolve, 1000))
    );
    
    renderWithRouter(<Overview />);
    
    const extractButton = screen.getByText('提取密钥');
    fireEvent.click(extractButton);
    
    expect(screen.getByText('提取中...')).toBeInTheDocument();
    
    await waitFor(() => {
      expect(screen.queryByText('提取中...')).not.toBeInTheDocument();
    });
  });
});
```

## 性能优化指南

### 1. 代码分割

#### 路由级别分割
```typescript
// src/App.tsx
import { lazy, Suspense } from 'react';
import { BrowserRouter, Routes, Route } from 'react-router-dom';
import { MainLayout } from './components/layout';
import { LoadingSpinner } from './components/ui';

// 懒加载页面组件
const Overview = lazy(() => import('./pages/Overview'));
const DataSource = lazy(() => import('./pages/DataSource'));
const DataProcessing = lazy(() => import('./pages/DataProcessing'));
const SystemMonitor = lazy(() => import('./pages/SystemMonitor'));
const ServiceManagement = lazy(() => import('./pages/ServiceManagement'));
const Settings = lazy(() => import('./pages/Settings'));

function App() {
  return (
    <BrowserRouter>
      <MainLayout>
        <Suspense fallback={<LoadingSpinner />}>
          <Routes>
            <Route path="/" element={<Overview />} />
            <Route path="/data-source" element={<DataSource />} />
            <Route path="/data-processing" element={<DataProcessing />} />
            <Route path="/system-monitor" element={<SystemMonitor />} />
            <Route path="/service-management" element={<ServiceManagement />} />
            <Route path="/settings" element={<Settings />} />
          </Routes>
        </Suspense>
      </MainLayout>
    </BrowserRouter>
  );
}

export default App;
```

### 2. 组件优化

#### 虚拟滚动实现
```typescript
// src/components/ui/VirtualList.tsx
import React, { useMemo, useCallback } from 'react';
import { FixedSizeList as List } from 'react-window';

interface VirtualListProps<T> {
  items: T[];
  height: number;
  itemHeight: number;
  renderItem: (item: T, index: number) => React.ReactNode;
  className?: string;
}

export function VirtualList<T>({
  items,
  height,
  itemHeight,
  renderItem,
  className
}: VirtualListProps<T>) {
  const Row = useCallback(
    
({ index, style }: { index: number; style: React.CSSProperties }) => (
      <div style={style}>
        {renderItem(items[index], index)}
      </div>
    ),
    [items, renderItem]
  );

  return (
    <List
      className={className}
      height={height}
      itemCount={items.length}
      itemSize={itemHeight}
      width="100%"
    >
      {Row}
    </List>
  );
}
```

### 3. 内存优化

#### 图片懒加载
```typescript
// src/components/ui/LazyImage.tsx
import React, { useState, useRef, useEffect } from 'react';

interface LazyImageProps {
  src: string;
  alt: string;
  placeholder?: string;
  className?: string;
}

export const LazyImage: React.FC<LazyImageProps> = ({
  src,
  alt,
  placeholder = '/placeholder.svg',
  className
}) => {
  const [isLoaded, setIsLoaded] = useState(false);
  const [isInView, setIsInView] = useState(false);
  const imgRef = useRef<HTMLImageElement>(null);

  useEffect(() => {
    const observer = new IntersectionObserver(
      ([entry]) => {
        if (entry.isIntersecting) {
          setIsInView(true);
          observer.disconnect();
        }
      },
      { threshold: 0.1 }
    );

    if (imgRef.current) {
      observer.observe(imgRef.current);
    }

    return () => observer.disconnect();
  }, []);

  return (
    <img
      ref={imgRef}
      src={isInView ? src : placeholder}
      alt={alt}
      className={className}
      onLoad={() => setIsLoaded(true)}
      style={{
        opacity: isLoaded ? 1 : 0.5,
        transition: 'opacity 0.3s ease'
      }}
    />
  );
};
```

## 部署和构建

### 1. 构建配置优化

#### Vite配置
```typescript
// vite.config.ts
import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import { resolve } from 'path';

export default defineConfig({
  plugins: [react()],
  resolve: {
    alias: {
      '@': resolve(__dirname, 'src'),
      '@components': resolve(__dirname, 'src/components'),
      '@pages': resolve(__dirname, 'src/pages'),
      '@utils': resolve(__dirname, 'src/utils'),
      '@store': resolve(__dirname, 'src/store'),
      '@hooks': resolve(__dirname, 'src/hooks'),
    },
  },
  build: {
    rollupOptions: {
      output: {
        manualChunks: {
          vendor: ['react', 'react-dom'],
          router: ['react-router-dom'],
          ui: ['react-aria-components'],
          charts: ['recharts'],
          motion: ['framer-motion'],
          store: ['zustand'],
          utils: ['clsx', 'date-fns'],
        },
      },
    },
    sourcemap: true,
    minify: 'terser',
    terserOptions: {
      compress: {
        drop_console: true,
        drop_debugger: true,
      },
    },
  },
  server: {
    port: 3000,
    open: true,
    cors: true,
  },
  preview: {
    port: 3001,
  },
});
```

### 2. 生产环境部署

#### 构建脚本
```json
{
  "scripts": {
    "dev": "vite",
    "build": "tsc && vite build",
    "build:analyze": "vite build --mode analyze",
    "preview": "vite preview",
    "tauri:dev": "tauri dev",
    "tauri:build": "tauri build",
    "tauri:build:debug": "tauri build --debug",
    "test": "vitest",
    "test:ui": "vitest --ui",
    "test:coverage": "vitest --coverage",
    "lint": "eslint src --ext ts,tsx --report-unused-disable-directives --max-warnings 0",
    "lint:fix": "eslint src --ext ts,tsx --fix",
    "format": "prettier --write src/**/*.{ts,tsx,css,md}",
    "type-check": "tsc --noEmit",
    "storybook": "storybook dev -p 6006",
    "build-storybook": "storybook build"
  }
}
```

## 故障排除

### 1. 常见问题解决

#### 构建问题
```bash
# 清理缓存
rm -rf node_modules
rm -rf dist
rm -rf .vite
yarn install

# TypeScript类型检查
yarn type-check

# 依赖版本冲突
yarn install --force
```

#### 运行时问题
```typescript
// 错误边界处理
class ErrorBoundary extends React.Component<
  { children: React.ReactNode },
  { hasError: boolean; error?: Error }
> {
  constructor(props: { children: React.ReactNode }) {
    super(props);
    this.state = { hasError: false };
  }

  static getDerivedStateFromError(error: Error) {
    return { hasError: true, error };
  }

  componentDidCatch(error: Error, errorInfo: React.ErrorInfo) {
    console.error('Error caught by boundary:', error, errorInfo);
  }

  render() {
    if (this.state.hasError) {
      return (
        <div className="min-h-screen flex items-center justify-center">
          <div className="text-center">
            <h1 className="text-2xl font-bold text-red-600 mb-4">
              应用程序出现错误
            </h1>
            <p className="text-gray-600 mb-4">
              {this.state.error?.message || '未知错误'}
            </p>
            <button
              onClick={() => window.location.reload()}
              className="px-4 py-2 bg-primary-500 text-white rounded hover:bg-primary-600"
            >
              重新加载
            </button>
          </div>
        </div>
      );
    }

    return this.props.children;
  }
}
```

### 2. 性能监控

#### 性能指标收集
```typescript
// src/utils/performance.ts
export const measurePerformance = (name: string, fn: () => void) => {
  const start = performance.now();
  fn();
  const end = performance.now();
  
  console.log(`${name} took ${end - start} milliseconds`);
  
  // 发送到监控服务
  if (end - start > 100) {
    console.warn(`Slow operation detected: ${name}`);
  }
};

export const usePerformanceMonitor = (componentName: string) => {
  useEffect(() => {
    const start = performance.now();
    
    return () => {
      const end = performance.now();
      const renderTime = end - start;
      
      if (renderTime > 16) { // 超过一帧的时间
        console.warn(`${componentName} render time: ${renderTime}ms`);
      }
    };
  });
};
```

## 总结

本详细实施指南为微信数据采集工具的现代化UI重构提供了全面的开发指导，涵盖了：

### 核心开发内容
1. **环境配置**: 完整的开发
环境搭建和工具链配置
2. **组件开发**: 标准化的组件开发流程和最佳实践
3. **状态管理**: Zustand状态管理和自定义Hook开发
4. **样式开发**: TailwindCSS使用规范和主题系统实现
5. **测试策略**: 完整的单元测试和集成测试指南

### 开发规范
1. **代码规范**: TypeScript、ESLint、Prettier配置和使用
2. **文件组织**: 清晰的项目结构和命名规范
3. **Git工作流**: 提交规范和代码审查流程
4. **文档维护**: Storybook和技术文档管理

### 性能优化
1. **代码分割**: 路由级别和组件级别的懒加载
2. **内存优化**: 虚拟滚动和图片懒加载实现
3. **构建优化**: Vite配置和生产环境优化
4. **监控调试**: 性能监控和错误处理机制

### 部署运维
1. **构建配置**: 生产环境构建和优化配置
2. **故障排除**: 常见问题解决方案和调试技巧
3. **性能监控**: 运行时性能监控和指标收集
4. **错误处理**: 全局错误边界和异常处理

通过严格遵循本实施指南，开发团队可以高效、规范地完成现代化UI重构工作，确保代码质量、用户体验和项目可维护性。

### 关键成功要素
- **团队协作**: 统一的开发规范和工作流程
- **代码质量**: 完善的测试覆盖和代码审查
- **性能优化**: 持续的性能监控和优化改进
- **文档维护**: 及时更新的技术文档和使用指南

### 后续改进
- 根据实际开发过程中遇到的问题持续完善指南
- 收集开发团队反馈，优化开发流程和工具配置
- 建立知识库，积累项目经验和最佳实践
- 定期评审和更新技术栈，保持技术先进性

---

**重要提醒**: 
- 在开始开发前，请确保所有团队成员都已阅读并理解本指南
- 开发过程中如遇到问题，请及时查阅相关章节或向技术负责人咨询
- 本指南将随着项目进展持续更新，请关注最新版本

**文档维护**: 本文档由项目技术团队维护，如有问题或建议请及时反馈。