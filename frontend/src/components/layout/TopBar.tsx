import { usePorts } from '@/contexts/PortContext'
import { useData } from '@/contexts/DataContext'
import { Activity, Radio } from 'lucide-react'

export function TopBar() {
  const { availablePorts } = usePorts()
  const { packets } = useData()
  const activePortsCount = availablePorts.length

  return (
    <header className="h-14 border-b border-border bg-bg-deep flex items-center justify-between px-6">
      <div className="flex items-center gap-6">
        {/* Port status */}
        <div className="flex items-center gap-2.5 text-sm">
          <Radio size={14} strokeWidth={1.5} className="text-signal" />
          <span className="text-text-tertiary">Ports:</span>
          <span className="font-mono text-signal">{activePortsCount}</span>
        </div>

        {/* Packet counter */}
        <div className="flex items-center gap-2.5 text-sm">
          <Activity size={14} strokeWidth={1.5} className="text-info" />
          <span className="text-text-tertiary">Packets:</span>
          <span className="font-mono text-info">{packets.length}</span>
        </div>

        {/* Data flow indicator */}
        {packets.length > 0 && (
          <div className="flex items-center gap-1">
            {[1, 2, 3, 4, 5].map((i) => (
              <div
                key={i}
                className="w-0.5 h-3 rounded-full bg-signal/30 animate-pulse"
                style={{
                  animationDelay: `${i * 0.1}s`,
                  animationDuration: '1s',
                }}
              />
            ))}
          </div>
        )}
      </div>

      <div className="flex items-center gap-4">
        {/* System status */}
        <div className="flex items-center gap-2 text-xs">
          <div className="flex items-center gap-1.5 px-2.5 py-1 rounded-full bg-signal/10 border border-signal/20">
            <div className="w-1.5 h-1.5 rounded-full bg-signal animate-pulse-slow"></div>
            <span className="text-signal font-medium tracking-wide">SYSTEM READY</span>
          </div>
        </div>
      </div>
    </header>
  )
}
