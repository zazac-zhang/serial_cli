// Copyright 2024 Serial CLI Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to these terms.

import { Component, createSignal, For } from 'solid-js';
import { PortConfig } from '../../stores/portStore';
import './PortConfig.css';

interface PortConfigDialogProps {
  portName: string;
  isOpen: boolean;
  onClose: () => void;
  onOpen: (config: PortConfig) => void;
}

const DEFAULT_CONFIG: PortConfig = {
  baudrate: 115200,
  databits: 8,
  stopbits: 1,
  parity: 'none',
  timeout_ms: 1000,
  flow_control: 'none',
};

const BAUDRATES = [9600, 19200, 38400, 57600, 115200, 230400, 460800, 921600];

const PortConfigDialog: Component<PortConfigDialogProps> = (props) => {
  const [config, setConfig] = createSignal<PortConfig>(DEFAULT_CONFIG);
  const [isOpening, setIsOpening] = createSignal(false);

  const handleOpen = async () => {
    setIsOpening(true);
    try {
      await props.onOpen(config());
      props.onClose();
    } catch (error) {
      console.error('Failed to open port:', error);
    } finally {
      setIsOpening(false);
    }
  };

  const handleCancel = () => {
    setConfig(DEFAULT_CONFIG);
    props.onClose();
  };

  if (!props.isOpen) return null;

  return (
    <div class="port-config-overlay" onClick={handleCancel}>
      <div class="port-config-dialog" onClick={(e) => e.stopPropagation()}>
        <div class="port-config-header">
          <h3 class="port-config-title">Configure Port</h3>
          <button class="port-config-close" onClick={handleCancel}>
            ×
          </button>
        </div>

        <div class="port-config-body">
          <div class="config-info">
            <div class="info-label">Port</div>
            <div class="info-value">{props.portName}</div>
          </div>

          <div class="config-field">
            <label class="config-label">Baud Rate</label>
            <select
              class="config-select"
              value={config().baudrate}
              onChange={(e) =>
                setConfig({ ...config(), baudrate: parseInt(e.target.value) })
              }
            >
              <For each={BAUDRATES}>
                {(rate) => <option value={rate}>{rate}</option>}
              </For>
            </select>
          </div>

          <div class="config-field">
            <label class="config-label">Data Bits</label>
            <select
              class="config-select"
              value={config().databits}
              onChange={(e) =>
                setConfig({ ...config(), databits: parseInt(e.target.value) })
              }
            >
              <option value={5}>5</option>
              <option value={6}>6</option>
              <option value={7}>7</option>
              <option value={8}>8</option>
            </select>
          </div>

          <div class="config-field">
            <label class="config-label">Stop Bits</label>
            <select
              class="config-select"
              value={config().stopbits}
              onChange={(e) =>
                setConfig({ ...config(), stopbits: parseInt(e.target.value) })
              }
            >
              <option value={1}>1</option>
              <option value={2}>2</option>
            </select>
          </div>

          <div class="config-field">
            <label class="config-label">Parity</label>
            <select
              class="config-select"
              value={config().parity}
              onChange={(e) =>
                setConfig({
                  ...config(),
                  parity: e.target.value as 'none' | 'odd' | 'even',
                })
              }
            >
              <option value="none">None</option>
              <option value="odd">Odd</option>
              <option value="even">Even</option>
            </select>
          </div>

          <div class="config-field">
            <label class="config-label">Flow Control</label>
            <select
              class="config-select"
              value={config().flow_control}
              onChange={(e) =>
                setConfig({
                  ...config(),
                  flow_control: e.target.value as 'none' | 'software' | 'hardware',
                })
              }
            >
              <option value="none">None</option>
              <option value="software">Software (XON/XOFF)</option>
              <option value="hardware">Hardware (RTS/CTS)</option>
            </select>
          </div>

          <div class="config-field">
            <label class="config-label">Timeout (ms)</label>
            <input
              type="number"
              class="config-input"
              value={config().timeout_ms}
              min="100"
              max="60000"
              step="100"
              onChange={(e) =>
                setConfig({
                  ...config(),
                  timeout_ms: parseInt(e.target.value) || 1000,
                })
              }
            />
          </div>
        </div>

        <div class="port-config-footer">
          <button
            class="config-button config-button-cancel"
            onClick={handleCancel}
            disabled={isOpening()}
          >
            Cancel
          </button>
          <button
            class="config-button config-button-open"
            onClick={handleOpen}
            disabled={isOpening()}
          >
            {isOpening() ? 'Opening...' : 'Open Port'}
          </button>
        </div>
      </div>
    </div>
  );
};

export default PortConfigDialog;
