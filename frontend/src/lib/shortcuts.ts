// Keyboard shortcuts configuration
export interface Shortcut {
  key: string
  description: string
  action: () => void
  category: 'navigation' | 'ports' | 'scripts' | 'data' | 'general'
  global?: boolean // Can be used from anywhere in the app
}

export const shortcuts: Shortcut[] = [
  // Navigation
  {
    key: 'mod+k',
    description: 'Open command palette',
    action: () => {}, // Will be set by CommandPalette
    category: 'general',
    global: true,
  },
  {
    key: 'mod+1',
    description: 'Ports View',
    action: () => {}, // Will be set dynamically
    category: 'navigation',
    global: true,
  },
  {
    key: 'mod+2',
    description: 'Data Monitor View',
    action: () => {},
    category: 'navigation',
    global: true,
  },
  {
    key: 'mod+3',
    description: 'Scripts View',
    action: () => {},
    category: 'navigation',
    global: true,
  },
  {
    key: 'mod+4',
    description: 'Protocols View',
    action: () => {},
    category: 'navigation',
    global: true,
  },
  {
    key: 'mod+5',
    description: 'Settings View',
    action: () => {},
    category: 'navigation',
    global: true,
  },
  // Ports
  {
    key: 'mod+r',
    description: 'Refresh ports',
    action: () => {},
    category: 'ports',
    global: false,
  },
  // Scripts
  {
    key: 'mod+n',
    description: 'New script',
    action: () => {},
    category: 'scripts',
    global: false,
  },
  {
    key: 'mod+enter',
    description: 'Run script',
    action: () => {},
    category: 'scripts',
    global: false,
  },
  // Data
  {
    key: 'mod+shift+c',
    description: 'Clear data',
    action: () => {},
    category: 'data',
    global: false,
  },
  // General
  {
    key: 'mod+,',
    description: 'Open settings',
    action: () => {},
    category: 'general',
    global: true,
  },
  {
    key: 'mod+/',
    description: 'Show keyboard shortcuts',
    action: () => {},
    category: 'general',
    global: true,
  },
  {
    key: 'escape',
    description: 'Close modal/dialog',
    action: () => {},
    category: 'general',
    global: true,
  },
]

// Platform-specific key display
export function formatKey(key: string): string {
  const isMac = navigator.platform.toUpperCase().indexOf('MAC') >= 0
  return key
    .replace('mod', isMac ? '⌘' : 'Ctrl')
    .replace('shift', '⇧')
    .replace('enter', '↩')
    .replace('escape', 'Esc')
}
