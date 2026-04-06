// Copyright 2024 Serial CLI Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to these terms.

import { createStore, produce } from 'solid-js/store';

export type ToastType = 'success' | 'error' | 'warning' | 'info';

export interface Toast {
  id: string;
  type: ToastType;
  title: string;
  message: string;
  timestamp: number;
  duration?: number; // Auto-dismiss duration in ms
}

interface ToastStore {
  toasts: Toast[];
}

const [toastStore, setToastStore] = createStore<ToastStore>({
  toasts: [],
});

let toastIdCounter = 0;

/**
 * Add a new toast notification
 */
export function addToast(toast: Omit<Toast, 'id' | 'timestamp'>) {
  const id = `toast-${++toastIdCounter}`;
  const newToast: Toast = {
    ...toast,
    id,
    timestamp: Date.now(),
    duration: toast.duration ?? (toast.type === 'error' ? 5000 : 3000),
  };

  setToastStore(
    produce((s) => {
      s.toasts.push(newToast);
    })
  );

  // Auto-dismiss after duration
  if (newToast.duration && newToast.duration > 0) {
    setTimeout(() => {
      removeToast(id);
    }, newToast.duration);
  }

  return id;
}

/**
 * Remove a toast by ID
 */
export function removeToast(id: string) {
  setToastStore(
    produce((s) => {
      s.toasts = s.toasts.filter((t) => t.id !== id);
    })
  );
}

/**
 * Clear all toasts
 */
export function clearAllToasts() {
  setToastStore({ toasts: [] });
}

/**
 * Convenience functions for common toast types
 */
export const toast = {
  success: (title: string, message: string, duration?: number) =>
    addToast({ type: 'success', title, message, duration }),

  error: (title: string, message: string, duration?: number) =>
    addToast({ type: 'error', title, message, duration }),

  warning: (title: string, message: string, duration?: number) =>
    addToast({ type: 'warning', title, message, duration }),

  info: (title: string, message: string, duration?: number) =>
    addToast({ type: 'info', title, message, duration }),
};

export default toastStore;
