import React from 'react';
import { useLocation } from 'react-router-dom';
import { cn } from '../../utils';
import { APP_NAME, NAVIGATION_ITEMS } from '../../utils/constants';
import { Button } from '../ui';

interface HeaderProps {
  onToggleSidebar: () => void;
  className?: string;
}

const Header: React.FC<HeaderProps> = ({ onToggleSidebar, className }) => {
  return (
    <header className={cn(
      'fixed top-0 left-0 right-0 z-50',
      'bg-white border-b border-gray-200',
      'h-16 px-4 lg:px-6',
      'flex items-center justify-between',
      'shadow-sm',
      className
    )}>
      {/* 左侧区域 */}
      <div className="flex items-center space-x-4">
        {/* 侧边栏切换按钮 */}
        <Button
          variant="ghost"
          size="sm"
          onPress={onToggleSidebar}
          className="lg:hidden p-2"
          aria-label="切换侧边栏"
        >
          <i className="fas fa-bars text-lg" />
        </Button>

        {/* 应用标题 */}
        <div className="flex items-center space-x-3">
          <div className="w-8 h-8 bg-primary-500 rounded-lg flex items-center justify-center">
            <i className="fas fa-database text-white text-sm" />
          </div>
          <h1 className="text-xl font-bold text-gray-900 hidden sm:block">
            {APP_NAME}
          </h1>
        </div>
      </div>

      {/* 右侧区域 */}
      <div className="flex items-center space-x-3">
        {/* 通知按钮 */}
        <Button
          variant="ghost"
          size="sm"
          className="relative p-2"
          aria-label="通知"
        >
          <i className="fas fa-bell text-lg" />
          {/* 通知徽章 */}
          <span className="absolute -top-1 -right-1 w-3 h-3 bg-error-500 rounded-full flex items-center justify-center">
            <span className="w-1.5 h-1.5 bg-white rounded-full" />
          </span>
        </Button>

        {/* 设置按钮 */}
        <Button
          variant="ghost"
          size="sm"
          className="p-2"
          aria-label="设置"
        >
          <i className="fas fa-cog text-lg" />
        </Button>

        {/* 用户菜单 */}
        <div className="relative">
          <Button
            variant="ghost"
            size="sm"
            className="flex items-center space-x-2 p-2"
          >
            <div className="w-8 h-8 bg-gray-300 rounded-full flex items-center justify-center">
              <i className="fas fa-user text-gray-600 text-sm" />
            </div>
            <span className="hidden md:block text-sm font-medium text-gray-700">
              管理员
            </span>
            <i className="fas fa-chevron-down text-xs text-gray-500" />
          </Button>
        </div>
      </div>
    </header>
  );
};

export { Header };
export default Header;