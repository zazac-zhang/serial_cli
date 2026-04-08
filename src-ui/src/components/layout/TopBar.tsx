import { usePorts } from '@/contexts/PortContext'
import { useData } from '@/contexts/DataContext'
import { minimizeWindow, hideWindow } from '@/hooks/useWindow'

export function TopBar() {
  const { availablePorts } = usePorts()
  const { packets } = useData()
  const activePortsCount = availablePorts.length

  return (
    <header className="h-14 border-b border-border bg-bg-deep flex items-center justify-between px-6">
      <div className="flex items-center gap-6">
        <div className="flex items-center gap-2 text-sm">
          <span className="text-text-tertiary">Ports:</span>
          <span className="font-mono text-signal">{activePortsCount}</span>
        </div>

        <div className="flex items-center gap-2 text-sm">
          <span className="text-text-tertiary">Packets:</span>
          <span className="font-mono text-info">{packets.length}</span>
        </div>
      </div>

      <div className="flex items-center gap-4">
        <div className="flex items-center gap-2 text-xs text-text-tertiary">
          <div className="w-2 h-2 rounded-full bg-signal animate-pulse"></div>
          <span>System Ready</span>
        </div>

        {/* Window controls */}
        <div className="flex items-center gap-1 ml-4">
          <button
            onClick={minimizeWindow}
            className="w-8 h-8 flex items-center justify-center rounded hover:bg-bg-elevated text-text-secondary hover:text-text-primary transition-colors"
            title="Minimize"
          >
            <svg width="14" height="14" viewBox="0 0 14 14" fill="none">
              <rect x="2" y="6" width="10" height="2" fill="currentColor"/>
            </svg>
          </button>
          <button
            onClick={hideWindow}
            className="w-8 h-8 flex items-center justify-center rounded hover:bg-bg-elevated text-text-secondary hover:text-text-primary transition-colors"
            title="Hide to tray"
          >
            <svg width="14" height="14" viewBox="0 0 14 14" fill="none">
              <rect x="3" y="3" width="8" height="8" fill="currentColor" opacity="0.3"/>
              <path d="M3 3h8v8H3z" stroke="currentColor" strokeWidth="1.5"/>
            </svg>
          </button>
        </div>
      </div>
    </header>
  )
}
