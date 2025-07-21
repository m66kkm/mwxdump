import React from 'react';
import { TextField, Label, Input as AriaInput, Text, FieldError, TextFieldProps } from 'react-aria-components';
import { cn } from '../../utils';
import { InputProps } from '../../types';

const Input: React.FC<InputProps> = ({
  type = 'text',
  size = 'md',
  state = 'default',
  placeholder,
  isDisabled = false,
  icon,
  label,
  helperText,
  errorMessage,
  className,
  ...props
}) => {
  const baseClasses = 'input-base w-full border rounded-lg focus:outline-none focus:ring-2 focus:ring-offset-0 transition-colors duration-200';
  
  const stateClasses = {
    default: 'border-gray-300 focus:ring-primary-500 focus:border-primary-500',
    error: 'border-error-500 focus:ring-error-500 focus:border-error-500',
    success: 'border-success-500 focus:ring-success-500 focus:border-success-500',
  };
  
  const sizeClasses = {
    sm: 'px-3 py-2 text-sm',
    md: 'px-4 py-3 text-base',
    lg: 'px-5 py-4 text-lg',
  };

  // 如果没有label，直接返回简单的Input
  if (!label) {
    return (
      <div className="relative w-full">
        {icon && (
          <div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
            <span className="text-gray-400">
              {icon}
            </span>
          </div>
        )}
        
        <AriaInput
          type={type}
          placeholder={placeholder}
          className={cn(
            baseClasses,
            stateClasses[state],
            sizeClasses[size],
            {
              'pl-10': icon,
              'opacity-50 cursor-not-allowed': isDisabled,
            },
            className
          )}
          {...props}
        />
      </div>
    );
  }

  // 使用完整的TextField结构
  return (
    <TextField
      isDisabled={isDisabled}
      isInvalid={state === 'error'}
      className="w-full"
      {...(props as TextFieldProps)}
    >
      <Label className="block text-sm font-medium text-gray-700 mb-2">
        {label}
      </Label>
      
      <div className="relative">
        {icon && (
          <div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
            <span className="text-gray-400">
              {icon}
            </span>
          </div>
        )}
        
        <AriaInput
          type={type}
          placeholder={placeholder}
          className={cn(
            baseClasses,
            stateClasses[state],
            sizeClasses[size],
            {
              'pl-10': icon,
              'opacity-50 cursor-not-allowed': isDisabled,
            },
            className
          )}
        />
      </div>
      
      {helperText && (
        <Text slot="description" className="mt-2 text-sm text-gray-600">
          {helperText}
        </Text>
      )}
      
      {errorMessage && (
        <FieldError className="mt-2 text-sm text-error-600 flex items-center">
          <i className="fas fa-exclamation-circle mr-1" />
          {errorMessage}
        </FieldError>
      )}
    </TextField>
  );
};

Input.displayName = 'Input';

export { Input };
export default Input;