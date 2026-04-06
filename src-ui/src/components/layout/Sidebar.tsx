// Copyright 2024 Serial CLI Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

import { Component, For } from 'solid-js';
import { getCurrentView, setCurrentView, NavigationView } from '../../stores/navigationStore';
import './Sidebar.css';

const Sidebar: Component = () => {
  const currentView = () => getCurrentView();

  const menuItems = () => [
    { icon: '🔌', label: 'Ports', view: 'ports' as NavigationView },
    { icon: '📊', label: 'Data Monitor', view: 'data' as NavigationView },
    { icon: '🔧', label: 'Scripts', view: 'scripts' as NavigationView },
    { icon: '📋', label: 'Protocols', view: 'protocols' as NavigationView },
    { icon: '⚙️', label: 'Settings', view: 'settings' as NavigationView },
  ];

  return (
    <aside class="sidebar">
      <div class="sidebar-header">
        <h2 class="sidebar-title">
          <span class="text-cyan">Serial</span> CLI
        </h2>
      </div>
      <nav class="sidebar-nav">
        <For each={menuItems()}>
          {(item) => (
            <button
              classList={{
                'sidebar-nav-item': true,
                'active': currentView() === item.view
              }}
              onClick={() => setCurrentView(item.view)}
            >
              <span class="sidebar-nav-icon">{item.icon}</span>
              <span class="sidebar-nav-label">{item.label}</span>
            </button>
          )}
        </For>
      </nav>
      <div class="sidebar-footer">
        <div class="status-indicator">
          <span class="status-dot"></span>
          <span class="status-text">Ready</span>
        </div>
      </div>
    </aside>
  );
};

export default Sidebar;
