import React from 'react';

interface SettingsState {
  primaryColor: string;
  iconSize: number;
  wheelVerticalOffset: number;
  transparency: number;
}

interface SettingsMenuProps {
  settings: SettingsState;
  onUpdate: (settings: SettingsState) => void;
  onClose: () => void;
}

const SettingsMenu: React.FC<SettingsMenuProps> = ({ settings, onUpdate, onClose }) => {
  const update = (key: keyof SettingsState, value: any) => {
    onUpdate({ ...settings, [key]: value });
  };

  const presets = [
    { name: 'ICE_COBALT', color: '#00A3FF' },
    { name: 'EMERALD', color: '#10B981' },
    { name: 'CRIMSON', color: '#EF4444' },
    { name: 'AMBER', color: '#F59E0B' },
    { name: 'VIOLET', color: '#8B5CF6' }
  ];

  return (
    <div className="fixed inset-0 z-[5000] flex items-center justify-end p-4 md:p-12 animate-in fade-in slide-in-from-right duration-500">
      <div className="absolute inset-0 bg-black/80" onClick={onClose} />
      
      <div className="w-full max-w-sm h-full bg-[#050505] rounded-[3rem] overflow-hidden flex flex-col relative z-10 border-4 border-[#1a1a1a] shadow-[0_0_100px_rgba(0,0,0,1)]">
        {/* Header */}
        <div className="h-24 bg-[#111] flex items-center px-10 justify-between border-b-4 border-[#1a1a1a]">
          <div className="flex flex-col">
            <span className="mono text-[8px] font-black text-[var(--ice-cobalt)] uppercase tracking-[0.5em]">Command</span>
            <span className="text-xl font-black text-white uppercase tracking-tighter italic">CONTROL_PANEL</span>
          </div>
          <button onClick={onClose} className="w-10 h-10 rounded-full border-2 border-[#333] flex items-center justify-center text-white hover:border-[var(--ice-cobalt)] transition-all text-xl">×</button>
        </div>

        {/* Body */}
        <div className="flex-1 p-8 space-y-8 overflow-auto scrollbar-hide bg-black">
          
          <section>
            <h4 className="mono text-[10px] text-[var(--ice-cobalt)] uppercase tracking-[0.4em] mb-4 font-black">RESONANCE_COLOR</h4>
            <div className="grid grid-cols-5 gap-2">
              {presets.map((p) => (
                <button
                  key={p.name}
                  onClick={() => update('primaryColor', p.color)}
                  className={`w-full aspect-square rounded-full border-2 transition-all ${
                    settings.primaryColor === p.color ? 'border-white scale-110' : 'border-transparent'
                  }`}
                  style={{ backgroundColor: p.color }}
                />
              ))}
            </div>
            <input 
              type="color" 
              value={settings.primaryColor}
              onChange={(e) => update('primaryColor', e.target.value)}
              className="w-full h-8 bg-black border-2 border-[#1a1a1a] cursor-pointer mt-4"
            />
          </section>

          <section>
            <div className="flex justify-between items-center mb-2">
              <h4 className="mono text-[10px] text-[var(--ice-cobalt)] uppercase tracking-[0.4em] font-black">ICON_SCALE</h4>
              <span className="mono text-[9px] text-white font-black">{settings.iconSize}px</span>
            </div>
            <input 
              type="range" 
              min="32" 
              max="64" 
              value={settings.iconSize}
              onChange={(e) => update('iconSize', parseInt(e.target.value))}
              className="w-full h-1.5 bg-[#1a1a1a] rounded-full appearance-none cursor-pointer"
            />
          </section>

          <section>
            <div className="flex justify-between items-center mb-2">
              <h4 className="mono text-[10px] text-[var(--ice-cobalt)] uppercase tracking-[0.4em] font-black">WHEEL_Y</h4>
              <span className="mono text-[9px] text-white font-black">{settings.wheelVerticalOffset}%</span>
            </div>
            <input 
              type="range" 
              min="10" 
              max="90" 
              value={settings.wheelVerticalOffset}
              onChange={(e) => update('wheelVerticalOffset', parseInt(e.target.value))}
              className="w-full h-1.5 bg-[#1a1a1a] rounded-full appearance-none cursor-pointer"
            />
          </section>

          <section>
            <div className="flex justify-between items-center mb-2">
              <h4 className="mono text-[10px] text-[var(--ice-cobalt)] uppercase tracking-[0.4em] font-black">OPTICAL_DENSITY</h4>
              <span className="mono text-[9px] text-white font-black">{(settings.transparency * 100).toFixed(0)}%</span>
            </div>
            <input 
              type="range" 
              min="0.1" 
              max="1.0" 
              step="0.1"
              value={settings.transparency}
              onChange={(e) => update('transparency', parseFloat(e.target.value))}
              className="w-full h-1.5 bg-[#1a1a1a] rounded-full appearance-none cursor-pointer"
            />
          </section>
        </div>

        <div className="p-8 bg-[#111] border-t-2 border-[#1a1a1a]">
           <button 
             onClick={onClose}
             className="w-full py-4 bg-[var(--ice-cobalt)] rounded-xl mono text-[11px] font-black text-black uppercase tracking-[0.2em] transition-all"
           >
             COMMIT_LATTICE
           </button>
        </div>
      </div>
    </div>
  );
};

export default SettingsMenu;