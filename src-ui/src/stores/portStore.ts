// Copyright 2024 Serial CLI Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to these terms.

import { createStore, produce } from 'solid-js/store';
import { invoke } from '@tauri-apps/api/core';
import { toast } from './toastStore';

// Types
export interface SerialPort {
  port_name: string;
  port_type: string;
}

export interface PortConfig {
  baudrate: number;
  databits: number;
  stopbits: number;
  parity: 'none' | 'odd' | 'even';
  timeout_ms: number;
  flow_control: 'none' | 'software' | 'hardware';
}

export interface PortStatus {
  id: string;
  port_name: string;
  is_open: boolean;
  config?: PortConfig;
  stats: {
    bytes_sent: number;
    bytes_received: number;
    packets_sent: number;
    packets_received: number;
    last_activity?: number;
  };
}

// Store state
interface PortStore {
  availablePorts: SerialPort[];
  activePorts: Map<string, PortStatus>;
  selectedPortId: string | null;
  isLoading: boolean;
  error: string | null;
}

// Create store
const [portStore, setPortStore] = createStore<PortStore>({
  availablePorts: [],
  activePorts: new Map(),
  selectedPortId: null,
  isLoading: false,
  error: null,
});

// Actions
export const portActions = {
  // List available ports
  async listPorts() {
    setPortStore('isLoading', true);
    setPortStore('error', null);

    try {
      const ports = await invoke<SerialPort[]>('list_ports');
      setPortStore('availablePorts', ports);
    } catch (error) {
      setPortStore('error', String(error));
    } finally {
      setPortStore('isLoading', false);
    }
  },

  // Open a port
  async openPort(portName: string, config: PortConfig) {
    setPortStore('isLoading', true);
    setPortStore('error', null);

    try {
      const portId = await invoke<string>('open_port', {
        portName,
        config,
      });

      // Add to active ports
      setPortStore(
        produce((s) => {
          s.activePorts.set(portId, {
            id: portId,
            port_name: portName,
            is_open: true,
            config,
            stats: {
              bytes_sent: 0,
              bytes_received: 0,
              packets_sent: 0,
              packets_received: 0,
            },
          });
        })
      );

      setPortStore('selectedPortId', portId);

      // Show success message
      toast.success(
        'Port Opened',
        `${portName} is now connected at ${config.baudrate} baud`
      );
    } catch (error) {
      setPortStore('error', String(error));
      toast.error(
        'Failed to Open Port',
        `Could not open ${portName}: ${String(error)}`
      );
      throw error;
    } finally {
      setPortStore('isLoading', false);
    }
  },

  // Close a port
  async closePort(portId: string) {
    setPortStore('isLoading', true);
    setPortStore('error', null);

    try {
      await invoke('close_port', { portId });

      // Remove from active ports
      setPortStore(
        produce((s) => {
          const port = s.activePorts.get(portId);
          s.activePorts.delete(portId);

          // Show success message
          if (port) {
            toast.success(
              'Port Closed',
              `${port.port_name} has been disconnected`
            );
          }
        })
      );

      if (portStore.selectedPortId === portId) {
        setPortStore('selectedPortId', null);
      }
    } catch (error) {
      setPortStore('error', String(error));
      toast.error(
        'Failed to Close Port',
        `Could not close port: ${String(error)}`
      );
      setPortStore('error', String(error));
      throw error;
    } finally {
      setPortStore('isLoading', false);
    }
  },

  // Get port status
  async getPortStatus(portId: string) {
    try {
      const status = await invoke<PortStatus>('get_port_status', { portId });

      setPortStore(
        produce((s) => {
          s.activePorts.set(portId, status);
        })
      );
    } catch (error) {
      setPortStore('error', String(error));
    }
  },

  // Get port status
  selectPort(portId: string | null) {
    setPortStore('selectedPortId', portId);
  },

  // Clear error
  clearError() {
    setPortStore('error', null);
  },

  // Expose active ports as an object
  getActivePorts() {
    return Object.fromEntries(portStore.activePorts);
  },

  // Expose available ports
  getAvailablePorts() {
    return portStore.availablePorts;
  },

  // Expose selected port ID
  getSelectedPortId() {
    return portStore.selectedPortId;
  },
};

export default portStore;
