// Copyright 2024 Serial CLI Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to these terms.

import { Component, createSignal } from 'solid-js';
import Panel from '../layout/Panel';
import ConfigurationPanel from './ConfigurationPanel';
import ConfigPresets from './ConfigPresets';
import './SettingsPanel.css';

const SettingsPanel: Component = () => {
  const [activeTab, setActiveTab] = createSignal<'editor' | 'presets'>('editor');

  return (
    <div class="settings-panel">
      <Panel title="Application Settings" glow="magenta">
        <div class="settings-panel-content">
          <div class="panel-toolbar">
            <div class="toolbar-info">
              <h3 class="toolbar-title">Settings Editor</h3>
              <p class="toolbar-description">
                Configure application behavior and preferences
              </p>
            </div>

            <div class="toolbar-controls">
              <button
                class="control-button"
                onClick={() => setActiveTab('editor')}
                classList={{ active: activeTab() === 'editor' }}
                title="Configuration editor"
              >
                <span class="button-icon">⚙️</span>
                <span class="button-label">Editor</span>
              </button>

              <button
                class="control-button"
                onClick={() => setActiveTab('presets')}
                classList={{ active: activeTab() === 'presets' }}
                title="Configuration presets"
              >
                <span class="button-icon">📁</span>
                <span class="button-label">Presets</span>
              </button>

              <a
                href="https://github.com/your-repo/serial-cli/blob/main/docs/CONFIGURATION.md"
                target="_blank"
                rel="noopener noreferrer"
                class="control-button"
                title="Configuration documentation (opens in new tab)"
              >
                <span class="button-icon">📚</span>
                <span class="button-label">Docs</span>
              </a>
            </div>
          </div>

          <div class="panel-workspace">
            <div class="workspace-main">
              {activeTab() === 'editor' && <ConfigurationPanel />}
              {activeTab() === 'presets' && <ConfigPresets />}
            </div>
          </div>
        </div>
      </Panel>
    </div>
  );
};

export default SettingsPanel;
