import React, { useState } from 'react';
import { cn } from '../../utils';
import Header from './Header';
import Sidebar from './Sidebar';

interface MainLayoutProps {
  children: React.ReactNode;
  className?: string;
}

const MainLayout: React.FC<MainLayoutProps> = ({ children, className }) => {
  const [sidebarCollapsed, setSidebarCollapsed] = useState(false);

  return (
    <div className={cn('min-h-screen bg-gray-50', className)}>
      {/* 顶部导航栏 */}
      <Header 
        onToggleSidebar={() => setSidebarCollapsed(!sidebarCollapsed)}
      />
      
      {/* 侧边栏 */}
      <Sidebar 
        collapsed={sidebarCollapsed}
        onCollapse={setSidebarCollapsed}
      />
      
      {/* 主内容区域 */}
      <main className={cn(
        'transition-all duration-300 pt-16 min-h-screen',
        sidebarCollapsed ? 'ml-16' : 'ml-64'
      )}>
        {/* 内容容器 */}
        <div className="p-6">
          {children}
        </div>
      </main>
      
      {/* 移动端遮罩层 */}
      {!sidebarCollapsed && (
        <div 
          className="fixed inset-0 bg-black bg-opacity-50 z-30 lg:hidden"
          onClick={() => setSidebarCollapsed(true)}
        />
      )}
    </div>
  );
};

export { MainLayout };
export default MainLayout;