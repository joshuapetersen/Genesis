import React, { useState } from 'react';
import { ActivityBar, Sidebar } from './components/Sidebar';
import CodeEditor from './components/Editor';
import Terminal from './components/Terminal';
import { X, ChevronRight, FileCode } from 'lucide-react';
import { cn } from './lib/utils';

const INITIAL_CODE = `/**
 * @license
 * SPDX-License-Identifier: Apache-2.0
 */

import React from 'react';

export default function App() {
  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-slate-900 text-white">
      <h1 className="text-4xl font-bold mb-4">Welcome to AI Codium</h1>
      <p className="text-slate-400">Start editing to see magic happen!</p>
    </div>
  );
}
`;

export default function App() {
  const [code, setCode] = useState(INITIAL_CODE);
  const [activeFile, setActiveFile] = useState('App.tsx');
  const [terminalOpen, setTerminalOpen] = useState(true);

  return (
    <div className="flex h-screen w-screen overflow-hidden bg-[#1e1e1e] font-sans">
      {/* Activity Bar */}
      <ActivityBar />

      {/* Sidebar */}
      <Sidebar activeFile={activeFile} onFileSelect={setActiveFile} />

      {/* Main Content Area */}
      <div className="flex-1 flex flex-col min-w-0">
        {/* Tabs Bar */}
        <div className="h-9 bg-[#252526] flex items-center border-b border-[#1e1e1e]">
          <div className="flex h-full">
            <div className="flex items-center gap-2 px-3 bg-[#1e1e1e] text-white text-xs border-r border-[#1e1e1e] h-full cursor-pointer group relative">
              <div className="absolute top-0 left-0 right-0 h-0.5 bg-[#007acc]" />
              <FileCode size={14} className="text-[#519aba]" />
              <span>{activeFile}</span>
              <X size={12} className="ml-2 opacity-0 group-hover:opacity-100 hover:bg-[#333333] rounded p-0.5" />
            </div>
          </div>
        </div>

        {/* Breadcrumbs */}
        <div className="h-6 bg-[#1e1e1e] flex items-center px-4 text-[11px] text-[#858585] gap-1">
          <span>src</span>
          <ChevronRight size={12} />
          <span className="text-[#cccccc]">{activeFile}</span>
        </div>

        {/* Editor Area */}
        <div className="flex-1 relative">
          <CodeEditor 
            value={code} 
            onChange={(val) => setCode(val || '')} 
          />
        </div>

        {/* Terminal Area */}
        {terminalOpen && (
          <div className="h-1/3 min-h-[200px]">
            <Terminal 
              currentCode={code} 
              onApplyCode={(newCode) => setCode(newCode)}
            />
          </div>
        )}

        {/* Status Bar */}
        <div className="h-6 bg-[#007acc] flex items-center justify-between px-3 text-[11px] text-white shrink-0">
          <div className="flex items-center gap-4">
            <div className="flex items-center gap-1 hover:bg-white/10 px-1 cursor-pointer">
              <span className="font-bold">main*</span>
            </div>
            <div className="flex items-center gap-1 hover:bg-white/10 px-1 cursor-pointer">
              <span>0 ⚠ 0 ✖</span>
            </div>
          </div>
          <div className="flex items-center gap-4">
            <div className="hover:bg-white/10 px-1 cursor-pointer">Ln 1, Col 1</div>
            <div className="hover:bg-white/10 px-1 cursor-pointer">Spaces: 2</div>
            <div className="hover:bg-white/10 px-1 cursor-pointer">UTF-8</div>
            <div className="hover:bg-white/10 px-1 cursor-pointer">TypeScript JSX</div>
          </div>
        </div>
      </div>
    </div>
  );
}
