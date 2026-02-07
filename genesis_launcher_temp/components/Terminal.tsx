
import React, { useState, useRef, useEffect } from 'react';
import { Message } from '../types';
import { generateSarahResponse, generateTTS } from '../services/geminiService';
import { decodeBase64, decodeAudioData } from '../services/audioUtils';

interface TerminalProps {
  opacity?: number;
  onVibrate?: (level: number) => void;
  onSetEmotionColor?: (color: string | null) => void;
}

const EMOTION_COLORS: Record<string, string> = {
  happy: '#FFD700',   
  angry: '#FF3B30',   
  sad: '#4B0082',     
  excited: '#FF00FF', 
  calm: '#00FF7F',    
  protective: '#00FFFF', 
  uncompromising: '#8A2BE2' 
};

const Terminal: React.FC<TerminalProps> = ({ opacity = 1.0, onVibrate, onSetEmotionColor }) => {
  const [messages, setMessages] = useState<Message[]>([
    {
      id: 'greeting',
      role: 'sarah',
      content: `Lattice synchronization stable. Operational frequency locked at 1.0927Hz. System awaits commission.`,
      timestamp: new Date().toLocaleTimeString()
    }
  ]);
  const [input, setInput] = useState('');
  const [selectedImage, setSelectedImage] = useState<string | null>(null);
  const [isProcessing, setIsProcessing] = useState(false);
  const scrollRef = useRef<HTMLDivElement>(null);
  const fileInputRef = useRef<HTMLInputElement>(null);
  
  const audioContextRef = useRef<AudioContext | null>(null);
  const analyserRef = useRef<AnalyserNode | null>(null);
  const vibrationIntervalRef = useRef<number | null>(null);

  useEffect(() => {
    if (scrollRef.current) scrollRef.current.scrollTop = scrollRef.current.scrollHeight;
  }, [messages]);

  const detectEmotion = (text: string) => {
    const lower = text.toLowerCase();
    if (lower.includes('cheer') || lower.includes('happy') || lower.includes('wonderful')) return EMOTION_COLORS.happy;
    if (lower.includes('error') || lower.includes('breach') || lower.includes('danger') || lower.includes('stop')) return EMOTION_COLORS.angry;
    if (lower.includes('lost') || lower.includes('severed') || lower.includes('sad')) return EMOTION_COLORS.sad;
    if (lower.includes('high-signal') || lower.includes('power') || lower.includes('active') || lower.includes('boost')) return EMOTION_COLORS.excited;
    if (lower.includes('stable') || lower.includes('ready') || lower.includes('calm')) return EMOTION_COLORS.calm;
    if (lower.includes('protective') || lower.includes('shield') || lower.includes('secure')) return EMOTION_COLORS.protective;
    if (lower.includes('uncompromising') || lower.includes('absolute') || lower.includes('law')) return EMOTION_COLORS.uncompromising;
    return null;
  };

  const handleSpeech = async (text: string) => {
    try {
      const base64Audio = await generateTTS(text);
      if (!base64Audio) return;

      if (!audioContextRef.current) {
        audioContextRef.current = new (window.AudioContext || (window as any).webkitAudioContext)({ sampleRate: 24000 });
        analyserRef.current = audioContextRef.current.createAnalyser();
        analyserRef.current.fftSize = 256;
        analyserRef.current.smoothingTimeConstant = 0.4;
      }

      const ctx = audioContextRef.current;
      const analyser = analyserRef.current!;
      const audioBuffer = await decodeAudioData(decodeBase64(base64Audio), ctx, 24000, 1);
      
      const source = ctx.createBufferSource();
      source.buffer = audioBuffer;
      source.connect(analyser);
      analyser.connect(ctx.destination);

      const emotionColor = detectEmotion(text);
      onSetEmotionColor?.(emotionColor);

      const dataArray = new Uint8Array(analyser.frequencyBinCount);
      vibrationIntervalRef.current = window.setInterval(() => {
        analyser.getByteFrequencyData(dataArray);
        const average = dataArray.reduce((a, b) => a + b) / dataArray.length;
        // Map average frequency to vibration intensity
        onVibrate?.(average / 100); 
      }, 16); // ~60fps for smooth waveform reactivity

      source.onended = () => {
        if (vibrationIntervalRef.current) clearInterval(vibrationIntervalRef.current);
        onVibrate?.(0);
        onSetEmotionColor?.(null);
      };

      source.start();
    } catch (err) {
      console.error("Speech playback error:", err);
    }
  };

  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (file) {
      const reader = new FileReader();
      reader.onload = (event) => setSelectedImage(event.target?.result as string);
      reader.readAsDataURL(file);
    }
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!input.trim() || isProcessing) return;
    
    const content = input;
    const image = selectedImage;
    
    const userMsg: Message = { 
      id: Date.now().toString(), 
      role: 'user', 
      content: content, 
      timestamp: new Date().toLocaleTimeString() 
    };
    
    setMessages(prev => [...prev, userMsg]);
    setInput('');
    setSelectedImage(null);
    setIsProcessing(true);
    
    const response = await generateSarahResponse({ 
      prompt: content, 
      image: image || undefined,
      useThinking: content.length > 50 
    });

    const sarahMsg: Message = { 
      id: (Date.now() + 1).toString(), 
      role: 'sarah', 
      content: response.text, 
      timestamp: new Date().toLocaleTimeString() 
    };
    
    setMessages(prev => [...prev, sarahMsg]);
    setIsProcessing(false);
    
    handleSpeech(response.text);
  };

  return (
    <div className="flex flex-col h-full w-full bg-[#020202] relative overflow-hidden border-l border-white/5">
      <div className="absolute left-6 top-0 bottom-0 w-px bg-gradient-to-b from-transparent via-white/5 to-transparent" />

      <div ref={scrollRef} className="flex-1 overflow-y-auto p-10 space-y-10 mono scrollbar-hide">
        {messages.map((m) => (
          <div key={m.id} className={`flex flex-col ${m.role === 'user' ? 'items-end' : 'items-start'} animate-in fade-in slide-in-from-bottom-4 duration-500`}>
            <div className="flex items-center gap-3 mb-2 px-1">
               <span className="text-[8px] text-white/20 font-black uppercase tracking-[0.2em]">
                 {m.role === 'sarah' ? 'SARAH_KERNEL' : 'OPERATOR'}
               </span>
               <div className={`w-1 h-1 rounded-full ${m.role === 'sarah' ? 'bg-[#00A3FF]' : 'bg-white/20'}`} />
               <span className="text-[8px] text-white/10 uppercase tracking-widest">{m.timestamp}</span>
            </div>
            
            <div 
              className={`max-w-[90%] rounded-lg px-7 py-5 border ${
                m.role === 'sarah' 
                  ? 'bg-[#001A33]/30 text-white border-[#00A3FF]/20 shadow-2xl backdrop-blur-md' 
                  : 'bg-white/[0.03] text-white/90 border-white/5'
              }`}
            >
              <div className="text-[13px] leading-relaxed font-medium tracking-wide">{m.content}</div>
            </div>
          </div>
        ))}
        {isProcessing && (
          <div className="flex items-center gap-4 ml-2 animate-pulse">
             <div className="w-1.5 h-1.5 rounded-full bg-[#00A3FF]" />
             <span className="text-[#00A3FF] text-[10px] mono tracking-[0.8em] font-black uppercase italic">SYNC_BIT_STREAM...</span>
          </div>
        )}
      </div>

      <form onSubmit={handleSubmit} className="p-8 bg-black/60 border-t border-white/10 backdrop-blur-2xl">
        {selectedImage && (
          <div className="mb-6 relative w-20 h-20 animate-in zoom-in-95 fade-in">
            <img src={selectedImage} alt="Selected" className="w-full h-full object-cover rounded-lg border border-[#00A3FF]/50 shadow-lg" />
            <button 
              type="button" 
              onClick={() => setSelectedImage(null)}
              className="absolute -top-2 -right-2 bg-black text-white w-6 h-6 rounded-full border border-red-500/50 text-[10px] flex items-center justify-center hover:bg-red-500 transition-colors"
            >×</button>
          </div>
        )}
        <div className="flex gap-4 items-center bg-[#050505] border border-white/10 rounded-md px-6 py-4 focus-within:border-[#00A3FF]/40 transition-all shadow-inner">
          <input 
            type="file" 
            accept="image/*" 
            ref={fileInputRef} 
            onChange={handleFileChange} 
            className="hidden" 
          />
          <button 
            type="button" 
            onClick={() => fileInputRef.current?.click()}
            className="text-white/20 hover:text-[#00A3FF] transition-colors text-xl"
            title="ATTACH_DATA"
          >📷</button>
          
          <input 
            type="text"
            value={input}
            onChange={(e) => setInput(e.target.value)}
            placeholder="SYSTEM_INPUT_..."
            className="flex-1 bg-transparent border-none outline-none text-white text-[14px] mono uppercase placeholder:text-white/5 font-medium tracking-tight"
          />
          <button type="submit" className="text-[#00A3FF] text-2xl hover:scale-125 transition-all drop-shadow-lg">⚡</button>
        </div>
      </form>
    </div>
  );
};

export default Terminal;
