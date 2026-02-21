
import React from 'react';
import { DetectedEntity } from '../types';
import { User, Car, Cpu, AlertTriangle, MapPin } from 'lucide-react';

interface RadarProps {
  entities: DetectedEntity[];
}

const Radar: React.FC<RadarProps> = ({ entities }) => {
  const getIcon = (type: string) => {
    switch (type) {
      case 'HUMAN': return User;
      case 'VEHICLE': return Car;
      case 'IOT': return Cpu;
      case 'HAZARD': return AlertTriangle;
      case 'PLACE': return MapPin;
      default: return Cpu;
    }
  };

  return (
    <div className="relative aspect-square w-64 bg-transparent rounded-full overflow-hidden">
      {/* Radar Sweep Effect - Subtle and transparent */}
      <div className="absolute inset-0 bg-gradient-to-r from-sky-500/10 to-transparent origin-center animate-[spin_4s_linear_infinite] pointer-events-none" style={{ clipPath: 'polygon(50% 50%, 100% 0, 100% 100%)' }}></div>
      
      {/* Grid Rings - Wireframe Only, No Fills */}
      <div className="absolute inset-0 flex items-center justify-center pointer-events-none">
        <div className="w-[33%] h-[33%] rounded-full border border-sky-500/30"></div>
        <div className="w-[66%] h-[66%] rounded-full border border-sky-500/30"></div>
        <div className="w-[100%] h-[100%] rounded-full border border-sky-500/50"></div>
        {/* Axis lines */}
        <div className="absolute w-full h-[1px] bg-sky-500/20"></div>
        <div className="absolute h-full w-[1px] bg-sky-500/20"></div>
      </div>

      {/* Detected Entities (Blips) */}
      {entities.map((entity) => {
        const Icon = getIcon(entity.type);
        const left = `${entity.pos.x}%`;
        const top = `${entity.pos.y}%`;
        
        return (
          <div 
            key={entity.id} 
            className="absolute transition-all duration-1000 group"
            style={{ left, top, transform: 'translate(-50%, -50%)' }}
          >
            <div className={`relative flex items-center justify-center w-4 h-4 rounded-full border ${entity.type === 'HAZARD' ? 'border-rose-500 bg-rose-500/20' : 'border-sky-400 bg-sky-400/20'} shadow-[0_0_10px_currentColor] animate-pulse`}>
              <Icon size={8} className={entity.type === 'HAZARD' ? 'text-rose-500' : 'text-sky-400'} />
            </div>
          </div>
        );
      })}

      {/* Center Reference Node */}
      <div className="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 flex flex-col items-center">
        <div className="w-1 h-1 rounded-full bg-sky-400"></div>
      </div>
    </div>
  );
};

export default Radar;
