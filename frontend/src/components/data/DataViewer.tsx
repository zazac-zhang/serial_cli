import { Panel } from '@/components/ui/panel'

export function DataViewer() {
  return (
    <div className="space-y-6">
      <Panel title="Data Monitor" variant="info" className="max-w-6xl">
        <div className="text-sm text-text-tertiary">
          Real-time data monitoring coming soon...
        </div>
      </Panel>
    </div>
  )
}
