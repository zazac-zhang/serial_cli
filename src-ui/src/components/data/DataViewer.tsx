// Copyright 2024 Serial CLI Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

import { Component, For, onMount, createSignal, Show } from 'solid-js';
import { dataStore, getFilteredPackets, toggleMonitoring, clearPackets, setDisplayOptions, setFilter } from '../../stores/dataStore';
import { toast } from '../../stores/toastStore';
import './DataViewer.css';

const DataViewer: Component = () => {
  const packets = () => dataStore.packets;
  const displayOptions = () => dataStore.displayOptions;
  const isMonitoring = () => dataStore.isMonitoring;

  const filteredPackets = () => getFilteredPackets();

  // Advanced filter states
  const [showAdvancedFilters, setShowAdvancedFilters] = createSignal(false);
  const [directionFilter, setDirectionFilter] = createSignal<'all' | 'tx' | 'rx'>('all');
  const [timeRange, setTimeRange] = createSignal<'all' | '1m' | '5m' | '15m' | '1h'>('all');
  const [contentSearch, setContentSearch] = createSignal('');
  const [showExportDialog, setShowExportDialog] = createSignal(false);
  const [exportFormat, setExportFormat] = createSignal<'csv' | 'json' | 'hex'>('csv');

  // Statistics
  const stats = () => {
    const allPackets = packets();
    const totalPackets = allPackets.length;
    const totalBytes = allPackets.reduce((sum, p) => sum + p.data.length, 0);
    const txPackets = allPackets.filter(p => p.direction === 'tx').length;
    const rxPackets = allPackets.filter(p => p.direction === 'rx').length;
    const txBytes = allPackets.filter(p => p.direction === 'tx').reduce((sum, p) => sum + p.data.length, 0);
    const rxBytes = allPackets.filter(p => p.direction === 'rx').reduce((sum, p) => sum + p.data.length, 0);

    return {
      totalPackets,
      totalBytes,
      txPackets,
      rxPackets,
      txBytes,
      rxBytes,
    };
  };

  onMount(() => {
    // Scroll to bottom when new packets arrive
    const observer = new MutationObserver(() => {
      if (displayOptions().autoScroll) {
        scrollToBottom();
      }
    });

    const container = document.querySelector('.data-packets');
    if (container) {
      observer.observe(container, { childList: true });
    }

    return () => observer.disconnect();
  });

  const scrollToBottom = () => {
    const container = document.querySelector('.data-packets');
    if (container) {
      container.scrollTop = container.scrollHeight;
    }
  };

  const formatTimestamp = (timestamp: number) => {
    const date = new Date(timestamp);
    const ms = date.getMilliseconds().toString().padStart(3, '0');
    const timeStr = date.toLocaleTimeString('en-US', {
      hour12: false,
      hour: '2-digit',
      minute: '2-digit',
      second: '2-digit',
    });
    return `${timeStr}.${ms}`;
  };

  const formatBytes = (bytes: number): string => {
    if (bytes < 1024) return `${bytes} B`;
    if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
    return `${(bytes / (1024 * 1024)).toFixed(2)} MB`;
  };

  const getFilteredWithAdvanced = () => {
    let filtered = filteredPackets();

    // Apply direction filter
    if (directionFilter() !== 'all') {
      filtered = filtered.filter(p => p.direction === directionFilter());
    }

    // Apply time range filter
    if (timeRange() !== 'all') {
      const now = Date.now();
      const ranges: Record<string, number> = {
        '1m': 60 * 1000,
        '5m': 5 * 60 * 1000,
        '15m': 15 * 60 * 1000,
        '1h': 60 * 60 * 1000,
      };
      const cutoff = now - (ranges[timeRange()] || 0);
      filtered = filtered.filter(p => p.timestamp >= cutoff);
    }

    // Apply content search
    if (contentSearch().trim()) {
      const search = contentSearch().toLowerCase();
      filtered = filtered.filter(p => {
        const hex = p.display?.hex.toLowerCase() || '';
        const ascii = p.display?.ascii.toLowerCase() || '';
        return hex.includes(search) || ascii.includes(search);
      });
    }

    return filtered;
  };

  const handleExport = () => {
    const dataToExport = getFilteredWithAdvanced();
    if (dataToExport.length === 0) {
      toast.warning('No Data', 'No data to export');
      return;
    }

    let content: string;
    let filename: string;
    let mimeType: string;

    switch (exportFormat()) {
      case 'csv':
        content = 'Timestamp,Direction,Size,Hex,ASCII\n';
        content += dataToExport.map(p =>
          `${formatTimestamp(p.timestamp)},${p.direction.toUpperCase()},${p.data.length},"${p.display?.hex}","${p.display?.ascii}"`
        ).join('\n');
        filename = `serial_data_${Date.now()}.csv`;
        mimeType = 'text/csv';
        break;

      case 'json':
        content = JSON.stringify(dataToExport.map(p => ({
          timestamp: p.timestamp,
          timestamp_formatted: formatTimestamp(p.timestamp),
          direction: p.direction,
          size: p.data.length,
          hex: p.display?.hex,
          ascii: p.display?.ascii,
        })), null, 2);
        filename = `serial_data_${Date.now()}.json`;
        mimeType = 'application/json';
        break;

      case 'hex':
        content = dataToExport.map(p => {
          const timestamp = formatTimestamp(p.timestamp);
          const header = `# ${timestamp} | ${p.direction.toUpperCase()} | ${p.data.length} bytes\n`;
          const hex = p.display?.hex || '';
          return header + hex;
        }).join('\n\n');
        filename = `serial_data_${Date.now()}.hex`;
        mimeType = 'text/plain';
        break;
    }

    // Create download link
    const blob = new Blob([content], { type: mimeType });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = filename;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);

    toast.success('Export Complete', `Exported ${dataToExport.length} packets as ${exportFormat().toUpperCase()}`);
    setShowExportDialog(false);
  };

  const currentStats = stats();
  const displayedPackets = getFilteredWithAdvanced();

  return (
    <div class="data-viewer">
      <div class="data-viewer-header">
        <h3 class="data-viewer-title">Data Monitor</h3>
        <div class="data-viewer-controls">
          <button
            class="control-button"
            onClick={() => setShowAdvancedFilters(!showAdvancedFilters())}
            title="Advanced filters"
            classList={{ active: showAdvancedFilters() }}
          >
            🔍
          </button>
          <button
            class="control-button"
            onClick={() => setShowExportDialog(true)}
            title="Export data"
          >
            📥
          </button>
          <button
            class={`control-button ${isMonitoring() ? 'active' : ''}`}
            onClick={() => toggleMonitoring()}
            title="Toggle monitoring"
          >
            {isMonitoring() ? '⏸' : '▶'}
          </button>
          <button
            class="control-button"
            onClick={() => clearPackets()}
            title="Clear all data"
          >
            🗑
          </button>
        </div>
      </div>

      {/* Statistics Panel */}
      <div class="data-stats">
        <div class="stat-item">
          <span class="stat-label">Total</span>
          <span class="stat-value">{currentStats.totalPackets}</span>
          <span class="stat-unit">pkts</span>
        </div>
        <div class="stat-item">
          <span class="stat-label">Size</span>
          <span class="stat-value">{formatBytes(currentStats.totalBytes)}</span>
        </div>
        <div class="stat-divider"></div>
        <div class="stat-item stat-tx">
          <span class="stat-label">TX</span>
          <span class="stat-value">{currentStats.txPackets}</span>
          <span class="stat-unit">pkts</span>
        </div>
        <div class="stat-item stat-tx">
          <span class="stat-label">TX</span>
          <span class="stat-value">{formatBytes(currentStats.txBytes)}</span>
        </div>
        <div class="stat-divider"></div>
        <div class="stat-item stat-rx">
          <span class="stat-label">RX</span>
          <span class="stat-value">{currentStats.rxPackets}</span>
          <span class="stat-unit">pkts</span>
        </div>
        <div class="stat-item stat-rx">
          <span class="stat-label">RX</span>
          <span class="stat-value">{formatBytes(currentStats.rxBytes)}</span>
        </div>
        <div class="stat-divider"></div>
        <div class="stat-item">
          <span class="stat-label">Shown</span>
          <span class="stat-value">{displayedPackets.length}</span>
          <span class="stat-unit">pkts</span>
        </div>
      </div>

      {/* Advanced Filters */}
      <Show when={showAdvancedFilters()}>
        <div class="data-filters-advanced">
          <div class="filter-group">
            <label class="filter-label">Direction</label>
            <select
              class="filter-select"
              value={directionFilter()}
              onInput={(e) => setDirectionFilter(e.target.value as 'all' | 'tx' | 'rx')}
            >
              <option value="all">All</option>
              <option value="tx">TX Only</option>
              <option value="rx">RX Only</option>
            </select>
          </div>

          <div class="filter-group">
            <label class="filter-label">Time Range</label>
            <select
              class="filter-select"
              value={timeRange()}
              onInput={(e) => setTimeRange(e.target.value as 'all' | '1m' | '5m' | '15m' | '1h')}
            >
              <option value="all">All Time</option>
              <option value="1m">Last 1 min</option>
              <option value="5m">Last 5 min</option>
              <option value="15m">Last 15 min</option>
              <option value="1h">Last 1 hour</option>
            </select>
          </div>

          <div class="filter-group filter-group-wide">
            <label class="filter-label">Content Search</label>
            <input
              type="text"
              class="filter-input"
              placeholder="Search in hex or ASCII..."
              value={contentSearch()}
              onInput={(e) => setContentSearch(e.target.value)}
            />
          </div>

          <button
            class="filter-reset"
            onClick={() => {
              setDirectionFilter('all');
              setTimeRange('all');
              setContentSearch('');
            }}
          >
            Reset Filters
          </button>
        </div>
      </Show>

      <div class="data-viewer-options">
        <label class="option-label">
          <input
            type="checkbox"
            checked={displayOptions().autoScroll}
            onChange={(e) =>
              setDisplayOptions({ autoScroll: e.target.checked })
            }
          />
          Auto-scroll
        </label>
        <label class="option-label">
          <input
            type="checkbox"
            checked={displayOptions().showTimestamp}
            onChange={(e) =>
              setDisplayOptions({ showTimestamp: e.target.checked })
            }
          />
          Timestamp
        </label>
        <select
          class="option-select"
          value={displayOptions().view}
          onChange={(e) =>
            setDisplayOptions({
              view: e.target.value as 'hex' | 'ascii' | 'both',
            })
          }
        >
          <option value="hex">Hex</option>
          <option value="ascii">ASCII</option>
          <option value="both">Both</option>
        </select>
        <input
          type="text"
          class="option-filter"
          placeholder="Quick filter..."
          value={dataStore.filter}
          onInput={(e) => setFilter(e.target.value)}
        />
      </div>

      <div class="data-packets">
        {displayedPackets.length === 0 ? (
          <div class="data-empty">
            <div class="empty-icon">📊</div>
            <div class="empty-text">No data to display</div>
            <div class="empty-hint">
              {filteredPackets().length === 0
                ? 'Connect a port and send/receive data to see it here'
                : 'Try adjusting your filters'}
            </div>
          </div>
        ) : (
          <For each={displayedPackets}>
            {(packet) => (
              <div
                class={`data-packet packet-${packet.direction}`}
              >
                <div class="packet-header">
                  <span class="packet-direction">
                    {packet.direction === 'tx' ? 'TX' : 'RX'}
                  </span>
                  {displayOptions().showTimestamp && (
                    <span class="packet-timestamp">
                      {formatTimestamp(packet.timestamp)}
                    </span>
                  )}
                  <span class="packet-size">{packet.data.length} bytes</span>
                </div>

                {displayOptions().view !== 'ascii' && (
                  <div class="packet-hex">{packet.display?.hex}</div>
                )}
                {displayOptions().view !== 'hex' && (
                  <div class="packet-ascii">{packet.display?.ascii}</div>
                )}
              </div>
            )}
          </For>
        )}
      </div>

      {/* Export Dialog */}
      <Show when={showExportDialog()}>
        <div class="dialog-overlay" onClick={() => setShowExportDialog(false)}>
          <div class="dialog" onClick={(e) => e.stopPropagation()}>
            <div class="dialog-header">
              <h3 class="dialog-title">Export Data</h3>
              <button
                class="dialog-close"
                onClick={() => setShowExportDialog(false)}
              >
                ×
              </button>
            </div>

            <div class="dialog-body">
              <div class="export-info">
                <div class="info-item">
                  <span class="info-label">Packets to export:</span>
                  <span class="info-value">{displayedPackets.length}</span>
                </div>
                <div class="info-item">
                  <span class="info-label">Total data:</span>
                  <span class="info-value">
                    {formatBytes(displayedPackets.reduce((sum, p) => sum + p.data.length, 0))}
                  </span>
                </div>
              </div>

              <div class="export-options">
                <label class="export-option">
                  <input
                    type="radio"
                    name="export-format"
                    value="csv"
                    checked={exportFormat() === 'csv'}
                    onInput={() => setExportFormat('csv')}
                  />
                  <div class="option-content">
                    <div class="option-title">CSV</div>
                    <div class="option-description">
                      Comma-separated values, compatible with Excel
                    </div>
                  </div>
                </label>

                <label class="export-option">
                  <input
                    type="radio"
                    name="export-format"
                    value="json"
                    checked={exportFormat() === 'json'}
                    onInput={() => setExportFormat('json')}
                  />
                  <div class="option-content">
                    <div class="option-title">JSON</div>
                    <div class="option-description">
                      Structured data format, easy to parse
                    </div>
                  </div>
                </label>

                <label class="export-option">
                  <input
                    type="radio"
                    name="export-format"
                    value="hex"
                    checked={exportFormat() === 'hex'}
                    onInput={() => setExportFormat('hex')}
                  />
                  <div class="option-content">
                    <div class="option-title">Hex Dump</div>
                    <div class="option-description">
                      Plain text hex dump with timestamps
                    </div>
                  </div>
                </label>
              </div>
            </div>

            <div class="dialog-footer">
              <button
                class="dialog-button dialog-cancel"
                onClick={() => setShowExportDialog(false)}
              >
                Cancel
              </button>
              <button
                class="dialog-button dialog-primary"
                onClick={handleExport}
              >
                Export
              </button>
            </div>
          </div>
        </div>
      </Show>
    </div>
  );
};

export default DataViewer;
