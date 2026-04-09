// Tauri command interfaces
export interface SerialPort {
  port_name: string
  port_type: string
}

export interface PortConfig {
  baudrate: number
  databits: number
  stopbits: number
  parity: string
  timeout_ms: number
  flow_control: string
}

export interface PortStatus {
  port_id: string
  port_name: string
  is_open: boolean
  config: PortConfig
  stats: PortStats
}

export interface PortStats {
  bytes_sent: number
  bytes_received: number
  packets_sent: number
  packets_received: number
  last_activity: number | null
}

export interface DataPacket {
  port_id: string
  direction: 'rx' | 'tx'
  data: number[]
  timestamp: number
}

export interface DataEvent {
  port_id: string
  data: number[]
  timestamp: number
}

export interface ProtocolInfo {
  name: string
  version: string
  description: string
  author: string
}

export interface ScriptResult {
  success: boolean
  output: string
  error?: string
}
