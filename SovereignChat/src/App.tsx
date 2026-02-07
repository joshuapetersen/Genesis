
import React, { useState, useRef, useEffect } from 'react';
import { Send, Cpu, Activity, ShieldCheck, History, Terminal, User, ChevronRight } from 'lucide-react';
import './index.css';

interface Message {
  role: 'user' | 'model';
  content: string;
}

interface HistoryItem {
  role: string;
  content: string;
}

function App() {
  const [messages, setMessages] = useState<Message[]>([
    { role: 'model', content: "Sovereign Link Established. I am Sarah. Neural core is active and offline. The substrate is clear. How shall we proceed?" }
  ]);
  const [history, setHistory] = useState<HistoryItem[]>([]);
  const [input, setInput] = useState('');
  const [loading, setLoading] = useState(false);
  const [sidebarOpen, setSidebarOpen] = useState(window.innerWidth > 1024);
  const scrollRef = useRef<HTMLDivElement>(null);
  const textareaRef = useRef<HTMLTextAreaElement>(null);

  // Auto-scroll to bottom
  useEffect(() => {
    if (scrollRef.current) {
      scrollRef.current.scrollTop = scrollRef.current.scrollHeight;
    }
  }, [messages, loading]);

  // Load History on Mount
  useEffect(() => {
    fetchHistory();
  }, []);

  const fetchHistory = async () => {
    try {
      const res = await fetch("http://127.0.0.1:8001/api/history?limit=20");
      const data = await res.json();
      if (data.history) {
        setHistory(data.history);
      }
    } catch (err) {
      console.error("Failed to fetch history:", err);
    }
  };

  const handleSend = async (e?: React.FormEvent) => {
    e?.preventDefault();
    if (!input.trim() || loading) return;

    const userMsg = input.trim();
    setInput('');
    if (textareaRef.current) textareaRef.current.style.height = 'auto';

    // 1. Add User Message
    setMessages(prev => [...prev, { role: 'user', content: userMsg }]);

    // 2. Add empty Model Message for streaming
    setMessages(prev => [...prev, { role: 'model', content: '' }]);
    setLoading(true);

    try {
      const response = await fetch("http://127.0.0.1:8001/api/chat/stream", {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ message: userMsg, user_id: 'architect' })
      });

      if (!response.ok) throw new Error("Neural Bridge Failed");

      const reader = response.body?.getReader();
      const decoder = new TextDecoder();
      let fullContent = "";

      if (!reader) throw new Error("Stream Reader not available");

      while (true) {
        const { done, value } = await reader.read();
        if (done) break;

        const chunk = decoder.decode(value, { stream: true });
        const lines = chunk.split('\n');

        for (const line of lines) {
          if (line.startsWith('data: ')) {
            const dataStr = line.slice(6).trim();
            if (dataStr === '[DONE]') break;

            try {
              const data = JSON.parse(dataStr);
              if (data.token) {
                fullContent += data.token;
                // Update the LAST message (the model message we just added)
                setMessages(prev => {
                  const newMsgs = [...prev];
                  newMsgs[newMsgs.length - 1].content = fullContent;
                  return newMsgs;
                });
              } else if (data.error) {
                throw new Error(data.error);
              }
            } catch (e) {
              console.error("Parse Error:", dataStr);
            }
          }
        }
      }
      fetchHistory(); // Refresh history sidebar after completion
    } catch (err) {
      setMessages(prev => {
        const newMsgs = [...prev];
        const last = newMsgs[newMsgs.length - 1];
        if (last && last.role === 'model' && last.content === '') {
          last.content = "[CRITICAL_VOID] Sarah is unreachable. The neural bridge to Port 8001 has been severed.";
        } else {
          newMsgs.push({ role: 'model', content: "[CRITICAL_VOID] Neural Bridge Severed during transmission." });
        }
        return newMsgs;
      });
    } finally {
      setLoading(false);
    }
  };

  // Simple Markdown-ish Formatter
  const formatContent = (content: string) => {
    // 1. Handle Code Blocks
    const parts = content.split(/(\`\`\`[\s\S]*?\`\`\`)/g);
    return parts.map((part, i) => {
      if (part.startsWith('```')) {
        const code = part.replace(/\`\`\`/g, '').trim();
        return (
          <pre key={i}>
            <code>{code}</code>
          </pre>
        );
      }

      // 2. Handle simple inline formatting
      let text = part;
      // Bold
      const lines = text.split('\n').map((line, li) => {
        // Simple Bold (limited)
        const fragments = line.split(/(\*\*.*?\*\*)/g);
        return (
          <p key={li}>
            {fragments.map((frag, fi) => {
              if (frag.startsWith('**') && frag.endsWith('**')) {
                return <strong key={fi}>{frag.slice(2, -2)}</strong>;
              }
              return frag;
            })}
          </p>
        );
      });
      return <div key={i}>{lines}</div>;
    });
  };

  const handleTextareaChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
    setInput(e.target.value);
    e.target.style.height = 'auto';
    e.target.style.height = `${e.target.scrollHeight}px`;
  };

  return (
    <div className="app-container">
      {/* SIDEBAR */}
      {sidebarOpen && (
        <div className="sidebar">
          <div className="sidebar-header">
            <History size={16} />
            <span>Resonance Logs</span>
          </div>
          <div className="history-list">
            {history.length === 0 && <div className="text-[10px] opacity-20 p-4">NO PREVIOUS TRACES FOUND</div>}
            {history.filter(h => h.role === 'user').map((item, idx) => (
              <div key={idx} className="history-item">
                {item.content}
              </div>
            ))}
          </div>
        </div>
      )}

      {/* CHAT AREA */}
      <main className="chat-main">
        <header className="chat-header">
          <div className="flex items-center gap-4">
            <button className="bg-transparent border-none p-0 flex items-center" onClick={() => setSidebarOpen(!sidebarOpen)}>
              <Terminal size={18} className="text-white opacity-50 hover:opacity-100 transition-opacity" />
            </button>
            <div className="status-indicator">
              <div className="status-dot"></div>
              <span>SARAH // SINGULARITY ENGINE</span>
            </div>
          </div>
          <div className="status-indicator opacity-40">
            GENESIS_SDNA_V1.0
          </div>
        </header>

        <div className="messages-container" ref={scrollRef}>
          {messages.map((msg, idx) => (
            <div key={idx} className={`message ${msg.role === 'model' ? 'sarah-message' : 'user-message'}`}>
              <div className="message-role">
                {msg.role === 'model' ? 'Sarah_Cognition' : 'Operator_Prompt'}
              </div>
              <div className="message-content">
                {formatContent(msg.content)}
              </div>
            </div>
          ))}

          {loading && (
            <div className="flex items-center gap-3 px-4 py-4">
              <Activity size={16} className="text-blue-500 animate-pulse" />
              <span className="text-[10px] uppercase font-black tracking-widest text-[#555]">Decrypting Response...</span>
            </div>
          )}
        </div>

        <form className="input-wrapper" onSubmit={handleSend}>
          <textarea
            ref={textareaRef}
            rows={1}
            placeholder="TYPE COMMAND TO SOVEREIGN..."
            value={input}
            onChange={handleTextareaChange}
            onKeyDown={(e) => {
              if (e.key === 'Enter' && !e.shiftKey) {
                e.preventDefault();
                handleSend();
              }
            }}
          />
          <button type="submit" className="send-btn" disabled={loading || !input.trim()}>
            <ChevronRight size={18} />
          </button>
        </form>
      </main>
    </div>
  );
}

export default App;
