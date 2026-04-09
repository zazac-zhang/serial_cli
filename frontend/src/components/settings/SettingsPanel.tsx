import { NotificationSettings } from './NotificationSettings'
import { Panel } from '@/components/ui/panel'
import { cn } from '@/lib/utils'
import { useState, useRef } from 'react'
import { Save, RotateCcw, Check, Download, Upload } from 'lucide-react'
import { exportSettings, importSettings } from '@/lib/storage'

type Tab = 'general' | 'serial' | 'data' | 'notifications'

interface SerialConfig {
  baudRate: number
  dataBits: number
  stopBits: number
  parity: 'none' | 'even' | 'odd'
  flowControl: 'none' | 'rts' | 'cts' | 'rtscts'
}

interface DataConfig {
  displayFormat: 'hex' | 'ascii' | 'both'
  showTimestamp: boolean
  maxPackets: number
  autoScroll: boolean
}

export function SettingsPanel() {
  const [activeTab, setActiveTab] = useState<Tab>('general')
  const [hasChanges, setHasChanges] = useState(false)
  const [isImporting, setIsImporting] = useState(false)
  const fileInputRef = useRef<HTMLInputElement>(null)

  // Serial port defaults
  const [serialConfig, setSerialConfig] = useState<SerialConfig>({
    baudRate: 9600,
    dataBits: 8,
    stopBits: 1,
    parity: 'none',
    flowControl: 'none',
  })

  // Data display defaults
  const [dataConfig, setDataConfig] = useState<DataConfig>({
    displayFormat: 'hex',
    showTimestamp: true,
    maxPackets: 1000,
    autoScroll: true,
  })

  const tabs: { id: Tab; label: string; icon: React.ElementType }[] = [
    { id: 'general', label: 'General', icon: () => null },
    { id: 'serial', label: 'Serial', icon: () => null },
    { id: 'data', label: 'Data', icon: () => null },
    { id: 'notifications', label: 'Notifications', icon: () => null },
  ]

  const saveChanges = () => {
    // Save settings to local storage or backend
    setHasChanges(false)
  }

  const resetToDefaults = () => {
    setSerialConfig({
      baudRate: 9600,
      dataBits: 8,
      stopBits: 1,
      parity: 'none',
      flowControl: 'none',
    })
    setDataConfig({
      displayFormat: 'hex',
      showTimestamp: true,
      maxPackets: 1000,
      autoScroll: true,
    })
    setHasChanges(true)
  }

  const handleExport = () => {
    const success = exportSettings()
    if (success) {
      alert('Settings exported successfully!')
    } else {
      alert('Failed to export settings')
    }
  }

  const handleImport = async (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0]
    if (!file) return

    setIsImporting(true)
    try {
      await importSettings(file)
      alert('Settings imported successfully! Page will reload.')
      window.location.reload()
    } catch (error) {
      alert('Failed to import settings: ' + (error instanceof Error ? error.message : 'Unknown error'))
    } finally {
      setIsImporting(false)
      // Reset file input
      if (fileInputRef.current) {
        fileInputRef.current.value = ''
      }
    }
  }

  return (
    <div className="space-y-6 w-full">
      {/* Settings Header */}
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-lg font-medium text-text-primary">Settings</h2>
          <p className="text-sm text-text-tertiary mt-0.5">Configure application preferences</p>
        </div>
        <div className="flex items-center gap-2">
          <button
            onClick={handleExport}
            className="flex items-center gap-1.5 px-3 py-1.5 text-sm rounded-md bg-bg-elevated text-text-secondary border border-border hover:text-text-primary transition-colors"
            title="Export all settings"
          >
            <Download size={14} strokeWidth={1.5} />
            Export
          </button>
          <button
            onClick={() => fileInputRef.current?.click()}
            disabled={isImporting}
            className="flex items-center gap-1.5 px-3 py-1.5 text-sm rounded-md bg-bg-elevated text-text-secondary border border-border hover:text-text-primary transition-colors disabled:opacity-50"
            title="Import settings from file"
          >
            <Upload size={14} strokeWidth={1.5} />
            {isImporting ? 'Importing...' : 'Import'}
          </button>
          <input
            ref={fileInputRef}
            type="file"
            accept=".json"
            className="hidden"
            onChange={handleImport}
          />
          <button
            onClick={resetToDefaults}
            className="flex items-center gap-1.5 px-3 py-1.5 text-sm rounded-md bg-bg-elevated text-text-secondary border border-border hover:text-text-primary transition-colors"
          >
            <RotateCcw size={14} strokeWidth={1.5} />
            Reset
          </button>
          <button
            onClick={saveChanges}
            disabled={!hasChanges}
            className={cn(
              'flex items-center gap-1.5 px-4 py-1.5 text-sm rounded-md border transition-colors',
              hasChanges
                ? 'bg-signal/10 text-signal border-signal/30 hover:bg-signal/20'
                : 'bg-bg-elevated text-text-tertiary border-border cursor-not-allowed'
            )}
          >
            <Save size={14} strokeWidth={1.5} />
            Save
          </button>
        </div>
      </div>

      {/* Tab Navigation */}
      <div className="flex items-center gap-1 border-b border-border">
        {tabs.map((tab) => (
          <button
            key={tab.id}
            onClick={() => setActiveTab(tab.id)}
            className={cn(
              'px-4 py-2 text-sm font-medium border-b-2 -mb-px transition-colors',
              activeTab === tab.id
                ? 'border-signal text-signal'
                : 'border-transparent text-text-tertiary hover:text-text-secondary'
            )}
          >
            {tab.label}
          </button>
        ))}
      </div>

      {/* Tab Content */}
      {activeTab === 'general' && (
        <Panel title="General Settings" variant="default">
          <div className="space-y-6">
            <div className="flex items-center justify-between">
              <div>
                <div className="text-sm text-text-primary">Auto-check for updates</div>
                <div className="text-xs text-text-tertiary">Check for new versions on startup</div>
              </div>
              <ToggleSwitch defaultChecked />
            </div>

            <div className="flex items-center justify-between">
              <div>
                <div className="text-sm text-text-primary">Send usage analytics</div>
                <div className="text-xs text-text-tertiary">Help improve the app with anonymous data</div>
              </div>
              <ToggleSwitch />
            </div>

            <div className="flex items-center justify-between">
              <div>
                <div className="text-sm text-text-primary">Minimize to tray</div>
                <div className="text-xs text-text-tertiary">Close button minimizes instead of quitting</div>
              </div>
              <ToggleSwitch defaultChecked />
            </div>

            <div className="pt-4 border-t border-border">
              <div className="text-sm text-text-primary mb-2">Language</div>
              <select className="w-full max-w-xs px-3 py-2 bg-bg-deep border border-border rounded-md text-sm text-text-primary">
                <option>English</option>
                <option>简体中文</option>
              </select>
            </div>
          </div>
        </Panel>
      )}

      {activeTab === 'serial' && (
        <Panel title="Serial Port Defaults" variant="signal">
          <div className="space-y-4">
            <div className="grid grid-cols-2 gap-4">
              <div>
                <label className="text-xs text-text-tertiary uppercase tracking-wider block mb-2">
                  Baud Rate
                </label>
                <select
                  value={serialConfig.baudRate}
                  onChange={(e) => {
                    setSerialConfig({ ...serialConfig, baudRate: parseInt(e.target.value) })
                    setHasChanges(true)
                  }}
                  className="w-full px-3 py-2 bg-bg-deep border border-border rounded-md text-sm text-text-primary font-mono"
                >
                  <option value={1200}>1200</option>
                  <option value={2400}>2400</option>
                  <option value={4800}>4800</option>
                  <option value={9600}>9600</option>
                  <option value={19200}>19200</option>
                  <option value={38400}>38400</option>
                  <option value={57600}>57600</option>
                  <option value={115200}>115200</option>
                  <option value={230400}>230400</option>
                  <option value={460800}>460800</option>
                  <option value={921600}>921600</option>
                </select>
              </div>

              <div>
                <label className="text-xs text-text-tertiary uppercase tracking-wider block mb-2">
                  Data Bits
                </label>
                <select
                  value={serialConfig.dataBits}
                  onChange={(e) => {
                    setSerialConfig({ ...serialConfig, dataBits: parseInt(e.target.value) })
                    setHasChanges(true)
                  }}
                  className="w-full px-3 py-2 bg-bg-deep border border-border rounded-md text-sm text-text-primary font-mono"
                >
                  <option value={5}>5</option>
                  <option value={6}>6</option>
                  <option value={7}>7</option>
                  <option value={8}>8</option>
                </select>
              </div>

              <div>
                <label className="text-xs text-text-tertiary uppercase tracking-wider block mb-2">
                  Stop Bits
                </label>
                <select
                  value={serialConfig.stopBits}
                  onChange={(e) => {
                    setSerialConfig({ ...serialConfig, stopBits: parseInt(e.target.value) })
                    setHasChanges(true)
                  }}
                  className="w-full px-3 py-2 bg-bg-deep border border-border rounded-md text-sm text-text-primary font-mono"
                >
                  <option value={1}>1</option>
                  <option value={2}>2</option>
                </select>
              </div>

              <div>
                <label className="text-xs text-text-tertiary uppercase tracking-wider block mb-2">
                  Parity
                </label>
                <select
                  value={serialConfig.parity}
                  onChange={(e) => {
                    setSerialConfig({ ...serialConfig, parity: e.target.value as SerialConfig['parity'] })
                    setHasChanges(true)
                  }}
                  className="w-full px-3 py-2 bg-bg-deep border border-border rounded-md text-sm text-text-primary font-mono"
                >
                  <option value="none">None</option>
                  <option value="even">Even</option>
                  <option value="odd">Odd</option>
                </select>
              </div>

              <div>
                <label className="text-xs text-text-tertiary uppercase tracking-wider block mb-2">
                  Flow Control
                </label>
                <select
                  value={serialConfig.flowControl}
                  onChange={(e) => {
                    setSerialConfig({ ...serialConfig, flowControl: e.target.value as SerialConfig['flowControl'] })
                    setHasChanges(true)
                  }}
                  className="w-full px-3 py-2 bg-bg-deep border border-border rounded-md text-sm text-text-primary font-mono"
                >
                  <option value="none">None</option>
                  <option value="rts">RTS</option>
                  <option value="cts">CTS</option>
                  <option value="rtscts">RTS/CTS</option>
                </select>
              </div>
            </div>

            <div className="pt-4 border-t border-border">
              <div className="flex items-center justify-between">
                <div>
                  <div className="text-sm text-text-primary">Auto-reconnect on disconnect</div>
                  <div className="text-xs text-text-tertiary">Automatically reconnect if connection is lost</div>
                </div>
                <ToggleSwitch defaultChecked />
              </div>
            </div>
          </div>
        </Panel>
      )}

      {activeTab === 'data' && (
        <Panel title="Data Display Settings" variant="info">
          <div className="space-y-4">
            <div>
              <label className="text-xs text-text-tertiary uppercase tracking-wider block mb-2">
                Default Display Format
              </label>
              <div className="flex items-center gap-2">
                {(['hex', 'ascii', 'both'] as const).map((format) => (
                  <button
                    key={format}
                    onClick={() => {
                      setDataConfig({ ...dataConfig, displayFormat: format })
                      setHasChanges(true)
                    }}
                    className={cn(
                      'px-4 py-2 text-sm rounded-md border transition-colors',
                      dataConfig.displayFormat === format
                        ? 'bg-info/10 text-info border-info/30'
                        : 'bg-bg-elevated text-text-tertiary border-border hover:text-text-primary'
                    )}
                  >
                    {format.toUpperCase()}
                  </button>
                ))}
              </div>
            </div>

            <div className="flex items-center justify-between">
              <div>
                <div className="text-sm text-text-primary">Show Timestamps</div>
                <div className="text-xs text-text-tertiary">Display timestamp for each packet</div>
              </div>
              <ToggleSwitch
                defaultChecked={dataConfig.showTimestamp}
                onChange={(checked) => {
                  setDataConfig({ ...dataConfig, showTimestamp: checked })
                  setHasChanges(true)
                }}
              />
            </div>

            <div className="flex items-center justify-between">
              <div>
                <div className="text-sm text-text-primary">Auto-scroll</div>
                <div className="text-xs text-text-tertiary">Automatically scroll to latest data</div>
              </div>
              <ToggleSwitch
                defaultChecked={dataConfig.autoScroll}
                onChange={(checked) => {
                  setDataConfig({ ...dataConfig, autoScroll: checked })
                  setHasChanges(true)
                }}
              />
            </div>

            <div className="pt-4 border-t border-border">
              <label className="text-sm font-medium text-text-primary block mb-2">
                Max Packets in Buffer
              </label>
              <input
                type="range"
                min="100"
                max="10000"
                step="100"
                value={dataConfig.maxPackets}
                onChange={(e) => {
                  setDataConfig({ ...dataConfig, maxPackets: parseInt(e.target.value) })
                  setHasChanges(true)
                }}
                className="w-full max-w-xs"
              />
              <div className="text-xs text-text-tertiary font-mono mt-1">
                Current: {dataConfig.maxPackets} packets
              </div>
            </div>
          </div>
        </Panel>
      )}

      {activeTab === 'notifications' && <NotificationSettings />}
    </div>
  )
}

function ToggleSwitch({
  defaultChecked = false,
  onChange,
}: {
  defaultChecked?: boolean
  onChange?: (checked: boolean) => void
}) {
  const [checked, setChecked] = useState(defaultChecked)

  const handleChange = () => {
    const newChecked = !checked
    setChecked(newChecked)
    onChange?.(newChecked)
  }

  return (
    <button
      onClick={handleChange}
      className={cn(
        'w-12 h-6 rounded-full p-1 transition-colors relative',
        checked ? 'bg-signal' : 'bg-bg-elevated'
      )}
    >
      <div className={cn(
        'w-4 h-4 rounded-full bg-white transition-transform flex items-center justify-center',
        checked ? 'translate-x-6' : 'translate-x-0'
      )}>
        {checked && <Check size={10} strokeWidth={3} className="text-signal" />}
      </div>
    </button>
  )
}
