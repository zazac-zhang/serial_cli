import React, { createContext, useContext, useState, useEffect, useCallback } from 'react'
import { listen } from '@tauri-apps/api/event'
import type { DataPacket, DataEvent } from '../types/tauri'
import { settingsStorage } from '@/lib/storage'

interface DataContextType {
  packets: DataPacket[]
  maxPackets: number
  displayOptions: {
    format: 'hex' | 'ascii'
    showTimestamp: boolean
  }
  addPacket: (packet: DataPacket) => void
  clearPackets: () => void
  setDisplayOptions: (options: Partial<DataContextType['displayOptions']>) => void
}

const DataContext = createContext<DataContextType | undefined>(undefined)

// Maximum number of packets to keep in memory
const CLEANUP_MULTIPLIER = 1.2

// Auto-cleanup threshold (when to start removing old packets)
const CLEANUP_THRESHOLD = 12000

export function DataProvider({ children }: { children: React.ReactNode }) {
  const defaultSettings = settingsStorage.get()
  const [packets, setPackets] = useState<DataPacket[]>([])
  const [displayOptions, setDisplayOptions] = useState<{
    format: 'hex' | 'ascii'
    showTimestamp: boolean
  }>({
    format: (defaultSettings.display.format === 'both' ? 'hex' : defaultSettings.display.format) as 'hex' | 'ascii',
    showTimestamp: defaultSettings.display.showTimestamp,
  })
  const [maxPackets, setMaxPackets] = useState(defaultSettings.display.maxPackets)

  // Sync maxPackets from storage
  useEffect(() => {
    const checkInterval = setInterval(() => {
      const current = settingsStorage.get()
      if (current.display.maxPackets !== maxPackets) {
        setMaxPackets(current.display.maxPackets)
      }
    }, 1000)
    return () => clearInterval(checkInterval)
  }, [maxPackets])

  const addPacket = useCallback((packet: DataPacket) => {
    setPackets(prev => {
      const newPackets = [...prev, packet]

      const cleanupThreshold = Math.floor(maxPackets * CLEANUP_MULTIPLIER)
      if (newPackets.length > cleanupThreshold) {
        return newPackets.slice(-maxPackets)
      }

      return newPackets
    })
  }, [maxPackets])

  const clearPackets = useCallback(() => {
    setPackets([])
  }, [])

  // Periodic cleanup to prevent memory buildup
  useEffect(() => {
    const cleanupInterval = setInterval(() => {
      setPackets(prev => {
        if (prev.length > maxPackets) {
          console.log(`Auto-cleanup: removing ${prev.length - maxPackets} old packets`)
          return prev.slice(-maxPackets)
        }
        return prev
      })
    }, 30000) // Check every 30 seconds

    return () => clearInterval(cleanupInterval)
  }, [maxPackets])

  // Listen for data-received events
  useEffect(() => {
    const unlistenPromise = listen<DataEvent>('data-received', (event) => {
      addPacket({
        port_id: event.payload.port_id,
        direction: 'rx',
        data: event.payload.data,
        timestamp: event.payload.timestamp,
      })
    })

    return () => {
      unlistenPromise.then(unlisten => unlisten())
    }
  }, [addPacket])

  // Listen for data-sent events
  useEffect(() => {
    const unlistenPromise = listen<DataEvent>('data-sent', (event) => {
      addPacket({
        port_id: event.payload.port_id,
        direction: 'tx',
        data: event.payload.data,
        timestamp: event.payload.timestamp,
      })
    })

    return () => {
      unlistenPromise.then(unlisten => unlisten())
    }
  }, [addPacket])

  return (
    <DataContext.Provider value={{
      packets,
      maxPackets,
      displayOptions,
      addPacket,
      clearPackets,
      setDisplayOptions: (options) => setDisplayOptions(prev => ({ ...prev, ...options })),
    }}>
      {children}
    </DataContext.Provider>
  )
}

export function useData() {
  const context = useContext(DataContext)
  if (!context) {
    throw new Error('useData must be used within DataProvider')
  }
  return context
}
