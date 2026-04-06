// Copyright 2024 Serial CLI Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

import { Component, createSignal } from 'solid-js';
import Panel from '../layout/Panel';
import ProtocolManager from './ProtocolManager';
import ProtocolTester from './ProtocolTester';
import './ProtocolPanel.css';

const ProtocolPanel: Component = () => {
  const [activeTab, setActiveTab] = createSignal<'manager' | 'tester'>('manager');

  return (
    <div class="protocol-panel">
      <Panel title="Protocol Management" glow="cyan">
        <div class="protocol-panel-content">
          <div class="panel-toolbar">
            <div class="toolbar-info">
              <h3 class="toolbar-title">Protocol Manager</h3>
              <p class="toolbar-description">
                Manage built-in and custom serial communication protocols
              </p>
            </div>

            <div class="toolbar-controls">
              <button
                class="control-button"
                onClick={() => setActiveTab('manager')}
                classList={{ active: activeTab() === 'manager' }}
                title="Protocol manager"
              >
                <span class="button-icon">📦</span>
                <span class="button-label">Manager</span>
              </button>

              <button
                class="control-button"
                onClick={() => setActiveTab('tester')}
                classList={{ active: activeTab() === 'tester' }}
                title="Protocol tester"
              >
                <span class="button-icon">🧪</span>
                <span class="button-label">Tester</span>
              </button>

              <a
                href="https://github.com/your-repo/serial-cli/blob/main/docs/PROTOCOLS.md"
                target="_blank"
                rel="noopener noreferrer"
                class="control-button"
                title="Protocol documentation (opens in new tab)"
              >
                <span class="button-icon">📚</span>
                <span class="button-label">Docs</span>
              </a>
            </div>
          </div>

          <div class="panel-workspace">
            <div class="workspace-main">
              {activeTab() === 'manager' && <ProtocolManager />}
              {activeTab() === 'tester' && <ProtocolTester />}
            </div>
          </div>
        </div>
      </Panel>
    </div>
  );
};

export default ProtocolPanel;
