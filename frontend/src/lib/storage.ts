/**
 * LocalStorage utility functions for data persistence
 */

const STORAGE_KEYS = {
  SETTINGS: 'serial-cli-settings',
  SCRIPTS: 'serial-cli-scripts',
  PROTOCOLS: 'serial-cli-protocols',
  RECENT_PORTS: 'serial-cli-recent-ports',
  WINDOW_STATE: 'serial-cli-window-state',
} as const

/**
 * Generic storage operations
 */
export const storage = {
  /**
   * Get item from localStorage
   */
  get: <T>(key: string, defaultValue: T): T => {
    try {
      const item = window.localStorage.getItem(key)
      return item ? JSON.parse(item) : defaultValue
    } catch (error) {
      console.error(`Error reading from localStorage (${key}):`, error)
      return defaultValue
    }
  },

  /**
   * Set item in localStorage
   */
  set: <T>(key: string, value: T): boolean => {
    try {
      window.localStorage.setItem(key, JSON.stringify(value))
      return true
    } catch (error) {
      console.error(`Error writing to localStorage (${key}):`, error)
      return false
    }
  },

  /**
   * Remove item from localStorage
   */
  remove: (key: string): boolean => {
    try {
      window.localStorage.removeItem(key)
      return true
    } catch (error) {
      console.error(`Error removing from localStorage (${key}):`, error)
      return false
    }
  },

  /**
   * Clear all items
   */
  clear: (): boolean => {
    try {
      window.localStorage.clear()
      return true
    } catch (error) {
      console.error('Error clearing localStorage:', error)
      return false
    }
  },
}

/**
 * Settings persistence
 */
export interface Settings {
  display: {
    format: 'hex' | 'ascii' | 'both'
    showTimestamp: boolean
    autoScroll: boolean
    maxPackets: number
  }
  serial: {
    baudRate: number
    dataBits: number
    stopBits: number
    parity: 'none' | 'even' | 'odd'
    flowControl: 'none' | 'rts' | 'cts' | 'rtscts'
  }
  notifications: {
    enabled: boolean
    sound: boolean
    portEvents: boolean
    errors: boolean
    scriptComplete: boolean
    duration: number
  }
  general: {
    autoCheckUpdates: boolean
    sendAnalytics: boolean
    minimizeToTray: boolean
    language: string
  }
}

export const settingsStorage = {
  get: (): Settings => {
    return storage.get<Settings>(STORAGE_KEYS.SETTINGS, {
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
    })
  },

  set: (settings: Settings): boolean => {
    return storage.set(STORAGE_KEYS.SETTINGS, settings)
  },

  update: (updates: Partial<Settings>): boolean => {
    const current = settingsStorage.get()
    const merged = deepMerge(current, updates)
    return settingsStorage.set(merged)
  },
}

/**
 * Scripts persistence
 */
export interface ScriptFile {
  id: string
  name: string
  content: string
  lastModified: number
}

export const scriptsStorage = {
  get: (): ScriptFile[] => {
    return storage.get<ScriptFile[]>(STORAGE_KEYS.SCRIPTS, [])
  },

  set: (scripts: ScriptFile[]): boolean => {
    return storage.set(STORAGE_KEYS.SCRIPTS, scripts)
  },

  add: (script: ScriptFile): boolean => {
    const scripts = scriptsStorage.get()
    scripts.push(script)
    return scriptsStorage.set(scripts)
  },

  update: (id: string, updates: Partial<ScriptFile>): boolean => {
    const scripts = scriptsStorage.get()
    const index = scripts.findIndex(s => s.id === id)
    if (index !== -1) {
      scripts[index] = { ...scripts[index], ...updates }
      return scriptsStorage.set(scripts)
    }
    return false
  },

  remove: (id: string): boolean => {
    const scripts = scriptsStorage.get()
    const filtered = scripts.filter(s => s.id !== id)
    return scriptsStorage.set(filtered)
  },
}

/**
 * Recent ports configuration
 */
export interface RecentPortConfig {
  portName: string
  config: {
    baudrate: number
    databits: number
    stopbits: number
    parity: string
    timeout_ms: number
    flow_control: string
  }
  lastUsed: number
}

export const recentPortsStorage = {
  get: (): RecentPortConfig[] => {
    return storage.get<RecentPortConfig[]>(STORAGE_KEYS.RECENT_PORTS, [])
  },

  set: (ports: RecentPortConfig[]): boolean => {
    // Keep only last 10 configs
    const sorted = ports.sort((a, b) => b.lastUsed - a.lastUsed).slice(0, 10)
    return storage.set(STORAGE_KEYS.RECENT_PORTS, sorted)
  },

  add: (port: RecentPortConfig): boolean => {
    const ports = recentPortsStorage.get()
    // Remove existing entry for same port
    const filtered = ports.filter((p: RecentPortConfig) => p.portName !== port.portName)
    filtered.push(port)
    return recentPortsStorage.set(filtered)
  },
}

/**
 * Window state persistence
 */
export interface WindowState {
  width: number
  height: number
  x: number
  y: number
  isMaximized: boolean
}

export const windowStateStorage = {
  get: (): WindowState | null => {
    return storage.get<WindowState | null>(STORAGE_KEYS.WINDOW_STATE, null)
  },

  set: (state: WindowState): boolean => {
    return storage.set(STORAGE_KEYS.WINDOW_STATE, state)
  },
}

/**
 * Deep merge utility
 */
function deepMerge<T>(target: T, source: Partial<T>): T {
  const result = { ...target }

  for (const key in source) {
    const sourceValue = source[key]
    const targetValue = result[key]

    if (
      sourceValue &&
      typeof sourceValue === 'object' &&
      !Array.isArray(sourceValue) &&
      targetValue &&
      typeof targetValue === 'object' &&
      !Array.isArray(targetValue)
    ) {
      result[key] = deepMerge(targetValue, sourceValue)
    } else {
      result[key] = sourceValue as T[Extract<keyof T, string>]
    }
  }

  return result
}
