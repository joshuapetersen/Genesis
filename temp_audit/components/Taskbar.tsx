
import React from 'react';

interface TaskbarProps {
  windows: Record<string, any>;
  onToggle: (id: string) => void;
}

const Taskbar: React.FC<TaskbarProps> = ({ windows, onToggle }) => {
  return (
    <div className="h-14 z-[9999] flex items-center justify-center pb-2 px-6">
      <div className="glass h-full px-4 flex items-center gap-3 rounded-2xl border border-cyan-500/20 shadow-[0_10px_30px_rgba(0,0,0,0.5)] bg-slate-950/40">
        
        {/* Start Button */}
        <div className="w-9 h-9 rounded-xl bg-cyan-600 flex items-center justify-center cursor-pointer hover:bg-cyan-500 shadow-lg shadow-cyan-500/20 transition-all active:scale-90 group">
          <div className="w-4 h-4 border-2 border-white rounded-sm group-hover:rotate-45 transition-transform" />
        </div>
        
        <div className="w-px h-6 bg-white/10 mx-1" />

        {/* App Slots */}
        {Object.values(windows).map((win: any) => (
          <button 
            key={win.id}
            onClick={() => onToggle(win.id)}
            className={`
              h-9 px-3 rounded-lg flex items-center gap-2 border transition-all relative
              ${win.isOpen && !win.isMinimized 
                ? 'bg-cyan-500/10 border-cyan-500/40 text-cyan-100 shadow-[inset_0_0_10px_rgba(6,182,212,0.1)]' 
                : 'bg-slate-900/40 border-white/5 text-slate-500 hover:border-white/20 hover:bg-slate-800/40'}
            `}
          >
            <span className="text-sm">{win.icon}</span>
            <span className="mono text-[9px] font-black uppercase tracking-widest hidden md:block">
              {win.id}
            </span>
            {win.isOpen && (
              <div className={`absolute -bottom-1.5 left-1/2 -translate-x-1/2 w-4 h-0.5 rounded-full ${win.isMinimized ? 'bg-slate-600' : 'bg-cyan-500 shadow-[0_0_5px_cyan]'}`} />
            )}
          </button>
        ))}

        <div className="w-px h-6 bg-white/10 mx-1" />
        
        {/* System Tray */}
        <div className="flex items-center gap-3 ml-2 pr-2">
          <div className="flex flex-col items-end mono text-[8px] text-slate-500 leading-tight">
             <span className="text-green-500 font-bold">ONLINE</span>
             <span>CPU: 4%</span>
          </div>
          <div className="w-8 h-8 rounded-full border border-cyan-500/20 flex items-center justify-center animate-pulse">
             <div className="w-2 h-2 bg-cyan-500 rounded-full" />
          </div>
        </div>
      </div>
    </div>
  );
};

export default Taskbar;
