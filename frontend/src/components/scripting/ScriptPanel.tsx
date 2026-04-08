import { Panel } from '@/components/ui/panel'
import { cn } from '@/lib/utils'
import { Play, FilePlus, Save, FolderOpen, Trash2, Download, Upload } from 'lucide-react'
import { useState, useRef, useEffect } from 'react'
import Editor from '@monaco-editor/react'

const DEFAULT_SCRIPT = `-- Lua Script for Serial CLI
-- Use the serial API to communicate with devices

function init()
  print("Initializing script...")
  -- Open serial port
  -- serial.open("/dev/ttyUSB0", 9600, 8, 'N', 1)
end

function main()
  print("Running main loop...")

  -- Send data
  -- serial.write("Hello, World!")

  -- Read data
  -- local data = serial.read()
  -- print("Received: " .. data)
end

function cleanup()
  print("Cleaning up...")
  -- serial.close()
end

-- Entry point
init()
main()
cleanup()
`

interface ScriptFile {
  id: string
  name: string
  content: string
  lastModified: number
}

export function ScriptPanel() {
  const [scripts, setScripts] = useState<ScriptFile[]>([])
  const [activeScriptId, setActiveScriptId] = useState<string | null>(null)
  const [scriptContent, setScriptContent] = useState(DEFAULT_SCRIPT)
  const [isRunning, setIsRunning] = useState(false)
  const [output, setOutput] = useState<string[]>([])
  const [selectedFile, setSelectedFile] = useState<File | null>(null)
  const fileInputRef = useRef<HTMLInputElement>(null)

  const activeScript = scripts.find(s => s.id === activeScriptId)

  const createNewScript = () => {
    const newScript: ScriptFile = {
      id: Date.now().toString(),
      name: `untitled-${scripts.length + 1}.lua`,
      content: DEFAULT_SCRIPT,
      lastModified: Date.now(),
    }
    setScripts(prev => [...prev, newScript])
    setActiveScriptId(newScript.id)
    setScriptContent(newScript.content)
  }

  const saveScript = () => {
    if (activeScriptId) {
      setScripts(prev => prev.map(s =>
        s.id === activeScriptId
          ? { ...s, content: scriptContent, lastModified: Date.now() }
          : s
      ))
      setOutput(prev => [...prev, `[${new Date().toLocaleTimeString()}] Script saved`])
    }
  }

  const deleteScript = (id: string) => {
    setScripts(prev => prev.filter(s => s.id !== id))
    if (activeScriptId === id) {
      setActiveScriptId(null)
      setScriptContent(DEFAULT_SCRIPT)
    }
  }

  const loadScriptFile = (file: File) => {
    const reader = new FileReader()
    reader.onload = (e) => {
      const content = e.target?.result as string
      const newScript: ScriptFile = {
        id: Date.now().toString(),
        name: file.name,
        content,
        lastModified: Date.now(),
      }
      setScripts(prev => [...prev, newScript])
      setActiveScriptId(newScript.id)
      setScriptContent(content)
      setOutput(prev => [...prev, `[${new Date().toLocaleTimeString()}] Loaded: ${file.name}`])
    }
    reader.readAsText(file)
  }

  const handleFileSelect = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0]
    if (file) {
      loadScriptFile(file)
    }
  }

  const exportScript = () => {
    if (activeScript) {
      const blob = new Blob([activeScript.content], { type: 'text/plain' })
      const url = URL.createObjectURL(blob)
      const a = document.createElement('a')
      a.href = url
      a.download = activeScript.name
      a.click()
      URL.revokeObjectURL(url)
      setOutput(prev => [...prev, `[${new Date().toLocaleTimeString()}] Exported: ${activeScript.name}`])
    }
  }

  const runScript = () => {
    setIsRunning(true)
    setOutput(prev => [...prev, `[${new Date().toLocaleTimeString()}] Starting script...`])

    // Simulate script execution
    setTimeout(() => {
      setOutput(prev => [...prev, `[${new Date().toLocaleTimeString()}] Script execution complete`])
      setIsRunning(false)
    }, 1500)
  }

  useEffect(() => {
    if (activeScript) {
      setScriptContent(activeScript.content)
    }
  }, [activeScriptId])

  return (
    <div className="space-y-6">
      {/* Scripts List & Editor */}
      <div className="grid grid-cols-4 gap-6 max-w-7xl">
        {/* Sidebar - Script Files */}
        <Panel title="Scripts" variant="amber" className="col-span-1">
          <div className="space-y-2">
            {/* Action buttons */}
            <div className="flex items-center gap-2 mb-4">
              <button
                onClick={createNewScript}
                className="flex-1 flex items-center justify-center gap-1.5 px-3 py-1.5 text-xs rounded-md bg-amber/10 text-amber border border-amber/30 hover:bg-amber/20 transition-colors"
              >
                <FilePlus size={14} strokeWidth={1.5} />
                New
              </button>
              <button
                onClick={() => fileInputRef.current?.click()}
                className="p-1.5 rounded-md hover:bg-bg-elevated text-text-tertiary hover:text-text-primary transition-colors"
                title="Load file"
              >
                <FolderOpen size={14} strokeWidth={1.5} />
              </button>
              <input
                ref={fileInputRef}
                type="file"
                accept=".lua"
                className="hidden"
                onChange={handleFileSelect}
              />
            </div>

            {/* Script list */}
            <div className="space-y-1">
              {scripts.length === 0 ? (
                <div className="py-8 text-center text-xs text-text-tertiary">
                  <p>No scripts yet</p>
                  <p className="mt-1">Create a new script or load a file</p>
                </div>
              ) : (
                scripts.map(script => (
                  <div
                    key={script.id}
                    className={cn(
                      'group flex items-center justify-between px-3 py-2 rounded-md text-xs cursor-pointer transition-colors',
                      activeScriptId === script.id
                        ? 'bg-amber/10 text-amber border border-amber/30'
                        : 'hover:bg-bg-elevated text-text-secondary'
                    )}
                    onClick={() => {
                      setActiveScriptId(script.id)
                      setScriptContent(script.content)
                    }}
                  >
                    <div className="flex items-center gap-2 min-w-0">
                      <FilePlus size={14} strokeWidth={1.5} className="flex-shrink-0" />
                      <span className="truncate">{script.name}</span>
                    </div>
                    <button
                      onClick={(e) => {
                        e.stopPropagation()
                        deleteScript(script.id)
                      }}
                      className="opacity-0 group-hover:opacity-100 p-1 rounded hover:bg-alert/20 text-text-tertiary hover:text-alert transition-all"
                    >
                      <Trash2 size={12} strokeWidth={1.5} />
                    </button>
                  </div>
                ))
              )}
            </div>
          </div>
        </Panel>

        {/* Editor */}
        <Panel
          title={activeScript?.name || 'Editor'}
          variant="default"
          className="col-span-3"
          actions={
            <>
              <button
                onClick={saveScript}
                disabled={!activeScriptId}
                className="p-1.5 rounded hover:bg-bg-elevated text-text-tertiary hover:text-text-primary transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
                title="Save script"
              >
                <Save size={14} strokeWidth={1.5} />
              </button>
              <button
                onClick={exportScript}
                disabled={!activeScript}
                className="p-1.5 rounded hover:bg-bg-elevated text-text-tertiary hover:text-text-primary transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
                title="Export script"
              >
                <Download size={14} strokeWidth={1.5} />
              </button>
              <button
                onClick={runScript}
                disabled={isRunning}
                className="flex items-center gap-1.5 px-3 py-1.5 text-xs rounded-md bg-signal/10 text-signal border border-signal/30 hover:bg-signal/20 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
              >
                <Play size={12} strokeWidth={1.5} className={isRunning ? 'animate-spin' : ''} />
                {isRunning ? 'Running...' : 'Run'}
              </button>
            </>
          }
        >
          <div className="h-[500px] rounded-md overflow-hidden border border-border/50">
            <Editor
              height="100%"
              defaultLanguage="lua"
              theme="vs-dark"
              value={scriptContent}
              onChange={(value) => setScriptContent(value || '')}
              options={{
                minimap: { enabled: false },
                fontSize: 13,
                lineNumbers: 'on',
                scrollBeyondLastLine: false,
                automaticLayout: true,
                padding: { top: 12, bottom: 12 },
              }}
            />
          </div>
        </Panel>
      </div>

      {/* Output Console */}
      <Panel title="Output" variant="default" className="max-w-7xl">
        <div className="h-32 overflow-y-auto font-mono text-xs bg-bg-deepest rounded-md p-3 border border-border/50">
          {output.length === 0 ? (
            <p className="text-text-tertiary">Script output will appear here...</p>
          ) : (
            output.map((line, i) => (
              <div key={i} className="text-text-secondary py-0.5">
                {line}
              </div>
            ))
          )}
        </div>
      </Panel>
    </div>
  )
}
