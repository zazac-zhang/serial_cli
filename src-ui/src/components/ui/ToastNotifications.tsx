// Copyright 2024 Serial CLI Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

import { Component, For } from 'solid-js';
import toastStore, { removeToast } from '../../stores/toastStore';
import './ToastNotifications.css';

const ToastNotifications: Component = () => {
  const toasts = () => toastStore.toasts;

  const getIcon = (type: string) => {
    switch (type) {
      case 'success':
        return '✓';
      case 'error':
        return '✕';
      case 'warning':
        return '⚠';
      case 'info':
        return 'ℹ';
      default:
        return '•';
    }
  };

  return (
    <div class="toast-container">
      <For each={toasts()}>
        {(toast) => (
          <div
            classList={{
              'toast': true,
              [`toast-${toast.type}`]: true,
            }}
            role="alert"
            aria-live="polite"
          >
            <div class="toast-icon">{getIcon(toast.type)}</div>
            <div class="toast-content">
              <div class="toast-title">{toast.title}</div>
              {toast.message && (
                <div class="toast-message">{toast.message}</div>
              )}
            </div>
            <button
              class="toast-close"
              onClick={() => removeToast(toast.id)}
              aria-label="Close notification"
            >
              ×
            </button>
          </div>
        )}
      </For>
    </div>
  );
};

export default ToastNotifications;
