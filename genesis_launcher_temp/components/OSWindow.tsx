
import React from 'react';

interface OSWindowProps {
  title: string;
  children: React.ReactNode;
  icon?: string;
  zIndex: number;
  x: number;
  y: number;
  onFocus: () => void;
  onMove: (x: number, y: number) => void;
  onClose: () => void;
  onMinimize: () => void;
  width?: string;
  height?: string;
}

const OSWindow: React.FC<OSWindowProps> = ({ 
  title, children, icon, zIndex, onClose, 
  width = 'w-full max-w-2xl', height = 'h-[80vh]' 
}) => {
  return (
    <div 
      className={`fixed top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 glass-panel rounded-[2rem] overflow-hidden flex flex-col transition-all duration-700 ${width} ${height}`}
      style={{ zIndex: zIndex + 100 }}
    >
      {/* Dynamic Header */}
      <div className="h-16 flex items-center justify-between px-8 border-b border-white/5 bg-white/5">
        <div className="flex items-center gap-4">
          <span className="text-cyan-400 text-xl">{icon}</span>
          <span className="mono text-[12px] font-black uppercase tracking-[0.4em] text-white/80">{title}</span>
        </div>
        <button 
          onClick={onClose}
          className="w-10 h-10 rounded-full bg-white/5 flex items-center justify-center hover:bg-white/10 transition-colors"
        >
          <div className="text-xl font-light text-white/40">×</div>
        </button>
      </div>
      
      {/* Content Area */}
      <div className="flex-1 overflow-auto bg-black/40 p-4">
        {children}
      </div>
      
      {/* App Drag Handle Simulation */}
      <div className="h-8 flex items-center justify-center pointer-events-none">
        <div className="w-12 h-1 bg-white/10 rounded-full" />
      </div>
    </div>
  );
};

export default OSWindow;
