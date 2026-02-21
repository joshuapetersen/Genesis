import React, { useRef, useEffect } from 'react';

type Message = {
  text: string;
  sender: 'user' | 'sarah' | 'system';
};

interface Props {
  messages: Message[];
}

const TacticalReadout: React.FC<Props> = ({ messages }) => {
  const scrollRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (scrollRef.current) {
      scrollRef.current.scrollTo({
        top: scrollRef.current.scrollHeight,
        behavior: 'smooth'
      });
    }
  }, [messages]);

  return (
    <div className="vibe-chat-readout">
      <div className="chat-history-viewport" ref={scrollRef}>
        {messages.map((msg, i) => (
          <div key={i} className={`chat-bubble-row ${msg.sender}`}>
            <div className="chat-bubble">
              {msg.sender === 'system' && <div className="system-tag">PROTOCOL_EVENT ::</div>}
              {msg.sender === 'sarah' && <div className="sarah-tag">SOVEREIGN_RESONANCE</div>}
              <div className="message-content">{msg.text}</div>
            </div>
          </div>
        ))}
      </div>

      <style>{`
        .vibe-chat-readout {
          flex: 1;
          display: flex;
          flex-direction: column;
          overflow: hidden;
          padding: 40px 20px;
          position: relative;
        }

        .chat-history-viewport {
          flex: 1;
          overflow-y: auto;
          display: flex;
          flex-direction: column;
          gap: 20px;
          padding-right: 10px;
          mask-image: linear-gradient(to bottom, transparent, black 8%, black 92%, transparent);
        }

        .chat-bubble-row {
          width: 100%;
          display: flex;
          animation: slide-up 0.4s cubic-bezier(0.16, 1, 0.3, 1) forwards;
        }

        @keyframes slide-up {
          from { opacity: 0; transform: translateY(20px) scale(0.98); }
          to { opacity: 1; transform: translateY(0) scale(1); }
        }

        .chat-bubble-row.user   { justify-content: flex-end; }
        .chat-bubble-row.sarah { justify-content: flex-start; }
        .chat-bubble-row.system { justify-content: center; }

        .chat-bubble {
          max-width: 85%;
          padding: 16px 24px;
          border-radius: 28px;
          font-family: var(--font-sans);
          font-size: 1.05rem;
          line-height: 1.6;
          position: relative;
          transition: transform 0.3s ease;
        }

        .user .chat-bubble {
          background: var(--neon-cobalt);
          color: #000;
          font-weight: 600;
          border-bottom-right-radius: 4px;
          box-shadow: 0 8px 24px var(--neon-glow);
        }

        .sarah .chat-bubble {
          background: rgba(255, 255, 255, 0.04);
          color: #fff;
          border: 1px solid rgba(0, 127, 255, 0.25);
          border-bottom-left-radius: 4px;
          backdrop-filter: blur(15px);
          box-shadow: 0 4px 15px rgba(0, 0, 0, 0.3);
        }

        .system .chat-bubble {
          background: transparent;
          color: var(--neon-cobalt);
          font-family: var(--font-mono);
          font-size: 0.8rem;
          text-align: center;
          opacity: 0.6;
          letter-spacing: 1px;
          padding: 8px 16px;
        }

        .system-tag, .sarah-tag {
          font-family: var(--font-mono);
          font-size: 0.7rem;
          font-weight: 700;
          margin-bottom: 6px;
          letter-spacing: 2px;
          opacity: 0.7;
        }

        .sarah-tag { color: var(--neon-cobalt); text-shadow: 0 0 10px var(--neon-glow); }

        .message-content {
          word-break: break-word;
          white-space: pre-wrap;
        }

        /* Fluid Scrollbar */
        .chat-history-viewport::-webkit-scrollbar { width: 4px; }
        .chat-history-viewport::-webkit-scrollbar-thumb { 
          background: rgba(0, 127, 255, 0.3); 
          border-radius: 10px; 
        }
      `}</style>
    </div>
  );
};

export default TacticalReadout;
