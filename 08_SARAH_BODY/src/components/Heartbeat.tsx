import React from 'react';

const Heartbeat: React.FC = () => {
    return (
        <div className="heartbeat-container">
            <div className="orb-outer">
                <div className="orb-inner">
                    <div className="orb-core"></div>
                </div>
            </div>
            <div className="pulse-label">1.09277703703703 HZ</div>
            <style>{`
        .heartbeat-container {
          position: absolute;
          top: 50%;
          left: 50%;
          transform: translate(-50%, -50%);
          display: flex;
          flex-direction: column;
          align-items: center;
          justify-content: center;
        }

        .orb-outer {
          width: 200px;
          height: 200px;
          border-radius: 50%;
          background: rgba(0, 245, 255, 0.05);
          border: 1px solid rgba(0, 245, 255, 0.1);
          display: flex;
          align-items: center;
          justify-content: center;
          backdrop-filter: blur(5px);
          animation: heart-pulse var(--anchor-hz) infinite ease-in-out;
        }

        .orb-inner {
          width: 140px;
          height: 140px;
          border-radius: 50%;
          background: radial-gradient(circle, rgba(0, 245, 255, 0.2) 0%, transparent 70%);
          display: flex;
          align-items: center;
          justify-content: center;
          border: 1px solid rgba(0, 245, 255, 0.2);
        }

        .orb-core {
          width: 60px;
          height: 60px;
          border-radius: 50%;
          background: var(--neon-cyan);
          box-shadow: 0 0 30px var(--neon-cyan), 0 0 60px var(--neon-blue);
        }

        .pulse-label {
          margin-top: 30px;
          font-family: var(--font-accent);
          font-size: 0.8rem;
          color: var(--neon-cyan);
          letter-spacing: 2px;
          opacity: 0.7;
          text-shadow: 0 0 10px var(--neon-cyan);
        }
      `}</style>
        </div>
    );
};

export default Heartbeat;
