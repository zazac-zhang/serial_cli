import * as React from "react"
import { cn } from "@/lib/utils"
import { cva, type VariantProps } from "class-variance-authority"

const panelVariants = cva(
  "rounded-lg border bg-card text-card-foreground shadow-md transition-all hover:shadow-lg",
  {
    variants: {
      variant: {
        default: "border-border bg-bg-floating",
        signal: "border-signal/30 bg-signal/5 hover:border-signal/50",
        amber: "border-amber/30 bg-amber/5 hover:border-amber/50",
        alert: "border-alert/30 bg-alert/5 hover:border-alert/50",
        info: "border-info/30 bg-info/5 hover:border-info/50",
      },
    },
    defaultVariants: {
      variant: "default",
    },
  }
)

export interface PanelProps extends VariantProps<typeof panelVariants> {
  title?: string
  children: React.ReactNode
  className?: string
}

export function Panel({ title, variant, children, className }: PanelProps) {
  return (
    <div className={cn(panelVariants({ variant }), className)}>
      {title && (
        <div className="border-b border-border/50 px-4 py-3">
          <h3 className="font-medium text-sm text-text-primary">{title}</h3>
        </div>
      )}
      <div className="p-4">{children}</div>
    </div>
  )
}
