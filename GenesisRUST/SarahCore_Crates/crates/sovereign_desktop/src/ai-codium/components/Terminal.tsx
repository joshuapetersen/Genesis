import React, { useState, useRef, useEffect } from 'react';
import { useSovereign } from '../hooks/useSovereign';
// Sovereign SarahCore Handshake
import { Terminal as TerminalIcon, Send, Bot, User, Loader2, Check, Copy } from 'lucide-react';
import { motion, AnimatePresence } from 'motion/react';
import ReactMarkdown from 'react-markdown';
import remarkGfm from 'remark-gfm';
import { cn } from '@/src/lib/utils';
import { ChatMessage } from '@/src/types';

interface TerminalProps {
  currentCode: string;
  onApplyCode?: (newCode: string) => void;
}

export default function Terminal({ currentCode, onApplyCode }: TerminalProps) {
  const [input, setInput] = useState('');
  const [messages, setMessages] = useState<ChatMessage[]>([
    {
      role: 'assistant',
      content: 'Welcome to GENESIS OS Terminal. I can help you improve your code, debug issues, or explain logic.',
      timestamp: Date.now(),
    },
  ]);
  const [isLoading, setIsLoading] = useState(false);
  const scrollRef = useRef<HTMLDivElement>(null);

  const { sendIntent, response: lastResponse, isProcessing } = useSovereign();

  useEffect(() => {
    if (lastResponse) {
      const assistantMessage: ChatMessage = {
        role: 'assistant',
        content: lastResponse,
        timestamp: Date.now(),
      };
      setMessages(prev => [...prev, assistantMessage]);
      setIsLoading(false);
    }
  }, [lastResponse]);

  useEffect(() => {
    if (scrollRef.current) {
      scrollRef.current.scrollTop = scrollRef.current.scrollHeight;
    }
  }, [messages]);

  const handleSend = async () => {
    if (!input.trim() || isLoading) return;

    const userMessage: ChatMessage = {
      role: 'user',
      content: input,
      timestamp: Date.now(),
    };

    setMessages(prev => [...prev, userMessage]);
    setInput('');
    setIsLoading(true);

    setMessages(prev => [...prev, userMessage]);
    setInput('');
    setIsLoading(true);
    sendIntent(input);
  };

  const extractCode = (content: string) => {
    const match = content.match(/```(?:typescript|javascript|tsx|jsx|css|html)?\n([\s\S]*?)```/);
    return match ? match[1] : null;
  };

  return (
    <div className="flex flex-col h-full bg-[#1e1e1e] text-[#cccccc] font-mono text-sm border-t border-[#333333]">
      <div className="flex items-center justify-between px-4 py-2 bg-[#252526] border-b border-[#333333]">
        <div className="flex items-center gap-2">
          <TerminalIcon size={14} className="text-[#007acc]" />
          <span className="uppercase text-[11px] font-bold tracking-wider opacity-70">GENESIS OS</span>
        </div>
        <div className="flex items-center gap-4 text-[10px] opacity-50">
          <span>Port: 3000</span>
          <span>Status: Online</span>
        </div>
      </div>

      <div 
        ref={scrollRef}
        className="flex-1 overflow-y-auto p-4 space-y-4 scrollbar-thin scrollbar-thumb-[#333333]"
      >
        <AnimatePresence initial={false}>
          {messages.map((msg, i) => {
            const suggestedCode = msg.role === 'assistant' ? extractCode(msg.content) : null;
            
            return (
              <motion.div
                key={msg.timestamp + i}
                initial={{ opacity: 0, y: 10 }}
                animate={{ opacity: 1, y: 0 }}
                className={cn(
                  "flex gap-3 max-w-4xl",
                  msg.role === 'user' ? "ml-auto flex-row-reverse" : ""
                )}
              >
                <div className={cn(
                  "w-6 h-6 rounded flex items-center justify-center shrink-0 mt-1",
                  msg.role === 'assistant' ? "bg-[#007acc]/20 text-[#007acc]" : "bg-[#333333] text-[#cccccc]"
                )}>
                  {msg.role === 'assistant' ? <Bot size={14} /> : <User size={14} />}
                </div>
                <div className="flex flex-col gap-2 max-w-full overflow-hidden">
                  <div className={cn(
                    "p-3 rounded-lg leading-relaxed overflow-x-auto markdown-body",
                    msg.role === 'assistant' ? "bg-[#252526] border border-[#333333]" : "bg-[#007acc]/10 border border-[#007acc]/20"
                  )}>
                    <ReactMarkdown remarkPlugins={[remarkGfm]}>
                      {msg.content}
                    </ReactMarkdown>
                  </div>
                  {suggestedCode && onApplyCode && (
                    <div className="flex gap-2">
                      <button
                        onClick={() => onApplyCode(suggestedCode)}
                        className="flex items-center gap-1.5 px-3 py-1 bg-[#007acc] hover:bg-[#005a9e] text-white rounded text-[11px] transition-colors"
                      >
                        <Check size={12} />
                        Apply Changes
                      </button>
                      <button
                        onClick={() => navigator.clipboard.writeText(suggestedCode)}
                        className="flex items-center gap-1.5 px-3 py-1 bg-[#333333] hover:bg-[#444444] text-[#cccccc] rounded text-[11px] transition-colors"
                      >
                        <Copy size={12} />
                        Copy Code
                      </button>
                    </div>
                  )}
                </div>
              </motion.div>
            );
          })}
        </AnimatePresence>
        {isLoading && (
          <div className="flex gap-3 items-center text-[#007acc] animate-pulse">
            <Loader2 size={14} className="animate-spin" />
            <span>AI is thinking...</span>
          </div>
        )}
      </div>

      <div className="p-4 bg-[#1e1e1e] border-t border-[#333333]">
        <div className="flex gap-2 items-center bg-[#252526] border border-[#333333] rounded px-3 py-2 focus-within:border-[#007acc] transition-colors">
          <span className="text-[#007acc] font-bold">$</span>
          <input
            type="text"
            value={input}
            onChange={(e) => setInput(e.target.value)}
            onKeyDown={(e) => e.key === 'Enter' && handleSend()}
            placeholder="Ask AI to improve code..."
            className="flex-1 bg-transparent border-none outline-none text-[#cccccc] placeholder-[#555555]"
          />
          <button 
            onClick={handleSend}
            disabled={isLoading || !input.trim()}
            className="text-[#007acc] hover:text-[#4daafc] disabled:opacity-50 transition-colors"
          >
            <Send size={16} />
          </button>
        </div>
      </div>
    </div>
  );
}
