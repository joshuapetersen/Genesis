import { useOSStore } from '../../store/osStore';
import { apps } from '../../config/apps';
import { Menu, Wifi, Battery, Volume2, Mic, MicOff, BrainCircuit } from 'lucide-react';

export const Taskbar: React.FC = () => {
  const [time, setTime] = useState(new Date());
  const { windows, activeWindowId, focusWindow, toggleMinimize, openApp, systemConfig, updateConfig } = useOSStore();
  const [startMenuOpen, setStartMenuOpen] = useState(false);

  useEffect(() => {
    const timer = setInterval(() => setTime(new Date()), 1000);
    return () => clearInterval(timer);
  }, []);

  const handleWindowClick = (id: string, isMinimized: boolean) => {
    if (isMinimized) {
      toggleMinimize(id);
      focusWindow(id);
    } else if (activeWindowId === id) {
      toggleMinimize(id);
    } else {
      focusWindow(id);
    }
  };

  return (
    <div className="absolute bottom-0 left-0 right-0 h-12 bg-white/70 dark:bg-gray-900/70 backdrop-blur-2xl border-t border-white/20 dark:border-gray-700/50 flex items-center justify-between px-2 z-50">
      
      {/* Start Button & Pinned Apps */}
      <div className="flex items-center gap-1 h-full">
        <button 
          onClick={() => setStartMenuOpen(!startMenuOpen)}
          className={`h-10 w-10 flex items-center justify-center rounded-xl transition-all ${startMenuOpen ? 'bg-blue-500/20 text-blue-600 dark:text-blue-400' : 'hover:bg-white/50 dark:hover:bg-gray-800/50 text-gray-800 dark:text-gray-200'}`}
        >
          <Menu size={20} />
        </button>

        <div className="w-px h-6 bg-gray-300 dark:bg-gray-700 mx-1" />

        {/* Open Windows */}
        {windows.map((w) => {
          const app = apps[w.appId];
          const isActive = activeWindowId === w.id && !w.isMinimized;
          const AppIcon = app?.icon;

          return (
            <button
              key={w.id}
              onClick={() => handleWindowClick(w.id, w.isMinimized)}
              className={`h-10 px-3 flex items-center gap-2 rounded-xl transition-all max-w-[160px] ${
                isActive 
                  ? 'bg-white/80 dark:bg-gray-800 shadow-sm border border-gray-200/50 dark:border-gray-700/50' 
                  : 'hover:bg-white/50 dark:hover:bg-gray-800/50 hover:shadow-sm'
              }`}
            >
              {AppIcon && <AppIcon size={16} className={isActive ? 'text-blue-600 dark:text-blue-400' : 'text-gray-600 dark:text-gray-400'} />}
              <span className={`text-sm truncate ${isActive ? 'font-medium text-gray-900 dark:text-gray-100' : 'text-gray-700 dark:text-gray-300'}`}>
                {w.title}
              </span>
            </button>
          );
        })}
      </div>

      {/* System Tray */}
      <div className="flex items-center gap-1 h-full pr-2 text-gray-700 dark:text-gray-300">
        <button
          onClick={() => updateConfig({ snapToGrid: !systemConfig.snapToGrid })}
          title={`Snap to Grid: ${systemConfig.snapToGrid ? 'ON' : 'OFF'}`}
          className={`px-3 h-10 rounded-xl transition-colors flex items-center gap-2 ${
            systemConfig.snapToGrid 
              ? 'bg-blue-500/20 text-blue-600 dark:text-blue-400' 
              : 'hover:bg-white/50 dark:hover:bg-gray-800/50'
          }`}
        >
          <div className={`w-2 h-2 rounded-full ${systemConfig.snapToGrid ? 'bg-blue-500 animate-pulse' : 'bg-gray-400'}`} />
          <span className="text-[10px] uppercase font-bold tracking-wider">Grid</span>
        </button>

        <button
          onClick={() => updateConfig({ voiceEnabled: !systemConfig.voiceEnabled })}
          title={`Sovereign Voice: ${systemConfig.voiceEnabled ? 'ON' : 'OFF'}`}
          className={`px-3 h-10 rounded-xl transition-all flex items-center gap-2 ${
            systemConfig.voiceEnabled 
              ? 'bg-purple-500/20 text-purple-600 dark:text-purple-400' 
              : 'hover:bg-white/50 dark:hover:bg-gray-800/50 opacity-60'
          }`}
        >
          {systemConfig.voiceEnabled ? <Mic size={16} className="animate-pulse" /> : <MicOff size={16} />}
          <span className="text-[10px] uppercase font-bold tracking-wider">Voice</span>
        </button>

        <button
          onClick={() => updateConfig({ showThoughts: !systemConfig.showThoughts })}
          title={`Reasoning Engine: ${systemConfig.showThoughts ? 'ON' : 'OFF'}`}
          className={`px-3 h-10 rounded-xl transition-all flex items-center gap-2 ${
            systemConfig.showThoughts 
              ? 'bg-pink-500/20 text-pink-600 dark:text-pink-400' 
              : 'hover:bg-white/50 dark:hover:bg-gray-800/50 opacity-60'
          }`}
        >
          <BrainCircuit size={16} className={systemConfig.showThoughts ? 'animate-bounce-subtle' : ''} />
          <span className="text-[10px] uppercase font-bold tracking-wider">Brain</span>
        </button>

        <div className="flex items-center gap-3 px-3 h-10 rounded-xl hover:bg-white/50 dark:hover:bg-gray-800/50 transition-colors cursor-default">
          <Wifi size={16} />
          <Volume2 size={16} />
          <Battery size={16} />
        </div>
        <div className="flex flex-col items-end justify-center px-3 h-10 rounded-xl hover:bg-white/50 dark:hover:bg-gray-800/50 transition-colors cursor-default">
          <span className="text-xs font-medium">{format(time, 'h:mm a')}</span>
          <span className="text-[10px] opacity-80">{format(time, 'M/d/yyyy')}</span>
        </div>
      </div>

      {/* Start Menu Popup */}
      {startMenuOpen && (
        <div className="absolute bottom-14 left-2 w-80 bg-white/90 dark:bg-gray-900/90 backdrop-blur-2xl border border-white/20 dark:border-gray-700/50 rounded-2xl shadow-2xl p-4 flex flex-col gap-4 animate-in fade-in slide-in-from-bottom-4 duration-200">
          <div className="relative">
            <input 
              type="text" 
              placeholder="Search apps..." 
              className="w-full bg-gray-100 dark:bg-gray-800 border-none rounded-xl px-4 py-2 text-sm focus:ring-2 focus:ring-blue-500 outline-none"
            />
          </div>
          <div className="grid grid-cols-4 gap-2">
            {Object.values(apps).map((app) => {
              const AppIcon = app.icon;
              return (
                <button
                  key={app.id}
                  onClick={() => {
                    openApp(app.id, app.title, app.defaultWidth, app.defaultHeight);
                    setStartMenuOpen(false);
                  }}
                  className="flex flex-col items-center gap-2 p-3 rounded-xl hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors"
                >
                  <div className="w-10 h-10 rounded-full bg-blue-100 dark:bg-blue-900/30 flex items-center justify-center text-blue-600 dark:text-blue-400">
                    <AppIcon size={20} />
                  </div>
                  <span className="text-xs font-medium text-gray-700 dark:text-gray-300 text-center line-clamp-1">{app.title}</span>
                </button>
              );
            })}
          </div>
        </div>
      )}
    </div>
  );
};
