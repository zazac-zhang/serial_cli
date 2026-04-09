import React, { createContext, useContext, useState, useEffect, useCallback } from 'react'
import { settingsStorage, type Settings } from '@/lib/storage'

interface SettingsContextType {
  settings: Settings
  updateSettings: (updates: Partial<Settings>) => boolean
  resetSettings: () => boolean
}

const SettingsContext = createContext<SettingsContextType | undefined>(undefined)

export function SettingsProvider({ children }: { children: React.ReactNode }) {
  const [settings, setSettings] = useState<Settings>(settingsStorage.get())

  const updateSettings = useCallback((updates: Partial<Settings>) => {
    const success = settingsStorage.update(updates)
    if (success) {
      setSettings(prev => {
        const merged = { ...prev }
        for (const key in updates) {
          const value = updates[key as keyof Settings]
          if (value && typeof value === 'object' && !Array.isArray(value)) {
            ;(merged as any)[key] = { ...prev[key as keyof Settings], ...value }
          } else {
            ;(merged as any)[key] = value
          }
        }
        return merged
      })
    }
    return success
  }, [])

  const resetSettings = useCallback(() => {
    const defaultSettings: Settings = {
      display: {
        format: 'hex',
        showTimestamp: true,
        autoScroll: true,
        maxPackets: 1000,
      },
      serial: {
        baudRate: 9600,
        dataBits: 8,
        stopBits: 1,
        parity: 'none',
        flowControl: 'none',
      },
      notifications: {
        enabled: true,
        sound: true,
        portEvents: true,
        errors: true,
        scriptComplete: true,
        duration: 3000,
      },
      general: {
        autoCheckUpdates: true,
        sendAnalytics: false,
        minimizeToTray: true,
        language: 'en',
      },
    }
    const success = settingsStorage.set(defaultSettings)
    if (success) {
      setSettings(defaultSettings)
    }
    return success
  }, [])

  return (
    <SettingsContext.Provider value={{
      settings,
      updateSettings,
      resetSettings,
    }}>
      {children}
    </SettingsContext.Provider>
  )
}

export function useSettings() {
  const context = useContext(SettingsContext)
  if (!context) {
    throw new Error('useSettings must be used within SettingsProvider')
  }
  return context
}
