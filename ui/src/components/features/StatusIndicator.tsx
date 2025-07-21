import React from 'react';
import { cn } from '../../utils';
import { StatusIndicatorProps } from '../../types';

const StatusIndicator: React.FC<StatusIndicatorProps> = ({
  status,
  text,
  size = 'md',
  showIcon = false,
  showDot = true,
  className,
  ...props
}) => {
  const statusConfig = {
    success: {
      color: 'text-success-600',
      bgColor: 'bg-success-500',
      icon: 'fas fa-check-circle',
    },
    warning: {
      color: 'text-warning-600',
      bgColor: 'bg-warning-500',
      icon: 'fas fa-exclamation-triangle',
    },
    error: {
      color: 'text-error-600',
      bgColor: 'bg-error-500',
      icon: 'fas fa-times-circle',
    },
    info: {
      color: 'text-info-600',
      bgColor: 'bg-info-500',
      icon: 'fas fa-info-circle',
    },
    idle: {
      color: 'text-gray-600',
      bgColor: 'bg-gray-400',
      icon: 'fas fa-circle',
    },
  };

  const sizeClasses = {
    sm: {
      text: 'text-xs',
      dot: 'w-2 h-2',
      icon: 'text-xs',
      gap: 'gap-1',
    },
    md: {
      text: 'text-sm',
      dot: 'w-2.5 h-2.5',
      icon: 'text-sm',
      gap: 'gap-2',
    },
    lg: {
      text: 'text-base',
      dot: 'w-3 h-3',
      icon: 'text-base',
      gap: 'gap-2',
    },
  };

  const config = statusConfig[status];
  const sizes = sizeClasses[size];

  return (
    <div
      className={cn(
        'inline-flex items-center font-medium',
        sizes.gap,
        className
      )}
      {...props}
    >
      {showIcon && !showDot && (
        <i className={cn(config.icon, config.color, sizes.icon)} />
      )}
      
      {showDot && !showIcon && (
        <span
          className={cn(
            'rounded-full flex-shrink-0',
            config.bgColor,
            sizes.dot
          )}
        />
      )}
      
      {showIcon && showDot && (
        <div className="relative">
          <span
            className={cn(
              'rounded-full flex-shrink-0',
              config.bgColor,
              sizes.dot
            )}
          />
          <i className={cn(
            config.icon, 
            'absolute inset-0 flex items-center justify-center text-white',
            'text-xs'
          )} />
        </div>
      )}
      
      <span className={cn(config.color, sizes.text)}>
        {text}
      </span>
    </div>
  );
};

export { StatusIndicator };
export default StatusIndicator;