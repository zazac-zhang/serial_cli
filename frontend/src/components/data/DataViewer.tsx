import { useData } from '@/contexts/DataContext'
import { usePorts } from '@/contexts/PortContext'
import { useSettings } from '@/contexts/SettingsContext'
import { Panel } from '@/components/ui/panel'
import { cn } from '@/lib/utils'
import { Trash2, Download, Settings2, ArrowUpRight, ArrowDownLeft, Send, Play, AlertCircle } from 'lucide-react'
import { useState, useMemo, useRef, useEffect } from 'react'
import React from 'react'
import { invoke } from '@tauri-apps/api/core'
import type { DataPacket } from '@/types/tauri'

// Utility functions for data formatting
export function formatData(data: number[], format: 'hex' | 'ascii'): string {
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

export function formatTimestamp(timestamp: number): string {
  const date = new Date(timestamp)
  return date.toLocaleTimeString('en-US', {
    hour12: false,
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit',
  })
}

// Memoized data packet row component
const DataPacketRow = React.memo(({ packet, displayFormat }: {
  packet: DataPacket
  displayFormat: 'hex' | 'ascii'
}) => {
  return (
    <div
      className={cn(
        'group flex items-center gap-3 px-3 py-2 rounded-md font-mono text-xs transition-colors',
        packet.direction === 'rx'
          ? 'bg-signal/5 hover:bg-signal/10'
          : 'bg-amber/5 hover:bg-amber/10',
      )}
    >
      {/* Timestamp */}
      {true && (
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
        {formatData(packet.data, displayFormat)}
      </span>

      {/* Byte count */}
      <span className="text-text-tertiary text-[10px] w-8 text-right flex-shrink-0">
        {packet.data.length}B
      </span>
    </div>
  )
})

DataPacketRow.displayName = 'DataPacketRow'

type ExportFormat = 'txt' | 'csv' | 'json'
type ExportOption = 'all' | 'rx-only' | 'tx-only'

export const DataViewer = React.memo(function DataViewer() {
  const { packets, clearPackets, displayOptions, setDisplayOptions, maxPackets } = useData()
  const { activePorts } = usePorts()
  const { settings, updateSettings } = useSettings()
  const [autoScroll, setAutoScroll] = useState(settings.display.autoScroll)
  const [exportFormat, setExportFormat] = useState<ExportFormat>(
    (localStorage.getItem('serial-cli-export-format') as ExportFormat) || 'txt'
  )
  const [exportOption, setExportOption] = useState<ExportOption>(
    (localStorage.getItem('serial-cli-export-option') as ExportOption) || 'all'
  )
  const [showExportMenu, setShowExportMenu] = useState(false)
  const dataListRef = useRef<HTMLDivElement>(null)

  // Auto-scroll to bottom when new packets arrive
  useEffect(() => {
    if (autoScroll && dataListRef.current && packets.length > 0) {
      dataListRef.current.scrollTo({ top: dataListRef.current.scrollHeight })
    }
  }, [packets.length, autoScroll])

  // Data sending states
  const [sendData, setSendData] = useState('')
  const [sendFormat, setSendFormat] = useState<'hex' | 'ascii'>('hex')
  const [selectedPort, setSelectedPort] = useState<string>('')
  const [isSending, setIsSending] = useState(false)

  const handleExportFormat = (fmt: ExportFormat) => {
    setExportFormat(fmt)
    localStorage.setItem('serial-cli-export-format', fmt)
  }

  const handleExportOption = (opt: ExportOption) => {
    setExportOption(opt)
    localStorage.setItem('serial-cli-export-option', opt)
  }

  const warningThreshold = Math.floor(maxPackets * 0.8)

  // Memoized calculations
  const packetStats = useMemo(() => ({
    total: packets.length,
    rx: packets.filter(p => p.direction === 'rx').length,
    tx: packets.filter(p => p.direction === 'tx').length,
    memoryUsagePercent: (packets.length / maxPackets) * 100,
  }), [packets.length, maxPackets])

  const exportData = () => {
    const filteredPackets = packets.filter(p => {
      if (exportOption === 'rx-only') return p.direction === 'rx'
      if (exportOption === 'tx-only') return p.direction === 'tx'
      return true
    })

    let content: string
    let filename: string
    let mimeType: string

    if (exportFormat === 'csv') {
      const headers = ['Timestamp', 'Direction', 'Port', 'Data (Hex)', 'Data (ASCII)', 'Bytes']
      const rows = filteredPackets.map(p => [
        formatTimestamp(p.timestamp),
        p.direction.toUpperCase(),
        p.port_id,
        formatData(p.data, 'hex'),
        formatData(p.data, 'ascii'),
        p.data.length.toString(),
      ])
      content = [headers, ...rows].map(row => row.join(',')).join('\n')
      filename = `serial-data-${Date.now()}.csv`
      mimeType = 'text/csv'
    } else if (exportFormat === 'json') {
      content = JSON.stringify(filteredPackets.map(p => ({
        timestamp: p.timestamp,
        timestamp_formatted: formatTimestamp(p.timestamp),
        direction: p.direction,
        port_id: p.port_id,
        data_hex: formatData(p.data, 'hex'),
        data_ascii: formatData(p.data, 'ascii'),
        bytes: p.data.length,
      })), null, 2)
      filename = `serial-data-${Date.now()}.json`
      mimeType = 'application/json'
    } else {
      content = filteredPackets.map(p => {
        const dir = p.direction === 'rx' ? 'RX' : 'TX'
        const data = formatData(p.data, displayOptions.format)
        return `[${formatTimestamp(p.timestamp)}] ${dir}: ${data}`
      }).join('\n')
      filename = `serial-data-${Date.now()}.txt`
      mimeType = 'text/plain'
    }

    const blob = new Blob([content], { type: mimeType })
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = filename
    a.click()
    URL.revokeObjectURL(url)
  }

  const handleSendData = async () => {
    if (!selectedPort || !sendData.trim()) {
      return
    }

    setIsSending(true)
    try {
      let dataToSend: number[] = []

      if (sendFormat === 'hex') {
        // Parse hex input (e.g., "01 02 AB CD" or "0102ABCD")
        const cleanHex = sendData.replace(/\s+/g, '')
        if (cleanHex.length % 2 !== 0) {
          throw new Error('Hex string must have even length')
        }
        for (let i = 0; i < cleanHex.length; i += 2) {
          const byte = parseInt(cleanHex.substr(i, 2), 16)
          if (isNaN(byte)) {
            throw new Error('Invalid hex character')
          }
          dataToSend.push(byte)
        }
      } else {
        // ASCII input
        dataToSend = sendData.split('').map(char => char.charCodeAt(0))
      }

      // Send data via Tauri
      await invoke('send_data', {
        portId: selectedPort,
        data: dataToSend,
      })

      console.log('Data sent successfully')
      setSendData('') // Clear input after successful send
    } catch (error) {
      console.error('Failed to send data:', error)
      alert(`Failed to send data: ${error instanceof Error ? error.message : 'Unknown error'}`)
    } finally {
      setIsSending(false)
    }
  }

  return (
    <div className="space-y-6">
      {/* Stats Overview */}
      <div className="grid grid-cols-1 md:grid-cols-4 gap-4 w-full">
        <Panel variant="info">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-xs text-text-tertiary uppercase tracking-wider">Total Packets</p>
              <p className="text-2xl font-mono font-semibold text-text-primary mt-1">{packetStats.total}</p>
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
              <p className="text-2xl font-mono font-semibold text-text-primary mt-1">{packetStats.rx}</p>
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
              <p className="text-2xl font-mono font-semibold text-text-primary mt-1">{packetStats.tx}</p>
            </div>
            <div className="p-2 rounded-lg bg-amber/10">
              <ArrowUpRight size={20} className="text-amber" strokeWidth={1.5} />
            </div>
          </div>
        </Panel>

        {/* Memory Usage Indicator */}
        <Panel
          variant={packetStats.memoryUsagePercent > 80 ? "alert" : "default"}
          className="relative"
        >
          <div className="flex items-center justify-between mb-2">
            <div>
              <p className="text-xs text-text-tertiary uppercase tracking-wider">Memory Usage</p>
              <p className="text-lg font-mono font-semibold text-text-primary mt-1">
                {packetStats.total}/{maxPackets}
              </p>
            </div>
            {packetStats.memoryUsagePercent > 80 && (
              <div className="p-2 rounded-lg bg-alert/10">
                <AlertCircle size={16} className="text-alert" strokeWidth={1.5} />
              </div>
            )}
          </div>

          {/* Progress bar */}
          <div className="w-full bg-bg-deep rounded-full h-1.5 overflow-hidden">
            <div
              className={cn(
                "h-full transition-all duration-300",
                packetStats.memoryUsagePercent > 80
                  ? "bg-alert animate-pulse"
                  : packetStats.memoryUsagePercent > 70
                  ? "bg-amber"
                  : "bg-signal"
              )}
              style={{ width: `${Math.min(packetStats.memoryUsagePercent, 100)}%` }}
            />
          </div>

          {packetStats.memoryUsagePercent > 80 && (
            <p className="text-xs text-alert mt-1.5">
              Auto-cleanup will trigger soon
            </p>
          )}
        </Panel>
      </div>

      {/* Data Send Panel */}
      <Panel
        title="Send Data"
        variant="amber"
        className="w-full"
        actions={
          <div className="flex items-center gap-2">
            <select
              value={sendFormat}
              onChange={(e) => setSendFormat(e.target.value as 'hex' | 'ascii')}
              className="px-2 py-1 text-xs rounded border border-border bg-bg-deep text-text-primary"
            >
              <option value="hex">HEX</option>
              <option value="ascii">ASCII</option>
            </select>
            <button
              onClick={handleSendData}
              disabled={!selectedPort || !sendData.trim() || isSending}
              className="flex items-center gap-1.5 px-3 py-1.5 text-xs rounded-md bg-amber/10 text-amber border border-amber/30 hover:bg-amber/20 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
            >
              {isSending ? (
                <>
                  <Play size={12} strokeWidth={1.5} className="animate-pulse" />
                  Sending...
                </>
              ) : (
                <>
                  <Send size={12} strokeWidth={1.5} />
                  Send
                </>
              )}
            </button>
          </div>
        }
      >
        <div className="space-y-4">
          {/* Port Selection */}
          <div>
            <label className="text-xs text-text-tertiary uppercase tracking-wider block mb-2">
              Target Port
            </label>
            <select
              value={selectedPort}
              onChange={(e) => setSelectedPort(e.target.value)}
              className="w-full px-3 py-2 bg-bg-deep border border-border rounded-md text-sm text-text-primary font-mono"
              disabled={activePorts.size === 0}
            >
              <option value="">Select a port...</option>
              {Array.from(activePorts.values()).map((port) => (
                <option key={port.port_id} value={port.port_id}>
                  {port.port_name} ({port.port_id})
                </option>
              ))}
            </select>
            {activePorts.size === 0 && (
              <p className="text-xs text-text-tertiary mt-1">No active ports. Open a port first.</p>
            )}
          </div>

          {/* Data Input */}
          <div>
            <label className="text-xs text-text-tertiary uppercase tracking-wider block mb-2">
              Data to Send ({sendFormat.toUpperCase()})
            </label>
            <textarea
              value={sendData}
              onChange={(e) => setSendData(e.target.value)}
              placeholder={sendFormat === 'hex' ? "01 02 AB CD ..." : "Enter text to send..."}
              className="w-full px-3 py-2 bg-bg-deep border border-border rounded-md text-sm text-text-primary font-mono resize-none"
              rows={3}
              disabled={!selectedPort}
            />
            <div className="flex items-center justify-between mt-2">
              <p className="text-xs text-text-tertiary">
                {sendFormat === 'hex' ? 'Enter hex bytes separated by spaces' : 'Enter plain text'}
              </p>
              {sendData && (
                <span className="text-xs text-text-tertiary font-mono">
                  {sendFormat === 'hex' ? `${sendData.replace(/\s+/g, '').length / 2} bytes` : `${sendData.length} bytes`}
                </span>
              )}
            </div>
          </div>

          {/* Quick Actions */}
          <div className="flex items-center gap-2 pt-2 border-t border-border">
            <span className="text-xs text-text-tertiary">Quick:</span>
            <button
              onClick={() => setSendData('Hello, World!')}
              className="px-2 py-1 text-xs rounded border border-border hover:bg-bg-elevated transition-colors"
              disabled={!selectedPort}
            >
              Hello
            </button>
            <button
              onClick={() => setSendData('AT')}
              className="px-2 py-1 text-xs rounded border border-border hover:bg-bg-elevated transition-colors"
              disabled={!selectedPort}
            >
              AT
            </button>
            <button
              onClick={() => setSendData('\\r\\n')}
              className="px-2 py-1 text-xs rounded border border-border hover:bg-bg-elevated transition-colors"
              disabled={!selectedPort}
            >
              CRLF
            </button>
          </div>
        </div>
      </Panel>

      {/* Data Monitor Panel */}
      <Panel
        title="Data Monitor"
        variant="default"
        className="w-full"
        actions={
          <>
            <button
              onClick={() => setDisplayOptions({ format: displayOptions.format === 'hex' ? 'ascii' : 'hex' })}
              className="px-2 py-1 text-xs rounded border border-border hover:bg-bg-elevated transition-colors"
            >
              {displayOptions.format.toUpperCase()}
            </button>
            <div className="relative">
              <button
                onClick={() => setShowExportMenu(!showExportMenu)}
                className="p-1.5 rounded hover:bg-bg-elevated text-text-tertiary hover:text-text-primary transition-colors"
                title="Export data"
              >
                <Download size={14} strokeWidth={1.5} />
              </button>
              {showExportMenu && (
                <div className="absolute right-0 top-full mt-2 w-64 bg-bg-floating border border-border rounded-lg shadow-xl z-10 p-3">
                  <div className="space-y-3">
                    <div>
                      <label className="text-xs text-text-tertiary uppercase tracking-wider block mb-2">
                        Format
                      </label>
                      <div className="flex items-center gap-2">
                        {(['txt', 'csv', 'json'] as ExportFormat[]).map((fmt) => (
                          <button
                            key={fmt}
                            onClick={() => handleExportFormat(fmt)}
                            className={cn(
                              'flex-1 px-2 py-1 text-xs rounded-md border transition-colors',
                              exportFormat === fmt
                                ? 'bg-info/10 text-info border-info/30'
                                : 'bg-bg-elevated text-text-tertiary border-border hover:text-text-primary'
                            )}
                          >
                            {fmt.toUpperCase()}
                          </button>
                        ))}
                      </div>
                    </div>
                    <div>
                      <label className="text-xs text-text-tertiary uppercase tracking-wider block mb-2">
                        Filter
                      </label>
                      <div className="flex items-center gap-2">
                        {(['all', 'rx-only', 'tx-only'] as ExportOption[]).map((opt) => (
                          <button
                            key={opt}
                            onClick={() => handleExportOption(opt)}
                            className={cn(
                              'flex-1 px-2 py-1 text-xs rounded-md border transition-colors',
                              exportOption === opt
                                ? 'bg-info/10 text-info border-info/30'
                                : 'bg-bg-elevated text-text-tertiary border-border hover:text-text-primary'
                            )}
                          >
                            {opt === 'all' ? 'All' : opt === 'rx-only' ? 'RX' : 'TX'}
                          </button>
                        ))}
                      </div>
                    </div>
                    <button
                      onClick={() => {
                        exportData()
                        setShowExportMenu(false)
                      }}
                      className="w-full px-3 py-2 text-sm rounded-md bg-signal/10 text-signal border border-signal/30 hover:bg-signal/20 transition-colors"
                    >
                      Export {packets.filter(p => {
                        if (exportOption === 'rx-only') return p.direction === 'rx'
                        if (exportOption === 'tx-only') return p.direction === 'tx'
                        return true
                      }).length} packets
                    </button>
                  </div>
                </div>
              )}
            </div>
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
                onChange={(e) => {
                  setDisplayOptions({ showTimestamp: e.target.checked })
                  updateSettings({ display: { showTimestamp: e.target.checked } })
                }}
                className="rounded border-border bg-bg-elevated text-signal focus:ring-signal/20"
              />
              <span>Timestamp</span>
            </label>
            <label className="flex items-center gap-2 text-sm text-text-secondary cursor-pointer">
              <input
                type="checkbox"
                checked={autoScroll}
                onChange={(e) => {
                  setAutoScroll(e.target.checked)
                  updateSettings({ display: { autoScroll: e.target.checked } })
                }}
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
        <div ref={dataListRef} className="space-y-1 flex-1 overflow-y-auto pr-2 scrollbar-thin" style={{ maxHeight: 'calc(100vh - 400px)' }}>
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
              <DataPacketRow
                key={`${packet.timestamp}-${index}`}
                packet={packet}
                displayFormat={displayOptions.format}
              />
            ))
          )}
        </div>
      </Panel>
    </div>
  )
})

DataViewer.displayName = 'DataViewer'
