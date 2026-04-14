import React from 'react';
import { 
  Files, 
  Search, 
  GitBranch, 
  Play, 
  Blocks, 
  Settings, 
  UserCircle,
  ChevronRight,
  FileCode,
  Folder
} from 'lucide-react';
import { cn } from '@/src/lib/utils';

interface SidebarProps {
  activeFile: string;
  onFileSelect: (fileName: string) => void;
}

export function ActivityBar() {
  const items = [
    { icon: Files, active: true },
    { icon: Search },
    { icon: GitBranch },
    { icon: Play },
    { icon: Blocks },
  ];

  return (
    <div className="w-12 bg-[#333333] flex flex-col items-center py-4 gap-4 shrink-0">
      {items.map((item, i) => (
        <button
          key={i}
          className={cn(
            "p-2 transition-colors relative group",
            item.active ? "text-white" : "text-[#858585] hover:text-white"
          )}
        >
          <item.icon size={24} strokeWidth={1.5} />
          {item.active && (
            <div className="absolute left-0 top-0 bottom-0 w-0.5 bg-white" />
          )}
        </button>
      ))}
      <div className="mt-auto flex flex-col gap-4">
        <button className="p-2 text-[#858585] hover:text-white transition-colors">
          <UserCircle size={24} strokeWidth={1.5} />
        </button>
        <button className="p-2 text-[#858585] hover:text-white transition-colors">
          <Settings size={24} strokeWidth={1.5} />
        </button>
      </div>
    </div>
  );
}

export function Sidebar({ activeFile, onFileSelect }: SidebarProps) {
  const files = [
    { name: 'App.tsx', icon: FileCode },
    { name: 'Terminal.tsx', icon: FileCode },
    { name: 'Editor.tsx', icon: FileCode },
    { name: 'index.css', icon: FileCode },
  ];

  return (
    <div className="w-64 bg-[#252526] flex flex-col border-r border-[#333333] shrink-0">
      <div className="px-4 py-3 text-[11px] font-bold uppercase tracking-wider text-[#bbbbbb] flex justify-between items-center">
        <span>Explorer</span>
        <button className="hover:bg-[#333333] p-1 rounded transition-colors">
          <ChevronRight size={14} className="rotate-90" />
        </button>
      </div>
      
      <div className="flex-1 overflow-y-auto">
        <div className="flex items-center gap-1 px-4 py-1 text-[#cccccc] font-bold text-xs hover:bg-[#2a2d2e] cursor-pointer">
          <ChevronRight size={14} className="rotate-90" />
          <Folder size={14} className="text-[#007acc]" />
          <span>SRC</span>
        </div>
        
        <div className="pl-4">
          {files.map((file) => (
            <div
              key={file.name}
              onClick={() => onFileSelect(file.name)}
              className={cn(
                "flex items-center gap-2 px-4 py-1 text-xs cursor-pointer transition-colors",
                activeFile === file.name 
                  ? "bg-[#37373d] text-white" 
                  : "text-[#cccccc] hover:bg-[#2a2d2e]"
              )}
            >
              <file.icon size={14} className={cn(
                file.name.endsWith('.tsx') ? "text-[#519aba]" : "text-[#ce9178]"
              )} />
              <span>{file.name}</span>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}
