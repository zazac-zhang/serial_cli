// Copyright 2024 Serial CLI Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

import { Component, createSignal, onMount, onCleanup } from 'solid-js';
import * as monaco from 'monaco-editor';
import loader from '@monaco-editor/loader';
import { invoke } from '@tauri-apps/api/core';
import { toast } from '../../stores/toastStore';
import './ScriptEditor.css';

// Configure Monaco Editor loader
loader.config({
  paths: {
    vs: 'https://cdnjs.cloudflare.com/ajax/libs/monaco-editor/0.45.0/min/vs',
  },
});

interface ScriptEditorProps {
  scriptId?: string;
  initialContent?: string;
  onSave?: (content: string) => void;
  onExecute?: (content: string) => void;
  readOnly?: boolean;
}

const ScriptEditor: Component<ScriptEditorProps> = (props) => {
  const [editorContainer, setEditorContainer] = createSignal<HTMLDivElement>();
  const [monacoEditor, setMonacoEditor] = createSignal<monaco.editor.IStandaloneCodeEditor | null>(null);
  const [isExecuting, setIsExecuting] = createSignal(false);
  const [isSaving, setIsSaving] = createSignal(false);

  // Default Lua template
  const defaultScript = `-- Serial CLI Lua Script
-- This template provides basic serial port operations

-- Open a serial port
local port = serial_open("/dev/ttyUSB0", 115200)

if port then
    print("Port opened successfully")

    -- Send data
    serial_send(port, "AT+CMD\\n")

    -- Read response (timeout: 1000ms)
    local data = serial_recv(port, 1000)

    if data then
        print("Received: " .. bytes_to_string(data))
    end

    -- Close port
    serial_close(port)
    print("Port closed")
else
    print("Failed to open port")
end
`;

  onMount(async () => {
    await loader.init();

    const container = editorContainer();
    if (!container) return;

    // Create Monaco Editor instance
    const instance = monaco.editor.create(container, {
      value: props.initialContent || defaultScript,
      language: 'lua',
      theme: 'vs-dark',
      automaticLayout: true,
      fontSize: 14,
      fontFamily: "'JetBrains Mono', 'Fira Code', 'Consolas', monospace",
      minimap: { enabled: true },
      scrollBeyondLastLine: false,
      wordWrap: 'on',
      lineNumbers: 'on',
      renderLineHighlight: 'all',
      cursorBlinking: 'smooth',
      cursorSmoothCaretAnimation: 'on',
      smoothScrolling: true,
      tabSize: 2,
      insertSpaces: true,
      detectIndentation: true,
      folding: true,
      foldingStrategy: 'indentation',
      showFoldingControls: 'always',
      formatOnPaste: true,
      formatOnType: true,
      autoIndent: 'full',
      suggestOnTriggerCharacters: true,
      quickSuggestions: {
        other: true,
        comments: false,
        strings: false,
      },
      parameterHints: {
        enabled: true,
      },
      acceptSuggestionOnCommitCharacter: true,
      acceptSuggestionOnEnter: 'on',
      tabCompletion: 'on',
    } as any); // Type assertion to avoid Monaco type issues

    setMonacoEditor(instance);

    // Apply custom theme for cyber-industrial style
    monaco.editor.defineTheme('serial-cli-dark', {
      base: 'vs-dark',
      inherit: true,
      rules: [
        { token: 'comment', foreground: '6A9955' },
        { token: 'keyword', foreground: 'C586C0' },
        { token: 'string', foreground: 'CE9178' },
        { token: 'number', foreground: 'B5CEA8' },
        { token: 'type', foreground: '4EC9B0' },
        { token: 'function', foreground: 'DCDCAA' },
        { token: 'variable', foreground: '9CDCFE' },
        { token: 'operator', foreground: 'D4D4D4' },
      ],
      colors: {
        'editor.background': '#0a0a0f',
        'editor.foreground': '#e0e0e0',
        'editor.lineHighlightBackground': '#12121a',
        'editorCursor.foreground': '#00F0FF',
        'editor.selectionBackground': '#264f78',
        'editor.inactiveSelectionBackground': '#3a3d41',
        'editorLineNumber.foreground': '#858585',
        'editorLineNumber.activeForeground': '#c6c6c6',
        'editorIndentGuide.background': '#404040',
        'editorIndentGuide.activeBackground': '#707070',
        'editorGroupHeader.tabsBackground': '#252526',
        'editorGroupHeader.noTabsBackground': '#252526',
        'editorGroup.border': '#444444',
        'editorWidget.background': '#252526',
        'editorWidget.foreground': '#e0e0e0',
        'editorWidget.border': '#444444',
      },
    });

    monaco.editor.setTheme('serial-cli-dark');

    // Cleanup on unmount
    onCleanup(() => {
      instance.dispose();
    });
  });

  const handleExecute = async () => {
    const editor = monacoEditor();
    if (!editor) return;

    const content = editor.getValue();
    if (!content.trim()) {
      toast.warning('Empty Script', 'Please enter some Lua code');
      return;
    }

    setIsExecuting(true);

    try {
      // Execute script via Tauri
      const result = await invoke<string>('execute_script', {
        script: content,
      });

      toast.success('Script Executed', result);

      // Call onExecute callback if provided
      if (props.onExecute) {
        props.onExecute(content);
      }
    } catch (error) {
      console.error('Script execution failed:', error);
      toast.error(
        'Execution Failed',
        `Script error: ${String(error)}`
      );
    } finally {
      setIsExecuting(false);
    }
  };

  const handleSave = async () => {
    const editor = monacoEditor();
    if (!editor) return;

    const content = editor.getValue();
    if (!content.trim()) {
      toast.warning('Empty Script', 'Nothing to save');
      return;
    }

    setIsSaving(true);

    try {
      // Save script via Tauri
      await invoke('save_script', {
        name: props.scriptId || 'untitled',
        content,
      });

      toast.success('Script Saved', 'Script has been saved successfully');

      // Call onSave callback if provided
      if (props.onSave) {
        props.onSave(content);
      }
    } catch (error) {
      console.error('Script save failed:', error);
      toast.error(
        'Save Failed',
        `Could not save script: ${String(error)}`
      );
    } finally {
      setIsSaving(false);
    }
  };

  const handleFormat = () => {
    const editor = monacoEditor();
    if (!editor) return;

    editor.getAction('editor.action.formatDocument')?.run();
  };

  const handleReset = () => {
    const editor = monacoEditor();
    if (!editor) return;

    if (confirm('Are you sure you want to reset to the default template?')) {
      editor.setValue(defaultScript);
      toast.info('Editor Reset', 'Content has been reset to default template');
    }
  };

  const handleClear = () => {
    const editor = monacoEditor();
    if (!editor) return;

    if (confirm('Are you sure you want to clear all content?')) {
      editor.setValue('');
      toast.info('Editor Cleared', 'All content has been cleared');
    }
  };

  return (
    <div class="script-editor">
      <div class="editor-toolbar">
        <div class="toolbar-group">
          <h3 class="editor-title">Lua Script Editor</h3>
          <div class="editor-status">
            <span class="status-item">Lua</span>
            <span class="status-item">UTF-8</span>
          </div>
        </div>

        <div class="toolbar-actions">
          <button
            class="toolbar-button"
            onClick={handleFormat}
            title="Format code (Shift+Alt+F)"
          >
            <span class="button-icon">✨</span>
            <span class="button-label">Format</span>
          </button>

          <button
            class="toolbar-button"
            onClick={handleReset}
            title="Reset to template"
          >
            <span class="button-icon">🔄</span>
            <span class="button-label">Reset</span>
          </button>

          <button
            class="toolbar-button"
            onClick={handleClear}
            title="Clear all content"
          >
            <span class="button-icon">🗑️</span>
            <span class="button-label">Clear</span>
          </button>

          <div class="toolbar-divider"></div>

          <button
            class="toolbar-button toolbar-primary"
            onClick={handleSave}
            disabled={isSaving() || props.readOnly}
            title="Save script (Ctrl+S)"
          >
            <span class="button-icon">{isSaving() ? '⟳' : '💾'}</span>
            <span class="button-label">{isSaving() ? 'Saving...' : 'Save'}</span>
          </button>

          <button
            class="toolbar-button toolbar-success"
            onClick={handleExecute}
            disabled={isExecuting() || props.readOnly}
            title="Execute script (F5)"
          >
            <span class="button-icon">{isExecuting() ? '⟳' : '▶️'}</span>
            <span class="button-label">{isExecuting() ? 'Executing...' : 'Execute'}</span>
          </button>
        </div>
      </div>

      <div class="editor-container" ref={setEditorContainer}></div>

      <div class="editor-footer">
        <div class="footer-hints">
          <span class="hint-item">Ctrl+Enter: Execute</span>
          <span class="hint-item">Ctrl+S: Save</span>
          <span class="hint-item">Ctrl+F: Find</span>
          <span class="hint-item">F1: Commands</span>
        </div>
      </div>
    </div>
  );
};

export default ScriptEditor;
