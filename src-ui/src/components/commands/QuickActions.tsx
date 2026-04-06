// Copyright 2024 Serial CLI Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

import { Component, For } from 'solid-js';
import { invoke } from '@tauri-apps/api/core';
import portStore from '../../stores/portStore';
import { addPacket } from '../../stores/dataStore';
import { toast } from '../../stores/toastStore';
import './QuickActions.css';

interface QuickCommand {
  id: string;
  label: string;
  command: string;
  type: 'ascii' | 'hex';
  description: string;
  icon: string;
  category: 'test' | 'control' | 'data';
}

const QUICK_COMMANDS: QuickCommand[] = [
  // Test commands
  {
    id: 'at',
    label: 'AT',
    command: 'AT',
    type: 'ascii',
    description: 'Basic AT command',
    icon: '📡',
    category: 'test',
  },
  {
    id: 'at-version',
    label: 'AT+VERSION',
    command: 'AT+VERSION',
    type: 'ascii',
    description: 'Get version info',
    icon: '🔍',
    category: 'test',
  },
  {
    id: 'at-reset',
    label: 'AT+RESET',
    command: 'AT+RESET',
    type: 'ascii',
    description: 'Reset device',
    icon: '🔄',
    category: 'control',
  },

  // Control commands
  {
    id: 'ping',
    label: 'Ping',
    command: '0x01 0x02 0x03 0x04',
    type: 'hex',
    description: 'Send ping packet',
    icon: '🏓',
    category: 'test',
  },
  {
    id: 'heartbeat',
    label: 'Heartbeat',
    command: '0xAA 0x55 0x01',
    type: 'hex',
    description: 'Heartbeat signal',
    icon: '💓',
    category: 'test',
  },

  // Data test commands
  {
    id: 'test-ascii',
    label: 'Test ASCII',
    command: 'Hello, World!',
    type: 'ascii',
    description: 'Send test string',
    icon: '📝',
    category: 'data',
  },
  {
    id: 'test-hex',
    label: 'Test Hex',
    command: '0x01 0x02 0x03 0x04 0x05',
    type: 'hex',
    description: 'Send test bytes',
    icon: '🔢',
    category: 'data',
  },
  {
    id: 'newline',
    label: 'New Line',
    command: '\\n',
    type: 'ascii',
    description: 'Send line break',
    icon: '↵',
    category: 'control',
  },
];

const QuickActions: Component = () => {
  const selectedPortId = () => portStore.selectedPortId;

  const handleQuickCommand = async (cmd: QuickCommand) => {
    if (!selectedPortId()) {
      toast.warning('No Port Selected', 'Please select a port first');
      return;
    }

    try {
      // Parse command based on type
      let data: number[];
      if (cmd.type === 'hex') {
        const hex = cmd.command.replace(/0x/gi, '').replace(/\s/g, '');
        data = [];
        for (let i = 0; i < hex.length; i += 2) {
          data.push(parseInt(hex.substr(i, 2), 16));
        }
      } else {
        // Handle escape sequences
        const processedCmd = cmd.command.replace('\\n', '\n').replace('\\r', '\r').replace('\\t', '\t');
        data = [...new TextEncoder().encode(processedCmd)];
      }

      // Send data
      await invoke('send_data', {
        portId: selectedPortId(),
        data,
      });

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
        `${cmd.label}: ${cmd.command}`
      );
    } catch (error) {
      console.error(`Failed to send quick command ${cmd.label}:`, error);
      toast.error(
        'Send Failed',
        `Failed to send ${cmd.label}: ${String(error)}`
      );
    }
  };

  const categories = () => {
    const cats = new Map<string, QuickCommand[]>();
    for (const cmd of QUICK_COMMANDS) {
      if (!cats.has(cmd.category)) {
        cats.set(cmd.category, []);
      }
      cats.get(cmd.category)!.push(cmd);
    }
    return cats;
  };

  const categoryTitles: Record<string, string> = {
    test: 'Test Commands',
    control: 'Control Commands',
    data: 'Data Test',
  };

  const canSend = () => !!selectedPortId();

  return (
    <div class="quick-actions">
      <div class="actions-header">
        <h3 class="actions-title">Quick Actions</h3>
        <div class="actions-status">
          {canSend() ? (
            <span class="status-enabled">Ready</span>
          ) : (
            <span class="status-disabled">Select Port</span>
          )}
        </div>
      </div>

      <div class="actions-content">
        <For each={Array.from(categories().entries())}>
          {([category, commands]) => (
            <div class="action-category">
              <div class="category-title">{categoryTitles[category]}</div>
              <div class="action-buttons">
                <For each={commands}>
                  {(cmd) => (
                    <button
                      class="action-button"
                      classList={{
                        'action-test': cmd.category === 'test',
                        'action-control': cmd.category === 'control',
                        'action-data': cmd.category === 'data',
                      }}
                      onClick={() => handleQuickCommand(cmd)}
                      disabled={!canSend()}
                      title={`${cmd.description}\nCommand: ${cmd.command}`}
                    >
                      <span class="action-icon">{cmd.icon}</span>
                      <span class="action-label">{cmd.label}</span>
                    </button>
                  )}
                </For>
              </div>
            </div>
          )}
        </For>
      </div>

      <div class="actions-footer">
        <div class="footer-hint">
          <span class="hint-icon">💡</span>
          <span class="hint-text">
            Click to send command instantly
          </span>
        </div>
      </div>
    </div>
  );
};

export default QuickActions;
