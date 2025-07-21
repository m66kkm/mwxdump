import React from 'react';
import { useLocation, useNavigate } from 'react-router-dom';
import { Menu, MenuItem } from 'react-aria-components';
import { cn } from '../../utils';
import { NAVIGATION_ITEMS } from '../../utils/constants';
import { NavigationItem } from '../../types';
import { Button } from '../ui';

interface SidebarProps {
  collapsed: boolean;
  onCollapse: (collapsed: boolean) => void;
  className?: string;
}

const Sidebar: React.FC<SidebarProps> = ({ collapsed, onCollapse, className }) => {
  const location = useLocation();
  const navigate = useNavigate();
  return (
    <aside className={cn(
      'fixed left-0 top-16 h-[calc(100vh-4rem)] bg-white shadow-lg',
      'transition-all duration-300 z-40 border-r border-gray-200',
      collapsed ? 'w-16' : 'w-64',
      className
    )}>
      {/* 折叠按钮 */}
      <div className="p-4 border-b border-gray-200">
        <Button
          variant="ghost"
          onPress={() => onCollapse(!collapsed)}
          className={cn(
            'w-full flex items-center justify-center p-2 rounded-lg',
            'hover:bg-gray-100 transition-colors duration-200',
            'text-gray-600 hover:text-gray-900'
          )}
          aria-label={collapsed ? '展开侧边栏' : '收起侧边栏'}
        >
          <i className={cn(
            'fas transition-transform duration-200',
            collapsed ? 'fa-chevron-right' : 'fa-chevron-left'
          )} />
        </Button>
      </div>

      {/* 导航菜单 */}
      <nav className="p-4 flex-1 overflow-y-auto">
        <Menu
          className="space-y-2 outline-none"
          aria-label="主导航菜单"
          onAction={(key) => {
            const item = NAVIGATION_ITEMS.find(nav => nav.key === key);
            if (item) {
              navigate(item.path);
            }
          }}
        >
          {NAVIGATION_ITEMS.map((item: NavigationItem) => {
            const isActive = location.pathname === item.path;
            return (
              <MenuItem
                key={item.key}
                id={item.key}
                className={({ isFocused, isPressed }) => cn(
                  'flex items-center p-3 rounded-lg transition-all duration-200',
                  'group relative cursor-default outline-none',
                  'focus:ring-2 focus:ring-primary-500 focus:ring-offset-2',
                  isActive
                    ? 'bg-primary-50 text-primary-600 border-r-2 border-primary-500'
                    : cn(
                        'text-gray-700',
                        isFocused && 'bg-gray-100 text-gray-900',
                        isPressed && 'bg-gray-200'
                      )
                )}
              >
                <i className={cn(
                  item.icon,
                  'flex-shrink-0 transition-colors duration-200',
                  collapsed ? 'text-lg' : 'mr-3 text-base',
                  isActive ? 'text-primary-600' : 'text-gray-600'
                )} />
                
                {!collapsed && (
                  <span className={cn(
                    'font-medium truncate',
                    isActive ? 'text-primary-600' : 'text-gray-700'
                  )}>
                    {item.label}
                  </span>
                )}
                
                {/* 折叠状态下的提示 */}
                {collapsed && (
                  <div className={cn(
                    'absolute left-full ml-2 px-2 py-1 bg-gray-900 text-white text-sm rounded',
                    'opacity-0 invisible group-hover:opacity-100 group-hover:visible',
                    'transition-all duration-200 whitespace-nowrap z-50'
                  )}>
                    {item.label}
                    <div className="absolute left-0 top-1/2 transform -translate-x-1 -translate-y-1/2 w-0 h-0 border-r-4 border-r-gray-900 border-t-2 border-b-2 border-t-transparent border-b-transparent" />
                  </div>
                )}
              </MenuItem>
            );
          })}
        </Menu>
      </nav>

      {/* 底部信息 */}
      <div className="p-4 border-t border-gray-200">
        {!collapsed ? (
          <div className="text-xs text-gray-500 text-center">
            <p>版本 1.0.0</p>
            <p className="mt-1">© 2025 微信数据采集工具</p>
          </div>
        ) : (
          <div className="flex justify-center">
            <div className="w-2 h-2 bg-green-500 rounded-full" title="系统正常运行" />
          </div>
        )}
      </div>
    </aside>
  );
};

export { Sidebar };
export default Sidebar;