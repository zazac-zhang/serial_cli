// Copyright 2024 Serial CLI Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

import { Component, createSignal, For, Show, onMount } from 'solid-js';
import { invoke } from '@tauri-apps/api/core';
import { toast } from '../../stores/toastStore';
import './ConfigPresets.css';

interface Preset {
  name: string;
  description: string;
  created_at: number;
  config: any;
}

const ConfigPresets: Component = () => {
  const [presets, setPresets] = createSignal<Preset[]>([]);
  const [showSaveDialog, setShowSaveDialog] = createSignal(false);
  const [presetName, setPresetName] = createSignal('');
  const [presetDescription, setPresetDescription] = createSignal('');
  const [isLoading, setIsLoading] = createSignal(false);

  onMount(() => {
    loadPresets();
  });

  const loadPresets = async () => {
    setIsLoading(true);
    try {
      // Get current config
      const config = await invoke('get_config');

      // For now, we'll store presets in localStorage
      // In production, this could be stored in a file
      const storedPresets = localStorage.getItem('config-presets');
      if (storedPresets) {
        setPresets(JSON.parse(storedPresets));
      }
    } catch (error) {
      console.error('Failed to load presets:', error);
      toast.error('Load Failed', 'Could not load presets');
    } finally {
      setIsLoading(false);
    }
  };

  const handleSavePreset = async () => {
    if (!presetName().trim()) {
      toast.warning('Name Required', 'Please enter a preset name');
      return;
    }

    try {
      // Get current config
      const config = await invoke('get_config');

      const newPreset: Preset = {
        name: presetName().trim(),
        description: presetDescription().trim(),
        created_at: Date.now(),
        config,
      };

      // Save to localStorage
      const updatedPresets = [...presets(), newPreset];
      localStorage.setItem('config-presets', JSON.stringify(updatedPresets));
      setPresets(updatedPresets);

      toast.success('Preset Saved', `${presetName()} has been saved`);
      setShowSaveDialog(false);
      setPresetName('');
      setPresetDescription('');
    } catch (error) {
      console.error('Failed to save preset:', error);
      toast.error('Save Failed', 'Could not save preset');
    }
  };

  const handleLoadPreset = async (preset: Preset) => {
    try {
      await invoke('update_config', { config: preset.config });
      toast.success('Preset Loaded', `${preset.name} has been applied`);
    } catch (error) {
      console.error('Failed to load preset:', error);
      toast.error('Load Failed', 'Could not load preset');
    }
  };

  const handleDeletePreset = (preset: Preset) => {
    if (!confirm(`Are you sure you want to delete "${preset.name}"?`)) {
      return;
    }

    try {
      const updatedPresets = presets().filter(p => p.name !== preset.name);
      localStorage.setItem('config-presets', JSON.stringify(updatedPresets));
      setPresets(updatedPresets);
      toast.success('Preset Deleted', `${preset.name} has been deleted`);
    } catch (error) {
      console.error('Failed to delete preset:', error);
      toast.error('Delete Failed', 'Could not delete preset');
    }
  };

  const formatDate = (timestamp: number): string => {
    return new Date(timestamp).toLocaleString();
  };

  return (
    <div class="config-presets">
      <div class="presets-header">
        <h3 class="presets-title">Configuration Presets</h3>
        <button
          class="header-button"
          onClick={() => setShowSaveDialog(true)}
          title="Save current configuration as preset"
        >
          <span class="button-icon">💾</span>
          <span class="button-label">Save Current</span>
        </button>
      </div>

      <div class="presets-content">
        {presets().length === 0 && !isLoading() ? (
          <div class="presets-empty">
            <div class="empty-icon">📁</div>
            <div class="empty-title">No presets found</div>
            <div class="empty-description">
              Save your current configuration as a preset to get started
            </div>
            <button
              class="empty-button"
              onClick={() => setShowSaveDialog(true)}
            >
              Create First Preset
            </button>
          </div>
        ) : (
          <div class="presets-list">
            <For each={presets()}>
              {(preset) => (
                <div class="preset-item">
                  <div class="preset-icon">📋</div>

                  <div class="preset-info">
                    <div class="preset-name">{preset.name}</div>
                    <div class="preset-description">
                      {preset.description || 'No description'}
                    </div>
                    <div class="preset-meta">
                      <span class="meta-item">
                        Created: {formatDate(preset.created_at)}
                      </span>
                    </div>
                  </div>

                  <div class="preset-actions">
                    <button
                      class="preset-action preset-load"
                      onClick={() => handleLoadPreset(preset)}
                      title="Load this preset"
                    >
                      📥 Load
                    </button>

                    <button
                      class="preset-action preset-delete"
                      onClick={() => handleDeletePreset(preset)}
                      title="Delete preset"
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
          <div class="presets-loading">
            <div class="loading-spinner">⟳</div>
            <div class="loading-text">Loading presets...</div>
          </div>
        )}
      </div>

      {/* Save Preset Dialog */}
      <Show when={showSaveDialog()}>
        <div class="dialog-overlay" onClick={() => setShowSaveDialog(false)}>
          <div class="dialog" onClick={(e) => e.stopPropagation()}>
            <div class="dialog-header">
              <h3 class="dialog-title">Save Configuration Preset</h3>
              <button
                class="dialog-close"
                onClick={() => setShowSaveDialog(false)}
              >
                ×
              </button>
            </div>

            <div class="dialog-body">
              <div class="form-field">
                <label class="form-label">Preset Name</label>
                <input
                  type="text"
                  class="form-input"
                  placeholder="My Configuration"
                  value={presetName()}
                  onInput={(e) => setPresetName(e.target.value)}
                  onKeyPress={(e) => {
                    if (e.key === 'Enter') handleSavePreset();
                  }}
                />
              </div>

              <div class="form-field">
                <label class="form-label">Description (optional)</label>
                <textarea
                  class="form-textarea"
                  placeholder="Describe this configuration..."
                  value={presetDescription()}
                  onInput={(e) => setPresetDescription(e.target.value)}
                  rows="3"
                />
              </div>

              <div class="form-info">
                <div class="info-icon">ℹ️</div>
                <div class="info-text">
                  This will save your current configuration settings as a preset that
                  you can load later. Presets are stored locally in your browser.
                </div>
              </div>
            </div>

            <div class="dialog-footer">
              <button
                class="dialog-button dialog-cancel"
                onClick={() => setShowSaveDialog(false)}
              >
                Cancel
              </button>
              <button
                class="dialog-button dialog-primary"
                onClick={handleSavePreset}
                disabled={!presetName().trim()}
              >
                Save Preset
              </button>
            </div>
          </div>
        </div>
      </Show>
    </div>
  );
};

export default ConfigPresets;
