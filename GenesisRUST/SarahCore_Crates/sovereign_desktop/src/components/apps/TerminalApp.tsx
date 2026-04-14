import React, { useState, useRef, useEffect } from 'react';
import { useSovereign } from '../../hooks/useSovereign';
import { useOSStore } from '../../store/osStore';
import { Brain, BrainCircuit } from 'lucide-react';
import { cn } from '../../ai-codium/lib/utils';

export default function TerminalApp() {
  const [history, setHistory] = useState<{ type: 'input' | 'output'; text: string }[]>([
    { type: 'output', text: 'GENESIS OS Terminal v1.0.9' },
    { type: 'output', text: 'Sarah Hypervisor: Online' },
  ]);
  const [input, setInput] = useState('');
  const [cwd, setCwd] = useState('/executive');
  const { sendIntent, response: lastResponse, thoughts: lastThoughts, isProcessing } = useSovereign();
  const { systemConfig, updateConfig } = useOSStore();
  const bottomRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (lastThoughts.length > 0 && systemConfig.showThoughts) {
      lastThoughts.forEach(thought => {
        setHistory(prev => [...prev, { type: 'output', text: `[THOUGHT] ${thought}`, isThought: true }]);
      });
    }
  }, [lastThoughts, systemConfig.showThoughts]);

  useEffect(() => {
    if (lastResponse) {
      print(lastResponse);
    }
  }, [lastResponse]);

  useEffect(() => {
    bottomRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [history]);

  const print = (text: string) => {
    setHistory(prev => [...prev, { type: 'output', text, isThought: false }]);
  };

  const handleCommand = (cmd: string) => {
    const trimmed = cmd.trim();
    if (!trimmed) return;

    setHistory(prev => [...prev, { type: 'input', text: `${cwd}$ ${trimmed}` }]);

    if (trimmed === 'clear') {
      setHistory([]);
      return;
    }

    sendIntent(trimmed);
  };

  return (
    <div className="h-full w-full bg-gray-950 text-green-400 font-mono p-4 overflow-y-auto text-sm flex flex-col relative">
      {/* Header with Toggle */}
      <div className="absolute top-4 right-4 z-10">
        <button 
          onClick={() => updateConfig({ showThoughts: !systemConfig.showThoughts })}
          className={`p-1.5 rounded-md transition-all flex items-center gap-2 border ${
            systemConfig.showThoughts 
              ? 'bg-blue-500/20 border-blue-500/50 text-blue-400' 
              : 'bg-gray-800/50 border-gray-700 text-gray-500 hover:text-gray-400'
          }`}
          title={systemConfig.showThoughts ? "Hide Internal Monologue" : "Show Internal Monologue"}
        >
          {systemConfig.showThoughts ? <BrainCircuit size={16} /> : <Brain size={16} />}
          <span className="text-[10px] uppercase tracking-wider font-bold">
            {systemConfig.showThoughts ? "CoT: Active" : "CoT: Hidden"}
          </span>
        </button>
      </div>

      {history.map((line, i) => (
        <div key={i} className="mb-1">
          {line.type === 'input' ? (
            <span className="text-blue-400">{line.text}</span>
          ) : (
            <span className={cn(
              "whitespace-pre-wrap",
              (line as any).isThought ? "text-gray-500 italic opacity-80" : "text-gray-300"
            )}>
              {line.text}
            </span>
          )}
        </div>
      ))}
      <div className="flex items-center mt-1">
        <span className="text-blue-400 mr-2">{cwd}$</span>
        <input
          type="text"
          value={input}
          onChange={(e) => setInput(e.target.value)}
          onKeyDown={(e) => {
            if (e.key === 'Enter') {
              handleCommand(input);
              setInput('');
            }
          }}
          className="flex-1 bg-transparent outline-none border-none text-green-400"
          autoFocus
        />
      </div>
      <div ref={bottomRef} />
    </div>
  );
}
