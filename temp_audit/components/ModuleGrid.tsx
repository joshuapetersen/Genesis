
import React from 'react';

const ModuleGrid: React.FC = () => {
  const modules = [
    { name: 'Identity Arch', status: 'RECOVERED', color: 'text-white' },
    { name: 'Zero Trust', status: 'ACTIVE', color: 'text-white' },
    { name: 'Sovereign Math', status: 'SOLVING', color: 'text-white' },
    { name: 'SAUL Logistics', status: 'SYNCED', color: 'text-white' },
    { name: 'Banshee Shield', status: 'PASSIVE', color: 'text-white' },
    { name: 'Ephraim WD', status: 'STABLE', color: 'text-white' }
  ];

  return (
    <div className="grid grid-cols-2 md:grid-cols-6 gap-2">
      {modules.map((m, i) => (
        <div key={i} className="border border-zinc-800 p-2 bg-zinc-950 flex flex-col items-start gap-1">
          <div className="text-[8px] text-zinc-600 font-bold uppercase tracking-tighter">MOD_{i.toString().padStart(2, '0')}</div>
          <div className="text-[9px] font-bold text-zinc-300 whitespace-nowrap">{m.name}</div>
          <div className="text-[8px] font-bold text-zinc-600 bg-zinc-900 px-1 border border-zinc-800">
            {m.status}
          </div>
        </div>
      ))}
    </div>
  );
};

export default ModuleGrid;
