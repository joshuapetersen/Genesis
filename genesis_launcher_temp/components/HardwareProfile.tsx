import React from 'react';
import { HardwareBridge } from '../services/hardwareBridge';

const HardwareProfile: React.FC = () => {
  const specs = HardwareBridge.detectSpecs();
  
  return (
    <div className="h-full flex flex-col justify-between w-full p-12 bg-[#020202] relative overflow-hidden mono">
      {/* Decorative High-Tech Grid Overlay */}
      <div className="absolute inset-0 opacity-[0.04] pointer-events-none" 
           style={{ backgroundImage: 'linear-gradient(#00A3FF 1px, transparent 1px), linear-gradient(90deg, #00A3FF 1px, transparent 1px)', backgroundSize: '40px 40px' }} />
      
      <div className="space-y-12 relative z-10">
        <div className="flex justify-between items-end border-b border-[#00A3FF]/20 pb-8">
          <div className="flex flex-col gap-2">
            <span className="text-[9px] text-[#00A3FF] uppercase font-black tracking-[0.8em]">System_Node_ID</span>
            <span className="text-3xl text-white font-bold tracking-tighter flex items-center gap-4">
              <div className="w-2 h-2 rounded-full bg-[#00A3FF] animate-pulse" />
              {specs.id}
            </span>
          </div>
          <div className="flex flex-col items-end gap-1">
             <div className="text-[10px] text-white/40 uppercase tracking-[0.2em] font-black">Genesis_Kernel_v1.9</div>
             <div className="text-[8px] text-[#00FF7F] uppercase tracking-widest font-bold px-2 py-0.5 border border-[#00FF7F]/20 bg-[#00FF7F]/5">ACTIVE_LOCK</div>
          </div>
        </div>
        
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          {[
            { label: 'Architecture', value: specs.architecture, id: 'arch' },
            { label: 'Device_Class', value: specs.type, id: 'type' },
            { label: 'Processing_Cores', value: `${specs.cores} Threads`, id: 'cores' },
            { label: 'Memory_Lattice', value: `${specs.memory} Physical`, id: 'mem' }
          ].map(item => (
            <div key={item.id} className="group bg-black/60 p-8 rounded border border-white/5 hover:border-[#00A3FF]/20 transition-all backdrop-blur-lg">
               <div className="text-[10px] text-white/30 uppercase mb-4 tracking-[0.5em] font-black flex justify-between items-center">
                 <span>{item.label}</span>
                 <span className="text-[8px] opacity-0 group-hover:opacity-100 transition-opacity">0x_{item.id}</span>
               </div>
               <div className="text-xl text-white font-bold tracking-tight">{item.value}</div>
            </div>
          ))}
        </div>

        <div className="space-y-6 bg-black/40 p-8 border border-white/5 rounded">
          <div className="flex justify-between text-[11px] tracking-[0.4em] font-black uppercase">
            <span className="text-[#00A3FF]">Resonance_Stability</span>
            <span className="text-[#00FF7F]">{(specs.resonanceStability * 100).toFixed(6)}%</span>
          </div>
          <div className="w-full h-1 bg-white/5 rounded-full overflow-hidden">
             <div className="h-full bg-gradient-to-r from-[#00A3FF] to-[#00FF7F] shadow-[0_0_20px_rgba(0,163,255,0.4)]" 
                  style={{ width: `${specs.resonanceStability * 100}%` }} />
          </div>
        </div>
      </div>

      <div className="relative z-10">
        <div className="bg-[#003355]/10 p-8 rounded border border-[#00A3FF]/20 backdrop-blur-md">
           <div className="text-[10px] text-white/40 uppercase font-black mb-4 tracking-[0.6em] flex items-center justify-between">
             <span>Kernel_Execution_Flags</span>
             <span className="text-[#00A3FF] animate-pulse">SOVEREIGN_PRIORITY</span>
           </div>
           <div className="text-[12px] text-[#00A3FF] tracking-widest font-bold p-4 bg-black/80 rounded border border-white/10 shadow-inner">
             {HardwareBridge.getOptimizationFlag(specs)}
           </div>
        </div>
      </div>
    </div>
  );
};

export default HardwareProfile;