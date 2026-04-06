// Copyright 2024 Serial CLI Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

import { Component, createSignal, onMount, onCleanup, For } from 'solid-js';
import { invoke } from '@tauri-apps/api/core';
import portStore, { portActions } from '../../stores/portStore';
import { addPacket } from '../../stores/dataStore';
import { toast } from '../../stores/toastStore';
import './CommandInput.css';

const CommandInput: Component = () => {
  const [command, setCommand] = createSignal('');
  const [history, setHistory] = createSignal<string[]>([]);
  const [historyIndex, setHistoryIndex] = createSignal(-1);
  const [isSending, setIsSending] = createSignal(false);

  const selectedPortId = () => portStore.selectedPortId;

  onMount(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'Enter' && !e.shiftKey) {
        e.preventDefault();
        handleSend();
      } else if (e.key === 'ArrowUp') {
        e.preventDefault();
        navigateHistory(-1);
      } else if (e.key === 'ArrowDown') {
        e.preventDefault();
        navigateHistory(1);
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    onCleanup(() => {
      window.removeEventListener('keydown', handleKeyDown);
    });
  });

  const handleSend = async () => {
    const cmd = command().trim();
    if (!cmd || !selectedPortId() || isSending()) return;

    setIsSending(true);

    try {
      // Parse command (hex string or ASCII)
      const data = parseCommand(cmd);

      // Send data
      await invoke('send_data', {
        portId: selectedPortId(),
        data,
      });

      // Add to history
      setHistory((h) => [...h, cmd]);
      setHistoryIndex(-1);
      setCommand('');

      // Add to data store as TX
      addPacket({
        port_id: selectedPortId()!,
        direction: 'tx',
        data,
        timestamp: Date.now(),
      });

      // Show success message
      toast.success(
        'Command Sent',
        `Sent ${data.length} byte(s): ${cmd}`
      );
    } catch (error) {
      console.error('Failed to send data:', error);
      toast.error(
        'Send Failed',
        `Failed to send command: ${String(error)}`
      );
    } finally {
      setIsSending(false);
    }
  };

  const parseCommand = (cmd: string): number[] => {
    // Check if it's a hex string (starts with 0x or contains hex characters)
    if (cmd.startsWith('0x') || /^[\dA-Fa-f\s]+$/.test(cmd)) {
      // Parse as hex
      const hex = cmd.replace(/^0x/i, '').replace(/\s/g, '');
      const bytes = [];
      for (let i = 0; i < hex.length; i += 2) {
        bytes.push(parseInt(hex.substr(i, 2), 16));
      }
      return bytes;
    } else {
      // Parse as ASCII string
      return [...new TextEncoder().encode(cmd)];
    }
  };

  const navigateHistory = (direction: number) => {
    const h = history();
    const newIndex = historyIndex() + direction;

    if (newIndex >= 0 && newIndex < h.length) {
      setHistoryIndex(newIndex);
      setCommand(h[newIndex]);
    } else if (newIndex < 0) {
      setHistoryIndex(-1);
      setCommand('');
    }
  };

  const canSend = () => {
    return command().trim() && selectedPortId() && !isSending();
  };

  return (
    <div class="command-input">
      <div class="input-header">
        <h3 class="input-title">Quick Command</h3>
        <div class="input-status">
          {selectedPortId() ? (
            <span class="status-connected">
              <span class="status-dot"></span>
              Connected
            </span>
          ) : (
            <span class="status-disconnected">No Connection</span>
          )}
        </div>
      </div>

      <div class="input-body">
        <div class="input-wrapper">
          <input
            type="text"
            class="input-field"
            placeholder="Enter command (hex or ASCII)..."
            value={command()}
            onInput={(e) => setCommand(e.target.value)}
            disabled={!selectedPortId() || isSending()}
            onKeyDown={(e) => {
              if (e.key === 'Enter' && e.shiftKey) {
                // Allow newline with Shift+Enter
                setCommand(command() + '\n');
              }
            }}
          />
          <button
            class="input-send"
            onClick={handleSend}
            disabled={!canSend()}
            title="Send command (Enter)"
          >
            {isSending() ? '⟳' : '➤'}
          </button>
        </div>

        <div class="input-hint">
          <span class="hint-text">Format:</span>
          <span class="hint-example">0x01 0x02</span>
          <span class="hint-separator">or</span>
          <span class="hint-example">AT+CMD</span>
          <span class="hint-separator">•</span>
          <span class="hint-shortcut">↑↓ History</span>
        </div>
      </div>

      {history().length > 0 && (
        <div class="input-history">
          <div class="history-header">
            <span class="history-title">History</span>
            <button
              class="history-clear"
              onClick={() => setHistory([])}
              title="Clear history"
            >
              Clear
            </button>
          </div>
          <div class="history-list">
            <For each={history().slice().reverse()}>
              {(cmd) => (
                <div
                  class="history-item"
                  onClick={() => setCommand(cmd)}
                >
                  <span class="history-command">{cmd}</span>
                </div>
              )}
            </For>
          </div>
        </div>
      )}
    </div>
  );
};

export default CommandInput;
