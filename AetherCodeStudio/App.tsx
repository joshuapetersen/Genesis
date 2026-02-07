import React, { useReducer, useEffect, useMemo, useState } from 'react';
import { AppState, FileAction, ModelType, Extension, Command } from './types';
import { INITIAL_FILES } from './constants';
import { FileExplorer } from './components/FileExplorer';
import { Editor } from './components/Editor';
import { AIChatPanel } from './components/AIChatPanel';
import { SettingsModal } from './components/SettingsModal';
import { Preview } from './components/Preview';
import { Icons } from './components/Icon';
import { ActivityBar } from './components/ActivityBar';
import { ExtensionsPanel } from './components/ExtensionsPanel';
import { CommandPalette } from './components/CommandPalette';
import { executeExtension, generateExtension } from './services/pluginRuntime';
import { fsService } from './services/fileSystem';

const initialState: AppState = {
  files: [], // Now managed dynamically or just current file
  activeFileId: null,
  model: ModelType.FLASH,
  chatHistory: [],
  extensions: [],
  activeView: 'explorer',
  isSettingsOpen: false,
  isPreviewOpen: false,
  isCommandPaletteOpen: false,
};

const reducer = (state: AppState, action: FileAction): AppState => {
  switch (action.type) {
    // Modified: Only tracks the currently open file content for editor state
    case 'SET_ACTIVE_FILE':
      return { ...state, activeFileId: action.payload };
    case 'UPDATE_FILE':
      // Update content in our local cache (files array) if we were tracking multiple
      // For now, we update the matched file in state.files if present
      return {
        ...state,
        files: state.files.map(f => f.id === action.payload.id ? { ...f, content: action.payload.content } : f)
      };
    case 'SET_FILE_CONTENT':
      // Helper to load content into the 'files' array so Editor can see it
      // Check if file exists, if not add it
      const exists = state.files.find(f => f.id === action.payload.id);
      if (exists) {
        return {
          ...state,
          files: state.files.map(f => f.id === action.payload.id ? { ...f, content: action.payload.content } : f),
          activeFileId: action.payload.id
        };
      } else {
        return {
          ...state,
          files: [...state.files, action.payload],
          activeFileId: action.payload.id
        };
      }

    case 'SET_MODEL': return { ...state, model: action.payload };
    case 'ADD_MESSAGE': return { ...state, chatHistory: [...state.chatHistory, action.payload] };
    case 'TOGGLE_SETTINGS': return { ...state, isSettingsOpen: !state.isSettingsOpen };
    case 'TOGGLE_PREVIEW': return { ...state, isPreviewOpen: !state.isPreviewOpen };
    case 'TOGGLE_COMMAND_PALETTE': return { ...state, isCommandPaletteOpen: !state.isCommandPaletteOpen };
    case 'SET_VIEW': return { ...state, activeView: action.payload };
    case 'INSTALL_EXTENSION': return { ...state, extensions: [...state.extensions, action.payload] };
    case 'TOGGLE_EXTENSION':
      return {
        ...state,
        extensions: state.extensions.map(e => e.id === action.payload ? { ...e, isEnabled: !e.isEnabled } : e)
      };
    default: return state;
  }
};

