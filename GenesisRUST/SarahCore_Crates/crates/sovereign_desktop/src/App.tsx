import React, { useEffect } from 'react';
import { Desktop } from './components/os/Desktop';
import { Taskbar } from './components/os/Taskbar';
import { Window } from './components/os/Window';
import { useOSStore } from './store/osStore';

export default function App() {
  const { windows, systemConfig } = useOSStore();

  // Prevent default right click
  useEffect(() => {
    const handleContextMenu = (e: MouseEvent) => e.preventDefault();
    document.addEventListener('contextmenu', handleContextMenu);
    return () => document.removeEventListener('contextmenu', handleContextMenu);
  }, []);
  
  return (
    <div 
      className="relative w-screen h-screen overflow-hidden bg-cover bg-center select-none"
      style={{ 
        backgroundImage: `url("${systemConfig.wallpaper}")`,
        backgroundColor: '#1a1a2e',
        fontFamily: systemConfig.fontFamily
      }}
    >
      {/* Desktop Icons */}
      <Desktop />

      {/* Windows Layer */}
      <div className="absolute inset-0 pointer-events-none z-10">
        {windows.map((window) => (
          <Window key={window.id} id={window.id} />
        ))}
      </div>

      {/* Taskbar */}
      <Taskbar />
    </div>
  );
}

