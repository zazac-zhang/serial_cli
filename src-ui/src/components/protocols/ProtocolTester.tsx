// Copyright 2024 Serial CLI Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

import { Component, createSignal, For, Show } from 'solid-js';
import { invoke } from '@tauri-apps/api/core';
import { toast } from '../../stores/toastStore';
import './ProtocolTester.css';

interface Protocol {
  name: string;
  description: string;
}

interface TestResult {
  input: string;
  output: string;
  operation: 'encode' | 'decode';
  protocol: string;
  timestamp: number;
  success: boolean;
  error?: string;
}

const ProtocolTester: Component = () => {
  const [protocols, setProtocols] = createSignal<Protocol[]>([]);
  const [selectedProtocol, setSelectedProtocol] = createSignal<string>('');
  const [testData, setTestData] = createSignal('');
  const [testMode, setTestMode] = createSignal<'encode' | 'decode'>('encode');
  const [results, setResults] = createSignal<TestResult[]>([]);
  const [isTesting, setIsTesting] = createSignal(false);

  // Load protocols on mount
  (async () => {
    try {
      const protoList = await invoke<{ name: string; description: string }[]>('list_protocols');
      setProtocols(protoList);
      if (protoList.length > 0) {
        setSelectedProtocol(protoList[0].name);
      }
    } catch (error) {
      console.error('Failed to load protocols:', error);
      toast.error('Load Failed', 'Could not load protocol list');
    }
  })();

  const handleTest = async () => {
    if (!selectedProtocol()) {
      toast.warning('No Protocol', 'Please select a protocol');
      return;
    }

    if (!testData().trim()) {
      toast.warning('No Data', 'Please enter test data');
      return;
    }

    setIsTesting(true);

    try {
      const operation = testMode();
      let data: Uint8Array;

      // Parse input data
      if (operation === 'encode') {
        // For encoding, treat input as hex string or ASCII
        data = parseInputData(testData());
      } else {
        // For decoding, treat input as hex string
        data = hexStringToBytes(testData());
      }

      // Call protocol encode/decode
      let result: Uint8Array;
      if (operation === 'encode') {
        result = await invoke('protocol_encode', {
          protocol: selectedProtocol(),
          data: Array.from(data),
        });
      } else {
        result = await invoke('protocol_decode', {
          protocol: selectedProtocol(),
          data: Array.from(data),
        });
      }

      // Add successful result
      const newResult: TestResult = {
        input: testData(),
        output: bytesToHex(result),
        operation,
        protocol: selectedProtocol(),
        timestamp: Date.now(),
        success: true,
      };

      setResults([newResult, ...results()]);
      toast.success('Test Complete', `${operation.toUpperCase()} successful`);
    } catch (error) {
      console.error('Test failed:', error);

      // Add failed result
      const newResult: TestResult = {
        input: testData(),
        output: '',
        operation: testMode(),
        protocol: selectedProtocol(),
        timestamp: Date.now(),
        success: false,
        error: String(error),
      };

      setResults([newResult, ...results()]);
      toast.error('Test Failed', String(error));
    } finally {
      setIsTesting(false);
    }
  };

  const parseInputData = (input: string): Uint8Array => {
    const trimmed = input.trim();

    // Check if it's a hex string (starts with 0x or contains only hex chars and spaces)
    if (trimmed.startsWith('0x') || /^[\dA-Fa-f\s]+$/.test(trimmed)) {
      return hexStringToBytes(trimmed);
    }

    // Treat as ASCII string
    return new TextEncoder().encode(trimmed);
  };

  const hexStringToBytes = (hex: string): Uint8Array => {
    const cleaned = hex.replace(/^0x/i, '').replace(/\s+/g, '');
    const bytes = new Uint8Array(cleaned.length / 2);
    for (let i = 0; i < bytes.length; i++) {
      bytes[i] = parseInt(cleaned.substr(i * 2, 2), 16);
    }
    return bytes;
  };

  const bytesToHex = (bytes: Uint8Array): string => {
    return Array.from(bytes)
      .map((b) => b.toString(16).padStart(2, '0').toUpperCase())
      .join(' ');
  };

  const bytesToAscii = (bytes: Uint8Array): string => {
    return Array.from(bytes)
      .map((b) => (b >= 32 && b <= 126 ? String.fromCharCode(b) : '.'))
      .join('');
  };

  const clearResults = () => {
    setResults([]);
    toast.info('Results Cleared', 'Test history has been cleared');
  };

  const loadExample = () => {
    const examples: Record<string, string> = {
      'line': 'Hello, World!\n',
      'at_command': 'AT+CMD\r',
      'modbus_rtu': '01 03 00 00 00 0A C4 0E',
      'modbus_ascii': ':01030000000AF1\r\n',
    };

    const protocol = selectedProtocol();
    if (examples[protocol]) {
      setTestData(examples[protocol]);
      toast.info('Example Loaded', `Loaded ${protocol} example`);
    } else {
      setTestData('Test data');
    }
  };

  return (
    <div class="protocol-tester">
      <div class="tester-header">
        <h3 class="tester-title">Protocol Tester</h3>
        <button
          class="header-button"
          onClick={loadExample}
          title="Load example data"
        >
          📝 Load Example
        </button>
      </div>

      <div class="tester-controls">
        <div class="control-group">
          <label class="control-label">Protocol</label>
          <select
            class="control-select"
            value={selectedProtocol()}
            onInput={(e) => setSelectedProtocol(e.target.value)}
          >
            <For each={protocols()}>
              {(protocol) => (
                <option value={protocol.name}>{protocol.name}</option>
              )}
            </For>
          </select>
        </div>

        <div class="control-group">
          <label class="control-label">Operation</label>
          <div class="mode-toggle">
            <button
              class="mode-button"
              classList={{ active: testMode() === 'encode' }}
              onClick={() => setTestMode('encode')}
            >
              Encode
            </button>
            <button
              class="mode-button"
              classList={{ active: testMode() === 'decode' }}
              onClick={() => setTestMode('decode')}
            >
              Decode
            </button>
          </div>
        </div>
      </div>

      <div class="tester-input">
        <label class="input-label">
          {testMode() === 'encode' ? 'Input Data (ASCII or Hex)' : 'Input Data (Hex)'}
        </label>
        <textarea
          class="input-textarea"
          placeholder={
            testMode() === 'encode'
              ? 'Enter ASCII text or hex bytes (e.g., "Hello" or "48 65 6C 6C 6F")'
              : 'Enter hex bytes (e.g., "48 65 6C 6C 6F")'
          }
          value={testData()}
          onInput={(e) => setTestData(e.target.value)}
          spellcheck={false}
        />
        <div class="input-actions">
          <button
            class="action-button action-primary"
            onClick={handleTest}
            disabled={isTesting()}
          >
            {isTesting() ? 'Testing...' : `Test ${testMode().toUpperCase()}`}
          </button>
        </div>
      </div>

      <div class="tester-results">
        <div class="results-header">
          <h4 class="results-title">Test Results</h4>
          <Show when={results().length > 0}>
            <button
              class="results-clear"
              onClick={clearResults}
            >
              Clear All
            </button>
          </Show>
        </div>

        <div class="results-list">
          {results().length === 0 ? (
            <div class="results-empty">
              <div class="empty-icon">🧪</div>
              <div class="empty-text">No test results yet</div>
              <div class="empty-hint">
                Run a test to see results here
              </div>
            </div>
          ) : (
            <For each={results()}>
              {(result) => (
                <div
                  class={`result-item ${result.success ? 'success' : 'error'}`}
                >
                  <div class="result-header">
                    <span class="result-operation">
                      {result.operation.toUpperCase()}
                    </span>
                    <span class="result-protocol">{result.protocol}</span>
                    <span class="result-timestamp">
                      {new Date(result.timestamp).toLocaleTimeString()}
                    </span>
                  </div>

                  <Show when={result.success}>
                    <div class="result-content">
                      <div class="result-field">
                        <span class="field-label">Input:</span>
                        <span class="field-value">{result.input}</span>
                      </div>
                      <div class="result-field">
                        <span class="field-label">Output (Hex):</span>
                        <span class="field-value field-hex">{result.output}</span>
                      </div>
                      <div class="result-field">
                        <span class="field-label">Output (ASCII):</span>
                        <span class="field-value">
                          {bytesToAscii(
                            hexStringToBytes(result.output)
                          )}
                        </span>
                      </div>
                    </div>
                  </Show>

                  <Show when={!result.success}>
                    <div class="result-error">
                      <span class="error-icon">❌</span>
                      <span class="error-message">{result.error}</span>
                    </div>
                  </Show>
                </div>
              )}
            </For>
          )}
        </div>
      </div>
    </div>
  );
};

export default ProtocolTester;
