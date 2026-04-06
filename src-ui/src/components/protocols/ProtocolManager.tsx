// Copyright 2024 Serial CLI Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

import { Component, createSignal, For, onMount, Show } from 'solid-js';
import { invoke } from '@tauri-apps/api/core';
import { toast } from '../../stores/toastStore';
import './ProtocolManager.css';

interface ProtocolInfo {
  name: string;
  description: string;
}

const ProtocolManager: Component = () => {
  const [protocols, setProtocols] = createSignal<ProtocolInfo[]>([]);
  const [selectedProtocol, setSelectedProtocol] = createSignal<string | null>(null);
  const [isLoading, setIsLoading] = createSignal(false);
  const [showLoadDialog, setShowLoadDialog] = createSignal(false);
  const [loadPath, setLoadPath] = createSignal('');

  // Load protocols on mount
  onMount(() => {
    loadProtocols();
  });

  const loadProtocols = async () => {
    setIsLoading(true);
    try {
      const result = await invoke<ProtocolInfo[]>('list_protocols');
      setProtocols(result);
    } catch (error) {
      console.error('Failed to load protocols:', error);
      toast.error('Load Failed', 'Could not load protocols list');
    } finally {
      setIsLoading(false);
    }
  };

  const handleSelectProtocol = (name: string) => {
    setSelectedProtocol(name);
  };

  const handleLoadProtocol = async () => {
    if (!loadPath().trim()) {
      toast.warning('Path Required', 'Please enter a protocol file path');
      return;
    }

    try {
      const result = await invoke<ProtocolInfo>('load_protocol', {
        path: loadPath().trim(),
      });

      toast.success('Protocol Loaded', `${result.name} loaded successfully`);
      setShowLoadDialog(false);
      setLoadPath('');
      loadProtocols();
    } catch (error) {
      console.error('Failed to load protocol:', error);
      toast.error('Load Failed', `Could not load protocol: ${String(error)}`);
    }
  };

  const handleUnloadProtocol = async (name: string) => {
    if (!confirm(`Are you sure you want to unload "${name}"?`)) {
      return;
    }

    // Check if it's a built-in protocol
    if (
      name === 'line' ||
      name === 'at_command' ||
      name === 'modbus_rtu' ||
      name === 'modbus_ascii'
    ) {
      toast.warning('Cannot Unload', 'Built-in protocols cannot be unloaded');
      return;
    }

    try {
      await invoke('unload_protocol', { name });
      toast.success('Protocol Unloaded', `${name} has been unloaded`);
      loadProtocols();

      if (selectedProtocol() === name) {
        setSelectedProtocol(null);
      }
    } catch (error) {
      console.error('Failed to unload protocol:', error);
      toast.error('Unload Failed', `Could not unload protocol: ${String(error)}`);
    }
  };

  const handleReloadProtocol = async (name: string) => {
    // Check if it's a built-in protocol
    if (
      name === 'line' ||
      name === 'at_command' ||
      name === 'modbus_rtu' ||
      name === 'modbus_ascii'
    ) {
      toast.warning('Cannot Reload', 'Built-in protocols cannot be reloaded');
      return;
    }

    try {
      await invoke('reload_protocol', { name });
      toast.success('Protocol Reloaded', `${name} has been reloaded`);
      loadProtocols();
    } catch (error) {
      console.error('Failed to reload protocol:', error);
      toast.error('Reload Failed', `Could not reload protocol: ${String(error)}`);
    }
  };

  const getProtocolType = (name: string): 'builtin' | 'custom' => {
    if (
      name === 'line' ||
      name === 'at_command' ||
      name === 'modbus_rtu' ||
      name === 'modbus_ascii'
    ) {
      return 'builtin';
    }
    return 'custom';
  };

  return (
    <div class="protocol-manager">
      <div class="manager-header">
        <div class="header-group">
          <h3 class="manager-title">Protocol Manager</h3>
          <div class="manager-stats">
            <span class="stat-item">{protocols().length} protocols</span>
            <span class="stat-item">
              {protocols().filter((p) => getProtocolType(p.name) === 'custom').length} custom
            </span>
          </div>
        </div>

        <div class="header-actions">
          <button
            class="action-button action-primary"
            onClick={() => setShowLoadDialog(true)}
            title="Load custom protocol"
          >
            <span class="button-icon">📥</span>
            <span class="button-label">Load</span>
          </button>

          <button
            class="action-button"
            onClick={loadProtocols}
            disabled={isLoading()}
            title="Refresh protocol list"
          >
            <span class="button-icon">{isLoading() ? '⟳' : '↻'}</span>
            <span class="button-label">Refresh</span>
          </button>
        </div>
      </div>

      <div class="manager-content">
        {protocols().length === 0 && !isLoading() ? (
          <div class="empty-state">
            <div class="empty-icon">📦</div>
            <div class="empty-title">No protocols found</div>
            <div class="empty-description">
              Load a custom protocol to get started
            </div>
            <button
              class="empty-button"
              onClick={() => setShowLoadDialog(true)}
            >
              Load Protocol
            </button>
          </div>
        ) : (
          <div class="protocol-list">
            <For each={protocols()}>
              {(protocol) => (
                <div
                  class="protocol-item"
                  classList={{
                    selected: selectedProtocol() === protocol.name,
                    builtin: getProtocolType(protocol.name) === 'builtin',
                    custom: getProtocolType(protocol.name) === 'custom',
                  }}
                  onClick={() => handleSelectProtocol(protocol.name)}
                >
                  <div class="protocol-icon">
                    {getProtocolType(protocol.name) === 'builtin' ? '⚙️' : '🔧'}
                  </div>

                  <div class="protocol-info">
                    <div class="protocol-name">{protocol.name}</div>
                    <div class="protocol-description">{protocol.description}</div>
                  </div>

                  <div class="protocol-actions">
                    <button
                      class="protocol-action"
                      onClick={(e) => {
                        e.stopPropagation();
                        handleReloadProtocol(protocol.name);
                      }}
                      title="Reload protocol"
                      disabled={getProtocolType(protocol.name) === 'builtin'}
                    >
                      🔄
                    </button>

                    <button
                      class="protocol-action protocol-danger"
                      onClick={(e) => {
                        e.stopPropagation();
                        handleUnloadProtocol(protocol.name);
                      }}
                      title="Unload protocol"
                      disabled={getProtocolType(protocol.name) === 'builtin'}
                    >
                      🗑️
                    </button>
                  </div>
                </div>
              )}
            </For>
          </div>
        )}

        {isLoading() && (
          <div class="loading-state">
            <div class="loading-spinner">⟳</div>
            <div class="loading-text">Loading protocols...</div>
          </div>
        )}
      </div>

      {/* Load Protocol Dialog */}
      <Show when={showLoadDialog()}>
        <div class="dialog-overlay" onClick={() => setShowLoadDialog(false)}>
          <div class="dialog" onClick={(e) => e.stopPropagation()}>
            <div class="dialog-header">
              <h3 class="dialog-title">Load Custom Protocol</h3>
              <button
                class="dialog-close"
                onClick={() => setShowLoadDialog(false)}
              >
                ×
              </button>
            </div>

            <div class="dialog-body">
              <div class="form-field">
                <label class="form-label">Protocol File Path</label>
                <input
                  type="text"
                  class="form-input"
                  placeholder="/path/to/protocol.lua"
                  value={loadPath()}
                  onInput={(e) => setLoadPath(e.target.value)}
                  onKeyPress={(e) => {
                    if (e.key === 'Enter') handleLoadProtocol();
                  }}
                />
                <div class="form-hint">
                  Enter the path to a Lua protocol file (.lua)
                </div>
              </div>

              <div class="form-field">
                <label class="form-label">Protocol Template</label>
                <div class="protocol-template">
                  <pre class="template-code">{`-- Custom Protocol Template
local protocol = {}

-- Protocol metadata
protocol.name = "custom_protocol"
protocol.version = "1.0"
protocol.description = "Custom serial protocol"

-- Parse incoming data
function protocol.parse(data)
    -- Parse data and return parsed frame
    return data
end

-- Encode outgoing data
function protocol.encode(data)
    -- Encode data for transmission
    return data
end

-- Reset protocol state
function protocol.reset()
    -- Reset internal state
end

return protocol`}</pre>
                </div>
              </div>
            </div>

            <div class="dialog-footer">
              <button
                class="dialog-button dialog-cancel"
                onClick={() => setShowLoadDialog(false)}
              >
                Cancel
              </button>
              <button
                class="dialog-button dialog-primary"
                onClick={handleLoadProtocol}
                disabled={!loadPath().trim()}
              >
                Load Protocol
              </button>
            </div>
          </div>
        </div>
      </Show>
    </div>
  );
};

export default ProtocolManager;
