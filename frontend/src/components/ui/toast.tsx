import { useToast } from '@/contexts/ToastContext'
import type { Toast } from '@/contexts/ToastContext'
import { cn } from '@/lib/utils'
import { useEffect } from 'react'
import { CheckCircle, AlertCircle, AlertTriangle, Info, X } from 'lucide-react'

export function Toaster() {
  const { toasts, removeToast } = useToast()

  return (
    <div className="fixed bottom-6 right-6 z-50 flex flex-col gap-2 pointer-events-none">
      {toasts.map((toast) => (
        <ToastItem key={toast.id} toast={toast} onRemove={() => removeToast(toast.id)} />
      ))}
    </div>
  )
}

function ToastItem({ toast, onRemove }: { toast: Toast; onRemove: () => void }) {
  useEffect(() => {
    if (toast.duration && toast.duration > 0) {
      const timer = setTimeout(onRemove, toast.duration)
      return () => clearTimeout(timer)
    }
  }, [toast.duration, onRemove])

  const typeConfig: Record<Toast['type'], { icon: React.ElementType; colorClass: string; bgColor: string }> = {
    success: {
      icon: CheckCircle,
      colorClass: 'text-signal',
      bgColor: 'bg-signal/10',
    },
    error: {
      icon: AlertCircle,
      colorClass: 'text-alert',
      bgColor: 'bg-alert/10',
    },
    warning: {
      icon: AlertTriangle,
      colorClass: 'text-amber',
      bgColor: 'bg-amber/10',
    },
    info: {
      icon: Info,
      colorClass: 'text-info',
      bgColor: 'bg-info/10',
    },
  }

  const { icon: Icon, colorClass, bgColor } = typeConfig[toast.type]

  return (
    <div
      className={cn(
        'pointer-events-auto w-80 rounded-lg border shadow-lg animate-slide-up backdrop-blur-sm',
        'border-border/50 bg-bg-deep/95',
      )}
    >
      <div className="flex items-start gap-3 p-4">
        {/* Left accent bar */}
        <div className={cn(
          'w-1 h-full min-h-[4rem] rounded-full -ml-4 mr-1',
          colorClass.replace('text-', 'bg-'),
          'opacity-80'
        )} />

        {/* Icon */}
        <div className={cn('flex-shrink-0 mt-0.5', colorClass)}>
          <Icon size={20} strokeWidth={1.5} />
        </div>

        {/* Content */}
        <div className="flex-1 min-w-0">
          <p className="text-sm font-medium text-text-primary break-words">
            {toast.message}
          </p>
        </div>

        {/* Close button */}
        <button
          onClick={onRemove}
          className="flex-shrink-0 text-text-tertiary hover:text-text-primary transition-colors -mr-1 -mt-1"
        >
          <X size={16} strokeWidth={1.5} />
        </button>
      </div>
    </div>
  )
}
