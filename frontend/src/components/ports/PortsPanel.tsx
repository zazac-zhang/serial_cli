import { usePorts } from '@/contexts/PortContext'
import { useSettings } from '@/contexts/SettingsContext'
import { Panel } from '@/components/ui/panel'
import { cn } from '@/lib/utils'
import { useState } from 'react'
import { RefreshCw, Plug, Settings, Play, Circle, Unplug } from 'lucide-react'
import type { PortConfig } from '@/types/tauri'
import { recentPortsStorage } from '@/lib/storage'

interface ConfiguringPort {
  portName: string
  config: PortConfig
}

const DEFAULT_CONFIG: PortConfig = {
  baudrate: 9600,
  databits: 8,
  stopbits: 1,
  parity: 'none',
  timeout_ms: 100,
  flow_control: 'none',
}

const BAUD_RATES = [1200, 2400, 4800, 9600, 19200, 38400, 57600, 115200, 230400, 460800, 921600]
const DATA_BITS = [5, 6, 7, 8]
const STOP_BITS = [1, 2]
const PARITY_OPTIONS = ['none', 'odd', 'even']
const FLOW_CONTROL_OPTIONS = ['none', 'software', 'hardware']

export function PortsPanel() {
  const {
    availablePorts,
    activePorts,
    isLoading,
    error,
    listPorts,
    openPort,
    closePort,
  } = usePorts()

  const { settings } = useSettings()
  const [configuringPort, setConfiguringPort] = useState<ConfiguringPort | null>(null)
  const [openingPort, setOpeningPort] = useState<string | null>(null)
  const [closingPort, setClosingPort] = useState<string | null>(null)

  // Load default config from settings
  const getDefaultConfig = (): PortConfig => ({
    baudrate: settings.serial.baudRate,
    databits: settings.serial.dataBits,
    stopbits: settings.serial.stopBits,
    parity: settings.serial.parity,
    timeout_ms: 100,
    flow_control: settings.serial.flowControl,
  })

  const handleOpenPort = async (portName: string) => {
    // Check for recent config
    const recentPorts = recentPortsStorage.get()
    const recentConfig = recentPorts.find(p => p.portName === portName)

    setConfiguringPort({
      portName,
      config: recentConfig?.config || getDefaultConfig(),
    })
  }

  const handleConfirmOpen = async () => {
    if (!configuringPort) return

    setOpeningPort(configuringPort.portName)
    try {
      const portId = await openPort(configuringPort.portName, configuringPort.config)

      // Save to recent ports
      recentPortsStorage.add({
        portName: configuringPort.portName,
        config: configuringPort.config,
        lastUsed: Date.now(),
      })

      setConfiguringPort(null)
    } catch (err) {
      console.error('Failed to open port:', err)
    } finally {
      setOpeningPort(null)
    }
  }

  const handleClosePort = async (portId: string) => {
    setClosingPort(portId)
    try {
      await closePort(portId)
    } catch (err) {
      console.error('Failed to close port:', err)
    } finally {
      setClosingPort(null)
    }
  }

  const getPortStatus = (portName: string) => {
    for (const [_, status] of activePorts) {
      if (status.port_name === portName) {
        return status
      }
    }
    return null
  }

  return (
    <div className="space-y-6">
      {/* Port List */}
      <Panel
        title="Serial Ports"
        variant="signal"
        className="w-full"
        actions={
          <button
            onClick={listPorts}
            disabled={isLoading}
            className="flex items-center gap-2 px-3 py-1.5 text-sm rounded-md bg-signal/10 text-signal border border-signal/30 hover:bg-signal/20 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          >
            <RefreshCw size={14} strokeWidth={1.5} className={isLoading ? 'animate-spin' : ''} />
            Refresh
          </button>
        }
      >
        <div className="space-y-4">
          {error && (
            <div className="p-3 rounded-md bg-alert/10 border border-alert/30 text-alert text-sm">
              {error}
            </div>
          )}

          <div className="space-y-2">
            {availablePorts.length === 0 ? (
              <div className="p-8 text-center text-text-tertiary text-sm">
                No serial ports detected. Click "Refresh" to scan.
              </div>
            ) : (
              availablePorts.map((port) => {
                const status = getPortStatus(port.port_name)
                const isOpen = status?.is_open ?? false
                const isConfiguring = configuringPort?.portName === port.port_name
                const isProcessing = openingPort === port.port_name || closingPort === status?.port_id

                return (
                  <div
                    key={port.port_name}
                    className={cn(
                      'p-4 rounded-md border transition-all',
                      isOpen
                        ? 'bg-signal/10 border-signal/30'
                        : 'bg-bg-deep border-border hover:border-signal/50'
                    )}
                  >
                    {isConfiguring ? (
                      <PortConfigForm
                        config={configuringPort.config}
                        onChange={(config) =>
                          setConfiguringPort({ ...configuringPort, config })
                        }
                        onConfirm={handleConfirmOpen}
                        onCancel={() => setConfiguringPort(null)}
                        isProcessing={isProcessing}
                      />
                    ) : (
                      <div className="flex items-center justify-between">
                        <div className="flex items-center gap-3">
                          <div className={cn(
                            'p-2 rounded-md',
                            isOpen ? 'bg-signal/20' : 'bg-bg-elevated'
                          )}>
                            {isOpen ? (
                              <Plug size={18} strokeWidth={1.5} className="text-signal" />
                            ) : (
                              <Unplug size={18} strokeWidth={1.5} className="text-text-tertiary" />
                            )}
                          </div>
                          <div>
                            <div className="font-mono text-sm text-text-primary">
                              {port.port_name}
                            </div>
                            <div className="flex items-center gap-2 mt-0.5">
                              <div className={cn(
                                'flex items-center gap-1.5 text-xs',
                                isOpen ? 'text-signal' : 'text-text-tertiary'
                              )}>
                                <div className={cn(
                                  'w-1.5 h-1.5 rounded-full',
                                  isOpen ? 'bg-signal animate-pulse-slow' : 'bg-text-tertiary'
                                )} />
                                {isOpen ? 'Connected' : 'Disconnected'}
                              </div>
                              <span className="text-xs text-text-tertiary">
                                {port.port_type}
                              </span>
                            </div>
                          </div>
                        </div>

                        <div className="flex items-center gap-2">
                          {isOpen && status ? (
                            <>
                              <button
                                className="p-1.5 rounded hover:bg-bg-elevated text-text-tertiary hover:text-text-primary transition-colors"
                                title="Port settings"
                              >
                                <Settings size={14} strokeWidth={1.5} />
                              </button>
                              <button
                                onClick={() => handleClosePort(status.port_id)}
                                disabled={isProcessing}
                                className="flex items-center gap-1.5 px-3 py-1.5 text-sm rounded-md bg-alert/10 text-alert border border-alert/30 hover:bg-alert/20 transition-colors disabled:opacity-50"
                              >
                                <Unplug size={14} strokeWidth={1.5} />
                                {closingPort === status.port_id ? 'Closing...' : 'Close'}
                              </button>
                            </>
                          ) : (
                            <button
                              onClick={() => handleOpenPort(port.port_name)}
                              disabled={isProcessing}
                              className="flex items-center gap-1.5 px-3 py-1.5 text-sm rounded-md bg-signal/10 text-signal border border-signal/30 hover:bg-signal/20 transition-colors disabled:opacity-50"
                            >
                              <Play size={14} strokeWidth={1.5} />
                              {openingPort === port.port_name ? 'Opening...' : 'Open'}
                            </button>
                          )}
                        </div>
                      </div>
                    )}
                  </div>
                )
              })
            )}
          </div>
        </div>
      </Panel>

      {/* Active Ports Status */}
      {activePorts.size > 0 && (
        <Panel title="Active Connections" variant="info" className="w-full">
          <div className="space-y-3">
            {Array.from(activePorts.values()).map((status) => (
              <div
                key={status.port_id}
                className="p-3 rounded-md bg-bg-deep border border-border"
              >
                <div className="flex items-center justify-between mb-2">
                  <div className="font-mono text-sm text-text-primary">
                    {status.port_name}
                  </div>
                  <div className="flex items-center gap-1.5 text-xs text-signal">
                    <Circle size={8} strokeWidth={2} className="animate-pulse-slow" fill="currentColor" />
                    Active
                  </div>
                </div>
                <div className="grid grid-cols-3 gap-3 text-xs">
                  <div>
                    <span className="text-text-tertiary">Baud Rate:</span>{' '}
                    <span className="font-mono text-text-primary">{status.config.baudrate}</span>
                  </div>
                  <div>
                    <span className="text-text-tertiary">Data Bits:</span>{' '}
                    <span className="font-mono text-text-primary">{status.config.databits}</span>
                  </div>
                  <div>
                    <span className="text-text-tertiary">Parity:</span>{' '}
                    <span className="font-mono text-text-primary">{status.config.parity}</span>
                  </div>
                </div>
              </div>
            ))}
          </div>
        </Panel>
      )}
    </div>
  )
}

