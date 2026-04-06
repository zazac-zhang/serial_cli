// Copyright 2024 Serial CLI Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

import { Component, For, onMount } from 'solid-js';
import portStore, { portActions, SerialPort } from '../../stores/portStore';
import './PortList.css';

interface PortListProps {
  onOpenPort?: (portName: string) => void;
}

const PortList: Component<PortListProps> = (props) => {
  const ports = () => portStore.availablePorts;
  const activePorts = () => Object.fromEntries(portStore.activePorts);
  const selectedPortId = () => portStore.selectedPortId;
  const isLoading = () => portStore.isLoading;
  const error = () => portStore.error;

  // Auto-refresh ports every 3 seconds
  onMount(() => {
    portActions.listPorts();
    const interval = setInterval(() => portActions.listPorts(), 3000);
    return () => clearInterval(interval);
  });

  const handleRefresh = () => {
    portActions.listPorts();
  };

  const handleSelectPort = (portName: string) => {
    // Check if port is already open
    for (const [id, status] of Object.entries(activePorts())) {
      if (status.port_name === portName) {
        portActions.selectPort(id);
        return;
      }
    }
    portActions.selectPort(null); // Clear selection if not open
  };

  const handlePortClick = (portName: string, status: string) => {
    if (status === 'closed' && props.onOpenPort) {
      // Open port config dialog
      props.onOpenPort(portName);
    } else {
      // Just select the port
      handleSelectPort(portName);
    }
  };

  const getPortStatus = (portName: string) => {
    for (const [id, status] of Object.entries(activePorts())) {
      if (status.port_name === portName) {
        return { id, status: 'open' as const };
      }
    }
    return { id: null, status: 'closed' as const };
  };

  return (
    <div class="port-list">
      <div class="port-list-header">
        <h3 class="port-list-title">Serial Ports</h3>
        <button
          class="refresh-button"
          onClick={handleRefresh}
          disabled={isLoading()}
          title="Refresh port list"
        >
          {isLoading() ? '⟳' : '↻'}
        </button>
      </div>

      {error() && (
        <div class="port-error">
          <span class="error-icon">⚠</span>
          <span class="error-message">{error()}</span>
          <button
            class="error-dismiss"
            onClick={() => portActions.clearError()}
          >
            ×
          </button>
        </div>
      )}

      <div class="port-list-content">
        {ports().length === 0 ? (
          <div class="port-empty">
            <div class="empty-icon">🔌</div>
            <div class="empty-text">No serial ports found</div>
            <div class="empty-hint">
              Connect a device or click refresh to scan
            </div>
          </div>
        ) : (
          <For each={ports()}>
            {(port) => {
              const { id, status } = getPortStatus(port.port_name);
              const isSelected = () => selectedPortId() === id;

              return (
                <div
                  class={`port-item ${status} ${isSelected() ? 'selected' : ''} ${status === 'closed' ? 'clickable' : ''}`}
                  onClick={() => handlePortClick(port.port_name, status)}
                  title={status === 'closed' ? 'Click to configure and open port' : 'Click to select port'}
                >
                  <div class="port-icon">
                    {status === 'open' ? '🔗' : '🔌'}
                  </div>
                  <div class="port-info">
                    <div class="port-name">{port.port_name}</div>
                    <div class="port-type">{port.port_type}</div>
                  </div>
                  <div class="port-status">
                    {status === 'open' ? (
                      <span class="status-badge status-open">
                        <span class="status-dot"></span>
                        Open
                      </span>
                    ) : (
                      <span class="status-badge status-closed">
                        Click to Open
                      </span>
                    )}
                  </div>
                </div>
              );
            }}
          </For>
        )}
      </div>
    </div>
  );
};

export default PortList;
