// Copyright 2024 Serial CLI Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

import { Component, createSignal, For, Show } from 'solid-js';
import { invoke } from '@tauri-apps/api/core';
import { toast } from '../../stores/toastStore';
import './ScriptOutput.css';

interface OutputLine {
  id: string;
  type: 'stdout' | 'stderr' | 'info' | 'error' | 'success';
  content: string;
  timestamp: number;
}

const ScriptOutput: Component = () => {
  const [outputs, setOutputs] = createSignal<OutputLine[]>([]);
  const [autoScroll, setAutoScroll] = createSignal(true);
  const [filter, setFilter] = createSignal<'all' | 'stdout' | 'stderr' | 'errors'>('all');
  const outputIdCounter = () => Math.random().toString(36).substr(2, 9);

  const addOutput = (type: OutputLine['type'], content: string) => {
    const newOutput: OutputLine = {
      id: outputIdCounter(),
      type,
      content,
      timestamp: Date.now(),
    };

    setOutputs((prev) => [...prev, newOutput]);

    // Auto-scroll if enabled
    if (autoScroll()) {
      setTimeout(() => {
        const container = document.querySelector('.output-content');
        if (container) {
          container.scrollTop = container.scrollHeight;
        }
      }, 100);
    }
  };

  const handleClear = () => {
    setOutputs([]);
    toast.info('Output Cleared', 'All output has been cleared');
  };

  const handleExport = () => {
    const content = outputs()
      .map((line) => {
        const timestamp = new Date(line.timestamp).toISOString();
        return `[${timestamp}] [${line.type.toUpperCase()}] ${line.content}`;
      })
      .join('\n');

    // Create blob and download
    const blob = new Blob([content], { type: 'text/plain' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `script-output-${Date.now()}.log`;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);

    toast.success('Output Exported', 'Output log has been exported');
  };

  const getFilteredOutputs = () => {
    const f = filter();
    if (f === 'all') return outputs();
    if (f === 'errors') return outputs().filter((o) => o.type === 'error' || o.type === 'stderr');
    return outputs().filter((o) => o.type === f);
  };

  const getIcon = (type: OutputLine['type']) => {
    switch (type) {
      case 'stdout':
        return '📝';
      case 'stderr':
        return '⚠️';
      case 'info':
        return 'ℹ️';
      case 'error':
        return '❌';
      case 'success':
        return '✅';
      default:
        return '•';
    }
  };

  return (
    <div class="script-output">
      <div class="output-header">
        <div class="header-group">
          <h3 class="output-title">Output</h3>
          <div class="output-stats">
            <span class="stat-item">{outputs().length} lines</span>
          </div>
        </div>

        <div class="header-actions">
          <select
            class="filter-select"
            value={filter()}
            onChange={(e) => setFilter(e.target.value as any)}
          >
            <option value="all">All</option>
            <option value="stdout">Stdout</option>
            <option value="stderr">Stderr</option>
            <option value="errors">Errors Only</option>
          </select>

          <button
            class="action-button"
            onClick={() => setAutoScroll(!autoScroll())}
            classList={{ active: autoScroll() }}
            title="Toggle auto-scroll"
          >
            <span class="button-icon">{autoScroll() ? '📜' : '📄'}</span>
          </button>

          <button
            class="action-button"
            onClick={handleClear}
            title="Clear output"
          >
            <span class="button-icon">🗑️</span>
          </button>

          <button
            class="action-button"
            onClick={handleExport}
            title="Export output log"
            disabled={outputs().length === 0}
          >
            <span class="button-icon">💾</span>
          </button>
        </div>
      </div>

      <div class="output-content">
        {getFilteredOutputs().length === 0 ? (
          <div class="output-empty">
            <div class="empty-icon">📤</div>
            <div class="empty-text">No output yet</div>
            <div class="empty-hint">
              Execute a script to see output here
            </div>
          </div>
        ) : (
          <For each={getFilteredOutputs()}>
            {(line) => (
              <div
                class={`output-line output-${line.type}`}
                title={new Date(line.timestamp).toLocaleString()}
              >
                <span class="line-icon">{getIcon(line.type)}</span>
                <span class="line-content">{line.content}</span>
                <span class="line-time">
                  {new Date(line.timestamp).toLocaleTimeString()}
                </span>
              </div>
            )}
          </For>
        )}
      </div>

      <div class="output-footer">
        <div class="footer-info">
          <span class="info-text">
            Showing {getFilteredOutputs().length} of {outputs().length} lines
          </span>
        </div>
      </div>
    </div>
  );
};

export default ScriptOutput;
