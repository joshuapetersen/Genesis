import React, { useState, useEffect, useRef } from 'react';

interface SecondaryCommandProps {
    onSelect: (app: string) => void;
    activeApp: string;
    iconSize?: number;
}

const SecondaryCommand: React.FC<SecondaryCommandProps> = ({ onSelect, activeApp, iconSize = 28 }) => {
    const [rotationOffset, setRotationOffset] = useState(0);
    const [isDragging, setIsDragging] = useState(false);
    const lastInteractionY = useRef<number>(0);

    // HUD Geometric Scales (Same as Primary for consistency)
    const innerRadius = 80;
    const outerRadius = 140;

    // OS APPS SHORTCUTS
    const innerFeatures = [
        { id: 'notepad', label: 'NOTEPAD', path: "M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z M14 2v6h6 M16 13H8 M16 17H8 M10 9H8" },
        { id: 'calc', label: 'CALC_GRID', path: "M6 2h12a2 2 0 0 1 2 2v16a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2zm2 14h2v2H8v-2zm4 0h2v2h-2v-2zm4 0h2v2h-2v-2zm-8-4h2v2H8v-2zm4 0h2v2h-2v-2zm4 0h2v2h-2v-2z" },
        { id: 'cmd', label: 'SHELL', path: "M4 17l6-6-6-6M12 19h8" }
    ];

    const outerFeatures = [
        { id: 'taskmgr', label: 'TASK_KILL', path: "M18 3a3 3 0 0 0-3 3v12a3 3 0 0 0 3 3 3 3 0 0 0 3-3 3 3 0 0 0-3-3H6a3 3 0 0 0-3 3 3 3 0 0 0 3 3 3 3 0 0 0 3-3V6a3 3 0 0 0-3-3 3 3 0 0 0-3 3 3 3 0 0 0 3 3h12a3 3 0 0 0 3-3 3 3 0 0 0-3-3z" },
        { id: 'browser', label: 'NET_LINK', path: "M12 2a10 10 0 1 0 10 10A10 10 0 0 0 12 2zm0 18a8 8 0 1 1 8-8 8 8 0 0 1-8 8z M2 12h20 M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z" },
        { id: 'explorer', label: 'FILE_SYS', path: "M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z" }
    ];

    const emptySlotsCount = 10; // Extra expansion slots for user apps

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

    // Modified position logic for TOP LEFT anchoring (Quadrant 4)
    const getSemiCirclePos = (index: number, total: number, radius: number, offset: number = 0, globalRotation: number = 0) => {
        // Arc from 0 to PI/2 (Bottom Right direction from Top Left anchor)
        const startAngle = 0;
        const endAngle = Math.PI / 2;
        const angleRange = endAngle - startAngle;
        const step = total > 1 ? angleRange / (total - 1) : 0;
        const baseAngle = startAngle + (index + offset) * step;

        // Apply rotation but constrain visual arc? Or let it spin?
        // Let's let it spin but map it to the quadrant.
        const angle = baseAngle + (globalRotation * (Math.PI / 180));

        return {
            x: 10 + radius * Math.cos(angle), // Offset +10px from edge
            y: 10 + radius * Math.sin(angle)  // Offset +10px from top
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
            <div className="wheel-ring-wrapper" style={{ transform: `rotate(${rotationOffset * 0.5}deg)`, top: 0, left: 0, bottom: 'auto', right: 'auto' }}>
                <div className="wheel-ring" style={{ width: innerRadius * 2, height: innerRadius * 2, top: -innerRadius, left: -innerRadius }} />
                {innerFeatures.map((f, i) => {
                    const { x, y } = getSemiCirclePos(i, innerFeatures.length, innerRadius);
                    const isActive = activeApp === f.id;
                    return (
                        <button
                            key={f.id}
                            onClick={(e) => { e.stopPropagation(); onSelect(f.id); }}
                            className={`icon-btn pointer-events-auto group ${isActive ? 'bg-[#FF3B30] border-white scale-125 shadow-[0_0_25px_rgba(255,59,48,0.6)]' : ''}`}
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
            <div className="wheel-ring-wrapper" style={{ transform: `rotate(${rotationOffset * -0.3}deg)`, top: 0, left: 0, bottom: 'auto', right: 'auto' }}>
                <div className="wheel-ring" style={{ width: outerRadius * 2, height: outerRadius * 2, top: -outerRadius, left: -outerRadius }} />

                {outerFeatures.map((f, i) => {
                    const { x, y } = getSemiCirclePos(i, outerFeatures.length + emptySlotsCount, outerRadius, outerFeatures.length);
                    const isActive = activeApp === f.id;
                    return (
                        <button
                            key={f.id}
                            onClick={(e) => { e.stopPropagation(); onSelect(f.id); }}
                            className={`icon-btn pointer-events-auto group ${isActive ? 'bg-[#FF3B30] border-white scale-125 shadow-[0_0_25px_rgba(255,59,48,0.6)]' : ''}`}
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
                            className="slot-btn pointer-events-auto group hover:bg-[#FF3B30]/20 hover:border-[#FF3B30]"
                            style={{
                                left: `${x}px`,
                                top: `${y}px`,
                                width: `${scaledSlotSize}px`,
                                height: `${scaledSlotSize}px`
                            }}
                            onClick={(e) => { e.stopPropagation(); onSelect('add_app'); }}
                            title="ADD_OS_LINK"
                        />
                    );
                })}
            </div>
        </div>
    );
};

export default SecondaryCommand;
