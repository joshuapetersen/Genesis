import React, { useState } from 'react';
import { motion } from 'motion/react';
import { useOSStore } from '../../store/osStore';
import { apps } from '../../config/apps';

export const Desktop: React.FC = () => {
  const { openApp, iconPositions, updateIconPosition, systemConfig } = useOSStore();
  const [selectedId, setSelectedId] = useState<string | null>(null);

  // Deselect when clicking the background
  const handleBackgroundClick = (e: React.MouseEvent) => {
    if (e.target === e.currentTarget) {
      setSelectedId(null);
    }
  };

  return (
    <div 
      className="absolute inset-0 p-4 z-0 overflow-hidden"
      onClick={handleBackgroundClick}
    >
      {Object.values(apps).map((app) => {
        const AppIcon = app.icon;
        const pos = iconPositions[app.id] || { x: 20, y: 20 };
        const isSelected = selectedId === app.id;

        return (
          <motion.div
            key={app.id}
            drag
            dragMomentum={false}
            onDragEnd={(_e, info) => {
              const gridSize = 100;
              let newX = pos.x + info.offset.x;
              let newY = pos.y + info.offset.y;

              if (systemConfig.snapToGrid) {
                newX = Math.round(newX / gridSize) * gridSize;
                newY = Math.round(newY / gridSize) * gridSize;
              }

              updateIconPosition(app.id, newX, newY);
            }}
            style={{ 
              x: pos.x, 
              y: pos.y, 
              position: 'absolute',
              zIndex: isSelected ? 10 : 1
            }}
            initial={false}
            whileHover={{ scale: 1.05 }}
            whileTap={{ scale: 0.95 }}
            className={`w-24 h-24 flex flex-col items-center justify-center gap-1 p-2 rounded-lg cursor-pointer transition-colors group select-none ${
              isSelected ? 'bg-white/20 ring-1 ring-white/30 shadow-lg' : 'hover:bg-white/10'
            }`}
            onClick={(e) => {
              e.stopPropagation();
              setSelectedId(app.id);
            }}
            onDoubleClick={(e) => {
              e.stopPropagation();
              openApp(app.id, app.title, app.defaultWidth, app.defaultHeight);
            }}
          >
            <div 
              className="w-14 h-14 rounded-2xl bg-gradient-to-br from-white/20 to-white/5 backdrop-blur-md border border-white/20 shadow-xl flex items-center justify-center text-white relative"
              style={{ color: isSelected ? systemConfig.accentColor : 'white' }}
            >
              <AppIcon size={32} />
              {isSelected && (
                <div className="absolute inset-0 rounded-2xl ring-2 ring-white/50 animate-pulse" />
              )}
            </div>
            <span 
              className="text-[11px] text-white font-semibold text-center drop-shadow-lg px-2 py-0.5 bg-black/40 rounded-full truncate w-full"
              style={{ fontFamily: systemConfig.fontFamily }}
            >
              {app.title}
            </span>
          </motion.div>
        );
      })}
    </div>
  );
};
