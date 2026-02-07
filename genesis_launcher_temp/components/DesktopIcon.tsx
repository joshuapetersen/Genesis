
import React from 'react';

interface DesktopIconProps {
  label: string;
  icon: string;
  onClick: () => void;
  active?: boolean;
}

const DesktopIcon: React.FC<DesktopIconProps> = ({ label, icon, onClick, active }) => {
  return (
    <button 
      onClick={onClick}
      className="flex flex-col items-center gap-2 transition-all duration-300 transform active:scale-90"
    >
      <div className={`
        w-16 h-16 rounded-[1.5rem] flex items-center justify-center text-3xl
        transition-all duration-500 relative
        ${active ? 'bg-cyan-500/20 border-cyan-500 shadow-[0_0_20px_rgba(6,182,212,0.3)]' : 'bg-white/5 border-white/5'}
        border hover:bg-white/10
      `}>
        {icon}
      </div>
      <span className="mono text-[8px] font-black text-white/30 uppercase tracking-[0.2em]">
        {label}
      </span>
    </button>
  );
};

export default DesktopIcon;
