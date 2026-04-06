// Copyright 2024 Serial CLI Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

import { createStore, produce } from 'solid-js/store';

// Types
export interface DataPacket {
  port_id: string;
  direction: 'tx' | 'rx';
  data: number[];
  timestamp: number;
  display?: {
    hex: string;
    ascii: string;
  };
}

export interface DisplayOptions {
  view: 'hex' | 'ascii' | 'both';
  autoScroll: boolean;
  showTimestamp: boolean;
  maxPackets: number;
}

// Store state
interface DataStore {
  packets: DataPacket[];
  displayOptions: DisplayOptions;
  isMonitoring: boolean;
  filter: string;
}

// Create store
const [dataStore, setDataStore] = createStore<DataStore>({
  packets: [],
  displayOptions: {
    view: 'both',
    autoScroll: true,
    showTimestamp: true,
    maxPackets: 1000,
  },
  isMonitoring: false,
  filter: '',
});

// Export store and actions
export { dataStore };
export default dataStore;

// Export actions separately
export function addPacket(packet: DataPacket) {
  // Generate display strings
  const display = {
    hex: dataToHex(packet.data),
    ascii: dataToAscii(packet.data),
  };

  setDataStore(
    produce((s) => {
      s.packets.push({ ...packet, display });

      // Keep only maxPackets
      const max = s.displayOptions.maxPackets;
      if (s.packets.length > max) {
        s.packets.splice(0, s.packets.length - max);
      }
    })
  );
}

export function clearPackets() {
  setDataStore('packets', []);
}

export function setDisplayOptions(options: Partial<DisplayOptions>) {
  setDataStore(
    produce((s) => {
      Object.assign(s.displayOptions, options);
    })
  );
}

export function toggleMonitoring() {
  setDataStore('isMonitoring', !dataStore.isMonitoring);
}

export function setFilter(filter: string) {
  setDataStore('filter', filter);
}

export function getFilteredPackets(): DataPacket[] {
  const filter = dataStore.filter.toLowerCase();
  if (!filter) {
    return dataStore.packets;
  }

  return dataStore.packets.filter((packet) => {
    const hex = dataToHex(packet.data).toLowerCase();
    const ascii = dataToAscii(packet.data).toLowerCase();
    return hex.includes(filter) || ascii.includes(filter);
  });
}

// Helper functions
function dataToHex(data: number[]): string {
  return data.map((b) => b.toString(16).padStart(2, '0').toUpperCase()).join(' ');
}

function dataToAscii(data: number[]): string {
  return data
    .map((b) => {
      if (b >= 32 && b <= 126) {
        return String.fromCharCode(b);
      } else if (b === 10) {
        return '\n';
      } else if (b === 13) {
        return '\r';
      } else {
        return '.';
      }
    })
    .join('');
}
