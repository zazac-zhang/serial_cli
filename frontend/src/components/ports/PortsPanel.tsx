import { usePorts } from '@/contexts/PortContext'
import { Panel } from '@/components/ui/panel'

export function PortsPanel() {
  const { availablePorts, isLoading, error, listPorts } = usePorts()

  return (
    <div className="space-y-6">
      <Panel title="Serial Ports" variant="signal" className="max-w-4xl">
        <div className="space-y-4">
          <div className="flex items-center justify-between">
            <div className="text-sm text-text-secondary">
              {availablePorts.length} port{availablePorts.length !== 1 ? 's' : ''} available
            </div>
            <button
              onClick={listPorts}
              disabled={isLoading}
              className="px-4 py-2 text-sm font-medium rounded-md bg-signal/10 text-signal border border-signal/30 hover:bg-signal/20 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
            >
              {isLoading ? 'Scanning...' : 'Refresh Ports'}
            </button>
          </div>

          {error && (
            <div className="p-3 rounded-md bg-alert/10 border border-alert/30 text-alert text-sm">
              {error}
            </div>
          )}

          <div className="space-y-2">
            {availablePorts.length === 0 ? (
              <div className="p-8 text-center text-text-tertiary text-sm">
                No serial ports detected. Click "Refresh Ports" to scan.
              </div>
            ) : (
              availablePorts.map((port) => (
                <div
                  key={port.port_name}
                  className="p-3 rounded-md bg-bg-deep border border-border hover:border-signal/50 transition-colors"
                >
                  <div className="flex items-center justify-between">
                    <span className="font-mono text-sm text-text-primary">{port.port_name}</span>
                    <span className="text-xs text-text-tertiary">{port.port_type}</span>
                  </div>
                </div>
              ))
            )}
          </div>
        </div>
      </Panel>

      <Panel title="Port Configuration" variant="default" className="max-w-4xl">
        <div className="text-sm text-text-tertiary">
          Select a port above to configure and open it.
        </div>
      </Panel>
    </div>
  )
}
