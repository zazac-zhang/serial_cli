import * as React from "react"
import { cn } from "@/lib/utils"
import { cva, type VariantProps } from "class-variance-authority"
import { ChevronDown, ChevronRight } from 'lucide-react'

const panelVariants = cva(
  "rounded-lg border bg-bg-floating text-text-primary transition-all duration-200",
  {
    variants: {
      variant: {
        default: "border-border shadow-md hover:shadow-lg",
        signal: "border-signal/30 shadow-md hover:shadow-lg hover:border-signal/50",
        amber: "border-amber/30 shadow-md hover:shadow-lg hover:border-amber/50",
        alert: "border-alert/30 shadow-md hover:shadow-lg hover:border-alert/50",
        info: "border-info/30 shadow-md hover:shadow-lg hover:border-info/50",
      },
    },
    defaultVariants: {
      variant: "default",
    },
  }
)

const variantAccentColors = {
  default: 'bg-border',
  signal: 'bg-signal',
  amber: 'bg-amber',
  alert: 'bg-alert',
  info: 'bg-info',
}

export interface PanelProps extends VariantProps<typeof panelVariants> {
  title?: string
  children: React.ReactNode
  className?: string
  collapsible?: boolean
  defaultExpanded?: boolean
  actions?: React.ReactNode
}

export function Panel({
  title,
  variant,
  children,
  className,
  collapsible = false,
  defaultExpanded = true,
  actions,
}: PanelProps) {
  const [isExpanded, setIsExpanded] = React.useState(defaultExpanded)

  const handleToggle = () => {
    if (collapsible) {
      setIsExpanded(!isExpanded)
    }
  }

  return (
    <div className={cn(panelVariants({ variant }), className)}>
      {title && (
        <div
          className={cn(
            "border-b border-border/50 px-4 py-3 flex items-center justify-between",
            collapsible && "cursor-pointer select-none hover:bg-bg-elevated/50 transition-colors"
          )}
          onClick={handleToggle}
        >
          <div className="flex items-center gap-3">
            {/* Colored accent bar */}
            <div className={cn(
              "w-0.5 h-4 rounded-full",
              variantAccentColors[variant || 'default']
            )} />
            {collapsible && (
              <span className="text-text-tertiary">
                {isExpanded ? <ChevronDown size={16} strokeWidth={1.5} /> : <ChevronRight size={16} strokeWidth={1.5} />}
              </span>
            )}
            <h3 className="font-medium text-sm tracking-wide uppercase">{title}</h3>
          </div>
          {actions && (
            <div className="flex items-center gap-2" onClick={(e) => e.stopPropagation()}>
              {actions}
            </div>
          )}
        </div>
      )}
      {(!collapsible || isExpanded) && (
        <div className="p-4">
          {children}
        </div>
      )}
    </div>
  )
}
