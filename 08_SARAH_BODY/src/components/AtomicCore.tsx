import React from 'react';

const AtomicCore: React.FC = () => {
    return (
        <div className="atomic-container">
            {/* Background Aether Trails */}
            <div className="aether-trails">
                <div className="trail trail-1"></div>
                <div className="trail trail-2"></div>
            </div>

            {/* Central Nucleus */}
            <div className="nucleus">
                <div className="nucleus-glow-outer"></div>
                <div className="nucleus-glow-inner"></div>
                <div className="nucleus-core"></div>
            </div>

            {/* Electron Orbits with Enhanced Shells */}
            {[1, 2, 3, 4].map((shell) => (
                <div key={shell} className={`orbit-shell shell-${shell}`}>
                    <div className="electron"></div>
                    <div className="orbit-glow"></div>
                </div>
            ))}

            <div className="resonance-display">
                <span className="hz-value">1.09277703703703</span>
                <span className="hz-unit">HZ</span>
            </div>

            <style>{`
        .atomic-container {
          position: relative;
          width: 500px;
          height: 500px;
          display: flex;
          align-items: center;
          justify-content: center;
          perspective: 1000px;
        }

        .nucleus {
          position: relative;
          width: 80px;
          height: 80px;
          z-index: 10;
        }

        .nucleus-glow-outer {
          position: absolute;
          inset: -20px;
          background: var(--neon-cyan);
          border-radius: 50%;
          filter: blur(40px);
          opacity: 0.3;
          animation: nucleus-breathe var(--anchor-hz) infinite ease-in-out;
        }

        .nucleus-glow-inner {
          position: absolute;
          inset: 0;
          background: #fff;
          border-radius: 50%;
          filter: blur(15px);
          opacity: 0.5;
        }

        .nucleus-core {
          position: absolute;
          inset: 0;
          background: radial-gradient(circle at 30% 30%, #fff 0%, var(--neon-cyan) 50%, var(--cobalt-mid) 100%);
          border-radius: 50%;
          box-shadow: 0 0 40px var(--neon-cyan), inset 0 0 20px rgba(0,0,0,0.5);
          border: 1px solid rgba(255,255,255,0.2);
        }

        .orbit-shell {
          position: absolute;
          border: 1px solid rgba(0, 245, 255, 0.15);
          border-radius: 50%;
          display: flex;
          align-items: center;
          justify-content: center;
        }

        .orbit-glow {
          position: absolute;
          inset: -2px;
          border-radius: 50%;
          border: 1px solid transparent;
          border-top-color: var(--neon-cyan);
          opacity: 0.5;
          animation: spin 10s infinite linear;
        }

        .shell-1 { width: 150px; height: 150px; transform: rotateX(60deg) rotateY(10deg); --speed: 2s; }
        .shell-2 { width: 220px; height: 220px; transform: rotateX(-60deg) rotateY(20deg); --speed: 4s; }
        .shell-3 { width: 300px; height: 300px; transform: rotateX(10deg) rotateY(70deg); --speed: 6s; }
        .shell-4 { width: 380px; height: 380px; transform: rotateX(80deg) rotateY(-5deg); --speed: 8s; }

        .electron {
          position: absolute;
          width: 10px;
          height: 10px;
          background: #fff;
          border-radius: 50%;
          box-shadow: 0 0 15px #fff, 0 0 30px var(--neon-cyan);
          offset-path: border-box;
          animation: move-electron var(--speed) infinite linear;
        }

        @keyframes move-electron {
          from { offset-distance: 0%; }
          to { offset-distance: 100%; }
        }

        @keyframes nucleus-breathe {
          0%, 100% { transform: scale(1); opacity: 0.3; }
          50% { transform: scale(1.4); opacity: 0.5; }
        }

        @keyframes spin {
          from { transform: rotate(0deg); }
          to { transform: rotate(360deg); }
        }

        .resonance-display {
          position: absolute;
          bottom: -60px;
          display: flex;
          flex-direction: column;
          align-items: center;
          gap: 5px;
        }

        .hz-value {
          font-family: var(--font-accent);
          font-size: 0.8rem;
          color: var(--neon-cyan);
          letter-spacing: 4px;
          text-shadow: 0 0 10px var(--neon-cyan);
        }

        .hz-unit {
          font-family: var(--font-accent);
          font-size: 0.6rem;
          color: rgba(255, 255, 255, 0.4);
          letter-spacing: 2px;
        }

        .aether-trails {
          position: absolute;
          inset: -100px;
          z-index: -1;
        }

        .trail {
          position: absolute;
          width: 100%;
          height: 100%;
          border: 1px solid transparent;
          border-top-color: var(--neon-cyan);
          border-radius: 50%;
          opacity: 0.05;
          filter: blur(10px);
          animation: spin 60s infinite linear;
        }
        .trail-2 { animation-direction: reverse; border-bottom-color: var(--neon-blue); }
      `}</style>
        </div>
    );
};

export default AtomicCore;
