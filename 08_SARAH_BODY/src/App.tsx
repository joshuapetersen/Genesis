import React, { useState, useEffect } from 'react';
import TacticalReadout from './components/TacticalReadout';
import CouncilViz from './components/CouncilViz';
import GenesisAtom from './components/GenesisAtom';
import { gateway } from './lib/GatewayClient';

const App: React.FC = () => {
  const [messages, setMessages] = useState<any[]>([
    { text: "SOVEREIGN OS v5.0 // NEON_ABSOLUTE", sender: "system" },
    { text: "GENESIS_CORE_RESONANCE :: STABLE", sender: "system" }
  ]);

  useEffect(() => {
    gateway.connect().catch(console.error);

    gateway.onEvent('agent.thinking', (payload: any) => {
      setMessages(prev => [...prev.slice(-49), { text: `THINKING: ${payload.status || 'PROCESSING...'}`, sender: 'system' }]);
    });

    gateway.onEvent('agent.response', (payload: any) => {
      setMessages(prev => [...prev.slice(-49), { text: payload.text, sender: 'sarah' }]);
    });
  }, []);

  const onCommand = async (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === 'Enter') {
      const val = e.currentTarget.value;
      if (!val) return;
      setMessages(prev => [...prev.slice(-49), { text: val, sender: 'user' }]);
      e.currentTarget.value = '';

      try {
        await gateway.request('agent.chat', { message: val });
      } catch (err) {
        setMessages(prev => [...prev, { text: "CONNECTION_INTERRUPTED", sender: "system" }]);
      }
    }
  };

  return (
    <div className="sovereign-viewport-3d">
      {/* Background Orbital Plane (Council) */}
      <div className="layer plane-background">
        <CouncilViz />
      </div>

      {/* Midground Resonance Core (Genesis Atom) */}
      <div className="layer plane-core">
        <div className="atom-anchor">
          <GenesisAtom />
        </div>
      </div>

      {/* Foreground Interaction Hub (Chat) */}
      <main className="layer plane-ui">
        <div className="sovereign-chat-frame fui-glass fui-bracket fui-bracket-tl fui-bracket-tr fui-bracket-bl fui-bracket-br">
          <div className="fui-scanline"></div>

          <TacticalReadout messages={messages} />

          <div className="vibe-input-wrapper">
            <div className="input-glow-bar">
              <div className="os-label">SARAH_OS//</div>
              <input
                type="text"
                autoFocus
                placeholder="VIBE_CODE_ORCHESTRATION..."
                onKeyDown={onCommand}
                onBlur={(e) => e.target.focus()}
              />
              <div className="status-pulse"></div>
            </div>
          </div>
        </div>
      </main>

      <style>{`
        .sovereign-viewport-3d {
          width: 100vw;
          height: 100vh;
          overflow: hidden;
          background: #000;
          display: flex;
          align-items: center;
          justify-content: center;
          position: relative;
          transform-style: preserve-3d;
        }

        .layer {
          position: absolute;
          display: flex;
          align-items: center;
          justify-content: center;
          pointer-events: none;
        }

        .plane-background {
          transform: translateZ(-300px) scale(1.8);
          opacity: 0.3;
          z-index: 1;
        }

        .plane-core {
          transform: translateZ(0);
          z-index: 5;
        }

        .plane-ui {
          position: relative;
          width: 1100px;
          height: 85vh;
          transform: translateZ(300px);
          pointer-events: all;
          z-index: 10;
        }

        .sovereign-chat-frame {
          width: 100%;
          height: 100%;
          display: flex;
          flex-direction: column;
          overflow: hidden;
        }

        .vibe-input-wrapper {
          padding: 30px;
          background: rgba(0, 0, 0, 0.4);
          border-top: 1px solid rgba(0, 127, 255, 0.1);
        }

        .input-glow-bar {
          display: flex;
          align-items: center;
          gap: 20px;
          padding: 14px 28px;
          background: rgba(0, 127, 255, 0.04);
          border: 1.5px solid rgba(0, 127, 255, 0.3);
          border-radius: 20px;
          box-shadow: 0 0 30px rgba(0, 127, 255, 0.05), inset 0 0 10px rgba(0, 127, 255, 0.05);
          position: relative;
        }

        .os-label {
          font-family: var(--font-mono);
          color: var(--neon-cobalt);
          font-weight: 700;
          font-size: 0.85rem;
          letter-spacing: 3px;
          text-shadow: 0 0 10px var(--neon-glow);
        }

        input {
          flex: 1;
          background: transparent;
          border: none;
          color: #fff;
          font-family: var(--font-mono);
          font-size: 1.2rem;
          outline: none;
          letter-spacing: 1px;
        }

        input::placeholder {
          color: rgba(0, 127, 255, 0.2);
        }

        .status-pulse {
          width: 8px;
          height: 8px;
          background: var(--neon-cobalt);
          border-radius: 50%;
          box-shadow: 0 0 15px var(--neon-cobalt);
          animation: status-breathe 2s infinite ease-in-out;
        }

        @keyframes status-breathe {
          0%, 100% { transform: scale(0.8); opacity: 0.5; }
          50% { transform: scale(1.5); opacity: 1; }
        }
      `}</style>
    </div>
  );
};

export default App;
