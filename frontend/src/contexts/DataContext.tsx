import React, { createContext, useContext, useState, useEffect, useCallback } from 'react'
import { listen } from '@tauri-apps/api/event'
import type { DataPacket, DataEvent } from '../types/tauri'

interface DataContextType {
  packets: DataPacket[]
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
const MAX_PACKETS = 10000

// Auto-cleanup threshold (when to start removing old packets)
const CLEANUP_THRESHOLD = 12000

export function DataProvider({ children }: { children: React.ReactNode }) {
  const [packets, setPackets] = useState<DataPacket[]>([])
  const [displayOptions, setDisplayOptions] = useState<{
    format: 'hex' | 'ascii'
    showTimestamp: boolean
  }>({
    format: 'hex',
    showTimestamp: true,
  })

  const addPacket = useCallback((packet: DataPacket) => {
    setPackets(prev => {
      const newPackets = [...prev, packet]

      // Auto-cleanup when approaching limit
      if (newPackets.length > CLEANUP_THRESHOLD) {
        // Keep only the most recent MAX_PACKETS
        return newPackets.slice(-MAX_PACKETS)
      }

      return newPackets
    })
  }, [])

  const clearPackets = useCallback(() => {
    setPackets([])
  }, [])

  // Periodic cleanup to prevent memory buildup
  useEffect(() => {
    const cleanupInterval = setInterval(() => {
      setPackets(prev => {
        if (prev.length > MAX_PACKETS) {
          console.log(`Auto-cleanup: removing ${prev.length - MAX_PACKETS} old packets`)
          return prev.slice(-MAX_PACKETS)
        }
        return prev
      })
    }, 30000) // Check every 30 seconds

    return () => clearInterval(cleanupInterval)
  }, [])

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
