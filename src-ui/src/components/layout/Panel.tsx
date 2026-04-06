// Copyright 2024 Serial CLI Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

import { Component, children, JSX } from 'solid-js';
import './Panel.css';

interface PanelProps {
  title: string;
  subtitle?: string;
  glow?: 'cyan' | 'magenta' | 'purple';
  children?: JSX.Element;
}

const Panel: Component<PanelProps> = (props) => {
  const resolved = children(() => props.children);

  return (
    <div class={`panel panel-${props.glow || 'cyan'}`}>
      <div class="panel-header">
        <div class="panel-title-group">
          <h3 class="panel-title">{props.title}</h3>
          {props.subtitle && (
            <p class="panel-subtitle">{props.subtitle}</p>
          )}
        </div>
        <div class="panel-actions">
          <button class="panel-action-button" title="More options">
            ⋯
          </button>
        </div>
      </div>
      <div class="panel-content">
        {resolved()}
      </div>
    </div>
  );
};

export default Panel;
