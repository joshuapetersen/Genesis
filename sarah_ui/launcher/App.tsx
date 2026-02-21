import React, { useState, useEffect } from 'react';
import ResonanceCore from './components/ResonanceCore';
import Terminal from './components/Terminal';
import SettingsMenu from './components/SettingsMenu';
import LiveController from './components/LiveController';
import SovereignCommand from './components/SovereignCommand';
import SecondaryCommand from './components/SecondaryCommand';
import CreatorModule from './components/CreatorModule';
import HardwareProfile from './components/HardwareProfile';
import ModuleGrid from './components/ModuleGrid';
import LawMonitor from './components/LawMonitor';

const App: React.FC = () => {
  const [activeFeature, setActiveFeature] = useState<string | null>(null);
  const [activeApp, setActiveApp] = useState<string | null>(null);
  const [settingsOpen, setSettingsOpen] = useState(false);
  const [hasApiKey, setHasApiKey] = useState(true); // Default trusted for local seating

  const [speechVibration, setSpeechVibration] = useState(0);
  const [modulatorColor, setModulatorColor] = useState<string | null>(null);

  const [uiSettings, setUiSettings] = useState({
    primaryColor: '#00A3FF',
    secondaryColor: '#FF3B30',
    iconSize: 28,
    wheelVerticalOffset: 75, // Lower for bottom anchor
    transparency: 1.0,
  });

  const handleSelectFeature = (id: string) => {
    if (id === 'config') {
      setSettingsOpen(true);
      return;
    }
    setActiveFeature(prev => prev === id ? null : id);
  };

  const handleSelectApp = async (id: string) => {
    console.log("OS APP LAUNCH:", id);
    if (id === 'add_app') return;

    try {
      const response = await fetch("http://127.0.0.1:8001/api/system/launch", {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ app_id: id })
      });
      const data = await response.json();
      console.log("Launch Response:", data);
    } catch (err) {
      console.error("Failed to launch app via gateway:", err);
    }

    setActiveApp(prev => prev === id ? null : id);
  };

  const rootStyle = {
    '--ice-cobalt': uiSettings.primaryColor,
    '--sarah-red': uiSettings.secondaryColor,
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

      {/* PRIMARY RADIAL MENU (Bottom Left) - HIDDEN WHEN FEATURE ACTIVE */}
      {!activeFeature && (
        <div
          className="wheel-container animate-in fade-in slide-in-from-left-10 duration-500"
          style={{
            opacity: uiSettings.transparency,
            left: '20px',
            bottom: '20px',
            top: 'auto',
            transform: 'none',
            zIndex: 9999
          }}
        >
          <SovereignCommand
            activeFeature={activeFeature || ''}
            onSelect={handleSelectFeature}
            iconSize={uiSettings.iconSize}
          />
        </div>
      )}

      {/* SECONDARY RADIAL MENU (Top Left - OS APPS) - HIDDEN WHEN FEATURE ACTIVE */}
      {!activeFeature && (
        <div
          className="wheel-container animate-in fade-in slide-in-from-left-10 duration-500"
          style={{
            opacity: uiSettings.transparency,
            left: '20px',
            top: '20px',
            transform: 'none',
            zIndex: 9999
          }}
        >
          <SecondaryCommand
            activeApp={activeApp || ''}
            onSelect={handleSelectApp}
            iconSize={uiSettings.iconSize * 0.9}
          />
        </div>
      )}

      <div className="system-controls fixed bottom-10 right-10 z-[100]" style={{ opacity: uiSettings.transparency }}>
        <LiveController />
      </div>

      <main className={`flex-1 flex flex-col relative z-[50] p-6 md:p-12 transition-all duration-500 ${activeFeature ? 'ml-0' : 'ml-[100px] md:ml-[150px]'}`}>

        {/* Header HUD Element */}
        <div className="absolute top-12 right-12 flex items-center gap-12 text-right" style={{ opacity: uiSettings.transparency }}>
          <div className="flex flex-col gap-1 items-end">
            <div className="mono text-[10px] text-[var(--ice-cobalt)] tracking-[2.2em] uppercase font-black">
              SARAH_SOVEREIGN_v1.3.6
            </div>
            <div className="w-64 h-px bg-gradient-to-l from-[var(--ice-cobalt)] to-transparent opacity-30" />
            <div className="text-[8px] text-white/30 uppercase tracking-[0.5em] mt-1">
              WINDOWS_KERNEL_HOOKED
            </div>
          </div>
        </div>

        <div className="flex-1 flex items-center justify-center p-4 overflow-hidden relative">
          {activeFeature ? (
            <div
              className="w-full h-full max-w-5xl max-h-[85vh] animate-in fade-in zoom-in-95 duration-500 relative flex flex-col glass-morphism rounded-lg overflow-hidden border border-white/10 shadow-[0_0_100px_rgba(0,163,255,0.1)]"
              style={{ backgroundColor: `rgba(2, 2, 2, ${uiSettings.transparency * 0.95})` }}
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

              <div className="flex-1 w-full bg-transparent overflow-hidden relative">

                {/* PRESERVED MODULE LOGIC FROM SOURCE */}
                {activeFeature === 'chat' && (
                  <Terminal
                    opacity={uiSettings.transparency}
                    onVibrate={setSpeechVibration}
                    onSetEmotionColor={setModulatorColor}
                  />
                )}

                {activeFeature === 'vision' && <CreatorModule />}

                {activeFeature === 'maps' && (
                  <div className="w-full h-full relative">
                    <img src="https://images.unsplash.com/photo-1451187580459-43490279c0fa?auto=format&fit=crop&q=80&w=2000" className="w-full h-full object-cover grayscale brightness-[0.2]" alt="Map" />
                    <div className="absolute inset-0 bg-black/40 backdrop-blur-[2px]" />
                    <div className="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 bg-black/90 border border-[var(--ice-cobalt)] px-12 py-6 rounded shadow-2xl">
                      <span className="mono text-[14px] font-black text-[var(--ice-cobalt)] uppercase tracking-[1.5em] ml-[1.5em]">LOCKED</span>
                    </div>
                  </div>
                )}

                {/* RESTORED: ModuleGrid used for 'Store' (System Monitor) */}
                {activeFeature === 'store' && (
                  <div className="flex flex-col h-full w-full p-12 bg-[#020202]">
                    <div className="mb-8 mono text-[10px] text-white/30 uppercase tracking-[0.2em] font-bold">SYSTEM_GRID_VIEW</div>
                    <ModuleGrid />
                  </div>
                )}

                {/* RESTORED: LawMonitor used for 'Data' (Laws/Shield) */}
                {activeFeature === 'data' && (
                  <div className="flex flex-col h-full w-full p-12 bg-[#020202]">
                    <LawMonitor />
                    <div className="mt-8 border-t border-white/10 pt-8">
                      <HardwareProfile />
                    </div>
                  </div>
                )}
              </div>
            </div>
          ) : (
            <div className="flex flex-col items-center gap-16 animate-in fade-in duration-1000">
              <div className="relative">
                <div className="w-12 h-12 rounded-full bg-[var(--ice-cobalt)] shadow-[0_0_100px_var(--ice-cobalt)] animate-pulse" />
                <div className="absolute inset-[-40px] border border-[var(--ice-cobalt)] opacity-10 rounded-full animate-ping" />
              </div>
              <span className="mono text-[22px] text-white/20 uppercase tracking-[2.5em] font-black ml-[2.5em]">SARAH_CORE</span>
              <div className="fixed bottom-32 text-[10px] text-white/10 uppercase tracking-widest flex flex-col items-center gap-2">
                <span>WAITING_FOR_DIRECTIVE</span>
                <div className="w-1 h-8 bg-white/5" />
              </div>
            </div>
          )}
        </div>

        <footer className="h-16 flex items-center px-20 justify-between border-t border-white/5 opacity-50 relative bg-black/20 backdrop-blur-sm">
          <div className="flex items-center gap-4">
            <div className="w-2 h-2 rounded-full bg-[var(--ice-cobalt)] animate-pulse" />
            <div className="flex flex-col">
              <span className="mono text-[10px] uppercase font-black text-[var(--ice-cobalt)] tracking-[0.5em]">SOVEREIGN_SYSTEM_READY</span>
              <span className="mono text-[7px] uppercase font-black text-white/20 tracking-[0.2em]">ALL_SYSTEMS_NOMINAL</span>
            </div>
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
