import React, { createContext, useContext, useState, useCallback, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/core'
import type { SerialPort, PortConfig, PortStatus } from '../types/tauri'

interface PortContextType {
  availablePorts: SerialPort[]
  activePorts: Map<string, PortStatus>
  isLoading: boolean
  error: string | null
  listPorts: () => Promise<void>
  openPort: (portName: string, config: PortConfig) => Promise<string>
  closePort: (portId: string) => Promise<void>
}

const PortContext = createContext<PortContextType | undefined>(undefined)

export function PortProvider({ children }: { children: React.ReactNode }) {
  const [availablePorts, setAvailablePorts] = useState<SerialPort[]>([])
  const [activePorts, setActivePorts] = useState<Map<string, PortStatus>>(new Map())
  const [isLoading, setIsLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const listPorts = useCallback(async () => {
    setIsLoading(true)
    try {
      const ports = await invoke<SerialPort[]>('list_ports')
      setAvailablePorts(ports)
      setError(null)
    } catch (e) {
      const errorMsg = e instanceof Error ? e.message : 'Failed to list ports'
      setError(errorMsg)
      console.error('listPorts error:', e)
    } finally {
      setIsLoading(false)
    }
  }, [])

  const openPort = useCallback(async (portName: string, config: PortConfig) => {
    try {
      const portId = await invoke<string>('open_port', { portName, config })

      // Start data sniffing for the opened port
      try {
        await invoke('start_sniffing', { portId })
        console.log('Started data sniffing for port:', portId)
      } catch (sniffError) {
        console.error('Failed to start sniffing:', sniffError)
        // Don't fail port opening if sniffing fails
      }

      setActivePorts(prev => {
        const next = new Map(prev)
        next.set(portId, {
          port_id: portId,
          port_name: portName,
          is_open: true,
          config,
          stats: {
            bytes_sent: 0,
            bytes_received: 0,
            packets_sent: 0,
            packets_received: 0,
            last_activity: null,
          },
        })
        return next
      })
      return portId
    } catch (e) {
      const errorMsg = e instanceof Error ? e.message : 'Failed to open port'
      setError(errorMsg)
      throw e
    }
  }, [])

  const closePort = useCallback(async (portId: string) => {
    try {
      // Stop data sniffing first
      try {
        await invoke('stop_sniffing', { portId })
        console.log('Stopped data sniffing for port:', portId)
      } catch (sniffError) {
        console.error('Failed to stop sniffing:', sniffError)
        // Continue with port closing even if stopping sniffing fails
      }

      await invoke('close_port', { portId })
      setActivePorts(prev => {
        const next = new Map(prev)
        next.delete(portId)
        return next
      })
    } catch (e) {
      const errorMsg = e instanceof Error ? e.message : 'Failed to close port'
      setError(errorMsg)
      throw e
    }
  }, [])

  // Auto-list ports on mount
  useEffect(() => {
    listPorts()
  }, [listPorts])

  return (
    <PortContext.Provider value={{
      availablePorts,
      activePorts,
      isLoading,
      error,
      listPorts,
      openPort,
      closePort,
    }}>
      {children}
    </PortContext.Provider>
  )
}

export function usePorts() {
  const context = useContext(PortContext)
  if (!context) {
    throw new Error('usePorts must be used within PortProvider')
  }
  return context
}
