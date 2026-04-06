// Copyright 2024 Serial CLI Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

import { Component, createSignal } from 'solid-js';
import portStore, { portActions, PortConfig } from '../../stores/portStore';
import Panel from '../layout/Panel';
import PortList from './PortList';
import PortConfigDialog from './PortConfig';
import './PortsPanel.css';

const PortsPanel: Component = () => {
  const [showConfigDialog, setShowConfigDialog] = createSignal(false);
  const [selectedPortName, setSelectedPortName] = createSignal('');

  const selectedPortId = () => portStore.selectedPortId;
  const activePorts = () => Object.fromEntries(portStore.activePorts);

  const handleOpenPort = () => {
    // Find a closed port to open
    for (const port of portStore.availablePorts) {
      const isOpen = Object.values(activePorts()).some(
        (p) => p.port_name === port.port_name
      );
      if (!isOpen) {
        setSelectedPortName(port.port_name);
        setShowConfigDialog(true);
        return;
      }
    }
  };

  const handleConfigPort = async (config: PortConfig) => {
    await portActions.openPort(selectedPortName(), config);
  };

  const handleOpenPortDialog = (portName: string) => {
    setSelectedPortName(portName);
    setShowConfigDialog(true);
  };

  const handleClosePort = async () => {
    if (selectedPortId()) {
      await portActions.closePort(selectedPortId()!);
    }
  };

  return (
    <div class="ports-panel">
      <Panel title="Serial Port Management" glow="cyan">
        <div class="ports-panel-content">
          <div class="ports-actions">
            <button
              class="action-button action-primary"
              onClick={handleOpenPort}
              title="Open a new serial port"
            >
              <span class="action-icon">➕</span>
              <span class="action-text">Open Port</span>
            </button>

            {selectedPortId() && (
              <button
                class="action-button action-danger"
                onClick={handleClosePort}
                title="Close the selected port"
              >
                <span class="action-icon">➖</span>
                <span class="action-text">Close Port</span>
              </button>
            )}

            <div class="ports-status">
              <div class="status-item">
                <span class="status-label">Active</span>
                <span class="status-value">
                  {Object.keys(activePorts()).length}
                </span>
              </div>
              <div class="status-item">
                <span class="status-label">Selected</span>
                <span class="status-value">
                  {selectedPortId() ? 'Yes' : 'None'}
                </span>
              </div>
            </div>
          </div>

          <PortList onOpenPort={handleOpenPortDialog} />

          {selectedPortId() && (
            <div class="port-details">
              <h4 class="details-title">Port Details</h4>
              <div class="details-content">
                <div class="detail-row">
                  <span class="detail-label">ID:</span>
                  <span class="detail-value mono">{selectedPortId()}</span>
                </div>
                {(() => {
                  const port = activePorts()[selectedPortId()!];
                  if (!port) return null;
                  return (
                    <>
                      <div class="detail-row">
                        <span class="detail-label">Port:</span>
                        <span class="detail-value mono">{port.port_name}</span>
                      </div>
                      <div class="detail-row">
                        <span class="detail-label">Baudrate:</span>
                        <span class="detail-value mono">{port.config?.baudrate}</span>
                      </div>
                      <div class="detail-row">
                        <span class="detail-label">Status:</span>
                        <span class="detail-value success">Connected</span>
                      </div>
                    </>
                  );
                })()}
              </div>
            </div>
          )}
        </div>
      </Panel>

      <PortConfigDialog
        portName={selectedPortName()}
        isOpen={showConfigDialog()}
        onClose={() => setShowConfigDialog(false)}
        onOpen={handleConfigPort}
      />
    </div>
  );
};

export default PortsPanel;
