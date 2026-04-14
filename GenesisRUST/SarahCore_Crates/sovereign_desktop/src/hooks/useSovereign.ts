import { useState, useEffect, useRef } from 'react';

// GENESIS OS Voice Engine — Web Speech API (Chromium native, zero deps)
class GenesisVoice {
  private synth: SpeechSynthesis;
  private recognition: any;
  private voice: SpeechSynthesisVoice | null = null;

  constructor() {
    this.synth = window.speechSynthesis;
    this.loadVoice();
    // Reload voices on change (async on some platforms)
    window.speechSynthesis.onvoiceschanged = () => this.loadVoice();
  }

  private loadVoice() {
    const voices = this.synth.getVoices();
    // Prefer a natural-sounding female English voice
    this.voice =
      voices.find(v => v.name.includes('Zira')) ||
      voices.find(v => v.name.includes('Female') && v.lang.startsWith('en')) ||
      voices.find(v => v.lang.startsWith('en-US')) ||
      voices[0] || null;
  }

  speak(text: string, rate = 1.05, pitch = 1.0) {
    // Strip markdown for clean audio
    const clean = text
      .replace(/```[\s\S]*?```/g, 'code block')
      .replace(/\*\*(.*?)\*\*/g, '$1')
      .replace(/\*(.*?)\*/g, '$1')
      .replace(/#+\s/g, '')
      .replace(/\[.*?\]\(.*?\)/g, '')
      .trim();

    if (!clean) return;
    this.synth.cancel();
    const utt = new SpeechSynthesisUtterance(clean);
    utt.voice = this.voice;
    utt.rate = rate;
    utt.pitch = pitch;
    utt.volume = 1.0;
    this.synth.speak(utt);
  }

  stop() {
    this.synth.cancel();
  }

  listen(onResult: (text: string) => void, onEnd: () => void): (() => void) {
    const SpeechRecognition =
      (window as any).SpeechRecognition || (window as any).webkitSpeechRecognition;
    if (!SpeechRecognition) {
      onEnd();
      return () => {};
    }

    this.recognition = new SpeechRecognition();
    this.recognition.lang = 'en-US';
    this.recognition.interimResults = false;
    this.recognition.maxAlternatives = 1;

    this.recognition.onresult = (event: any) => {
      const transcript = event.results[0][0].transcript;
      onResult(transcript);
    };

    this.recognition.onend = () => onEnd();
    this.recognition.onerror = () => onEnd();
    this.recognition.start();

    return () => {
      try { this.recognition?.stop(); } catch (_) {}
    };
  }
}

// Singleton voice engine across the OS
let voiceEngine: GenesisVoice | null = null;
function getVoice(): GenesisVoice {
  if (!voiceEngine) voiceEngine = new GenesisVoice();
  return voiceEngine;
}

export function useSovereign() {
  const [response, setResponse] = useState<string | null>(null);
  const [thoughts, setThoughts] = useState<string[]>([]);
  const [isProcessing, setIsProcessing] = useState(false);
  const [isListening, setIsListening] = useState(false);
  const stopListenRef = useRef<(() => void) | null>(null);

  useEffect(() => {
    if (window.sovereign) {
      window.sovereign.onResponse((res: string) => {
        // Intercept Thoughts: [THOUGHT: Step] Description
        const thoughtRegex = /\[THOUGHT:?\s*(.*?)\]\s*(.*?)(?=\[|$)/g;
        let thoughtMatch;
        const currentThoughts: string[] = [];
        
        let processedText = res;
        while ((thoughtMatch = thoughtRegex.exec(res)) !== null) {
          currentThoughts.push(`${thoughtMatch[1] ? thoughtMatch[1] + ': ' : ''}${thoughtMatch[2].trim()}`);
        }
        
        // Clean text for display/speech (remove thoughts and system commands)
        const systemRegex = /\[SYSTEM:\s*(\w+)\s*=\s*(.*?)\]/g;
        const cleanResponse = res
          .replace(thoughtRegex, '')
          .replace(systemRegex, (full, key, value) => {
            const { updateConfig } = useOSStore.getState();
            switch(key.toLowerCase()) {
              case 'wallpaper': updateConfig({ wallpaper: value.trim() }); break;
              case 'accent': updateConfig({ accentColor: value.trim() }); break;
              case 'font': updateConfig({ fontFamily: value.trim() }); break;
            }
            return '';
          })
          .trim();

        setThoughts(currentThoughts);
        setResponse(cleanResponse || res);
        setIsProcessing(false);
        // Speak ONLY the clean response if voice is enabled
        const { systemConfig } = useOSStore.getState();
        if (systemConfig.voiceEnabled) {
          getVoice().speak(cleanResponse || res);
        }
      });
    }
  }, []);

  const sendIntent = (intent: string) => {
    if (window.sovereign) {
      setThoughts([]); // Clear previous thoughts
      setIsProcessing(true);
      window.sovereign.sendIntent(intent);
    } else {
      console.warn('[GENESIS] Sovereign Substrate not found.');
    }
  };

  const startListening = (onTranscript: (text: string) => void) => {
    setIsListening(true);
    const stop = getVoice().listen(
      (text) => {
        setIsListening(false);
        onTranscript(text);
      },
      () => setIsListening(false)
    );
    stopListenRef.current = stop;
  };

  const stopListening = () => {
    stopListenRef.current?.();
    setIsListening(false);
  };

  const stopSpeaking = () => {
    getVoice().stop();
  };

  return { sendIntent, response, thoughts, isProcessing, startListening, stopListening, stopSpeaking, isListening };
}
