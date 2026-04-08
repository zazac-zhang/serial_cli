import React from 'react'
import { NavigationProvider, useNavigation } from './contexts/NavigationContext'
import { PortProvider } from './contexts/PortContext'
import { DataProvider } from './contexts/DataContext'
import { ToastProvider } from './contexts/ToastContext'
import { ShortcutProvider } from './contexts/ShortcutContext'
import { NotificationProvider } from './contexts/NotificationContext'
import { useGlobalShortcuts } from './hooks/useGlobalShortcuts'
import { Sidebar } from './components/layout/Sidebar'
import { TopBar } from './components/layout/TopBar'
import { PortsPanel } from './components/ports/PortsPanel'
import { DataViewer } from './components/data/DataViewer'
import { ScriptPanel } from './components/scripting/ScriptPanel'
import { ProtocolPanel } from './components/protocols/ProtocolPanel'
import { SettingsPanel } from './components/settings/SettingsPanel'
import { Toaster } from './components/ui/toast'
import { CommandPalette } from './components/shortcuts/CommandPalette'
import { KeyboardShortcutsHelp } from './components/shortcuts/KeyboardShortcutsHelp'

function AppContent() {
  const { currentView } = useNavigation()

  // Register global shortcuts
  useGlobalShortcuts()

  return (
    <div className="app-background h-screen flex flex-col">
      <TopBar />
      <div className="flex flex-1 overflow-hidden">
        <Sidebar />
        <main className="flex-1 overflow-auto">
          <div className="p-6">
            {currentView === 'ports' && <PortsPanel />}
            {currentView === 'data' && <DataViewer />}
            {currentView === 'scripts' && <ScriptPanel />}
            {currentView === 'protocols' && <ProtocolPanel />}
            {currentView === 'settings' && <SettingsPanel />}
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
            <NavigationProvider>
              <PortProvider>
                <DataProvider>
                  <AppContent />
                </DataProvider>
              </PortProvider>
            </NavigationProvider>
          </ShortcutProvider>
        </NotificationProvider>
      </ToastProvider>
    </React.StrictMode>
  )
}

export default App
