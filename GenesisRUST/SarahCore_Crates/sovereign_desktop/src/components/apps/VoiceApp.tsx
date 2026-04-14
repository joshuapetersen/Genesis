import React, { useEffect, useState, useRef } from 'react';
import { Mic, MicOff, Volume2, Waveform } from 'lucide-react';
import { useSovereign } from '../../hooks/useSovereign';
import { useOSStore } from '../../store/osStore';
import { cn } from '../../ai-codium/lib/utils';

export default function VoiceApp() {
  const { isListening, isProcessing, response, stopListening, startListening, stopSpeaking } = useSovereign();
  const { systemConfig, updateConfig } = useOSStore();
  const [pulse, setPulse] = useState(1);
  
  // Simulated Waveform logic
  useEffect(() => {
    if (isListening || isProcessing) {
      const interval = setInterval(() => {
        setPulse(0.8 + Math.random() * 0.4);
      }, 100);
      return () => clearInterval(interval);
    } else {
      setPulse(1);
    }
  }, [isListening, isProcessing]);

  return (
    <div className="flex flex-col h-full bg-gradient-to-b from-purple-950 to-black p-8 items-center justify-center relative overflow-hidden">
      {/* Background Glow */}
      <div className={cn(
        "absolute inset-0 opacity-20 transition-all duration-1000",
        isListening ? "bg-red-500" : isProcessing ? "bg-purple-500" : "bg-blue-500"
      )} />

      {/* Central Visualizer */}
      <div className="relative z-10 flex flex-col items-center gap-12">
        <div className="relative group">
          <div 
            className="w-48 h-48 rounded-full border-4 border-white/10 flex items-center justify-center transition-all duration-300"
            style={{ 
              transform: `scale(${pulse})`,
              boxShadow: isListening 
                ? '0 0 80px rgba(239, 68, 68, 0.4)' 
                : isProcessing 
                  ? '0 0 80px rgba(168, 85, 247, 0.4)' 
                  : '0 0 40px rgba(59, 130, 246, 0.1)'
            }}
          >
            <div className="w-40 h-40 rounded-full bg-white/5 backdrop-blur-3xl flex items-center justify-center border border-white/20">
               {isListening ? (
                 <Mic size={64} className="text-red-400 animate-pulse" />
               ) : (
                 <Volume2 size={64} className="text-purple-400" />
               )}
            </div>
          </div>
          
          {/* Orbital Rings */}
          <div className="absolute inset-0 -m-4 border border-white/5 rounded-full animate-reverse-spin duration-[10s]" />
          <div className="absolute inset-0 -m-8 border border-white/5 rounded-full animate-spin duration-[15s]" />
        </div>

        <div className="text-center">
          <h2 className="text-2xl font-black tracking-tighter text-white uppercase mb-2">
            {isListening ? "Listening..." : isProcessing ? "Processing Resonance..." : "Sovereign Voice Active"}
          </h2>
          <p className="text-purple-400/60 font-mono text-xs uppercase tracking-[0.2em]">
            1.09277703703 Hz · Fidelity Lock
          </p>
        </div>

        {/* Controls */}
        <div className="flex gap-4">
          <button
            onClick={() => updateConfig({ voiceEnabled: !systemConfig.voiceEnabled })}
            className={cn(
              "px-6 py-3 rounded-2xl font-bold uppercase text-xs tracking-widest transition-all",
              systemConfig.voiceEnabled 
                ? "bg-purple-600 text-white shadow-lg shadow-purple-500/20" 
                : "bg-white/10 text-white/40 border border-white/10"
            )}
          >
            {systemConfig.voiceEnabled ? "Voice Enabled" : "Voice Disabled"}
          </button>
          
          <button
            onClick={() => {
              if (isListening) stopListening();
              else startListening(() => {});
            }}
            className={cn(
              "px-6 py-3 rounded-2xl font-bold uppercase text-xs tracking-widest transition-all",
              isListening 
                ? "bg-red-500 text-white animate-pulse" 
                : "bg-white/10 text-white hover:bg-white/20 border border-white/10"
            )}
          >
            {isListening ? "Stop Mic" : "Start Mic"}
          </button>
        </div>
      </div>

      {/* Subtitles / Latest Response */}
      {response && !isListening && (
        <div className="absolute bottom-12 left-8 right-8 text-center animate-in fade-in slide-in-from-bottom-4">
          <p className="text-gray-400 text-sm max-w-2xl mx-auto italic">
            "{response.length > 100 ? response.substring(0, 100) + '...' : response}"
          </p>
        </div>
      )}
    </div>
  );
}
