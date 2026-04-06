// Copyright 2024 Serial CLI Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

import { Component, createSignal, Show } from 'solid-js';
import Panel from '../layout/Panel';
import ScriptEditor from './ScriptEditor';
import ScriptOutput from './ScriptOutput';
import ScriptManager from './ScriptManager';
import './ScriptPanel.css';

const ScriptPanel: Component = () => {
  const [showOutput, setShowOutput] = createSignal(true);
  const [showManager, setShowManager] = createSignal(true);
  const [outputHeight, setOutputHeight] = createSignal(200);
  const [activeTab, setActiveTab] = createSignal<'editor' | 'manager'>('editor');

  return (
    <div class="script-panel">
      <Panel title="Lua Script Development" glow="purple">
        <div class="script-panel-content">
          <div class="panel-toolbar">
            <div class="toolbar-info">
              <h3 class="toolbar-title">Script Editor</h3>
              <p class="toolbar-description">
                Write and execute Lua scripts for serial communication automation
              </p>
            </div>

            <div class="toolbar-controls">
              <button
                class="control-button"
                onClick={() => setActiveTab('editor')}
                classList={{ active: activeTab() === 'editor' }}
                title="Switch to editor"
              >
                <span class="button-icon">✏️</span>
                <span class="button-label">Editor</span>
              </button>

              <button
                class="control-button"
                onClick={() => setActiveTab('manager')}
                classList={{ active: activeTab() === 'manager' }}
                title="Switch to file manager"
              >
                <span class="button-icon">📁</span>
                <span class="button-label">Files</span>
              </button>

              <div class="toolbar-divider"></div>

              <button
                class="control-button"
                onClick={() => setShowOutput(!showOutput())}
                classList={{ active: showOutput() }}
                title="Toggle output panel"
              >
                <span class="button-icon">{showOutput() ? '📊' : '📄'}</span>
                <span class="button-label">Output</span>
              </button>

              <a
                href="https://www.lua.org/manual/5.4/"
                target="_blank"
                rel="noopener noreferrer"
                class="control-button"
                title="Lua documentation (opens in new tab)"
              >
                <span class="button-icon">📚</span>
                <span class="button-label">Docs</span>
              </a>
            </div>
          </div>

          <div
            class="panel-workspace"
            style={showOutput() ? `--output-height: ${outputHeight()}px` : ''}
          >
            <div class="workspace-main">
              <Show when={activeTab() === 'editor'}>
                <div class="editor-section">
                  <ScriptEditor />
                </div>
              </Show>

              <Show when={activeTab() === 'manager'}>
                <div class="manager-section">
                  <ScriptManager />
                </div>
              </Show>
            </div>

            <Show when={showOutput()}>
              <div class="output-section">
                <ScriptOutput />
              </div>
            </Show>
          </div>
        </div>
      </Panel>
    </div>
  );
};

export default ScriptPanel;
