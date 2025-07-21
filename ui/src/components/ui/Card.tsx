import React from 'react';
import { cn } from '../../utils';
import { CardProps } from '../../types';

const Card: React.FC<CardProps> = ({
  title,
  subtitle,
  actions,
  padding = 'md',
  shadow = 'sm',
  children,
  className,
  ...props
}) => {
  const baseClasses = 'card-base bg-white rounded-xl border border-gray-200 transition-shadow duration-200';
  
  const shadowClasses = {
    sm: 'shadow-sm hover:shadow-md',
    md: 'shadow-md hover:shadow-lg',
    lg: 'shadow-lg hover:shadow-xl',
  };
  
  const paddingClasses = {
    sm: 'p-4',
    md: 'p-6',
    lg: 'p-8',
  };

  return (
    <div
      className={cn(
        baseClasses,
        shadowClasses[shadow],
        className
      )}
      {...props}
    >
      {(title || subtitle || actions) && (
        <div className={cn(
          'flex items-start justify-between border-b border-gray-200 pb-4 mb-4',
          paddingClasses[padding]
        )}>
          <div className="flex-1">
            {title && (
              <h3 className="text-lg font-semibold text-gray-900 mb-1">
                {title}
              </h3>
            )}
            {subtitle && (
              <p className="text-sm text-gray-600">
                {subtitle}
              </p>
            )}
          </div>
          {actions && (
            <div className="flex items-center space-x-2 ml-4">
              {actions}
            </div>
          )}
        </div>
      )}
      
      <div className={cn(
        'flex-1',
        paddingClasses[padding]
      )}>
        {children}
      </div>
    </div>
  );
};

export { Card };
export default Card;