interface PortConfigFormProps {
  config: PortConfig
  onChange: (config: PortConfig) => void
  onConfirm: () => void
  onCancel: () => void
  isProcessing: boolean
}

function PortConfigForm({
  config,
  onChange,
  onConfirm,
  onCancel,
  isProcessing,
}: PortConfigFormProps) {
  return (
    <div className="space-y-4">
      <div className="grid grid-cols-2 gap-4">
        {/* Baud Rate */}
        <div>
          <label className="text-xs text-text-tertiary uppercase tracking-wider block mb-2">
            Baud Rate
          </label>
          <select
            value={config.baudrate}
            onChange={(e) => onChange({ ...config, baudrate: parseInt(e.target.value) })}
            className="w-full px-3 py-2 bg-bg-deep border border-border rounded-md text-sm text-text-primary font-mono"
          >
            {BAUD_RATES.map((rate) => (
              <option key={rate} value={rate}>
                {rate}
              </option>
            ))}
          </select>
        </div>

        {/* Data Bits */}
        <div>
          <label className="text-xs text-text-tertiary uppercase tracking-wider block mb-2">
            Data Bits
          </label>
          <select
            value={config.databits}
            onChange={(e) => onChange({ ...config, databits: parseInt(e.target.value) })}
            className="w-full px-3 py-2 bg-bg-deep border border-border rounded-md text-sm text-text-primary font-mono"
          >
            {DATA_BITS.map((bits) => (
              <option key={bits} value={bits}>
                {bits}
              </option>
            ))}
          </select>
        </div>

        {/* Stop Bits */}
        <div>
          <label className="text-xs text-text-tertiary uppercase tracking-wider block mb-2">
            Stop Bits
          </label>
          <select
            value={config.stopbits}
            onChange={(e) => onChange({ ...config, stopbits: parseInt(e.target.value) })}
            className="w-full px-3 py-2 bg-bg-deep border border-border rounded-md text-sm text-text-primary font-mono"
          >
            {STOP_BITS.map((bits) => (
              <option key={bits} value={bits}>
                {bits}
              </option>
            ))}
          </select>
        </div>

        {/* Parity */}
        <div>
          <label className="text-xs text-text-tertiary uppercase tracking-wider block mb-2">
            Parity
          </label>
          <select
            value={config.parity}
            onChange={(e) => onChange({ ...config, parity: e.target.value })}
            className="w-full px-3 py-2 bg-bg-deep border border-border rounded-md text-sm text-text-primary"
          >
            {PARITY_OPTIONS.map((parity) => (
              <option key={parity} value={parity}>
                {parity.charAt(0).toUpperCase() + parity.slice(1)}
              </option>
            ))}
          </select>
        </div>

        {/* Flow Control */}
        <div className="col-span-2">
          <label className="text-xs text-text-tertiary uppercase tracking-wider block mb-2">
            Flow Control
          </label>
          <div className="flex items-center gap-2">
            {FLOW_CONTROL_OPTIONS.map((flow) => (
              <button
                key={flow}
                onClick={() => onChange({ ...config, flow_control: flow })}
                className={cn(
                  'px-4 py-2 text-sm rounded-md border transition-colors',
                  config.flow_control === flow
                    ? 'bg-info/10 text-info border-info/30'
                    : 'bg-bg-elevated text-text-tertiary border-border hover:text-text-primary'
                )}
              >
                {flow.charAt(0).toUpperCase() + flow.slice(1)}
              </button>
            ))}
          </div>
        </div>
      </div>

      {/* Actions */}
      <div className="flex items-center gap-2 pt-2">
        <button
          onClick={onConfirm}
          disabled={isProcessing}
          className="flex items-center gap-1.5 px-4 py-2 text-sm rounded-md bg-signal/10 text-signal border border-signal/30 hover:bg-signal/20 transition-colors disabled:opacity-50"
        >
          <Play size={14} strokeWidth={1.5} />
          {isProcessing ? 'Opening...' : 'Open Port'}
        </button>
        <button
          onClick={onCancel}
          disabled={isProcessing}
          className="px-4 py-2 text-sm rounded-md bg-bg-elevated text-text-secondary border border-border hover:text-text-primary transition-colors disabled:opacity-50"
        >
          Cancel
        </button>
      </div>
    </div>
  )
}
