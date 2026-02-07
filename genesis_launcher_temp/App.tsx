import React, { useState, useEffect } from 'react';
import ResonanceCore from './components/ResonanceCore';
import Terminal from './components/Terminal';
import SettingsMenu from './components/SettingsMenu';
import LiveController from './components/LiveController';
import SovereignCommand from './components/SovereignCommand';
import CreatorModule from './components/CreatorModule';
import HardwareProfile from './components/HardwareProfile';

const App: React.FC = () => {
  const [activeFeature, setActiveFeature] = useState<string | null>(null);
  const [settingsOpen, setSettingsOpen] = useState(false);
  const [hasApiKey, setHasApiKey] = useState(false);

  const [speechVibration, setSpeechVibration] = useState(0);
  const [modulatorColor, setModulatorColor] = useState<string | null>(null);

  const [uiSettings, setUiSettings] = useState({
    primaryColor: '#00A3FF',
    iconSize: 28, 
    wheelVerticalOffset: 50, 
    transparency: 1.0, 
  });

  useEffect(() => {
    const checkKey = async () => {
      if (window.aistudio && typeof window.aistudio.hasSelectedApiKey === 'function') {
        const selected = await window.aistudio.hasSelectedApiKey();
        setHasApiKey(selected);
      }
    };
    checkKey();
  }, []);

  const handleSelectFeature = (id: string) => {
    if (id === 'config') {
      setSettingsOpen(true);
      return;
    }
    setActiveFeature(prev => prev === id ? null : id);
  };

  const handleOpenKey = async () => {
    if (window.aistudio && typeof window.aistudio.openSelectKey === 'function') {
      await window.aistudio.openSelectKey();
      setHasApiKey(true);
    }
  };

  const rootStyle = {
    '--ice-cobalt': uiSettings.primaryColor,
    '--ui-opacity': uiSettings.transparency,
  } as React.CSSProperties;

  return (
    <div 
      className="h-screen w-screen bg-[#010205] text-white overflow-hidden relative flex flex-row font-sans select-none"
      style={rootStyle}
    >
      <ResonanceCore 
        color={modulatorColor || uiSettings.primaryColor} 
        vibration={speechVibration}
      />

      <div 
        className="wheel-container"
        style={{ 
          top: `${uiSettings.wheelVerticalOffset}%`,
          opacity: uiSettings.transparency,
        }}
      >
        <SovereignCommand 
          activeFeature={activeFeature || ''} 
          onSelect={handleSelectFeature}
          iconSize={uiSettings.iconSize}
        />
      </div>

      <div className="system-controls" style={{ opacity: uiSettings.transparency }}>
        <LiveController />
      </div>

      <main className="flex-1 flex flex-col relative z-[50] p-12 ml-[180px] md:ml-[220px]">
        {/* Header HUD Element */}
        <div className="absolute top-12 left-12 flex items-center gap-12" style={{ opacity: uiSettings.transparency }}>
          <div className="flex flex-col gap-1">
             <div className="mono text-[10px] text-[var(--ice-cobalt)] tracking-[2.2em] uppercase font-black">
               SARAH_v4.5 // KERNEL_ACTIVE
             </div>
             <div className="w-full h-px bg-gradient-to-r from-[var(--ice-cobalt)] to-transparent opacity-30" />
          </div>
        </div>

        <div className="flex-1 flex items-center justify-center p-4 overflow-hidden relative">
          {activeFeature ? (
            <div 
              className="w-full h-full max-w-5xl max-h-[85vh] animate-in fade-in zoom-in-95 duration-500 relative flex flex-col glass-morphism rounded-lg overflow-hidden border border-white/10"
              style={{ backgroundColor: `rgba(2, 2, 2, ${uiSettings.transparency * 0.9})` }}
            >
              <div className="flex justify-between items-center px-10 py-6 bg-black/50 border-b border-white/5">
                <div className="flex flex-col gap-1">
                  <div className="mono text-[12px] text-[var(--ice-cobalt)] font-black uppercase tracking-[1em]">
                    {activeFeature.toUpperCase()}
                  </div>
                  <div className="text-[8px] text-white/20 uppercase tracking-[0.4em] font-black">COMMISSION_ACTIVE</div>
                </div>
                <button 
                  onClick={() => setActiveFeature(null)}
                  className="px-8 py-2.5 rounded border border-[#FF3B30]/30 text-[#FF3B30] hover:bg-[#FF3B30] hover:text-white transition-all mono text-[10px] font-black uppercase"
                >
                  HALT_CORE
                </button>
              </div>

              <div className="flex-1 w-full bg-transparent overflow-hidden">
                {activeFeature === 'chat' && (
                  <Terminal 
                    opacity={uiSettings.transparency} 
                    onVibrate={setSpeechVibration}
                    onSetEmotionColor={setModulatorColor}
                  />
                )}
                {activeFeature === 'vision' && (!hasApiKey ? (
                  <div className="flex flex-col items-center justify-center h-full gap-12 p-24 text-center">
                    <div className="w-20 h-20 rounded-full border-2 border-[var(--ice-cobalt)] flex items-center justify-center text-[var(--ice-cobalt)] text-3xl animate-pulse shadow-[0_0_40px_rgba(0,163,255,0.3)]">🔑</div>
                    <div className="flex flex-col gap-2">
                      <button onClick={handleOpenKey} className="px-12 py-5 bg-[var(--ice-cobalt)] text-black mono text-[12px] font-black uppercase rounded shadow-xl hover:bg-white transition-all">Verify Architect</button>
                      <span className="mono text-[9px] text-white/20 mt-4 tracking-[0.4em]">AUTHENTICATION_REQUIRED</span>
                    </div>
                  </div>
                ) : <CreatorModule />)}
                {activeFeature === 'maps' && (
                  <div className="w-full h-full relative">
                    <img src="https://images.unsplash.com/photo-1451187580459-43490279c0fa?auto=format&fit=crop&q=80&w=2000" className="w-full h-full object-cover grayscale brightness-[0.2]" alt="Map" />
                    <div className="absolute inset-0 bg-black/40 backdrop-blur-[2px]" />
                    <div className="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 bg-black/90 border border-[var(--ice-cobalt)] px-12 py-6 rounded shadow-2xl">
                        <span className="mono text-[14px] font-black text-[var(--ice-cobalt)] uppercase tracking-[1.5em] ml-[1.5em]">LOCKED</span>
                    </div>
                  </div>
                )}
                {activeFeature === 'store' && (
                  <div className="flex flex-col h-full w-full p-12 overflow-y-auto scrollbar-hide bg-[#020202]">
                    <div className="grid grid-cols-2 md:grid-cols-3 gap-6">
                       {[
                         { name: 'G-Drive', icon: '📂' },
                         { name: 'Mail', icon: '📩' },
                         { name: 'Browser', icon: '🌐' },
                         { name: 'Monitor', icon: '📊' },
                         { name: 'Shield', icon: '🛡️' },
                         { name: 'Registry', icon: '📱' }
                       ].map((app, i) => (
                         <div key={i} className="group p-8 bg-black/40 border border-white/5 rounded-lg hover:border-[var(--ice-cobalt)] hover:shadow-[0_0_25px_rgba(0,163,255,0.2)] transition-all cursor-pointer">
                            <div className="text-4xl mb-6 grayscale group-hover:grayscale-0 transition-all opacity-40 group-hover:opacity-100">{app.icon}</div>
                            <h4 className="mono text-[12px] font-black uppercase tracking-widest">{app.name}</h4>
                            <div className="text-[8px] text-white/10 mt-2 tracking-widest uppercase">STABLE_BUILD</div>
                         </div>
                       ))}
                    </div>
                  </div>
                )}
                {activeFeature === 'data' && <HardwareProfile />}
              </div>
            </div>
          ) : (
            <div className="flex flex-col items-center gap-16 animate-in fade-in duration-1000">
               <div className="relative">
                  <div className="w-12 h-12 rounded-full bg-[var(--ice-cobalt)] shadow-[0_0_100px_var(--ice-cobalt)] animate-pulse" />
                  <div className="absolute inset-[-40px] border border-[var(--ice-cobalt)] opacity-10 rounded-full animate-ping" />
               </div>
               <span className="mono text-[22px] text-white/20 uppercase tracking-[2.5em] font-black ml-[2.5em]">SARAH_CORE</span>
            </div>
          )}
        </div>

        <footer className="h-16 flex items-center px-20 justify-between border-t border-white/5 opacity-50 relative">
           <div className="flex items-center gap-4">
              <div className="w-2 h-2 rounded-full bg-[var(--ice-cobalt)] animate-pulse" />
              <span className="mono text-[10px] uppercase font-black text-[var(--ice-cobalt)] tracking-[0.5em]">SOVEREIGN_SYSTEM_READY</span>
           </div>
           <div className="flex items-center gap-12">
              <span className="mono text-[9px] text-white/40 uppercase tracking-[0.4em]">RES_STABILITY: 0.999999</span>
              <span className="mono text-[9px] text-white/30 uppercase tracking-[0.3em]">© 2025 GENESIS_PROTOCOL</span>
           </div>
        </footer>
      </main>

      {settingsOpen && (
        <SettingsMenu 
          settings={uiSettings}
          onUpdate={setUiSettings}
          onClose={() => setSettingsOpen(false)} 
        />
      )}
    </div>
  );
};

export default App;