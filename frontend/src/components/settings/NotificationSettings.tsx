import { useNotification } from '@/contexts/NotificationContext'
import { Panel } from '@/components/ui/panel'
import { cn } from '@/lib/utils'
import { useState } from 'react'

export function NotificationSettings() {
  const { settings, updateSettings, requestNotificationPermission, sendSystemNotification } = useNotification()
  const [isRequesting, setIsRequesting] = useState(false)
  const [permissionStatus, setPermissionStatus] = useState<NotificationPermission | 'unsupported'>(
    'Notification' in window ? Notification.permission : 'unsupported'
  )

  const handleRequestPermission = async () => {
    setIsRequesting(true)
    const granted = await requestNotificationPermission()
    setPermissionStatus(granted ? 'granted' : 'denied')
    setIsRequesting(false)
  }

  const getPermissionStatus = (): 'granted' | 'denied' | 'default' | 'unsupported' => {
    if (permissionStatus === 'unsupported') return 'unsupported'
    if (permissionStatus === 'granted') return 'granted'
    if (permissionStatus === 'denied') return 'denied'
    return 'default'
  }

  return (
    <div className="space-y-4">
      <Panel title="Notification Settings" variant="default">
        <div className="space-y-6">
          {/* Permission Status */}
          <div>
            <div className="flex items-center justify-between mb-3">
              <label className="text-sm font-medium text-text-primary">
                System Notifications
              </label>
              <div className={cn(
                'text-xs px-2 py-1 rounded font-medium',
                getPermissionStatus() === 'granted' && 'bg-signal/10 text-signal',
                getPermissionStatus() === 'denied' && 'bg-alert/10 text-alert',
                getPermissionStatus() === 'default' && 'bg-amber/10 text-amber',
                getPermissionStatus() === 'unsupported' && 'bg-bg-elevated text-text-tertiary'
              )}>
                {getPermissionStatus() === 'granted' && '✓ Enabled'}
                {getPermissionStatus() === 'denied' && '✕ Blocked'}
                {getPermissionStatus() === 'default' && 'Request Permission'}
                {getPermissionStatus() === 'unsupported' && 'Not Supported'}
              </div>
            </div>
            <p className="text-xs text-text-tertiary mb-3">
              Enable system notifications to stay informed about important events even when the app is in the background.
            </p>
            {getPermissionStatus() === 'default' && (
              <button
                onClick={handleRequestPermission}
                disabled={isRequesting}
                className="px-4 py-2 text-sm font-medium rounded-md bg-signal/10 text-signal border border-signal/30 hover:bg-signal/20 transition-colors disabled:opacity-50"
              >
                {isRequesting ? 'Requesting...' : 'Enable Notifications'}
              </button>
            )}
            {getPermissionStatus() === 'denied' && (
              <p className="text-xs text-alert">
                Notifications are blocked. Enable them in your browser/system settings.
              </p>
            )}
          </div>

          {/* Notification Settings */}
          <div className="space-y-4 pt-4 border-t border-border">
            <h3 className="text-sm font-medium text-text-primary">Notification Preferences</h3>

            {/* Master Toggle */}
            <div className="flex items-center justify-between">
              <div>
                <div className="text-sm text-text-primary">Enable Notifications</div>
                <div className="text-xs text-text-tertiary">Master toggle for all notifications</div>
              </div>
              <button
                onClick={() => updateSettings({ enabled: !settings.enabled })}
                className={cn(
                  'w-12 h-6 rounded-full p-1 transition-colors relative',
                  settings.enabled ? 'bg-signal' : 'bg-bg-elevated'
                )}
              >
                <div className={cn(
                  'w-4 h-4 rounded-full bg-white transition-transform',
                  settings.enabled ? 'translate-x-6' : 'translate-x-0'
                )} />
              </button>
            </div>

            {/* Sound */}
            <div className="flex items-center justify-between">
              <div>
                <div className="text-sm text-text-primary">Notification Sound</div>
                <div className="text-xs text-text-tertiary">Play sound for notifications</div>
              </div>
              <button
                onClick={() => updateSettings({ sound: !settings.sound })}
                disabled={!settings.enabled}
                className={cn(
                  'w-12 h-6 rounded-full p-1 transition-colors relative',
                  settings.sound ? 'bg-signal' : 'bg-bg-elevated'
                )}
              >
                <div className={cn(
                  'w-4 h-4 rounded-full bg-white transition-transform',
                  settings.sound ? 'translate-x-6' : 'translate-x-0'
                )} />
              </button>
            </div>

            {/* Port Events */}
            <div className="flex items-center justify-between">
              <div>
                <div className="text-sm text-text-primary">Port Events</div>
                <div className="text-xs text-text-tertiary">Notify when ports connect/disconnect</div>
              </div>
              <button
                onClick={() => updateSettings({ portEvents: !settings.portEvents })}
                disabled={!settings.enabled}
                className={cn(
                  'w-12 h-6 rounded-full p-1 transition-colors relative',
                  settings.portEvents ? 'bg-signal' : 'bg-bg-elevated'
                )}
              >
                <div className={cn(
                  'w-4 h-4 rounded-full bg-white transition-transform',
                  settings.portEvents ? 'translate-x-6' : 'translate-x-0'
                )} />
              </button>
            </div>

            {/* Errors */}
            <div className="flex items-center justify-between">
              <div>
                <div className="text-sm text-text-primary">Error Notifications</div>
                <div className="text-xs text-text-tertiary">Notify on errors and failures</div>
              </div>
              <button
                onClick={() => updateSettings({ errors: !settings.errors })}
                disabled={!settings.enabled}
                className={cn(
                  'w-12 h-6 rounded-full p-1 transition-colors relative',
                  settings.errors ? 'bg-signal' : 'bg-bg-elevated'
                )}
              >
                <div className={cn(
                  'w-4 h-4 rounded-full bg-white transition-transform',
                  settings.errors ? 'translate-x-6' : 'translate-x-0'
                )} />
              </button>
            </div>

            {/* Script Complete */}
            <div className="flex items-center justify-between">
              <div>
                <div className="text-sm text-text-primary">Script Completion</div>
                <div className="text-xs text-text-tertiary">Notify when scripts finish</div>
              </div>
              <button
                onClick={() => updateSettings({ scriptComplete: !settings.scriptComplete })}
                disabled={!settings.enabled}
                className={cn(
                  'w-12 h-6 rounded-full p-1 transition-colors relative',
                  settings.scriptComplete ? 'bg-signal' : 'bg-bg-elevated'
                )}
              >
                <div className={cn(
                  'w-4 h-4 rounded-full bg-white transition-transform',
                  settings.scriptComplete ? 'translate-x-6' : 'translate-x-0'
                )} />
              </button>
            </div>
          </div>

          {/* Duration */}
          <div className="pt-4 border-t border-border">
            <label className="text-sm font-medium text-text-primary block mb-2">
              Notification Duration
            </label>
            <select
              value={settings.duration}
              onChange={(e) => updateSettings({ duration: parseInt(e.target.value) })}
              disabled={!settings.enabled}
              className="w-full px-3 py-2 bg-bg-deep border border-border rounded-md text-sm text-text-primary disabled:opacity-50"
            >
              <option value={2000}>2 seconds</option>
              <option value={3000}>3 seconds</option>
              <option value={5000}>5 seconds</option>
              <option value={10000}>10 seconds</option>
              <option value={0}>Until closed</option>
            </select>
          </div>

          {/* Test Button */}
          <div className="pt-4 border-t border-border">
            <button
              onClick={() => {
                updateSettings({ enabled: true })
                requestNotificationPermission().then(() => {
                  sendSystemNotification({
                    title: '测试通知',
                    body: '这是一条测试通知，如果你看到了它说明功能正常。',
                    type: 'info',
                  })
                })
              }}
              className="px-4 py-2 text-sm font-medium rounded-md bg-info/10 text-info border border-info/30 hover:bg-info/20 transition-colors"
            >
              Send Test Notification
            </button>
          </div>
        </div>
      </Panel>
    </div>
  )
}
