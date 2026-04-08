import { useNavigation } from '@/contexts/NavigationContext'
import { cn } from '@/lib/utils'

const navItems = [
  { id: 'ports', label: 'Ports', icon: '🔌', shortcut: '⌘1' },
  { id: 'data', label: 'Data Monitor', icon: '📊', shortcut: '⌘2' },
  { id: 'scripts', label: 'Scripts', icon: '📜', shortcut: '⌘3' },
  { id: 'protocols', label: 'Protocols', icon: '🔧', shortcut: '⌘4' },
  { id: 'settings', label: 'Settings', icon: '⚙️', shortcut: '⌘5' },
] as const

export function Sidebar() {
  const { currentView, setCurrentView } = useNavigation()

  return (
    <aside className="w-64 border-r border-border bg-bg-deep flex flex-col">
      <div className="p-6 border-b border-border/50">
        <h1 className="text-xl font-display font-semibold text-signal">
          Serial CLI
        </h1>
        <p className="text-sm text-text-tertiary mt-1">
          Command Center
        </p>
      </div>

      <nav className="flex-1 px-3 py-4 space-y-1">
        {navItems.map((item) => (
          <button
            key={item.id}
            onClick={() => setCurrentView(item.id)}
            className={cn(
              "w-full flex items-center justify-between px-3 py-2 rounded-md text-sm font-medium transition-colors",
              currentView === item.id
                ? "bg-signal/10 text-signal border border-signal/30"
                : "text-text-secondary hover:text-text-primary hover:bg-bg-elevated"
            )}
          >
            <div className="flex items-center gap-3">
              <span className="text-base">{item.icon}</span>
              <span>{item.label}</span>
            </div>
            <span className="text-xs text-text-tertiary opacity-0 group-hover:opacity-100 transition-opacity">
              {item.shortcut}
            </span>
          </button>
        ))}
      </nav>

      <div className="p-4 border-t border-border/50 space-y-2">
        <div className="text-xs text-text-tertiary">
          <div className="flex items-center justify-between">
            <span>Version 0.1.0</span>
            <span>Tauri 2.0</span>
          </div>
        </div>

        {/* Keyboard shortcut hint */}
        <div className="pt-2 border-t border-border/30">
          <div className="flex items-center gap-2 text-xs text-text-tertiary">
            <kbd className="px-1.5 py-0.5 font-mono text-[10px] bg-bg-elevated border border-border rounded">
              ⌘K
            </kbd>
            <span>Command Palette</span>
          </div>
        </div>
      </div>
    </aside>
  )
}
