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
import './ScriptManager.css';

interface ScriptInfo {
  name: string;
  path: string;
  size: number;
  modified: number;
}

interface ScriptTemplate {
  id: string;
  name: string;
  description: string;
  content: string;
  category: 'basic' | 'serial' | 'protocol' | 'advanced';
}

const SCRIPT_TEMPLATES: ScriptTemplate[] = [
  {
    id: 'empty',
    name: 'Empty Script',
    description: '空白脚本模板',
    category: 'basic',
    content: `-- Empty Lua Script\n-- Create your automation here\n\n`,
  },
  {
    id: 'basic-serial',
    name: 'Basic Serial',
    description: '基础串口通信',
    category: 'serial',
    content: `-- Basic Serial Communication
local port = serial_open("/dev/ttyUSB0", 115200)

if port then
    print("Port opened successfully")

    -- Send data
    serial_send(port, "AT+CMD\\n")

    -- Read response
    local data = serial_recv(port, 1000)

    if data then
        print("Received: " .. bytes_to_string(data))
    end

    -- Close port
    serial_close(port)
    print("Done")
else
    print("Failed to open port")
end
`,
  },
  {
    id: 'loop-test',
    name: 'Loop Test',
    description: '循环测试',
    category: 'serial',
    content: `-- Loop Test Script
local port = serial_open("/dev/ttyUSB0", 115200)

if port then
    -- Send test data in a loop
    for i = 1, 10 do
        local data = string.format("Test %d\\n", i)
        serial_send(port, data)
        print("Sent: " .. data:gsub("\\n", ""))
        serial_recv(port, 500)
    end

    serial_close(port)
else
    print("Failed to open port")
end
`,
  },
  {
    id: 'protocol-encode',
    name: 'Protocol Encode',
    description: '协议编码示例',
    category: 'protocol',
    content: `-- Protocol Encoding Example
local data = { 0x01, 0x02, 0x03, 0x04 }

-- Encode with protocol
local encoded = protocol_encode("Modbus", data)
print("Encoded: " .. bytes_to_hex(encoded))

-- Decode back
local decoded = protocol_decode("Modbus", encoded)
print("Decoded: " .. bytes_to_hex(decoded))
`,
  },
  {
    id: 'data-conversion',
    name: 'Data Conversion',
    description: '数据转换工具',
    category: 'advanced',
    content: `-- Data Conversion Examples

-- Hex to ASCII
local hex_data = { 0x48, 0x65, 0x6C, 0x6C, 0x6F }
local ascii_str = bytes_to_string(hex_data)
print("Hex to ASCII: " .. ascii_str)

-- ASCII to Hex
local ascii_data = "Hello"
local hex_bytes = string_to_bytes(ascii_data)
print("ASCII to Hex: " .. bytes_to_hex(hex_bytes))

-- Number conversions
local num = 42
print("Number to hex: " .. string.format("%02X", num))
`,
  },
];

