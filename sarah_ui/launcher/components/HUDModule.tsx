
import React, { useState, useRef, useEffect } from 'react';
import { Maximize2, Minimize2 } from 'lucide-react';

interface HUDModuleProps {
  id: string;
  title: string;
  initialPosition: { x: number; y: number };
  children: React.ReactNode;
  visible?: boolean;
  onPositionChange: (id: string, x: number, y: number) => void;
  exitSide?: 'left' | 'right' | 'bottom' | 'top';
  isFocused?: boolean;
  onFocus?: () => void;
}

const HUDModule: React.FC<HUDModuleProps> = ({ 
  id, 
  title, 
  initialPosition, 
  children, 
  visible = true, 
  onPositionChange,
  exitSide = 'right',
  isFocused = false,
  onFocus
}) => {
  const [pos, setPos] = useState(initialPosition);
  const [isDragging, setIsDragging] = useState(false);
  const [isHovered, setIsHovered] = useState(false);
  
  const dragStart = useRef({ x: 0, y: 0 });

  useEffect(() => {
    setPos(initialPosition);
  }, [initialPosition]);

  const startDrag = (e: React.MouseEvent) => {
    e.stopPropagation();
    setIsDragging(true);
    onFocus?.();
    dragStart.current = { x: e.clientX - pos.x, y: e.clientY - pos.y };
  };

  const onDrag = (clientX: number, clientY: number) => {
    if (!isDragging) return;
    setPos({ x: clientX - dragStart.current.x, y: clientY - dragStart.current.y });
  };

  const endDrag = () => {
    if (isDragging) {
      onPositionChange(id, pos.x, pos.y);
    }
    setIsDragging(false);
  };

  useEffect(() => {
    const handleMove = (e: MouseEvent) => onDrag(e.clientX, e.clientY);
    const handleUp = () => endDrag();
    if (isDragging) {
      window.addEventListener('mousemove', handleMove);
      window.addEventListener('mouseup', handleUp);
    }
    return () => {
      window.removeEventListener('mousemove', handleMove);
      window.removeEventListener('mouseup', handleUp);
    };
  }, [isDragging, pos, id, onPositionChange]);

  // INTERACTION LOGIC:
  const active = isHovered || isFocused || isDragging;

  // LAYOUT LOGIC: Determines where the Window sits relative to the Anchor
  const getPositionClass = () => {
    switch (exitSide) {
      case 'left': return 'bottom-0 left-0'; // Health: Anchors bottom-left, grows up/right
      case 'right': return 'top-0 right-0';  // Radar: Anchors top-right, grows down/left
      case 'bottom': return 'bottom-0 left-1/2 -translate-x-1/2'; // Dock: Anchors bottom-center, grows up
      case 'top': return 'top-0 left-1/2 -translate-x-1/2'; // Settings: Anchors top-center, grows down
      default: return 'top-0 left-0';
    }
  };

  // ANIMATION ORIGIN: Determines the vector of the scale animation
  const getOriginClass = () => {
    switch (exitSide) {
      case 'left': return 'origin-bottom-left';
      case 'right': return 'origin-top-right';
      case 'bottom': return 'origin-bottom';
      case 'top': return 'origin-top';
      default: return 'origin-center';
    }
  };

  const getAnimationState = () => {
    if (active) return 'opacity-100 scale-100 translate-y-0 blur-0';
    
    // Animation vector logic
    const isUpward = exitSide === 'bottom' || exitSide === 'left';
    const verticalOffset = isUpward ? 'translate-y-4' : '-translate-y-4';
    return `opacity-0 scale-0 ${verticalOffset} blur-md pointer-events-none`;
  };
  
  const containerOpacity = visible ? 1 : 0;
  const pointerEvents = visible ? 'auto' : 'none';

  return (
    <div 
      style={{
        position: 'fixed',
        left: pos.x,
        top: pos.y,
        zIndex: isDragging ? 20000 : (active ? 10000 : 5000),
        width: '16px', // MICRO-NODE SIZE (Unnoticeable)
        height: '16px', // MICRO-NODE SIZE
        opacity: containerOpacity,
        pointerEvents: pointerEvents as any
      }} 
      onMouseEnter={() => setIsHovered(true)}
      onMouseLeave={() => setIsHovered(false)}
      onClick={(e) => { e.stopPropagation(); onFocus?.(); }}
      className="bg-transparent" // Removed origin-center from wrapper to allow absolute children to anchor correctly
    >
      
      {/* --- STATE 1: THE ICON (ANCHOR) --- */}
      {/* Visible when NOT active. Tiny 16px touch target. */}
      {/* We add data-hud-interact for the AI Hand Controller to find it */}
      <div 
        data-hud-interact="true"
        className={`absolute inset-0 flex items-center justify-center transition-all duration-300 ${active ? 'opacity-0 scale-50 pointer-events-none' : 'opacity-100 scale-100'}`}
      >
         <div className="relative group cursor-pointer w-full h-full flex items-center justify-center">
            {/* The Visual Micro-Dot */}
            <div className="w-3 h-3 rounded-[2px] border border-sky-400/30 bg-black/40 backdrop-blur-sm flex items-center justify-center shadow-[0_0_5px_rgba(14,165,233,0.2)] group-hover:border-sky-400 group-hover:bg-sky-400/20 transition-all">
                <div className="w-1 h-1 bg-sky-400 rounded-full animate-pulse"></div>
            </div>
         </div>
      </div>

      {/* --- STATE 2: THE WINDOW (MENU) --- */}
      {/* Absolute positioned relative to the 16px anchor */}
      <div 
        className={`absolute ${getPositionClass()} transition-all duration-500 ease-[cubic-bezier(0.34,1.56,0.64,1)] ${getOriginClass()} ${getAnimationState()}`}
      >
        {/* Header / Drag Handle */}
        <div 
          onMouseDown={startDrag}
          className="flex items-center justify-between mb-2 cursor-grab active:cursor-grabbing group select-none min-w-[320px]"
        >
          <div className="flex items-center gap-2">
             <div className="w-1 h-3 bg-sky-400 shadow-[0_0_8px_cyan]"></div>
             <span className="text-[10px] font-black tracking-[0.2em] uppercase text-white drop-shadow-md">
                {title}
             </span>
          </div>
          <Minimize2 size={12} className="text-white/20 group-hover:text-sky-400 transition-colors" />
        </div>
        
        {/* WINDOW CONTENT CONTAINER */}
        <div className="bg-black/10 backdrop-blur-sm border border-sky-500/10 shadow-[0_4px_16px_rgba(0,0,0,0.2)] rounded-lg overflow-hidden relative min-w-[320px]">
          
          {/* Content Area */}
          <div className="relative z-10 p-4">
             {children}
          </div>

          {/* Minimal Footer */}
          <div className="h-4 bg-sky-500/5 border-t border-sky-500/5 flex items-center justify-between px-3">
             <div className="flex gap-1">
                <div className="w-0.5 h-0.5 rounded-full bg-sky-500/50"></div>
                <div className="w-0.5 h-0.5 rounded-full bg-sky-500/30"></div>
             </div>
             <span className="text-[5px] font-mono text-sky-500/30 uppercase tracking-widest">
                ID_{id.toUpperCase().slice(0,3)}
             </span>
          </div>
        </div>
      </div>

    </div>
  );
};

export default HUDModule;
