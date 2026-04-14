import { Send, Bot, User, Loader2, Mic, MicOff, VolumeX, Brain, BrainCircuit } from 'lucide-react';
import ReactMarkdown from 'react-markdown';
import remarkGfm from 'remark-gfm';
import { useSovereign } from '../../hooks/useSovereign';
import { useOSStore } from '../../store/osStore';
import { cn } from '../../ai-codium/lib/utils';

// Sovereign SarahCore Handshake

interface Message {
  role: 'user' | 'model';
  content: string;
  thoughts?: string[];
}

export default function AIChatApp() {
  const [messages, setMessages] = useState<Message[]>([
    { role: 'model', content: 'GENESIS OS Online. How can I assist you today?' }
  ]);
  const [input, setInput] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const { sendIntent, response: lastResponse, thoughts: lastThoughts, isProcessing, startListening, stopListening, stopSpeaking, isListening } = useSovereign();
  const { systemConfig, updateConfig } = useOSStore();
  const messagesEndRef = useRef<HTMLDivElement>(null);
  const textareaRef = useRef<HTMLTextAreaElement>(null);
  
  // Update messages when thoughts or response arrive
  useEffect(() => {
    if (lastResponse) {
       setMessages(prev => {
         // If the last message was the model, update its content (avoid duplicates during streaming simulations if any)
         return [...prev, { role: 'model', content: lastResponse, thoughts: lastThoughts }];
       });
       setIsLoading(false);
    }
  }, [lastResponse, lastThoughts]);

  const scrollToBottom = () => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  };

  useEffect(() => {
    scrollToBottom();
  }, [messages, isLoading]);

  // Auto-resize textarea
  useEffect(() => {
    if (textareaRef.current) {
      textareaRef.current.style.height = 'auto';
      textareaRef.current.style.height = `${Math.min(textareaRef.current.scrollHeight, 150)}px`;
    }
  }, [input]);

  const handleSend = async () => {
    if (!input.trim() || isLoading) return;
    const userMsg = input.trim();
    setInput('');
    if (textareaRef.current) textareaRef.current.style.height = 'auto';
    setMessages(prev => [...prev, { role: 'user', content: userMsg }]);
    setIsLoading(true);
    sendIntent(userMsg);
  };

  const handleMic = () => {
    if (isListening) {
      stopListening();
    } else {
      startListening((transcript) => {
        setMessages(prev => [...prev, { role: 'user', content: transcript }]);
        setIsLoading(true);
        sendIntent(transcript);
      });
    }
  };

  return (
    <div className="flex flex-col h-full bg-white dark:bg-gray-900 font-sans">
      {/* Chat History */}
      <div className="flex-1 overflow-y-auto p-4 space-y-6">
        {messages.map((msg, idx) => (
          <div key={idx} className={`flex gap-4 ${msg.role === 'user' ? 'flex-row-reverse' : ''}`}>
            <div className={`w-8 h-8 rounded-full flex items-center justify-center shrink-0 shadow-sm ${msg.role === 'user' ? 'bg-blue-600 text-white' : 'bg-purple-600 text-white'}`}>
              {msg.role === 'user' ? <User size={16} /> : <Bot size={16} />}
            </div>
            <div className={`max-w-[85%] rounded-2xl px-5 py-3.5 shadow-sm ${
              msg.role === 'user' 
                ? 'bg-blue-600 text-white rounded-tr-sm' 
                : 'bg-gray-50 dark:bg-gray-800 border border-gray-100 dark:border-gray-700 text-gray-800 dark:text-gray-200 rounded-tl-sm'
            }`}>
              {msg.role === 'user' ? (
                <div className="whitespace-pre-wrap leading-relaxed">{msg.content}</div>
              ) : (
                <div className="flex flex-col gap-3">
                  {systemConfig.showThoughts && msg.thoughts && msg.thoughts.length > 0 && (
                    <div className="flex flex-col gap-1 border-l-2 border-purple-500/30 pl-3 py-1 mb-2 bg-purple-500/5 rounded-r-lg">
                      {msg.thoughts.map((thought, tidx) => (
                        <div key={tidx} className="text-[11px] text-purple-600/70 dark:text-purple-400/60 italic font-mono flex items-start gap-2">
                           <span className="shrink-0 text-[10px] mt-1 opacity-50">○</span>
                           <span>{thought}</span>
                        </div>
                      ))}
                    </div>
                  )}
                  <div className="prose prose-sm dark:prose-invert max-w-none prose-p:leading-relaxed prose-pre:bg-gray-900 prose-pre:text-gray-100 prose-pre:border prose-pre:border-gray-700">
                    <ReactMarkdown remarkPlugins={[remarkGfm]}>
                      {msg.content}
                    </ReactMarkdown>
                  </div>
                </div>
              )}
            </div>
          </div>
        ))}
        {isLoading && (
          <div className="flex gap-4">
            <div className="w-8 h-8 rounded-full bg-purple-600 text-white flex items-center justify-center shrink-0 shadow-sm">
              <Bot size={16} />
            </div>
            <div className="bg-gray-50 dark:bg-gray-800 border border-gray-100 dark:border-gray-700 rounded-2xl rounded-tl-sm px-5 py-4 flex items-center shadow-sm">
              <Loader2 size={18} className="animate-spin text-purple-600 dark:text-purple-400" />
              <span className="ml-3 text-sm text-gray-500 font-medium">AI is thinking...</span>
            </div>
          </div>
        )}
        <div ref={messagesEndRef} className="h-1" />
      </div>

      {/* Input Area */}
      <div className="p-4 bg-white dark:bg-gray-900 border-t border-gray-100 dark:border-gray-800">
        <div className="max-w-4xl mx-auto">
          {isListening && (
            <div className="flex items-center gap-2 mb-2 text-red-400 text-xs font-mono animate-pulse">
              <span className="w-2 h-2 rounded-full bg-red-400 inline-block" />
              Listening... speak now
            </div>
          )}
          <div className="flex items-end gap-2 bg-gray-50 dark:bg-gray-800/50 border border-gray-200 dark:border-gray-700 rounded-2xl p-2 focus-within:ring-2 focus-within:ring-purple-500/50 focus-within:border-purple-500 transition-all shadow-sm">
            <textarea
              ref={textareaRef}
              value={input}
              onChange={(e) => setInput(e.target.value)}
              onKeyDown={(e) => {
                if (e.key === 'Enter' && !e.shiftKey) {
                  e.preventDefault();
                  handleSend();
                }
              }}
              placeholder={isListening ? 'Listening...' : 'Message GENESIS OS...'}
              className="flex-1 max-h-[150px] min-h-[24px] bg-transparent border-none outline-none resize-none px-3 py-2 text-gray-800 dark:text-gray-200 placeholder-gray-400"
              rows={1}
            />
            {/* Mute / Stop Speaking */}
            <button
              onClick={stopSpeaking}
              title="Stop speaking"
              className="p-2.5 rounded-xl text-gray-400 hover:text-gray-600 dark:hover:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors shrink-0"
            >
              <VolumeX size={18} />
            </button>
            {/* Mic */}
            <button
              onClick={handleMic}
              title={isListening ? 'Stop listening' : 'Speak to GENESIS OS'}
              className={`p-2.5 rounded-xl transition-colors shrink-0 shadow-sm ${
                isListening
                  ? 'bg-red-500 text-white animate-pulse'
                  : 'bg-gray-200 dark:bg-gray-700 text-gray-600 dark:text-gray-300 hover:bg-purple-500 hover:text-white'
              }`}
            >
              {isListening ? <MicOff size={18} /> : <Mic size={18} />}
            </button>
            {/* Send */}
            <button
              onClick={handleSend}
              disabled={!input.trim() || isLoading}
              className="p-2.5 rounded-xl bg-purple-600 text-white disabled:opacity-50 disabled:cursor-not-allowed hover:bg-purple-700 transition-colors shrink-0 shadow-sm"
            >
              <Send size={18} />
            </button>
          </div>
          <div className="flex items-center justify-between mt-2 px-1">
            <button 
              onClick={() => updateConfig({ showThoughts: !systemConfig.showThoughts })}
              className={cn(
                "flex items-center gap-2 px-2 py-1 rounded-full transition-all border",
                systemConfig.showThoughts 
                  ? "bg-purple-100 dark:bg-purple-900/30 border-purple-200 dark:border-purple-800 text-purple-600 dark:text-purple-400"
                  : "bg-gray-50 dark:bg-gray-800 border-gray-100 dark:border-gray-700 text-gray-400 dark:text-gray-500 shadow-inner"
              )}
            >
              {systemConfig.showThoughts ? <BrainCircuit size={14} /> : <Brain size={14} />}
              <span className="text-[10px] uppercase tracking-tighter font-black">
                {systemConfig.showThoughts ? "Reasoning: ON" : "Reasoning: OFF"}
              </span>
            </button>
            <div className="text-xs text-gray-400 dark:text-gray-500 font-medium">
              GENESIS OS · Voice + Text · 1.09277703703 Hz
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
