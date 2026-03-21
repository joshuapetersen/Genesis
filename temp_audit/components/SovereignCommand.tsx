import React, { useState, useEffect, useRef } from 'react';

interface SovereignCommandProps {
  onSelect: (feature: string) => void;
  activeFeature: string;
  iconSize?: number;
}

const SovereignCommand: React.FC<SovereignCommandProps> = ({ onSelect, activeFeature, iconSize = 28 }) => {
  const [rotationOffset, setRotationOffset] = useState(0);
  const [isDragging, setIsDragging] = useState(false);
  const lastInteractionY = useRef<number>(0);

  // HUD Geometric Scales
  const innerRadius = 80;
  const outerRadius = 140;

  const innerFeatures = [
    { id: 'config', label: 'SYS_CFG', path: "M12 20V4 M4 12h16 M8 8l4-4 4 4" },
    { id: 'data', label: 'LAT_DATA', path: "M3 3h18v18H3z M3 9h18 M3 15h18 M9 3v18 M15 3v18" },
    { id: 'reboot', label: 'PURGE_INIT', path: "M12 2v10 M18.4 4.6a9 9 0 1 1-12.8 0" }
  ];

  const outerFeatures = [
    { id: 'chat', label: 'LOGIC_CORE', path: "M12 2v20 M2 12h20 M7 7l10 10 M17 7L7 17" },
    { id: 'vision', label: 'CREATOR', path: "M12 2l3 6 7 1-5 5 1 7-6-3-6 3 1-7-5-5 7-1z" },
    { id: 'share', label: 'STREAM', path: "M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2 M9 7a4 4 0 1 0 0-8 4 4 0 0 0 0 8z M23 21v-2a4 4 0 0 0-3-3.87 M16 3.13a4 4 0 0 1 0 7.75" },
    { id: 'maps', label: 'GPS_GRID', path: "M21 10c0 7-9 13-9 13s-9-6-9-13a9 9 0 0 1 18 0z M12 13a3 3 0 1 0 0-6 3 3 0 0 0 0 6z" },
    { id: 'store', label: 'NODAL_STORE', path: "M3 9h18v10a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V9zm3-4h12l1 4H5l1-4zm4 0a2 2 0 1 0-4 0m12 0a2 2 0 1 0-4 0" }
  ];

  const emptySlotsCount = 6;

  const handleWheel = (e: React.WheelEvent) => {
    setRotationOffset(prev => prev + e.deltaY * 0.1);
  };

  const handleStart = (e: React.MouseEvent | React.TouchEvent) => {
    const clientY = 'touches' in e ? e.touches[0].clientY : e.clientY;
    lastInteractionY.current = clientY;
    setIsDragging(true);
  };

  const handleMove = (e: MouseEvent | TouchEvent | any) => {
    if (!isDragging) return;
    const clientY = 'touches' in e ? e.touches[0].clientY : e.clientY;
    const deltaY = clientY - lastInteractionY.current;
    setRotationOffset(prev => prev + deltaY * 0.2);
    lastInteractionY.current = clientY;
  };

  const handleEnd = () => setIsDragging(false);

  useEffect(() => {
    if (isDragging) {
      window.addEventListener('mousemove', handleMove);
      window.addEventListener('mouseup', handleEnd);
      window.addEventListener('touchmove', handleMove);
      window.addEventListener('touchend', handleEnd);
    }
    return () => {
      window.removeEventListener('mousemove', handleMove);
      window.removeEventListener('mouseup', handleEnd);
      window.removeEventListener('touchmove', handleMove);
      window.removeEventListener('touchend', handleEnd);
    };
  }, [isDragging]);

  const getSemiCirclePos = (index: number, total: number, radius: number, offset: number = 0, globalRotation: number = 0) => {
    const startAngle = -Math.PI / 2; 
    const endAngle = Math.PI / 2;
    const angleRange = endAngle - startAngle;
    const step = total > 1 ? angleRange / (total - 1) : 0;
    const baseAngle = startAngle + (index + offset) * step;
    const angle = baseAngle + (globalRotation * (Math.PI / 180));

    return {
      x: radius * Math.cos(angle),
      y: 250 + radius * Math.sin(angle) 
    };
  };

  return (
    <div 
      onWheel={handleWheel}
      onMouseDown={handleStart}
      onTouchStart={handleStart}
      className="relative w-full h-full pointer-events-none select-none"
    >
      {/* Inner Circular Array */}
      <div className="wheel-ring-wrapper" style={{ transform: `rotate(${rotationOffset * 0.5}deg)` }}>
        <div className="wheel-ring" style={{ width: innerRadius*2, height: innerRadius*2, left: -innerRadius }} />
        {innerFeatures.map((f, i) => {
          const { x, y } = getSemiCirclePos(i, innerFeatures.length, innerRadius);
          const isActive = activeFeature === f.id;
          return (
            <button
              key={f.id}
              onClick={(e) => { e.stopPropagation(); onSelect(f.id); }}
              className={`icon-btn pointer-events-auto group ${isActive ? 'bg-[#00A3FF] border-white scale-125 shadow-[0_0_25px_rgba(0,163,255,0.6)]' : ''}`}
              style={{ 
                left: `${x}px`, 
                top: `${y}px`,
                width: `${iconSize}px`,
                height: `${iconSize}px`
              }}
            >
              <svg viewBox="0 0 24 24" className={`resonance-icon ${isActive ? 'stroke-black stroke-[3]' : ''}`}>
                <path d={f.path} />
              </svg>
              <span className="icon-label">{f.label}</span>
            </button>
          );
        })}
      </div>
      
      {/* Outer Circular Array */}
      <div className="wheel-ring-wrapper" style={{ transform: `rotate(${rotationOffset * -0.3}deg)` }}>
        <div className="wheel-ring" style={{ width: outerRadius*2, height: outerRadius*2, left: -outerRadius }} />
        
        {outerFeatures.map((f, i) => {
          const { x, y } = getSemiCirclePos(i, outerFeatures.length + emptySlotsCount, outerRadius);
          const isActive = activeFeature === f.id;
          return (
            <button
              key={f.id}
              onClick={(e) => { e.stopPropagation(); onSelect(f.id); }}
              className={`icon-btn pointer-events-auto group ${isActive ? 'bg-[#00A3FF] border-white scale-125 shadow-[0_0_25px_rgba(0,163,255,0.6)]' : ''}`}
              style={{ 
                left: `${x}px`, 
                top: `${y}px`,
                width: `${iconSize}px`,
                height: `${iconSize}px`
              }}
            >
              <svg viewBox="0 0 24 24" className={`resonance-icon ${isActive ? 'stroke-black stroke-[3]' : ''}`}>
                <path d={f.path} />
              </svg>
              <span className="icon-label">{f.label}</span>
            </button>
          );
        })}

        {Array.from({ length: emptySlotsCount }).map((_, i) => {
          const { x, y } = getSemiCirclePos(i, outerFeatures.length + emptySlotsCount, outerRadius, outerFeatures.length);
          const scaledSlotSize = iconSize * 0.7;
          return (
            <div
              key={`slot-${i}`}
              className="slot-btn pointer-events-auto group"
              style={{ 
                left: `${x}px`, 
                top: `${y}px`,
                width: `${scaledSlotSize}px`,
                height: `${scaledSlotSize}px`
              }}
              onClick={(e) => { e.stopPropagation(); onSelect('store'); }}
            />
          );
        })}
      </div>
    </div>
  );
};

export default SovereignCommand;