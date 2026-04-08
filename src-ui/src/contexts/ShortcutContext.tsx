import React, { createContext, useContext, useCallback, useState } from 'react'
import type { Shortcut } from '@/lib/shortcuts'

interface ShortcutContextType {
  isCommandPaletteOpen: boolean
  isShortcutsHelpOpen: boolean
  openCommandPalette: () => void
  closeCommandPalette: () => void
  toggleCommandPalette: () => void
  openShortcutsHelp: () => void
  closeShortcutsHelp: () => void
  registerShortcut: (id: string, shortcut: Omit<Shortcut, 'key'>) => void
  executeShortcut: (id: string) => void
}

const ShortcutContext = createContext<ShortcutContextType | undefined>(undefined)

export function ShortcutProvider({ children }: { children: React.ReactNode }) {
  const [isCommandPaletteOpen, setIsCommandPaletteOpen] = useState(false)
  const [isShortcutsHelpOpen, setIsShortcutsHelpOpen] = useState(false)
  const [shortcuts, setShortcuts] = useState<Map<string, Shortcut>>(new Map())

  const openCommandPalette = useCallback(() => {
    setIsCommandPaletteOpen(true)
  }, [])

  const closeCommandPalette = useCallback(() => {
    setIsCommandPaletteOpen(false)
  }, [])

  const toggleCommandPalette = useCallback(() => {
    setIsCommandPaletteOpen(prev => !prev)
  }, [])

  const openShortcutsHelp = useCallback(() => {
    setIsShortcutsHelpOpen(true)
  }, [])

  const closeShortcutsHelp = useCallback(() => {
    setIsShortcutsHelpOpen(false)
  }, [])

  const registerShortcut = useCallback((id: string, shortcut: Omit<Shortcut, 'key'>) => {
    setShortcuts(prev => new Map(prev).set(id, shortcut as Shortcut))
  }, [])

  const executeShortcut = useCallback((id: string) => {
    const shortcut = shortcuts.get(id)
    if (shortcut) {
      shortcut.action()
    }
  }, [shortcuts])

  return (
    <ShortcutContext.Provider value={{
      isCommandPaletteOpen,
      isShortcutsHelpOpen,
      openCommandPalette,
      closeCommandPalette,
      toggleCommandPalette,
      openShortcutsHelp,
      closeShortcutsHelp,
      registerShortcut,
      executeShortcut,
    }}>
      {children}
    </ShortcutContext.Provider>
  )
}

export function useShortcuts() {
  const context = useContext(ShortcutContext)
  if (!context) {
    throw new Error('useShortcuts must be used within ShortcutProvider')
  }
  return context
}
