import { invoke } from '@tauri-apps/api/core'
import { getCurrentWindow } from '@tauri-apps/api/window'

export async function showWindow() {
  const window = getCurrentWindow()
  await window.show()
  await window.setFocus()
}

export async function hideWindow() {
  const window = getCurrentWindow()
  await window.hide()
}

export async function toggleWindow() {
  const window = getCurrentWindow()
  const isVisible = await window.isVisible()

  if (isVisible) {
    await window.hide()
    return false
  } else {
    await window.show()
    await window.setFocus()
    return true
  }
}

export async function minimizeWindow() {
  const window = getCurrentWindow()
  await window.minimize()
}

export async function maximizeWindow() {
  const window = getCurrentWindow()
  await window.toggleMaximize()
}

export async function closeWindow() {
  const window = getCurrentWindow()
  await window.close()
}

// Convenience hook
export function useWindow() {
  return {
    showWindow,
    hideWindow,
    toggleWindow,
    minimizeWindow,
    maximizeWindow,
    closeWindow,
  }
}
