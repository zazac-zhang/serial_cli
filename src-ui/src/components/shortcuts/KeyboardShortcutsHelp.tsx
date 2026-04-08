import { shortcuts, formatKey } from '@/lib/shortcuts'
import { useShortcuts } from '@/contexts/ShortcutContext'
import { cn } from '@/lib/utils'

export function KeyboardShortcutsHelp() {
  const { isShortcutsHelpOpen, closeShortcutsHelp } = useShortcuts()

  if (!isShortcutsHelpOpen) return null

  // Group shortcuts by category
  const categories = Array.from(new Set(shortcuts.map(s => s.category)))

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center">
      {/* Backdrop */}
      <div
        className="absolute inset-0 bg-black/50 backdrop-blur-sm animate-fade-in"
        onClick={closeShortcutsHelp}
      />

      {/* Modal */}
      <div className="relative w-full max-w-3xl mx-4 bg-bg-floating border border-border rounded-lg shadow-xl overflow-hidden animate-slide-up">
        {/* Header */}
        <div className="flex items-center justify-between px-6 py-4 border-b border-border">
          <div className="flex items-center gap-3">
            <span className="text-2xl">⌨️</span>
            <div>
              <h2 className="text-lg font-semibold text-text-primary">Keyboard Shortcuts</h2>
              <p className="text-sm text-text-tertiary">Quick commands for power users</p>
            </div>
          </div>
          <button
            onClick={closeShortcutsHelp}
            className="w-8 h-8 flex items-center justify-center rounded hover:bg-bg-elevated text-text-secondary hover:text-text-primary transition-colors"
          >
            ×
          </button>
        </div>

        {/* Content */}
        <div className="p-6 max-h-[60vh] overflow-y-auto">
          <div className="grid gap-6">
            {categories.map(category => {
              const categoryShortcuts = shortcuts.filter(s => s.category === category)
              return (
                <div key={category}>
                  <h3 className="text-sm font-semibold text-text-tertiary uppercase tracking-wide mb-3">
                    {category}
                  </h3>
                  <div className="space-y-2">
                    {categoryShortcuts.map(shortcut => (
                      <div
                        key={shortcut.key}
                        className={cn(
                          'flex items-center justify-between p-3 rounded-md',
                          'border border-border hover:border-signal/30 transition-colors'
                        )}
                      >
                        <span className="text-sm text-text-primary">{shortcut.description}</span>
                        <kbd className="px-2 py-1 text-xs font-mono text-text-secondary bg-bg-deep border border-border rounded">
                          {formatKey(shortcut.key)}
                        </kbd>
                      </div>
                    ))}
                  </div>
                </div>
              )
            })}
          </div>
        </div>

        {/* Footer */}
        <div className="px-6 py-4 border-t border-border bg-bg-deep">
          <div className="flex items-center justify-between text-sm text-text-tertiary">
            <div>Press <kbd className="px-1.5 py-0.5 text-xs font-mono bg-bg-elevated border border-border rounded">ESC</kbd> to close</div>
            <div>Serial CLI v0.1.0</div>
          </div>
        </div>
      </div>
    </div>
  )
}
