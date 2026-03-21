import React, { useState } from 'react';
import { generateImage } from '../services/geminiService';

const CreatorModule: React.FC = () => {
  const [prompt, setPrompt] = useState('');
  const [image, setImage] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);

  const handleGenerate = async () => {
    if (!prompt.trim() || loading) return;
    setLoading(true);
    try {
      const result = await generateImage(prompt, { size: '1K', ratio: '1:1' });
      setImage(result);
    } catch (e) { 
      console.error(e); 
      if (e instanceof Error && e.message.includes("Requested entity was not found")) {
        window.location.reload(); 
      }
    } finally { 
      setLoading(false); 
    }
  };

  return (
    <div className="p-10 flex flex-col h-full w-full gap-8 bg-[#020202] relative overflow-hidden mono">
       {/* Grid background */}
       <div className="absolute inset-0 opacity-[0.02] pointer-events-none" 
           style={{ backgroundImage: 'radial-gradient(#fff 1px, transparent 0)', backgroundSize: '30px 30px' }} />

      <div className="flex-1 bg-black/40 rounded-xl border border-white/5 relative flex items-center justify-center overflow-hidden backdrop-blur-sm">
        {loading ? (
          <div className="flex flex-col items-center gap-8">
            <div className="w-10 h-10 rounded-full border-t-2 border-[#00A3FF] animate-spin" />
            <div className="text-[#00A3FF] animate-pulse text-[9px] tracking-[1em] font-black uppercase italic">RESOLUTION_IN_PROGRESS</div>
          </div>
        ) : image ? (
          <img src={image} className="w-full h-full object-contain animate-in fade-in zoom-in-95 duration-700" alt="Generated Output" />
        ) : (
          <div className="text-white/5 text-[9px] uppercase tracking-[2em] font-black select-none pointer-events-none text-center px-20">
            SOVEREIGN_CANVAS_AWAITING_RESONANCE
          </div>
        )}
      </div>

      <div className="flex gap-4 p-4 bg-black/60 rounded-lg border border-white/10 focus-within:border-[#00A3FF]/30 transition-all shadow-2xl">
        <input 
          type="text"
          value={prompt}
          onChange={(e) => setPrompt(e.target.value)}
          onKeyDown={(e) => e.key === 'Enter' && handleGenerate()}
          placeholder="ENTER_VISUAL_SEED_..."
          className="flex-1 bg-transparent border-none outline-none text-white text-[12px] px-4 py-2 placeholder:text-white/5 uppercase tracking-widest font-bold"
        />
        <button 
          onClick={handleGenerate} 
          disabled={loading}
          className="px-10 py-3 bg-[#00A3FF] text-black text-[10px] font-black uppercase rounded transition-all hover:bg-white active:scale-95 disabled:opacity-20 shadow-[0_0_15px_rgba(0,163,255,0.4)]"
        >
          {loading ? 'BUSY' : 'RESOLVE'}
        </button>
      </div>
      
      <div className="flex justify-between text-[8px] text-white/20 uppercase tracking-widest font-black">
        <span>IMAGEN_ENGINE_ACTIVE</span>
        <span>OUTPUT_DENSITY_1K</span>
      </div>
    </div>
  );
};

export default CreatorModule;