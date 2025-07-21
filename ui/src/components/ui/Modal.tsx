import React from 'react';
import { Modal as AriaModal, ModalOverlay, Dialog, Heading, Button } from 'react-aria-components';
import { cn } from '../../utils';
import { ModalProps } from '../../types';

const Modal: React.FC<ModalProps> = ({
  isOpen,
  onClose,
  title,
  size = 'md',
  closable = true,
  children,
  className,
  ...props
}) => {
  const sizeClasses = {
    sm: 'max-w-md',
    md: 'max-w-lg',
    lg: 'max-w-2xl',
    xl: 'max-w-4xl',
  };

  return (
    <ModalOverlay
      isOpen={isOpen}
      onOpenChange={onClose}
      isDismissable={closable}
      className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/50"
    >
      <AriaModal
        className={cn(
          'relative w-full bg-white rounded-xl shadow-2xl max-h-[90vh] overflow-hidden animate-scale-in',
          sizeClasses[size],
          className
        )}
        {...props}
      >
        <Dialog className="outline-hidden">
          {/* 头部 */}
          {(title || closable) && (
            <div className="flex items-center justify-between p-6 border-b border-gray-200">
              <div className="flex-1">
                {title && (
                  <Heading slot="title" className="text-xl font-semibold text-gray-900">
                    {title}
                  </Heading>
                )}
              </div>
              {closable && (
                <Button
                  slot="close"
                  className="ml-4 p-2 text-gray-400 hover:text-gray-600 hover:bg-gray-100 rounded-lg transition-colors"
                  aria-label="关闭"
                >
                  <i className="fas fa-times text-lg" />
                </Button>
              )}
            </div>
          )}
          
          {/* 内容区域 */}
          <div className="p-6 overflow-y-auto max-h-[calc(90vh-8rem)]">
            {children}
          </div>
        </Dialog>
      </AriaModal>
    </ModalOverlay>
  );
};

export { Modal };
export default Modal;