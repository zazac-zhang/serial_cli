import React, { createContext, useContext, useEffect, useRef, useState } from 'react'

type View = 'ports' | 'data' | 'scripts' | 'protocols' | 'settings'

const VIEW_STORAGE_KEY = 'serial-cli-view'

interface NavigationContextType {
  currentView: View
  setCurrentView: (view: View) => void
}

const NavigationContext = createContext<NavigationContextType | undefined>(undefined)

export function NavigationProvider({ children }: { children: React.ReactNode }) {
  const savedView = ((): View => {
    try {
      const item = window.localStorage.getItem(VIEW_STORAGE_KEY)
      if (item && ['ports', 'data', 'scripts', 'protocols', 'settings'].includes(item)) {
        return item as View
      }
    } catch { /* ignore */ }
    return 'ports'
  })()

  // Use a ref-based approach to avoid re-renders on init
  const [currentView, setCurrentView] = useState<View>(savedView)

  // Sync to localStorage on change
  const prevViewRef = useRef(savedView)
  useEffect(() => {
    if (currentView !== prevViewRef.current) {
      prevViewRef.current = currentView
      try {
        window.localStorage.setItem(VIEW_STORAGE_KEY, currentView)
      } catch { /* ignore */ }
    }
  }, [currentView])

  return (
    <NavigationContext.Provider value={{ currentView, setCurrentView }}>
      {children}
    </NavigationContext.Provider>
  )
}

export function useNavigation() {
  const context = useContext(NavigationContext)
  if (!context) {
    throw new Error('useNavigation must be used within NavigationProvider')
  }
  return context
}
