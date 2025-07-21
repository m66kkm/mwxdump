import React from 'react';
import { cn } from '../../utils';
import { StatCardProps } from '../../types';

const StatCard: React.FC<StatCardProps> = ({
  title,
  value,
  change,
  icon,
  color = 'blue',
  loading = false,
  chart,
  className,
  ...props
}) => {
  const colorClasses = {
    blue: {
      bg: 'bg-blue-50',
      text: 'text-blue-600',
      icon: 'bg-blue-500',
    },
    green: {
      bg: 'bg-green-50',
      text: 'text-green-600',
      icon: 'bg-green-500',
    },
    orange: {
      bg: 'bg-orange-50',
      text: 'text-orange-600',
      icon: 'bg-orange-500',
    },
    purple: {
      bg: 'bg-purple-50',
      text: 'text-purple-600',
      icon: 'bg-purple-500',
    },
  };

  const colors = colorClasses[color];

  return (
    <div
      className={cn(
        'bg-white rounded-xl shadow-sm border border-gray-200 p-6 hover:shadow-md transition-all duration-200',
        'animate-fade-in',
        className
      )}
      {...props}
    >
      <div className="flex items-center justify-between">
        <div className="flex-1">
          <p className="text-sm font-medium text-gray-600 mb-1">{title}</p>
          
          {loading ? (
            <div className="animate-pulse">
              <div className="h-8 bg-gray-200 rounded w-24 mb-2"></div>
              {change && <div className="h-4 bg-gray-200 rounded w-16"></div>}
            </div>
          ) : (
            <>
              <p className="text-3xl font-bold text-gray-900 mb-2">
                {typeof value === 'number' ? value.toLocaleString() : value}
              </p>

              {change && (
                <div className="flex items-center text-sm">
                  <i className={cn(
                    'fas mr-1',
                    change.type === 'increase' 
                      ? 'fa-arrow-up text-green-500' 
                      : 'fa-arrow-down text-red-500'
                  )} />
                  <span className={cn(
                    'font-medium',
                    change.type === 'increase' ? 'text-green-600' : 'text-red-600'
                  )}>
                    {Math.abs(change.value)}%
                  </span>
                  <span className="text-gray-500 ml-1">{change.period}</span>
                </div>
              )}
            </>
          )}
        </div>

        {icon && (
          <div className={cn(
            'w-12 h-12 rounded-lg flex items-center justify-center',
            colors.bg
          )}>
            <i className={cn(icon, 'text-xl text-white')} />
          </div>
        )}
      </div>

      {chart && (
        <div className="mt-4 pt-4 border-t border-gray-200">
          {chart}
        </div>
      )}
    </div>
  );
};

export { StatCard };
export default StatCard;