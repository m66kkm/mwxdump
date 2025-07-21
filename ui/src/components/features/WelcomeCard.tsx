import React from 'react';
import { cn } from '../../utils';
import { WelcomeCardProps } from '../../types';
import { Button } from '../ui';

const WelcomeCard: React.FC<WelcomeCardProps> = ({
  userName,
  title,
  description,
  actionText,
  onAction,
  illustration,
  className,
  ...props
}) => {
  return (
    <div
      className={cn(
        'relative overflow-hidden rounded-xl p-8',
        'bg-gradient-to-br from-primary-500 to-primary-700',
        'text-white shadow-lg',
        'animate-fade-in',
        className
      )}
      {...props}
    >
      {/* 背景装饰 */}
      <div className="absolute inset-0 bg-gradient-to-br from-white/10 to-transparent" />
      <div className="absolute -top-4 -right-4 w-24 h-24 bg-white/5 rounded-full" />
      <div className="absolute -bottom-8 -left-8 w-32 h-32 bg-white/5 rounded-full" />
      
      {/* 内容区域 */}
      <div className="relative z-10 flex items-center justify-between">
        <div className="flex-1 max-w-2xl">
          {userName && (
            <p className="text-primary-100 text-sm font-medium mb-2">
              欢迎回来，{userName}
            </p>
          )}
          
          <h2 className="text-2xl md:text-3xl font-bold mb-3">
            {title}
          </h2>
          
          <p className="text-primary-100 text-base md:text-lg mb-6 leading-relaxed">
            {description}
          </p>
          
          <Button
            variant="secondary"
            size="lg"
            onClick={onAction}
            className="bg-white/20 border border-white/30 text-white backdrop-blur-sm hover:bg-white/30 focus:ring-white/50"
          >
            <i className="fas fa-rocket mr-2" />
            {actionText}
          </Button>
        </div>
        
        {/* 插图区域 */}
        {illustration && (
          <div className="hidden lg:block ml-8 flex-shrink-0">
            <div className="w-48 h-48 flex items-center justify-center">
              {illustration}
            </div>
          </div>
        )}
      </div>
      
      {/* 默认插图 */}
      {!illustration && (
        <div className="absolute right-8 top-1/2 transform -translate-y-1/2 hidden lg:block">
          <div className="w-32 h-32 bg-white/10 rounded-full flex items-center justify-center">
            <i className="fas fa-database text-4xl text-white/60" />
          </div>
        </div>
      )}
      
      {/* 装饰性图标 */}
      <div className="absolute top-6 right-6 opacity-20">
        <i className="fas fa-chart-line text-2xl" />
      </div>
      <div className="absolute bottom-6 left-6 opacity-20">
        <i className="fas fa-cogs text-xl" />
      </div>
    </div>
  );
};

export { WelcomeCard };
export default WelcomeCard;