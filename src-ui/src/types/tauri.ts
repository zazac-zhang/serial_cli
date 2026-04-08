// Tauri command interfaces
export interface SerialPort {
  port_name: string
  port_type: string
}

export interface PortConfig {
  baud_rate: number
  data_bits: number
  stop_bits: number
  parity: string
  flow_control: string
}

export interface PortStatus {
  port_id: string
  port_name: string
  is_open: boolean
  config: PortConfig
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
