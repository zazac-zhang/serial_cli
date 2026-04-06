// Copyright 2024 Serial CLI Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

import { Component, Show } from 'solid-js';
import Sidebar from './components/layout/Sidebar';
import TopBar from './components/layout/TopBar';
import Panel from './components/layout/Panel';
import PortsPanel from './components/ports/PortsPanel';
import CommandInput from './components/commands/CommandInput';
import QuickActions from './components/commands/QuickActions';
import DataViewer from './components/data/DataViewer';
import ScriptPanel from './components/scripting/ScriptPanel';
import ProtocolPanel from './components/protocols/ProtocolPanel';
import SettingsPanel from './components/settings/SettingsPanel';
import ToastNotifications from './components/ui/ToastNotifications';
import { getCurrentView, NavigationView } from './stores/navigationStore';

const App: Component = () => {
  const currentView = () => getCurrentView();

  return (
    <div class="app-container">
      <TopBar />
      <div class="app-content">
        <Sidebar />
        <main class="main-content">
          <Show when={currentView() === 'ports'}>
            <div class="content-grid">
              <div class="content-row">
                <div class="content-col">
                  <PortsPanel />
                </div>
                <div class="content-col">
                  <Panel title="Quick Command" glow="magenta">
                    <CommandInput />
                  </Panel>
                </div>
              </div>
            </div>
          </Show>

          <Show when={currentView() === 'data'}>
            <div class="content-grid">
              <div class="content-row">
                <div class="content-col">
                  <Panel title="Quick Actions" glow="purple">
                    <QuickActions />
                  </Panel>
                </div>
                <div class="content-col">
                  <Panel title="Quick Command" glow="magenta">
                    <CommandInput />
                  </Panel>
                </div>
              </div>
              <div class="content-row">
                <div class="content-col full-width">
                  <Panel title="Data Monitor" glow="cyan">
                    <DataViewer />
                  </Panel>
                </div>
              </div>
            </div>
          </Show>

          <Show when={currentView() === 'scripts'}>
            <div class="content-grid">
              <div class="content-row">
                <div class="content-col full-width">
                  <ScriptPanel />
                </div>
              </div>
            </div>
          </Show>

          <Show when={currentView() === 'protocols'}>
            <div class="content-grid">
              <div class="content-row">
                <div class="content-col full-width">
                  <ProtocolPanel />
                </div>
              </div>
            </div>
          </Show>

          <Show when={currentView() === 'settings'}>
            <div class="content-grid">
              <div class="content-row">
                <div class="content-col full-width">
                  <SettingsPanel />
                </div>
              </div>
            </div>
          </Show>
        </main>
      </div>
      <ToastNotifications />
    </div>
  );
};

export default App;
