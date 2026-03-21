import React from 'react';

const GenesisAtom: React.FC = () => {
    return (
        <div className="genesis-atom-viewport">
            <div className="atom-core-group">
                {/* Stylized 3D G */}
                <div className="genesis-letter-3d">
                    <div className="face front">G</div>
                    <div className="face back">G</div>
                    <div className="face left"></div>
                    <div className="face right"></div>
                    <div className="face top"></div>
                    <div className="face bottom"></div>
                </div>

                {/* Core Energy Aura */}
                <div className="core-resonance-glow"></div>
            </div>

            {/* Electron Shells */}
            <div className="atomic-shell shell-alpha">
                <div className="electron-node"></div>
            </div>
            <div className="atomic-shell shell-beta">
                <div className="electron-node"></div>
            </div>
            <div className="atomic-shell shell-gamma">
                <div className="electron-node"></div>
            </div>

            <style>{`
        .genesis-atom-viewport {
          position: relative;
          width: 500px;
          height: 500px;
          display: flex;
          align-items: center;
          justify-content: center;
          transform-style: preserve-3d;
          animation: viewport-drift 20s infinite ease-in-out;
        }

        @keyframes viewport-drift {
          0%, 100% { transform: translateY(0) rotateX(0deg) rotateY(0deg); }
          50% { transform: translateY(-30px) rotateX(10deg) rotateY(15deg); }
        }

        .atom-core-group {
          position: relative;
          width: 120px;
          height: 120px;
          transform-style: preserve-3d;
          animation: core-spin 12s infinite linear;
        }

        @keyframes core-spin {
          from { transform: rotateY(0deg) rotateX(15deg); }
          to { transform: rotateY(360deg) rotateX(15deg); }
        }

        .genesis-letter-3d {
          position: absolute;
          inset: 0;
          transform-style: preserve-3d;
        }

        .face {
          position: absolute;
          width: 100%;
          height: 100%;
          display: flex;
          align-items: center;
          justify-content: center;
          font-family: var(--font-sans);
          font-weight: 900;
          font-size: 5rem;
          color: var(--neon-cobalt);
          text-shadow: 0 0 30px var(--neon-glow);
          background: rgba(0, 127, 255, 0.03);
          border: 1.5px solid var(--neon-cobalt);
          backface-visibility: hidden;
        }

        .front  { transform: translateZ(20px); }
        .back   { transform: rotateY(180deg) translateZ(20px); }
        .left   { transform: rotateY(-90deg) translateZ(60px); width: 40px; }
        .right  { transform: rotateY(90deg) translateZ(60px); width: 40px; }
        .top    { transform: rotateX(90deg) translateZ(60px); height: 40px; }
        .bottom { transform: rotateX(-90deg) translateZ(60px); height: 40px; }

        .core-resonance-glow {
          position: absolute;
          inset: -40px;
          background: radial-gradient(circle, var(--neon-glow) 0%, transparent 70%);
          filter: blur(40px);
          animation: resonance-pulse 4s infinite alternate ease-in-out;
          pointer-events: none;
        }

        @keyframes resonance-pulse {
          from { opacity: 0.2; transform: scale(1); }
          to { opacity: 0.5; transform: scale(1.4); }
        }

        .atomic-shell {
          position: absolute;
          border: 1px solid rgba(0, 127, 255, 0.15);
          border-radius: 50%;
          transform-style: preserve-3d;
          pointer-events: none;
        }

        .shell-alpha { width: 220px; height: 220px; transform: rotateX(85deg); animation: shell-orbit-1 10s infinite linear; }
        .shell-beta  { width: 340px; height: 340px; transform: rotateX(-85deg) rotateY(30deg); animation: shell-orbit-2 18s infinite linear; }
        .shell-gamma { width: 480px; height: 480px; transform: rotateY(45deg) rotateX(45deg); animation: shell-orbit-3 30s infinite linear; }

        .electron-node {
          position: absolute;
          top: 0;
          left: 50%;
          width: 12px;
          height: 12px;
          background: #fff;
          border-radius: 50%;
          box-shadow: 0 0 20px #fff, 0 0 40px var(--neon-cobalt);
          margin-left: -6px;
          margin-top: -6px;
          transform: translateZ(0);
        }

        @keyframes shell-orbit-1 { from { transform: rotateX(85deg) rotateZ(0deg); } to { transform: rotateX(85deg) rotateZ(360deg); } }
        @keyframes shell-orbit-2 { from { transform: rotateX(-85deg) rotateY(30deg) rotateZ(360deg); } to { transform: rotateX(-85deg) rotateY(30deg) rotateZ(0deg); } }
        @keyframes shell-orbit-3 { from { transform: rotateY(45deg) rotateX(45deg) rotateZ(0deg); } to { transform: rotateY(45deg) rotateX(45deg) rotateZ(360deg); } }
      `}</style>
        </div>
    );
};

export default GenesisAtom;
