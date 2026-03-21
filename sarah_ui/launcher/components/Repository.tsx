
import React, { useState } from 'react';
import { Download, Check, Search, Package, ExternalLink, Shield, Zap, Eye, Cpu, Box, Loader2, Play } from 'lucide-react';

interface AppItem {
  id: string;
  name: string;
  category: string;
  size: string;
  installed: boolean;
  description: string;
  icon: any;
}

const MOCK_APPS: AppItem[] = [
  { id: '1', name: 'Night Vision+', category: 'OPTICS', size: '12MB', installed: false, description: 'Enhanced low-light sensor fusion.', icon: Eye },
  { id: '2', name: 'Tactical Maps', category: 'NAV', size: '45MB', installed: true, description: 'Offline terrain data with elevation.', icon: Box },
  { id: '3', name: 'Bio-Monitor', category: 'HEALTH', size: '8MB', installed: false, description: 'Advanced HRV tracking.', icon: Shield },
  { id: '4', name: 'Neural Overclock', category: 'SYSTEM', size: '2MB', installed: false, description: 'Unlocks CPU voltage limits.', icon: Zap },
  { id: '5', name: 'Spectre Cam', category: 'OPTICS', size: '24MB', installed: false, description: 'Thermal spectrum emulation.', icon: Eye },
  { id: '6', name: 'Genesis Connect', category: 'NET', size: '15MB', installed: false, description: 'External device bridge.', icon: Cpu },
];

const Repository: React.FC = () => {
  const [searchTerm, setSearchTerm] = useState('');
  const [apps, setApps] = useState(MOCK_APPS);
  const [installing, setInstalling] = useState<string | null>(null);

  const handleInstall = (id: string) => {
    setInstalling(id);
    setTimeout(() => {
      setApps(prev => prev.map(a => a.id === id ? { ...a, installed: true } : a));
      setInstalling(null);
    }, 1500);
  };

  const filteredApps = apps.filter(a => a.name.toLowerCase().includes(searchTerm.toLowerCase()));

  return (
    <div className="flex flex-col h-[500px] w-[500px] text-sky-400 font-mono select-none">
      {/* Header */}
      <div className="flex justify-between items-end mb-6 border-b border-sky-500/20 pb-4">
        <div>
          <h2 className="text-xl font-black uppercase tracking-tighter italic flex items-center gap-2">
            <Package size={20} className="text-sky-400" /> Repository
          </h2>
          <span className="text-[8px] uppercase tracking-[0.3em] opacity-50">Secure_Extensions</span>
        </div>
        
        {/* Play Store Link for External Access */}
        <button 
            onClick={() => window.open('https://play.google.com/store', '_blank')}
            className="flex items-center gap-2 px-3 py-1 bg-sky-500/10 hover:bg-sky-500/20 border border-sky-500/30 rounded-full transition-all group"
        >
          <Play size={10} className="group-hover:text-white fill-current" />
          <span className="text-[7px] font-bold uppercase tracking-widest group-hover:text-white">Play_Store</span>
        </button>
      </div>

      {/* Search Bar */}
      <div className="relative mb-4">
        <Search className="absolute left-3 top-1/2 -translate-y-1/2 text-sky-500/50" size={14} />
        <input 
          type="text" 
          placeholder="SEARCH_MODULES..." 
          value={searchTerm}
          onChange={(e) => setSearchTerm(e.target.value)}
          className="w-full bg-black/40 border border-sky-500/20 rounded-lg py-3 pl-10 text-[10px] font-bold text-white uppercase tracking-widest outline-none focus:border-sky-500/50 transition-colors placeholder:text-sky-500/30"
        />
      </div>

      {/* App List */}
      <div className="flex-1 overflow-y-auto pr-2 custom-scrollbar flex flex-col gap-2">
        {filteredApps.map(app => (
          <div key={app.id} className="group flex items-center justify-between p-3 bg-white/5 border border-white/5 hover:border-sky-500/30 hover:bg-white/10 transition-all rounded-lg">
            <div className="flex items-center gap-4">
              <div className="w-10 h-10 bg-black/50 rounded-lg flex items-center justify-center border border-white/10 group-hover:border-sky-500/30 transition-colors">
                <app.icon size={18} className="text-sky-400" />
              </div>
              <div className="flex flex-col">
                <div className="flex items-center gap-2">
                  <span className="text-[11px] font-bold text-white uppercase">{app.name}</span>
                  <span className="text-[7px] font-black bg-sky-500/20 px-1.5 rounded text-sky-300">{app.category}</span>
                </div>
                <span className="text-[9px] text-white/40">{app.description}</span>
              </div>
            </div>
            
            <button 
              onClick={() => !app.installed && !installing && handleInstall(app.id)}
              disabled={app.installed || installing === app.id}
              className={`px-4 py-2 rounded-md border text-[9px] font-black uppercase tracking-widest transition-all min-w-[90px] flex items-center justify-center
                ${app.installed 
                  ? 'border-green-500/30 text-green-400 bg-green-500/5' 
                  : installing === app.id
                    ? 'border-sky-500/30 text-sky-400 bg-sky-500/10'
                    : 'border-white/10 text-white hover:bg-sky-500/20 hover:border-sky-500/50'
                }`}
            >
              {app.installed ? (
                <span className="flex items-center gap-1"><Check size={10} /> Active</span>
              ) : installing === app.id ? (
                <Loader2 size={12} className="animate-spin" />
              ) : (
                <span className="flex items-center gap-1"><Download size={10} /> Get</span>
              )}
            </button>
          </div>
        ))}
      </div>
      
      <div className="mt-4 pt-2 border-t border-sky-500/10 flex justify-between items-center opacity-40">
        <span className="text-[6px] uppercase tracking-widest">Genesis_Repository_v1.0</span>
        <span className="text-[6px] uppercase tracking-widest flex items-center gap-1"><Shield size={8} /> Verified_Secure</span>
      </div>
    </div>
  );
};

export default Repository;
