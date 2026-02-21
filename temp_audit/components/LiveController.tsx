
import React, { useState, useRef, useEffect } from 'react';
import { connectLiveAPI } from '../services/geminiService';
import { decodeAudioData } from '../services/audioUtils';

const LiveController: React.FC = () => {
  const [micOn, setMicOn] = useState(false);
  const [camOn, setCamOn] = useState(false);
  const [sharing, setSharing] = useState(false);
  const [position, setPosition] = useState({ x: window.innerWidth - 300, y: 120 });
  const [isDragging, setIsDragging] = useState(false);
  const dragOffset = useRef({ x: 0, y: 0 });
  const videoRef = useRef<HTMLVideoElement>(null);

  // Live API Session refs
  const sessionPromiseRef = useRef<any>(null);
  const audioContextOutRef = useRef<AudioContext | null>(null);
  const audioContextInRef = useRef<AudioContext | null>(null);
  const nextStartTimeRef = useRef(0);
  const sourcesRef = useRef<Set<AudioBufferSourceNode>>(new Set());

  useEffect(() => {
    if (camOn) {
      navigator.mediaDevices.getUserMedia({ video: true })
        .then(stream => { if (videoRef.current) videoRef.current.srcObject = stream; })
        .catch(err => console.error("Camera access denied", err));
    } else {
      if (videoRef.current?.srcObject) {
        (videoRef.current.srcObject as MediaStream).getTracks().forEach(t => t.stop());
        videoRef.current.srcObject = null;
      }
    }
  }, [camOn]);

  const encodePCM = (data: Float32Array): string => {
    const int16 = new Int16Array(data.length);
    for (let i = 0; i < data.length; i++) {
      int16[i] = data[i] * 32768;
    }
    let binary = '';
    const bytes = new Uint8Array(int16.buffer);
    for (let i = 0; i < bytes.byteLength; i++) {
      binary += String.fromCharCode(bytes[i]);
    }
    return btoa(binary);
  };

  const decodeBase64ToUint8 = (base64: string): Uint8Array => {
    const binary = atob(base64);
    const bytes = new Uint8Array(binary.length);
    for (let i = 0; i < binary.length; i++) {
      bytes[i] = binary.charCodeAt(i);
    }
    return bytes;
  };

  const startLiveSession = async () => {
    audioContextOutRef.current = new (window.AudioContext || (window as any).webkitAudioContext)({ sampleRate: 24000 });
    audioContextInRef.current = new (window.AudioContext || (window as any).webkitAudioContext)({ sampleRate: 16000 });
    
    const stream = await navigator.mediaDevices.getUserMedia({ audio: true });
    
    sessionPromiseRef.current = connectLiveAPI({
      onopen: () => {
        const source = audioContextInRef.current!.createMediaStreamSource(stream);
        const scriptProcessor = audioContextInRef.current!.createScriptProcessor(4096, 1, 1);
        scriptProcessor.onaudioprocess = (e) => {
          const inputData = e.inputBuffer.getChannelData(0);
          const base64 = encodePCM(inputData);
          sessionPromiseRef.current.then((session: any) => {
            session.sendRealtimeInput({ media: { data: base64, mimeType: 'audio/pcm;rate=16000' } });
          });
        };
        source.connect(scriptProcessor);
        scriptProcessor.connect(audioContextInRef.current!.destination);
      },
      onmessage: async (message: any) => {
        const audioData = message.serverContent?.modelTurn?.parts[0]?.inlineData?.data;
        if (audioData) {
          const ctx = audioContextOutRef.current!;
          nextStartTimeRef.current = Math.max(nextStartTimeRef.current, ctx.currentTime);
          const buffer = await decodeAudioData(decodeBase64ToUint8(audioData), ctx, 24000, 1);
          const source = ctx.createBufferSource();
          source.buffer = buffer;
          source.connect(ctx.destination);
          source.start(nextStartTimeRef.current);
          nextStartTimeRef.current += buffer.duration;
          sourcesRef.current.add(source);
          source.onended = () => sourcesRef.current.delete(source);
        }
        if (message.serverContent?.interrupted) {
          sourcesRef.current.forEach(s => s.stop());
          sourcesRef.current.clear();
          nextStartTimeRef.current = 0;
        }
      },
      onerror: (e: any) => console.error("Live API Error:", e),
      onclose: () => setMicOn(false)
    });
  };

  const stopLiveSession = () => {
    sessionPromiseRef.current?.then((session: any) => session.close());
    audioContextInRef.current?.close();
    audioContextOutRef.current?.close();
    sourcesRef.current.forEach(s => s.stop());
    sourcesRef.current.clear();
  };

  const handleMicToggle = () => {
    const newState = !micOn;
    setMicOn(newState);
    if (newState) {
      startLiveSession();
    } else {
      stopLiveSession();
    }
  };

  const toggleScreen = async () => {
    if (!sharing) {
      try {
        await (navigator.mediaDevices as any).getDisplayMedia({ video: true });
        setSharing(true);
      } catch (err) { console.error(err); }
    } else {
      setSharing(false);
    }
  };

  const handleMouseDown = (e: React.MouseEvent | React.TouchEvent) => {
    setIsDragging(true);
    const clientX = 'touches' in e ? e.touches[0].clientX : e.clientX;
    const clientY = 'touches' in e ? e.touches[0].clientY : e.clientY;
    dragOffset.current = {
      x: clientX - position.x,
      y: clientY - position.y
    };
  };

  useEffect(() => {
    const handleMove = (e: MouseEvent | TouchEvent) => {
      if (!isDragging) return;
      const clientX = 'touches' in e ? e.touches[0].clientX : (e as MouseEvent).clientX;
      const clientY = 'touches' in e ? e.touches[0].clientY : (e as MouseEvent).clientY;
      setPosition({
        x: clientX - dragOffset.current.x,
        y: clientY - dragOffset.current.y
      });
    };
    const handleUp = () => setIsDragging(false);
    if (isDragging) {
      window.addEventListener('mousemove', handleMove);
      window.addEventListener('mouseup', handleUp);
      window.addEventListener('touchmove', handleMove);
      window.addEventListener('touchend', handleUp);
    }
    return () => {
      window.removeEventListener('mousemove', handleMove);
      window.removeEventListener('mouseup', handleUp);
      window.removeEventListener('touchmove', handleMove);
      window.removeEventListener('touchend', handleUp);
    };
  }, [isDragging]);

  return (
    <div className="flex gap-4 items-center">
      {camOn && (
        <div 
          onMouseDown={handleMouseDown}
          onTouchStart={handleMouseDown}
          className="fixed w-48 h-48 rounded-full border-2 border-[#00A3FF]/40 overflow-hidden shadow-[0_0_50px_rgba(0,163,255,0.2)] bg-black/80 backdrop-blur-3xl z-[1001] cursor-move active:scale-95 transition-transform"
          style={{ left: position.x, top: position.y }}
        >
          <video ref={videoRef} autoPlay playsInline muted className="w-full h-full object-cover grayscale brightness-125 contrast-125 scale-x-[-1] pointer-events-none" />
          <div className="absolute inset-0 bg-[#00A3FF]/10 pointer-events-none" />
          <div className="absolute top-2 left-1/2 -translate-x-1/2 w-8 h-1 bg-white/10 rounded-full" />
        </div>
      )}

      <button 
        onClick={toggleScreen}
        className="sys-icon-btn group relative"
        title="LATTICE_CAST"
      >
        <svg viewBox="0 0 24 24" className={`resonance-icon ${sharing ? '!stroke-[#00A3FF] !opacity-100' : ''}`}>
           <path d="M4 6a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2v7" />
           <path d="M12 17v4" />
           <path d="M8 21h8" />
           <path d="m17 17 3-3 3 3" />
           <path d="M20 14v6" />
        </svg>
        <span className="icon-label">SCREEN_SYNC</span>
      </button>

      <button 
        onClick={handleMicToggle}
        className="sys-icon-btn group relative"
        title="VOICE_SYNC"
      >
        <svg viewBox="0 0 24 24" className={`resonance-icon ${micOn ? 'mic-active !opacity-100' : ''}`}>
           <path d="M12 1a3 3 0 0 0-3 3v8a3 3 0 0 0 6 0V4a3 3 0 0 0-3-3z" />
           <path d="M19 10v2a7 7 0 0 1-14 0v-2" />
           {!micOn && (
             <line x1="1" y1="1" x2="23" y2="23" stroke="#FF3B30" strokeOpacity="0.8" strokeWidth="2" />
           )}
        </svg>
        <span className="icon-label">{micOn ? 'LIVE_ACTIVE' : 'MUTED'}</span>
        {micOn && (
          <div className="absolute inset-0 rounded-full border-2 border-cyan-400 animate-ping opacity-20 pointer-events-none" />
        )}
      </button>

      <button 
        onClick={() => setCamOn(!camOn)}
        className="sys-icon-btn group relative"
        title="EYE_LINK"
      >
        <svg viewBox="0 0 24 24" className={`resonance-icon ${camOn ? '!opacity-100' : ''}`}>
           <path d="M23 7l-7 5 7 5V7z" />
           <rect x="1" y="5" width="15" height="14" rx="2" ry="2" />
           {!camOn && (
             <line x1="1" y1="1" x2="23" y2="23" stroke="#FF3B30" strokeOpacity="0.8" strokeWidth="2" />
           )}
        </svg>
        <span className="icon-label">EYE_LINK</span>
      </button>
    </div>
  );
};

export default LiveController;
