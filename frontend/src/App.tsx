import React, { useState } from 'react'
import { NavigationProvider, useNavigation } from './contexts/NavigationContext'
import { PortProvider } from './contexts/PortContext'
import { VirtualPortProvider } from './contexts/VirtualPortContext'
import { DataProvider } from './contexts/DataContext'
import { ToastProvider } from './contexts/ToastContext'
import { ShortcutProvider } from './contexts/ShortcutContext'
import { NotificationProvider } from './contexts/NotificationContext'
import { ScriptActionProvider } from './contexts/ScriptActionContext'
import { SettingsProvider } from './contexts/SettingsContext'
import { useGlobalShortcuts } from './hooks/useGlobalShortcuts'
import { Sidebar } from './components/layout/Sidebar'
import { TopBar } from './components/layout/TopBar'
import { PortsPanel } from './components/ports/PortsPanel'
import { VirtualPortsPanel } from './components/virtual/VirtualPortsPanel'
import { DataViewer } from './components/data/DataViewer'
import { ScriptPanel } from './components/scripting/ScriptPanel'
import { ProtocolPanel } from './components/protocols/ProtocolPanel'
import { SettingsPanel } from './components/settings/SettingsPanel'
import { Toaster } from './components/ui/toast'
import { CommandPalette } from './components/shortcuts/CommandPalette'
import { KeyboardShortcutsHelp } from './components/shortcuts/KeyboardShortcutsHelp'
import { cn } from './lib/utils'

function AppContent() {
  const { currentView } = useNavigation()
  const [previousView, setPreviousView] = useState<string>(currentView)

  React.useEffect(() => {
    if (currentView !== previousView) {
      setPreviousView(currentView)
    }
  }, [currentView, previousView])

  // Register global shortcuts
  useGlobalShortcuts()

  const viewComponents: Record<string, React.ComponentType> = {
    ports: PortsPanel,
    virtual: VirtualPortsPanel,
    data: DataViewer,
    scripts: ScriptPanel,
    protocols: ProtocolPanel,
    settings: SettingsPanel,
  }

  const CurrentView = viewComponents[currentView] || PortsPanel

  return (
    <div className="app-background h-screen flex flex-col">
      <TopBar />
      <div className="flex flex-1 overflow-hidden">
        <Sidebar />
        <main className="flex-1 overflow-auto">
          <div className={cn(
            "p-6 min-h-full",
            "animate-fade-in"
          )}>
            <CurrentView />
          </div>
        </main>
      </div>
      <Toaster />

      {/* Global overlays */}
      <CommandPalette />
      <KeyboardShortcutsHelp />
    </div>
  )
}

function App() {
  return (
    <React.StrictMode>
      <ToastProvider>
        <NotificationProvider>
          <ShortcutProvider>
            <ScriptActionProvider>
              <SettingsProvider>
                <NavigationProvider>
                  <PortProvider>
                    <VirtualPortProvider>
                      <DataProvider>
                        <AppContent />
                      </DataProvider>
                    </VirtualPortProvider>
                  </PortProvider>
                </NavigationProvider>
              </SettingsProvider>
            </ScriptActionProvider>
          </ShortcutProvider>
        </NotificationProvider>
      </ToastProvider>
    </React.StrictMode>
  )
}

export default App
