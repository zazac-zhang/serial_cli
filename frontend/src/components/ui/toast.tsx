import { useToast } from '@/contexts/ToastContext'
import type { Toast } from '@/contexts/ToastContext'
import { cn } from '@/lib/utils'
import { useEffect } from 'react'

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

  const typeStyles: Record<Toast['type'], string> = {
    success: 'border-signal/30 bg-signal/5 text-signal',
    error: 'border-alert/30 bg-alert/5 text-alert',
    warning: 'border-amber/30 bg-amber/5 text-amber',
    info: 'border-info/30 bg-info/5 text-info',
  }

  return (
    <div
      className={cn(
        'pointer-events-auto max-w-sm rounded-lg border p-4 shadow-lg animate-slide-up',
        typeStyles[toast.type]
      )}
    >
      <div className="flex items-start justify-between gap-3">
        <p className="text-sm font-medium">{toast.message}</p>
        <button
          onClick={onRemove}
          className="text-current/60 hover:text-current transition-colors"
        >
          ×
        </button>
      </div>
    </div>
  )
}