const ScriptManager: Component = () => {
  const [scripts, setScripts] = createSignal<ScriptInfo[]>([]);
  const [selectedScript, setSelectedScript] = createSignal<string | null>(null);
  const [isLoading, setIsLoading] = createSignal(false);
  const [showNewDialog, setShowNewDialog] = createSignal(false);
  const [showTemplateDialog, setShowTemplateDialog] = createSignal(false);
  const [newScriptName, setNewScriptName] = createSignal('');
  const [selectedTemplate, setSelectedTemplate] = createSignal<string | null>(null);

  // Load scripts on mount
  onMount(() => {
    loadScripts();
  });

  const loadScripts = async () => {
    setIsLoading(true);
    try {
      const result = await invoke<ScriptInfo[]>('list_scripts');
      setScripts(result);
    } catch (error) {
      console.error('Failed to load scripts:', error);
      toast.error('Load Failed', 'Could not load scripts list');
    } finally {
      setIsLoading(false);
    }
  };

  const handleSelectScript = async (name: string) => {
    setSelectedScript(name);
  };

  const handleNewScript = () => {
    setNewScriptName('');
    setShowTemplateDialog(true);
  };

  const handleCreateFromTemplate = async () => {
    if (!newScriptName().trim()) {
      toast.warning('Name Required', 'Please enter a script name');
      return;
    }

    if (!selectedTemplate()) {
      toast.warning('Template Required', 'Please select a template');
      return;
    }

    const template = SCRIPT_TEMPLATES.find((t) => t.id === selectedTemplate());
    if (!template) {
      toast.error('Template Error', 'Selected template not found');
      return;
    }

    try {
      await invoke('save_script', {
        name: newScriptName().trim(),
        content: template.content,
      });

      toast.success('Script Created', `${newScriptName()} created from template`);
      setShowTemplateDialog(false);
      loadScripts();
    } catch (error) {
      console.error('Failed to create script:', error);
      toast.error('Create Failed', `Could not create script: ${String(error)}`);
    }
  };

  const handleDeleteScript = async (name: string) => {
    if (!confirm(`Are you sure you want to delete "${name}"?`)) {
      return;
    }

    try {
      await invoke('delete_script', { name });
      toast.success('Script Deleted', `${name} has been deleted`);
      loadScripts();

      if (selectedScript() === name) {
        setSelectedScript(null);
      }
    } catch (error) {
      console.error('Failed to delete script:', error);
      toast.error('Delete Failed', `Could not delete script: ${String(error)}`);
    }
  };

  const handleRenameScript = async (oldName: string) => {
    const newName = prompt('Enter new name:', oldName);
    if (!newName || newName === oldName) return;

    try {
      // Load the old script content
      const content = await invoke<string>('load_script', { name: oldName });

      // Save with new name
      await invoke('save_script', {
        name: newName,
        content,
      });

      // Delete old script
      await invoke('delete_script', { name: oldName });

      toast.success('Script Renamed', `Renamed to ${newName}`);
      loadScripts();
    } catch (error) {
      console.error('Failed to rename script:', error);
      toast.error('Rename Failed', `Could not rename script: ${String(error)}`);
    }
  };

  const handleDuplicateScript = async (name: string) => {
    const newName = prompt('Enter name for duplicate:', `${name}_copy`);
    if (!newName) return;

    try {
      // Load the original script content
      const content = await invoke<string>('load_script', { name });

      // Save as new script
      await invoke('save_script', {
        name: newName,
        content,
      });

      toast.success('Script Duplicated', `Created ${newName}`);
      loadScripts();
    } catch (error) {
      console.error('Failed to duplicate script:', error);
      toast.error('Duplicate Failed', `Could not duplicate script: ${String(error)}`);
    }
  };

  const formatFileSize = (bytes: number): string => {
    if (bytes < 1024) return `${bytes} B`;
    if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
    return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
  };

  const formatDate = (timestamp: number): string => {
    return new Date(timestamp * 1000).toLocaleString();
  };

  const getTemplatesByCategory = (category: ScriptTemplate['category']) => {
    return SCRIPT_TEMPLATES.filter((t) => t.category === category);
  };

  return (
    <div class="script-manager">
      <div class="manager-header">
        <div class="header-group">
          <h3 class="manager-title">Script Manager</h3>
          <div class="manager-stats">
            <span class="stat-item">{scripts().length} scripts</span>
          </div>
        </div>

        <div class="header-actions">
          <button
            class="action-button action-primary"
            onClick={handleNewScript}
            title="Create new script from template"
          >
            <span class="button-icon">➕</span>
            <span class="button-label">New</span>
          </button>

          <button
            class="action-button"
            onClick={loadScripts}
            disabled={isLoading()}
            title="Refresh script list"
          >
            <span class="button-icon">{isLoading() ? '⟳' : '↻'}</span>
            <span class="button-label">Refresh</span>
          </button>
        </div>
      </div>

      <div class="manager-content">
        {scripts().length === 0 && !isLoading() ? (
          <div class="empty-state">
            <div class="empty-icon">📜</div>
            <div class="empty-title">No scripts found</div>
            <div class="empty-description">
              Create your first script from a template to get started
            </div>
            <button
              class="empty-button"
              onClick={handleNewScript}
            >
              Create First Script
            </button>
          </div>
        ) : (
          <div class="script-list">
            <For each={scripts()}>
              {(script) => (
                <div
                  class="script-item"
                  classList={{ selected: selectedScript() === script.name }}
                  onClick={() => handleSelectScript(script.name)}
                >
                  <div class="script-icon">📄</div>

                  <div class="script-info">
                    <div class="script-name">{script.name}</div>
                    <div class="script-meta">
                      <span class="meta-item">{formatFileSize(script.size)}</span>
                      <span class="meta-separator">•</span>
                      <span class="meta-item">{formatDate(script.modified)}</span>
                    </div>
                  </div>

                  <div class="script-actions">
                    <button
                      class="script-action"
                      onClick={(e) => {
                        e.stopPropagation();
                        handleRenameScript(script.name);
                      }}
                      title="Rename script"
                    >
                      ✏️
                    </button>

                    <button
                      class="script-action"
                      onClick={(e) => {
                        e.stopPropagation();
                        handleDuplicateScript(script.name);
                      }}
                      title="Duplicate script"
                    >
                      📋
                    </button>

                    <button
                      class="script-action script-danger"
                      onClick={(e) => {
                        e.stopPropagation();
                        handleDeleteScript(script.name);
                      }}
                      title="Delete script"
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
            <div class="loading-text">Loading scripts...</div>
          </div>
        )}
      </div>

      {/* New Script Template Dialog */}
      <Show when={showTemplateDialog()}>
        <div class="dialog-overlay" onClick={() => setShowTemplateDialog(false)}>
          <div class="dialog" onClick={(e) => e.stopPropagation()}>
            <div class="dialog-header">
              <h3 class="dialog-title">Create New Script</h3>
              <button
                class="dialog-close"
                onClick={() => setShowTemplateDialog(false)}
              >
                ×
              </button>
            </div>

            <div class="dialog-body">
              <div class="form-field">
                <label class="form-label">Script Name</label>
                <input
                  type="text"
                  class="form-input"
                  placeholder="my_script"
                  value={newScriptName()}
                  onInput={(e) => setNewScriptName(e.target.value)}
                  onKeyPress={(e) => {
                    if (e.key === 'Enter') handleCreateFromTemplate();
                  }}
                />
              </div>

              <div class="form-field">
                <label class="form-label">Select Template</label>
                <div class="template-categories">
                  <For each={['basic', 'serial', 'protocol', 'advanced'] as const}>
                    {(category) => (
                      <div class="template-category">
                        <div class="category-title">{category}</div>
                        <div class="template-list">
                          <For each={getTemplatesByCategory(category)}>
                            {(template) => (
                              <button
                                class="template-option"
                                classList={{
                                  selected: selectedTemplate() === template.id,
                                }}
                                onClick={() => setSelectedTemplate(template.id)}
                              >
                                <div class="template-name">{template.name}</div>
                                <div class="template-description">
                                  {template.description}
                                </div>
                              </button>
                            )}
                          </For>
                        </div>
                      </div>
                    )}
                  </For>
                </div>
              </div>
            </div>

            <div class="dialog-footer">
              <button
                class="dialog-button dialog-cancel"
                onClick={() => setShowTemplateDialog(false)}
              >
                Cancel
              </button>
              <button
                class="dialog-button dialog-primary"
                onClick={handleCreateFromTemplate}
                disabled={!newScriptName().trim() || !selectedTemplate()}
              >
                Create Script
              </button>
            </div>
          </div>
        </div>
      </Show>
    </div>
  );
};

export default ScriptManager;
