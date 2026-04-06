// Copyright 2024 Serial CLI Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

import { Component } from 'solid-js';
import './TopBar.css';

const TopBar: Component = () => {
  return (
    <header class="topbar">
      <div class="topbar-left">
        <div class="topbar-logo">
          <span class="logo-icon">⚡</span>
          <span class="logo-text">
            <span class="text-cyan">Serial</span> CLI
            <span class="logo-version">v0.1.0</span>
          </span>
        </div>
      </div>
      <div class="topbar-center">
        <div class="connection-status">
          <span class="status-badge status-disconnected">
            <span class="status-dot"></span>
            No Connection
          </span>
        </div>
      </div>
      <div class="topbar-right">
        <button class="topbar-button" title="Settings">
          ⚙️
        </button>
        <button class="topbar-button" title="Help">
          ❓
        </button>
      </div>
    </header>
  );
};

export default TopBar;
