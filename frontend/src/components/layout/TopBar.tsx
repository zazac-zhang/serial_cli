import { usePorts } from '@/contexts/PortContext'
import { useData } from '@/contexts/DataContext'
import { Activity, Radio, Zap } from 'lucide-react'
import { useEffect, useState } from 'react'
import { cn } from '@/lib/utils'

export function TopBar() {
  const { availablePorts, activePorts } = usePorts()
  const { packets } = useData()
  const [dataFlowRate, setDataFlowRate] = useState(0)
  const [lastPacketCount, setLastPacketCount] = useState(0)

  const activePortsCount = availablePorts.length
  const totalPackets = packets.length

  // Calculate data flow rate (packets per second)
  useEffect(() => {
    const interval = setInterval(() => {
      const currentCount = packets.length
      const packetsPerSecond = currentCount - lastPacketCount
      setDataFlowRate(packetsPerSecond)
      setLastPacketCount(currentCount)
    }, 1000)

    return () => clearInterval(interval)
  }, [packets.length, lastPacketCount])

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
          <span className="font-mono text-info">{totalPackets}</span>
        </div>

        {/* Data flow rate */}
        <div className="flex items-center gap-2.5 text-sm">
          <Zap size={14} strokeWidth={1.5} className="text-amber" />
          <span className="text-text-tertiary">Flow:</span>
          <span className="font-mono text-amber">{dataFlowRate}/s</span>
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
          <div className={cn(
            "flex items-center gap-1.5 px-2.5 py-1 rounded-full border",
            activePorts.size > 0
              ? "bg-signal/10 border-signal/20"
              : "bg-bg-elevated border-border"
          )}>
            <div className={cn(
              "w-1.5 h-1.5 rounded-full",
              activePorts.size > 0 ? "bg-signal animate-pulse-slow" : "bg-text-tertiary"
            )}></div>
            <span className={cn(
              "font-medium tracking-wide",
              activePorts.size > 0 ? "text-signal" : "text-text-tertiary"
            )}>
              {activePorts.size > 0 ? "ACTIVE" : "IDLE"}
            </span>
          </div>
        </div>
      </div>
    </header>
  )
}
