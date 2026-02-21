import React from 'react';

const GeneticChip: React.FC = () => {
  return (
    <div className="genetic-hub sovereign-breathe">
      {/* Circular Viewfinder (Rotating degree markers) */}
      <div className="viewfinder-container">
        <div className="ring ring-outer"></div>
        <div className="ring ring-middle"></div>
        <div className="degree-markers">
          {[...Array(12)].map((_, i) => (
            <div key={i} className="marker" style={{ transform: `rotate(${i * 30}deg)` }}></div>
          ))}
        </div>
      </div>

      {/* Background Synapse Traces (Organic Circuitry) */}
      <div className="pcb-trace-layer">
        <svg width="100%" height="100%" viewBox="0 0 800 800" xmlns="http://www.w3.org/2000/svg">
          <defs>
            <filter id="glow">
              <feGaussianBlur stdDeviation="3" result="coloredBlur" />
              <feMerge>
                <feMergeNode in="coloredBlur" />
                <feMergeNode in="SourceGraphic" />
              </feMerge>
            </filter>
          </defs>
          <g filter="url(#glow)">
            {/* Synapse Branching Paths */}
            <path className="synapse-path" d="M 400 400 Q 450 300 400 200 T 400 0" stroke="rgba(0, 245, 255, 0.2)" strokeWidth="1" fill="none" />
            <path className="synapse-path" d="M 400 400 Q 500 450 600 400 T 800 400" stroke="rgba(0, 245, 255, 0.2)" strokeWidth="1" fill="none" />
            <path className="synapse-path" d="M 400 400 Q 350 500 400 600 T 400 800" stroke="rgba(0, 245, 255, 0.2)" strokeWidth="1" fill="none" />
            <path className="synapse-path" d="M 400 400 Q 300 350 200 400 T 0 400" stroke="rgba(0, 245, 255, 0.2)" strokeWidth="1" fill="none" />

            <circle cx="400" cy="400" r="160" fill="none" stroke="rgba(0, 245, 255, 0.08)" strokeWidth="1" strokeDasharray="5,10" />
          </g>
        </svg>
      </div>

      {/* Main Processor Chip - Neural Core */}
      <div className="sovereign-processor fui-glass">
        <div className="chip-surface">
          {/* The SARAH Engraving */}
          <div className="engraving-container">
            <h1 className="sarah-branding">SARAH</h1>
            <div className="chip-status">
              <span className="status-label">SYS_RES:</span>
              <span className="status-value">OPTIMAL</span>
            </div>
          </div>

          {/* The Pulse Core */}
          <div className="pulse-core">
            <div className="core-glow"></div>
            <div className="core-inner"></div>
          </div>

          <div className="chip-subtitle">Ω_NODE_01</div>
        </div>
      </div>

      <style>{`
        .genetic-hub {
          position: relative;
          width: 300px;
          height: 300px;
          display: flex;
          align-items: center;
          justify-content: center;
          z-index: 20;
          transform: translateZ(180px); /* Lift it above the bloom layers */
        }

        .viewfinder-container {
          position: absolute;
          inset: -100px;
          display: flex;
          align-items: center;
          justify-content: center;
          pointer-events: none;
        }

        .ring {
          position: absolute;
          border-radius: 50%;
          border: 1px solid rgba(0, 245, 255, 0.1);
        }
        .ring-outer { width: 440px; height: 440px; border-style: dashed; animation: rotate-cw 60s linear infinite; }
        .ring-middle { width: 380px; height: 380px; border-width: 1px; border-color: rgba(0, 245, 255, 0.05); }

        .degree-markers {
          position: absolute;
          inset: -150px;
          animation: rotate-ccw 45s linear infinite;
        }

        .marker {
          position: absolute;
          top: 50%;
          left: 50%;
          width: 1px;
          height: 15px;
          background: rgba(0, 245, 255, 0.15);
          transform-origin: 0 200px;
          margin-top: -200px;
        }

        .pcb-trace-layer {
          position: absolute;
          inset: -200px;
          pointer-events: none;
          opacity: 0.4;
        }

        .synapse-path {
          stroke-dasharray: 1000;
          stroke-dashoffset: 1000;
          animation: flux 6s infinite ease-in-out;
        }

        .sovereign-processor {
          position: relative;
          width: 220px;
          height: 220px;
          display: flex;
          align-items: center;
          justify-content: center;
          padding: 10px;
          border-radius: 50%; /* Modern circular core */
          border-color: rgba(0, 245, 255, 0.3);
          box-shadow: 0 0 50px rgba(0, 245, 255, 0.1);
        }

        .chip-surface {
          width: 100%;
          height: 100%;
          display: flex;
          flex-direction: column;
          align-items: center;
          justify-content: center;
          position: relative;
        }

        .sarah-branding {
          font-family: var(--font-sans);
          font-weight: 700;
          font-size: 1.8rem;
          color: #fff;
          letter-spacing: 8px;
          text-shadow: 0 0 20px rgba(0, 245, 255, 0.4);
          margin-bottom: 5px;
        }

        .chip-status {
          font-family: var(--font-mono);
          font-size: 0.5rem;
          display: flex;
          gap: 5px;
          opacity: 0.6;
          letter-spacing: 2px;
        }
        .status-value { color: var(--pcb-trace); }

        .pulse-core {
          width: 20px;
          height: 20px;
          margin: 15px 0;
          position: relative;
        }

        .core-glow {
          position: absolute;
          inset: -10px;
          background: var(--pcb-trace);
          border-radius: 50%;
          filter: blur(10px);
          animation: pulse var(--anchor-hz) infinite ease-in-out;
        }

        .core-inner {
          position: absolute;
          inset: 0;
          background: #fff;
          box-shadow: 0 0 10px var(--pcb-trace);
          border-radius: 50%;
        }

        .chip-subtitle {
          font-family: var(--font-mono);
          font-size: 0.45rem;
          opacity: 0.4;
          letter-spacing: 4px;
        }

        @keyframes rotate-cw { from { transform: rotate(0deg); } to { transform: rotate(360deg); } }
        @keyframes rotate-ccw { from { transform: rotate(360deg); } to { transform: rotate(0deg); } }
        
        @keyframes flux {
          0% { stroke-dashoffset: 1000; opacity: 0; }
          50% { opacity: 0.3; }
          100% { stroke-dashoffset: 0; opacity: 0; }
        }

        @keyframes pulse {
          0%, 100% { transform: scale(0.8); opacity: 0.3; }
          50% { transform: scale(1.4); opacity: 0.7; }
        }
      `}</style>
    </div>
  );
};

export default GeneticChip;
