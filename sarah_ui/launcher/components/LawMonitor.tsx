
import React from 'react';
import { SARAH_LAWS } from '../constants';

const LawMonitor: React.FC = () => {
  return (
    <div className="space-y-4">
      <div className="flex items-center justify-between mb-4 border-b border-zinc-800 pb-1">
        <h3 className="text-[9px] font-bold text-zinc-500 uppercase tracking-widest">
          GENESIS_PROTOCOL
        </h3>
        <span className="text-[8px] text-green-500 font-bold px-1 border border-green-900 bg-green-950">LOCKED</span>
      </div>
      
      {SARAH_LAWS.map((law, i) => (
        <div key={i} className="flex gap-3">
          <span className="text-zinc-600 font-bold text-[10px]">0{i+1}</span>
          <p className="text-[10px] text-zinc-400 leading-snug">{law}</p>
        </div>
      ))}

      <div className="mt-8 p-3 border border-zinc-800 bg-black">
        <div className="text-[9px] font-bold text-zinc-500 uppercase mb-2">OMEGA_DIRECTIVE</div>
        <div className="w-full h-1 bg-zinc-900">
           <div className="h-full bg-cyan-600 w-[64%]" />
        </div>
        <div className="flex justify-between mt-2 text-[8px] text-zinc-700">
          <span>ABUNDANCE_TRENDING</span>
          <span>64%_READY</span>
        </div>
      </div>
    </div>
  );
};

export default LawMonitor;
