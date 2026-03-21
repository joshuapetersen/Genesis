import React from 'react';

type AgentNode = {
  id: string;
  label: string;
  status: 'idle' | 'thinking' | 'authorized' | 'rejected';
};

const CouncilViz: React.FC = () => {
  const intelAgents: AgentNode[] = [
    { id: 'researcher', label: 'RESEARCHER', status: 'idle' },
    { id: 'librarian', label: 'LIBRARIAN', status: 'idle' },
    { id: 'analyst', label: 'ANALYST', status: 'idle' },
    { id: 'designer', label: 'DESIGNER', status: 'idle' },
  ];

  const logicAgents: AgentNode[] = [
    { id: 'architect', label: 'ARCHITECT', status: 'idle' },
    { id: 'engineer', label: 'ENGINEER', status: 'idle' },
    { id: 'advocate', label: 'ADVOCATE', status: 'idle' },
    { id: 'critic', label: 'CRITIC', status: 'idle' },
  ];

  const actionAgents: AgentNode[] = [
    { id: 'arbiter', label: 'ARBITER', status: 'idle' },
    { id: 'ethicist', label: 'ETHICIST', status: 'idle' },
    { id: 'prophet', label: 'PROPHET', status: 'idle' },
    { id: 'scribe', label: 'SCRIBE', status: 'idle' },
  ];

  return (
    <div className="neural-bloom-container">
      {/* 3D Perspective Wrapper */}
      <div className="bloom-space">
        {/* Layer 1: Intel (Outer) */}
        <div className="bloom-layer layer-outer">
          <div className="orbital-ring"></div>
          {intelAgents.map((agent, i) => (
            <div key={agent.id} className={`node ${agent.status}`} style={{ '--index': i, '--total': intelAgents.length } as any}>
              <div className="node-dot"></div>
              <div className="node-tag">{agent.label}</div>
            </div>
          ))}
        </div>

        {/* Layer 2: Logic (Middle) */}
        <div className="bloom-layer layer-middle">
          <div className="orbital-ring"></div>
          {logicAgents.map((agent, i) => (
            <div key={agent.id} className={`node ${agent.status}`} style={{ '--index': i, '--total': logicAgents.length } as any}>
              <div className="node-dot"></div>
              <div className="node-tag">{agent.label}</div>
            </div>
          ))}
        </div>

        {/* Layer 3: Action (Inner) */}
        <div className="bloom-layer layer-inner">
          <div className="orbital-ring"></div>
          {actionAgents.map((agent, i) => (
            <div key={agent.id} className={`node ${agent.status}`} style={{ '--index': i, '--total': actionAgents.length } as any}>
              <div className="node-dot"></div>
              <div className="node-tag">{agent.label}</div>
            </div>
          ))}
        </div>
      </div>

      <style>{`
        .neural-bloom-container {
          position: absolute;
          width: 800px;
          height: 600px;
          display: flex;
          align-items: center;
          justify-content: center;
          perspective: 1200px;
          z-index: 5;
        }

        .bloom-space {
          position: relative;
          width: 100%;
          height: 100%;
          transform-style: preserve-3d;
          transform: rotateX(65deg) rotateZ(0deg);
          animation: space-drift 60s infinite linear;
        }

        @keyframes space-drift {
          0% { transform: rotateX(65deg) rotateZ(0deg); }
          100% { transform: rotateX(65deg) rotateZ(360deg); }
        }

        .bloom-layer {
          position: absolute;
          top: 50%;
          left: 50%;
          transform-style: preserve-3d;
        }

        .layer-outer { transform: translateZ(-50px); }
        .layer-middle { transform: translateZ(50px); }
        .layer-inner { transform: translateZ(150px); }

        .orbital-ring {
          position: absolute;
          top: 50%;
          left: 50%;
          border: 1px solid rgba(0, 245, 255, 0.1);
          border-radius: 50%;
          transform: translate(-50%, -50%);
        }

        .layer-outer .orbital-ring { width: 480px; height: 480px; }
        .layer-middle .orbital-ring { width: 320px; height: 320px; }
        .layer-inner .orbital-ring { width: 180px; height: 180px; }

        .node {
          position: absolute;
          top: 50%;
          left: 50%;
          width: 20px;
          height: 20px;
          margin-top: -10px;
          margin-left: -100px;
          transform-style: preserve-3d;
          animation: orbit-sync var(--speed, 30s) infinite linear;
          animation-delay: calc(var(--index) * (var(--speed, 30s) / var(--total)) * -1);
        }

        .layer-outer .node { --speed: 40s; --radius: 240px; }
        .layer-middle .node { --speed: 30s; --radius: 160px; }
        .layer-inner .node { --speed: 20s; --radius: 90px; }

        @keyframes orbit-sync {
          from { transform: rotate(0deg) translateX(var(--radius)) rotate(0deg) rotateX(-90deg); }
          to { transform: rotate(360deg) translateX(var(--radius)) rotate(-360deg) rotateX(-90deg); }
        }

        .node-dot {
          width: 6px;
          height: 6px;
          background: #00f5ff;
          border-radius: 50%;
          box-shadow: 0 0 10px #00f5ff;
          transition: all 0.3s ease;
        }

        .node-tag {
          position: absolute;
          top: -20px;
          left: 50%;
          transform: translateX(-50%);
          font-family: var(--font-mono);
          font-size: 0.5rem;
          color: rgba(0, 245, 255, 0.6);
          white-space: nowrap;
          letter-spacing: 1px;
          opacity: 0;
          transition: opacity 0.3s ease;
        }

        .node:hover .node-tag {
          opacity: 1;
        }

        .thinking .node-dot {
          animation: pulse-glow 0.8s infinite alternate;
          background: #fff;
          box-shadow: 0 0 20px #00f5ff, 0 0 40px #00f5ff;
        }

        @keyframes pulse-glow {
          from { transform: scale(1); opacity: 0.8; }
          to { transform: scale(1.5); opacity: 1; }
        }
      `}</style>
    </div>
  );
};

export default CouncilViz;