const App: React.FC = () => {
  const [state, dispatch] = useReducer(reducer, initialState);
  const [rootPath, setRootPath] = useState("C:\\SarahCore");

  // Derived active file
  const activeFile = state.files.find(f => f.id === state.activeFileId);

  // Command Palette Logic
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if ((e.metaKey || e.ctrlKey) && e.key === 'p') {
        e.preventDefault();
        dispatch({ type: 'TOGGLE_COMMAND_PALETTE' });
      }
      if ((e.metaKey || e.ctrlKey) && e.key === 's') {
        e.preventDefault();
        handleSave();
      }
    };
    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [state.activeFileId, state.files]); // Re-bind on file status change

  const handleFileSelect = async (path: string) => {
    try {
      const content = await fsService.readFile(path);
      // Infer language
      const ext = path.split('.').pop() || 'txt';
      let lang = 'text';
      if (['js', 'jsx', 'ts', 'tsx'].includes(ext)) lang = 'javascript';
      if (ext === 'py') lang = 'python';
      if (ext === 'html') lang = 'html';
      if (ext === 'css') lang = 'css';
      if (ext === 'json') lang = 'json';
      if (ext === 'md') lang = 'markdown';

      dispatch({
        type: 'SET_FILE_CONTENT',
        payload: { id: path, name: path.split('\\').pop() || 'file', language: lang, content }
      });
    } catch (e) {
      console.error("Failed to open file", e);
      alert("Failed to read file: " + path);
    }
  };

  const handleSave = async () => {
    if (activeFile) {
      const success = await fsService.writeFile(activeFile.id, activeFile.content);
      if (success) {
        // Minimal feedback? Or rely on console
        console.log("Saved: " + activeFile.id);
      } else {
        alert("Failed to save: " + activeFile.id);
      }
    }
  };

  // Plugin System Interface
  const pluginApi = useMemo(() => ({
    registerCommand: (cmd: Command) => console.log('Plugin Registered Command:', cmd.title),
    getActiveFile: () => activeFile,
    modifyFile: (fileId: string, content: string) => {
      dispatch({ type: 'UPDATE_FILE', payload: { id: fileId, content } });
    },
    executeSystemCommand: async (command: string) => {
      try {
        console.log("[Sovereign] Executing System Command:", command);
        const response = await fetch("http://127.0.0.1:8001/api/system/execute", {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ command })
        });
        return await response.json();
      } catch (err) {
        return { status: "ERROR", message: String(err) };
      }
    },
    showNotification: (msg: string) => alert(msg),
    log: (msg: string) => console.log('[Extension Log]', msg)
  }), [activeFile]);

  useEffect(() => {
    state.extensions.forEach(ext => executeExtension(ext, pluginApi));
  }, [state.extensions, pluginApi]);

  const handleGenerateAI = (query: string) => {
    const newExtension = generateExtension(query);
    dispatch({ type: 'INSTALL_EXTENSION', payload: newExtension });
  };

  const commands: Command[] = [
    {
      id: '1', title: 'Open Folder...', action: () => {
        const newPath = prompt("Enter full path to open:", rootPath);
        if (newPath) setRootPath(newPath);
      }
    },
    { id: '2', title: 'Toggle Preview', action: () => dispatch({ type: 'TOGGLE_PREVIEW' }) },
    { id: '3', title: 'Open Settings', action: () => dispatch({ type: 'TOGGLE_SETTINGS' }) },
    { id: '4', title: 'Install Extension...', action: () => dispatch({ type: 'SET_VIEW', payload: 'extensions' }) },
  ];

  return (
    <div className="flex flex-col h-screen overflow-hidden text-gray-300 font-sans bg-gray-900">

      <header className="h-10 bg-gray-950 border-b border-gray-800 flex items-center justify-center relative shrink-0 -mb-px z-50">
        <div className="flex items-center gap-2 absolute left-4 text-xs font-medium text-gray-500">
          <Icons.Cpu /> Sarah Sovereign
        </div>
        <button
          onClick={() => dispatch({ type: 'TOGGLE_COMMAND_PALETTE' })}
          className="bg-gray-900 border border-gray-800 text-gray-400 px-32 py-1 rounded text-xs hover:border-gray-600 transition-colors flex items-center gap-2"
        >
          <Icons.Search /> Aether Code Studio
        </button>
      </header>

      <div className="flex-1 flex overflow-hidden">
        <ActivityBar
          activeView={state.activeView}
          onViewChange={(v) => dispatch({ type: 'SET_VIEW', payload: v })}
          onSettingsClick={() => dispatch({ type: 'TOGGLE_SETTINGS' })}
        />

        {state.activeView === 'explorer' && (
          <FileExplorer
            activeFileId={state.activeFileId}
            onFileSelect={handleFileSelect}
            rootPath={rootPath}
            setRootPath={setRootPath}
          />
        )}

        {state.activeView === 'extensions' && (
          <ExtensionsPanel
            extensions={state.extensions}
            onInstall={(ext) => dispatch({ type: 'INSTALL_EXTENSION', payload: ext })}
            onGenerateAI={handleGenerateAI}
            onToggle={(id) => dispatch({ type: 'TOGGLE_EXTENSION', payload: id })}
            onDelete={(id) => { }}

          />
        )}

        {state.activeView === 'search' && (
          <div className="w-64 bg-gray-900 border-r border-gray-800 p-4 text-sm text-gray-500">
            Global search not available.
          </div>
        )}

        <div className="flex-1 flex flex-col min-w-0 bg-gray-900">
          {activeFile ? (
            <Editor
              content={activeFile.content}
              language={activeFile.language}
              fileName={activeFile.name}
              onChange={(val) => dispatch({ type: 'UPDATE_FILE', payload: { id: activeFile.id, content: val } })}
            />
          ) : (
            <div className="flex-1 flex items-center justify-center text-gray-600">
              <div className="text-center opacity-50">
                <div className="flex justify-center mb-2"><Icons.Command /></div>
                <p className="text-sm">Cmd+P to search files</p>
                <p className="text-xs mt-2">Currently watching: {rootPath}</p>
              </div>
            </div>
          )}
        </div>

        <AIChatPanel
          messages={state.chatHistory}
          onSendMessage={(text, role) => dispatch({
            type: 'ADD_MESSAGE',
            payload: { id: Date.now().toString(), role, text, timestamp: Date.now() }
          })}
          model={state.model}
          currentFileContent={activeFile?.content || 'No file open.'}
        />
      </div>

      <SettingsModal
        isOpen={state.isSettingsOpen}
        onClose={() => dispatch({ type: 'TOGGLE_SETTINGS' })}
        model={state.model}
        setModel={(model) => dispatch({ type: 'SET_MODEL', payload: model })}
      />

      <Preview
        files={state.files} // Note: This might be partial now
        isOpen={state.isPreviewOpen}
        onClose={() => dispatch({ type: 'TOGGLE_PREVIEW' })}
      />

      <CommandPalette
        isOpen={state.isCommandPaletteOpen}
        onClose={() => dispatch({ type: 'TOGGLE_COMMAND_PALETTE' })}
        commands={commands}
      />
    </div>
  );
};

export default App;
