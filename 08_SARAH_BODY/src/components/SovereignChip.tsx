import React from 'react';

const SovereignChip: React.FC = () => {
    return (
        <div className="chip-container">
            {/* Background Circuitry */}
            <div className="pcb-grid"></div>

            {/* Main Chip Body */}
            <div className="chip-body">
                <div className="chip-surface">
                    <div className="hex-g-logo">
                        <div className="hex-outline"></div>
                        <div className="g-glyph">G</div>
                    </div>

                    <div className="chip-branding">
                        <h1 className="sarah-text">SARAH</h1>
                        <div className="circuit-line"></div>
                    </div>

                    <div className="chip-details">
                        <span className="detail-line">GENESIS SOVEREIGN AI CORE</span>
                        <span className="detail-line">NEW WORLD PROJECT // 1.092777</span>
                    </div>

                    {/* Atomic Pulse overlaid */}
                    <div className="atomic-overlay">
                        <div className="atom-core"></div>
                        <div className="orbit orbit-1"></div>
                        <div className="orbit orbit-2"></div>
                        <div className="orbit orbit-3"></div>
                    </div>
                </div>

                {/* Metal Pins with Glow */}
                {[...Array(12)].map((_, i) => (
                    <React.Fragment key={i}>
                        <div className="pin pin-top" style={{ left: `${15 + i * 6}%` }}></div>
                        <div className="pin pin-bottom" style={{ left: `${15 + i * 6}%` }}></div>
                        <div className="pin pin-left" style={{ top: `${15 + i * 6}%` }}></div>
                        <div className="pin pin-right" style={{ top: `${15 + i * 6}%` }}></div>
                    </React.Fragment>
                ))}
            </div>

            <style>{`
        .chip-container {
          position: relative;
          width: 600px;
          height: 600px;
          display: flex;
          align-items: center;
          justify-content: center;
        }

        .pcb-grid {
          position: absolute;
          inset: 0;
          background-image: 
            radial-gradient(circle at center, transparent 0%, var(--pcb-bg) 70%);
          opacity: 0.5;
        }

        .chip-body {
          position: relative;
          width: 380px;
          height: 380px;
          background: #0a0a0a;
          border: 2px solid #222;
          border-radius: 4px;
          box-shadow: 
            0 30px 60px rgba(0,0,0,0.8),
            inset 0 0 40px rgba(0, 245, 255, 0.05);
          display: flex;
          align-items: center;
          justify-content: center;
        }

        .chip-surface {
          width: 85%;
          height: 85%;
          background: radial-gradient(circle at center, #1a1a1a, #050505);
          border: 1px solid #333;
          display: flex;
          flex-direction: column;
          align-items: center;
          justify-content: center;
          gap: 25px;
          position: relative;
          padding: 20px;
        }

        .hex-g-logo {
          position: relative;
          width: 80px;
          height: 80px;
          display: flex;
          align-items: center;
          justify-content: center;
          margin-bottom: -10px;
        }

        .hex-outline {
          position: absolute;
          width: 100%;
          height: 100%;
          background: var(--pcb-trace);
          clip-path: polygon(50% 0%, 100% 25%, 100% 75%, 50% 100%, 0% 75%, 0% 25%);
          opacity: 0.2;
          animation: core-pulse 2s infinite;
        }

        .g-glyph {
          font-family: var(--font-accent);
          font-size: 2.5rem;
          color: var(--pcb-trace);
          font-weight: bold;
          text-shadow: 0 0 15px var(--pcb-trace);
        }

        .sarah-text {
          font-family: var(--font-accent);
          font-size: 4rem;
          color: #fff;
          letter-spacing: 12px;
          text-shadow: 0 0 30px var(--pcb-trace);
          margin: 0;
          background: linear-gradient(to bottom, #fff, var(--pcb-trace));
          -webkit-background-clip: text;
          -webkit-text-fill-color: transparent;
        }

        .circuit-line {
          height: 1px;
          width: 150px;
          background: linear-gradient(90deg, transparent, var(--pcb-trace), transparent);
          box-shadow: 0 0 10px var(--pcb-trace);
          margin-top: 5px;
        }

        .atomic-overlay {
          position: absolute;
          top: 20px;
          right: 20px;
          width: 40px;
          height: 40px;
          opacity: 0.6;
        }

        .atom-core {
          position: absolute;
          top: 50%; left: 50%;
          width: 6px; height: 6px;
          background: #fff;
          border-radius: 50%;
          transform: translate(-50%, -50%);
          box-shadow: 0 0 10px var(--pcb-trace);
        }

        .orbit {
          position: absolute;
          top: 0; left: 0; right: 0; bottom: 0;
          border: 1px solid var(--pcb-trace);
          border-radius: 50%;
          opacity: 0.3;
        }
        .orbit-1 { transform: rotateX(70deg) rotateY(10deg); animation: spin 3s infinite linear; }
        .orbit-2 { transform: rotateX(-70deg) rotateY(20deg); animation: spin 4s reverse linear; }
        .orbit-3 { transform: rotateY(70deg); animation: spin 5s infinite linear; }

        .chip-details {
          display: flex;
          flex-direction: column;
          align-items: center;
          gap: 8px;
        }

        .detail-line {
          font-family: var(--font-accent);
          font-size: 0.7rem;
          color: rgba(0, 245, 255, 0.5);
          letter-spacing: 3px;
        }

        .pin {
          position: absolute;
          background: linear-gradient(to bottom, #444, #222);
          box-shadow: inset 0 0 2px rgba(255,255,255,0.1);
        }

        .pin-top, .pin-bottom { width: 4px; height: 15px; }
        .pin-left, .pin-right { width: 15px; height: 4px; }

        .pin-top { top: -15px; }
        .pin-bottom { bottom: -15px; }
        .pin-left { left: -15px; }
        .pin-right { right: -15px; }

        @keyframes core-pulse {
          0%, 100% { opacity: 0.2; transform: scale(1); }
          50% { opacity: 0.4; transform: scale(1.05); }
        }

        @keyframes spin {
          from { transform: rotate(0deg); }
          to { transform: rotate(360deg); }
        }
      `}</style>
        </div>
    );
};

export default SovereignChip;
