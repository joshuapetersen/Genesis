
import React, { useState, useRef, useEffect } from 'react';
import { Message } from '../types';
import { generateSarahResponse, generateTTS } from '../services/geminiService';
import { generateLocalResponse } from '../services/offlineService';
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
        if (scrollRef.current) {
            scrollRef.current.scrollTop = scrollRef.current.scrollHeight;
        }
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
                onVibrate?.(average / 100);
            }, 16);

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

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();
        if (!input.trim() || isProcessing) return;

        const content = input;
        const userMsg: Message = {
            id: Date.now().toString(),
            role: 'user',
            content: content,
            timestamp: new Date().toLocaleTimeString()
        };

        setMessages(prev => [...prev, userMsg]);
        setInput('');
        setIsProcessing(true);

        try {
            const responseText = await generateLocalResponse([...messages, userMsg]);
            const sarahMsg: Message = {
                id: (Date.now() + 1).toString(),
                role: 'sarah',
                content: responseText,
                timestamp: new Date().toLocaleTimeString()
            };
            setMessages(prev => [...prev, sarahMsg]);
            if (!responseText.includes("UNREACHABLE")) {
                handleSpeech(responseText);
            }
        } catch (error) {
            console.error("Local inference failed:", error);
        } finally {
            setIsProcessing(false);
        }
    };

    return (
        <div className="flex flex-col h-full w-full bg-[#020202] relative overflow-hidden border-l border-white/5">
            <div className="absolute left-6 top-0 bottom-0 w-px bg-gradient-to-b from-transparent via-white/5 to-transparent" />

            {/* MESSAGES AREA - FIXED SCROLLING */}
            <div
                ref={scrollRef}
                className="flex-1 overflow-y-auto p-6 md:p-10 space-y-8 mono"
                style={{ scrollBehavior: 'smooth' }}
            >
                {messages.map((m) => (
                    <div key={m.id} className={`flex flex-col ${m.role === 'user' ? 'items-end' : 'items-start'} animate-in fade-in slide-in-from-bottom-2 duration-300`}>
                        <div className="flex items-center gap-3 mb-2 px-1">
                            <span className="text-[8px] text-white/20 font-black uppercase tracking-[0.2em]">
                                {m.role === 'sarah' ? 'SARAH_KERNEL' : 'OPERATOR'}
                            </span>
                            <div className={`w-1 h-1 rounded-full ${m.role === 'sarah' ? 'bg-[#00A3FF]' : 'bg-white/20'}`} />
                            <span className="text-[8px] text-white/10 uppercase tracking-widest">{m.timestamp}</span>
                        </div>

                        <div
                            className={`max-w-[85%] rounded-lg px-6 py-4 border ${m.role === 'sarah'
                                ? 'bg-[#001A33]/40 text-white border-[#00A3FF]/30 shadow-2xl backdrop-blur-xl'
                                : 'bg-white/[0.05] text-white/90 border-white/10'
                                }`}
                        >
                            <div className="text-[14px] leading-relaxed font-medium tracking-wide whitespace-pre-wrap">{m.content}</div>
                        </div>
                    </div>
                ))}
                {isProcessing && (
                    <div className="flex items-center gap-4 ml-2 animate-pulse">
                        <div className="w-1.5 h-1.5 rounded-full bg-[#00A3FF]" />
                        <span className="text-[#00A3FF] text-[10px] mono tracking-[0.5em] font-black uppercase italic">PROCESSING_SIGNAL...</span>
                    </div>
                )}
            </div>

            {/* INPUT AREA - FIXED VISIBILITY */}
            <form onSubmit={handleSubmit} className="p-6 bg-black/80 border-t border-white/10 backdrop-blur-3xl">
                <div className="flex gap-4 items-center bg-[#050505] border border-white/20 rounded-md px-6 py-4 focus-within:border-[#00A3FF]/60 transition-all shadow-2xl">
                    <span className="text-[#00A3FF] text-xl opacity-50">🧬</span>
                    <input
                        type="text"
                        value={input}
                        onChange={(e) => setInput(e.target.value)}
                        placeholder="INPUT_COMMAND_..."
                        className="flex-1 bg-transparent border-none outline-none text-white text-[16px] mono placeholder:text-white/10 font-bold"
                        autoFocus
                    />
                    <button
                        type="submit"
                        disabled={isProcessing}
                        className="text-[#00A3FF] text-2xl hover:scale-125 active:scale-90 transition-all"
                    >
                        {isProcessing ? '⌛' : '⚡'}
                    </button>
                </div>
                <div className="mt-2 text-center">
                    <span className="text-[7px] text-white/10 uppercase tracking-[0.8em]">Sovereign Core Alpha 1.3.6 // Offline Mode</span>
                </div>
            </form>
        </div>
    );
};

export default Terminal;
