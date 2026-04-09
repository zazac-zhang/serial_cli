import { useData } from '@/contexts/DataContext'
import { Panel } from '@/components/ui/panel'
import { cn } from '@/lib/utils'
import { Trash2, Download, Settings2, ArrowUpRight, ArrowDownLeft } from 'lucide-react'
import { useState } from 'react'

export function DataViewer() {
  const { packets, clearPackets, displayOptions, setDisplayOptions } = useData()
  const [autoScroll, setAutoScroll] = useState(true)

  const formatData = (data: number[], format: 'hex' | 'ascii') => {
    if (format === 'hex') {
      return data.map(b => b.toString(16).padStart(2, '0').toUpperCase()).join(' ')
    }
    return data.map(b => {
      if (b >= 32 && b <= 126) {
        return String.fromCharCode(b)
      }
      return '·'
    }).join('')
  }

  const formatTimestamp = (timestamp: number) => {
    const date = new Date(timestamp)
    return date.toLocaleTimeString('en-US', {
      hour12: false,
      hour: '2-digit',
      minute: '2-digit',
      second: '2-digit',
    })
  }

  const exportData = () => {
    const content = packets.map(p => {
      const dir = p.direction === 'rx' ? 'RX' : 'TX'
      const data = formatData(p.data, displayOptions.format)
      return `[${formatTimestamp(p.timestamp)}] ${dir}: ${data}`
    }).join('\n')

    const blob = new Blob([content], { type: 'text/plain' })
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = `serial-data-${Date.now()}.txt`
    a.click()
    URL.revokeObjectURL(url)
  }

  const rxCount = packets.filter(p => p.direction === 'rx').length
  const txCount = packets.filter(p => p.direction === 'tx').length

  return (
    <div className="space-y-6">
      {/* Stats Overview */}
      <div className="grid grid-cols-3 gap-4 max-w-4xl">
        <Panel variant="info">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-xs text-text-tertiary uppercase tracking-wider">Total Packets</p>
              <p className="text-2xl font-mono font-semibold text-text-primary mt-1">{packets.length}</p>
            </div>
            <div className="p-2 rounded-lg bg-info/10">
              <ArrowUpRight size={20} className="text-info" strokeWidth={1.5} />
            </div>
          </div>
        </Panel>

        <Panel variant="signal">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-xs text-text-tertiary uppercase tracking-wider">Received (RX)</p>
              <p className="text-2xl font-mono font-semibold text-text-primary mt-1">{rxCount}</p>
            </div>
            <div className="p-2 rounded-lg bg-signal/10">
              <ArrowDownLeft size={20} className="text-signal" strokeWidth={1.5} />
            </div>
          </div>
        </Panel>

        <Panel variant="amber">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-xs text-text-tertiary uppercase tracking-wider">Transmitted (TX)</p>
              <p className="text-2xl font-mono font-semibold text-text-primary mt-1">{txCount}</p>
            </div>
            <div className="p-2 rounded-lg bg-amber/10">
              <ArrowUpRight size={20} className="text-amber" strokeWidth={1.5} />
            </div>
          </div>
        </Panel>
      </div>

      {/* Data Monitor Panel */}
      <Panel
        title="Data Monitor"
        variant="default"
        className="max-w-6xl"
        actions={
          <>
            <button
              onClick={() => setDisplayOptions({ format: displayOptions.format === 'hex' ? 'ascii' : 'hex' })}
              className="px-2 py-1 text-xs rounded border border-border hover:bg-bg-elevated transition-colors"
            >
              {displayOptions.format.toUpperCase()}
            </button>
            <button
              onClick={exportData}
              className="p-1.5 rounded hover:bg-bg-elevated text-text-tertiary hover:text-text-primary transition-colors"
              title="Export data"
            >
              <Download size={14} strokeWidth={1.5} />
            </button>
            <button
              onClick={clearPackets}
              className="p-1.5 rounded hover:bg-alert/20 text-text-tertiary hover:text-alert transition-colors"
              title="Clear data"
            >
              <Trash2 size={14} strokeWidth={1.5} />
            </button>
          </>
        }
      >
        {/* Display options */}
        <div className="flex items-center justify-between mb-4 pb-3 border-b border-border/50">
          <div className="flex items-center gap-4">
            <label className="flex items-center gap-2 text-sm text-text-secondary cursor-pointer">
              <input
                type="checkbox"
                checked={displayOptions.showTimestamp}
                onChange={(e) => setDisplayOptions({ showTimestamp: e.target.checked })}
                className="rounded border-border bg-bg-elevated text-signal focus:ring-signal/20"
              />
              <span>Timestamp</span>
            </label>
            <label className="flex items-center gap-2 text-sm text-text-secondary cursor-pointer">
              <input
                type="checkbox"
                checked={autoScroll}
                onChange={(e) => setAutoScroll(e.target.checked)}
                className="rounded border-border bg-bg-elevated text-signal focus:ring-signal/20"
              />
              <span>Auto-scroll</span>
            </label>
          </div>
          <div className="text-xs text-text-tertiary font-mono">
            {packets.length > 0 ? (
              <span>Latest: {formatTimestamp(packets[packets.length - 1].timestamp)}</span>
            ) : (
              <span>Waiting for data...</span>
            )}
          </div>
        </div>

        {/* Data table */}
        <div className="space-y-1 max-h-[600px] overflow-y-auto pr-2 scrollbar-thin">
          {packets.length === 0 ? (
            <div className="py-16 text-center">
              <div className="inline-flex items-center justify-center w-16 h-16 rounded-full bg-bg-elevated mb-4">
                <ArrowDownLeft size={24} className="text-text-tertiary" strokeWidth={1.5} />
              </div>
              <p className="text-text-tertiary text-sm">No data received yet</p>
              <p className="text-text-tertiary text-xs mt-1">Data will appear here when serial communication is active</p>
            </div>
          ) : (
            packets.map((packet, index) => (
              <div
                key={`${packet.timestamp}-${index}`}
                className={cn(
                  'group flex items-center gap-3 px-3 py-2 rounded-md font-mono text-xs transition-colors',
                  packet.direction === 'rx'
                    ? 'bg-signal/5 hover:bg-signal/10'
                    : 'bg-amber/5 hover:bg-amber/10',
                )}
              >
                {/* Timestamp */}
                {displayOptions.showTimestamp && (
                  <span className="text-text-tertiary w-24 flex-shrink-0">
                    {formatTimestamp(packet.timestamp)}
                  </span>
                )}

                {/* Direction badge */}
                <span className={cn(
                  'px-1.5 py-0.5 rounded text-[10px] font-medium uppercase tracking-wider w-12 text-center flex-shrink-0',
                  packet.direction === 'rx'
                    ? 'bg-signal/20 text-signal'
                    : 'bg-amber/20 text-amber',
                )}>
                  {packet.direction === 'rx' ? 'RX' : 'TX'}
                </span>

                {/* Port ID */}
                <span className="text-text-tertiary w-20 flex-shrink-0 truncate">
                  {packet.port_id}
                </span>

                {/* Data */}
                <span className={cn(
                  'flex-1 break-all',
                  packet.direction === 'rx' ? 'text-text-primary' : 'text-text-secondary',
                )}>
                  {formatData(packet.data, displayOptions.format)}
                </span>

                {/* Byte count */}
                <span className="text-text-tertiary text-[10px] w-8 text-right flex-shrink-0">
                  {packet.data.length}B
                </span>
              </div>
            ))
          )}
        </div>
      </Panel>
    </div>
  )
}
