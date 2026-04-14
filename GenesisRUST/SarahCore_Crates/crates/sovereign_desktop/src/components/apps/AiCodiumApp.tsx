import React, { useState } from 'react';
import { ActivityBar, Sidebar } from '../../ai-codium/components/Sidebar';
import CodeEditor from '../../ai-codium/components/Editor';
import Terminal from '../../ai-codium/components/Terminal';
import { X, ChevronRight, FileCode } from 'lucide-react';

export default function AiCodiumApp() {
  const [code, setCode] = useState('// Sovereign SarahCore Code Substrate');
  const [activeFile, setActiveFile] = useState('SarahCore.rs');

  return (
    <div className="flex h-full w-full overflow-hidden bg-[#1e1e1e] font-sans text-[#cccccc]">
      <ActivityBar />
      <Sidebar activeFile={activeFile} onFileSelect={setActiveFile} />
      <div className="flex-1 flex flex-col min-w-0">
        <div className="h-9 bg-[#252526] flex items-center px-4 border-b border-[#1e1e1e] text-xs">
           <FileCode size={14} className="mr-2 text-blue-400" />
           {activeFile}
        </div>
        <div className="flex-1 relative">
          <CodeEditor value={code} onChange={(v) => setCode(v || '')} />
        </div>
      </div>
    </div>
  );
}
