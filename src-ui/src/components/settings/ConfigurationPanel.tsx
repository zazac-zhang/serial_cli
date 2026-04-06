// Copyright 2024 Serial CLI Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

import { Component, createSignal, onMount, Show } from 'solid-js';
import { invoke } from '@tauri-apps/api/core';
import { toast } from '../../stores/toastStore';
import './ConfigurationPanel.css';

const ConfigurationPanel: Component = () => {
  const [configContent, setConfigContent] = createSignal('');
  const [configPath, setConfigPath] = createSignal('');
  const [isLoading, setIsLoading] = createSignal(false);
  const [isSaving, setIsSaving] = createSignal(false);
  const [hasChanges, setHasChanges] = createSignal(false);
  const [showResetDialog, setShowResetDialog] = createSignal(false);
  const [validationError, setValidationError] = createSignal<string | null>(null);

  // Load configuration on mount
  onMount(() => {
    loadConfig();
    loadConfigPath();
  });

  const loadConfig = async () => {
    setIsLoading(true);
    setValidationError(null);
    try {
      const content = await invoke<string>('get_config_raw');
      setConfigContent(content);
      setHasChanges(false);
    } catch (error) {
      console.error('Failed to load configuration:', error);
      toast.error('Load Failed', 'Could not load configuration file');
    } finally {
      setIsLoading(false);
    }
  };

  const loadConfigPath = async () => {
    try {
      const path = await invoke<string>('get_config_file_path');
      setConfigPath(path);
    } catch (error) {
      console.error('Failed to get config path:', error);
    }
  };

  const handleSave = async () => {
    if (!configContent().trim()) {
      toast.warning('Empty Config', 'Configuration cannot be empty');
      return;
    }

    setIsSaving(true);
    setValidationError(null);

    try {
      await invoke('save_config_raw', {
        content: configContent(),
      });

      toast.success('Configuration Saved', 'Settings have been updated');
      setHasChanges(false);
      loadConfigPath();
    } catch (error) {
      console.error('Failed to save configuration:', error);
      const errorMsg = String(error);

      // Check if it's a TOML validation error
      if (errorMsg.includes('Invalid TOML syntax')) {
        setValidationError(errorMsg);
        toast.error('Validation Failed', 'Invalid TOML syntax');
      } else {
        toast.error('Save Failed', `Could not save configuration: ${errorMsg}`);
      }
    } finally {
      setIsSaving(false);
    }
  };

  const handleReset = async () => {
    try {
      await invoke('reset_config');
      toast.success('Configuration Reset', 'Settings have been reset to defaults');
      setShowResetDialog(false);
      loadConfig();
    } catch (error) {
      console.error('Failed to reset configuration:', error);
      toast.error('Reset Failed', `Could not reset configuration: ${String(error)}`);
    }
  };

  const handleReload = () => {
    if (hasChanges() && !confirm('You have unsaved changes. Are you sure you want to reload?')) {
      return;
    }
    loadConfig();
  };

  const handleContentChange = (content: string) => {
    setConfigContent(content);
    setHasChanges(true);
    setValidationError(null);
  };

  return (
    <div class="configuration-panel">
      <div class="panel-toolbar">
        <div class="toolbar-info">
          <h3 class="toolbar-title">Configuration Editor</h3>
          <p class="toolbar-description">
            Edit application settings in TOML format
          </p>
          <div class="toolbar-meta">
            <span class="meta-item">📁 {configPath() || '.serial-cli.toml'}</span>
            {hasChanges() && <span class="meta-item meta-warning">⚠️ Unsaved changes</span>}
          </div>
        </div>

        <div class="toolbar-actions">
          <button
            class="action-button"
            onClick={handleReload}
            disabled={isLoading()}
            title="Reload configuration"
          >
            <span class="button-icon">{isLoading() ? '⟳' : '↻'}</span>
            <span class="button-label">Reload</span>
          </button>

          <button
            class="action-button action-warning"
            onClick={() => setShowResetDialog(true)}
            title="Reset to defaults"
          >
            <span class="button-icon">🔄</span>
            <span class="button-label">Reset</span>
          </button>

          <div class="toolbar-divider"></div>

          <button
            class="action-button action-primary"
            onClick={handleSave}
            disabled={isSaving() || !hasChanges()}
            title="Save configuration (Ctrl+S)"
          >
            <span class="button-icon">{isSaving() ? '⟳' : '💾'}</span>
            <span class="button-label">{isSaving() ? 'Saving...' : 'Save'}</span>
          </button>
        </div>
      </div>

      <Show when={validationError()}>
        <div class="validation-error">
          <div class="error-icon">⚠️</div>
          <div class="error-content">
            <div class="error-title">TOML Validation Error</div>
            <div class="error-message">{validationError()}</div>
          </div>
          <button
            class="error-close"
            onClick={() => setValidationError(null)}
          >
            ×
          </button>
        </div>
      </Show>

      <div class="editor-container">
        <Show
          when={!isLoading()}
          fallback={
            <div class="loading-state">
              <div class="loading-spinner">⟳</div>
              <div class="loading-text">Loading configuration...</div>
            </div>
          }
        >
          <textarea
            class="config-editor"
            value={configContent()}
            onInput={(e) => handleContentChange(e.target.value)}
            placeholder="# Configuration will be displayed here"
            spellcheck={false}
          />
        </Show>
      </div>

      <div class="editor-footer">
        <div class="footer-hints">
          <span class="hint-item">Ctrl+S: Save</span>
          <span class="hint-item">Ctrl+R: Reload</span>
          <span class="hint-item">TOML Format</span>
        </div>
        <div class="footer-stats">
          <span class="stat-item">{configContent().split('\n').length} lines</span>
          <span class="stat-item">{configContent().length} chars</span>
        </div>
      </div>

      {/* Reset Confirmation Dialog */}
      <Show when={showResetDialog()}>
        <div class="dialog-overlay" onClick={() => setShowResetDialog(false)}>
          <div class="dialog" onClick={(e) => e.stopPropagation()}>
            <div class="dialog-header">
              <h3 class="dialog-title">Reset Configuration</h3>
              <button
                class="dialog-close"
                onClick={() => setShowResetDialog(false)}
              >
                ×
              </button>
            </div>

            <div class="dialog-body">
              <div class="warning-message">
                <div class="warning-icon">⚠️</div>
                <div class="warning-content">
                  <div class="warning-title">Are you sure?</div>
                  <div class="warning-text">
                    This will reset all configuration settings to their default values.
                    This action cannot be undone.
                  </div>
                </div>
              </div>
            </div>

            <div class="dialog-footer">
              <button
                class="dialog-button dialog-cancel"
                onClick={() => setShowResetDialog(false)}
              >
                Cancel
              </button>
              <button
                class="dialog-button dialog-danger"
                onClick={handleReset}
              >
                Reset to Defaults
              </button>
            </div>
          </div>
        </div>
      </Show>
    </div>
  );
};

export default ConfigurationPanel;
