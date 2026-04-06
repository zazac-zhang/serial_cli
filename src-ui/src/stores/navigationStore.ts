// Copyright 2024 Serial CLI Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

import { createStore } from 'solid-js/store';

export type NavigationView = 'ports' | 'data' | 'scripts' | 'protocols' | 'settings';

interface NavigationStore {
  currentView: NavigationView;
}

const [navigationStore, setNavigationStore] = createStore<NavigationStore>({
  currentView: 'ports',
});

export function setCurrentView(view: NavigationView) {
  setNavigationStore('currentView', view);
}

export function getCurrentView(): NavigationView {
  return navigationStore.currentView;
}

export default navigationStore;
