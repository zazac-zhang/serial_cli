// Copyright 2024 Serial CLI Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';
import portStore from '../stores/portStore';
import { addPacket } from '../stores/dataStore';

interface DataEvent {
  port_id: string;
  data: number[];
  timestamp: number;
  direction: 'tx' | 'rx';
}

interface PortStatusEvent {
  port_id: string;
  status: any;
  timestamp: number;
}

interface ErrorEvent {
  error: string;
  timestamp: number;
}

/**
 * Setup event listeners for real-time updates from Rust backend
 */
export function setupEventListeners(): () => void {
  let cleanupFns: Array<() => void | Promise<void>> = [];

  // Listen for data received events
  listen<DataEvent>('data-received', (event) => {
    console.log('Data received:', event.payload);
    addPacket({
      port_id: event.payload.port_id,
      direction: 'rx',
      data: event.payload.data,
      timestamp: event.payload.timestamp,
    });
  }).then((unlisten) => cleanupFns.push(unlisten));

  // Listen for data sent events
  listen<DataEvent>('data-sent', (event) => {
    console.log('Data sent:', event.payload);
    addPacket({
      port_id: event.payload.port_id,
      direction: 'tx',
      data: event.payload.data,
      timestamp: event.payload.timestamp,
    });
  }).then((unlisten) => cleanupFns.push(unlisten));

  // Listen for port status changes
  listen<PortStatusEvent>('port-status-changed', (event) => {
    console.log('Port status changed:', event.payload);
    // Update port status in store - will be implemented later
  }).then((unlisten) => cleanupFns.push(unlisten));

  // Listen for errors
  listen<ErrorEvent>('error-occurred', (event) => {
    console.error('Error occurred:', event.payload);
    // Show error to user
    // TODO: Implement error toast/notification
  }).then((unlisten) => cleanupFns.push(unlisten));

  // Return cleanup function
  return () => {
    cleanupFns.forEach((fn) => fn());
  };
}

/**
 * Helper to invoke Tauri commands
 */
export async function invokeCommand<T>(
  command: string,
  args?: Record<string, unknown>
): Promise<T> {
  return invoke<T>(command, args);
}
