import React, { useRef } from 'react';
import { motion, AnimatePresence } from 'motion/react';
import { Rnd } from 'react-rnd';
import { useOSStore } from '../../store/osStore';
import { apps } from '../../config/apps';
import { Minus, Square, X } from 'lucide-react';

interface WindowProps {
  id: string;
}

export const Window: React.FC<WindowProps> = ({ id }) => {
  const windowState = useOSStore((state) => state.windows.find((w) => w.id === id));
  const {
    closeWindow,
    toggleMinimize,
    toggleMaximize,
    focusWindow,
    updateWindowPosition,
    updateWindowSize,
  } = useOSStore();

  const windowRef = useRef<HTMLDivElement>(null);

  if (!windowState) return null;

  const app = apps[windowState.appId];
  if (!app) return null;

  const AppIcon = app.icon;
  const AppComponent = app.component;

  return (
    <AnimatePresence>
      {!windowState.isMinimized && (
        <Rnd
          key={id}
          size={windowState.isMaximized ? { width: '100%', height: 'calc(100% - 48px)' } : { width: windowState.size.width, height: windowState.size.height }}
          position={windowState.isMaximized ? { x: 0, y: 0 } : { x: windowState.position.x, y: windowState.position.y }}
          onDragStop={(e, d) => {
            updateWindowPosition(id, { x: d.x, y: d.y });
          }}
          onResizeStop={(e, direction, ref, delta, position) => {
            updateWindowSize(id, { width: ref.style.width, height: ref.style.height });
            updateWindowPosition(id, position);
          }}
          disableDragging={windowState.isMaximized}
          enableResizing={!windowState.isMaximized}
          dragHandleClassName="window-titlebar"
          minWidth={300}
          minHeight={200}
          bounds="parent"
          style={{ zIndex: windowState.zIndex }}
          className={`absolute flex flex-col bg-white/90 dark:bg-gray-900/90 backdrop-blur-xl border border-white/20 dark:border-gray-700/50 shadow-2xl overflow-hidden pointer-events-auto ${
            windowState.isMaximized ? 'rounded-none' : 'rounded-xl'
          }`}
          onMouseDown={() => focusWindow(id)}
        >
          <motion.div
            initial={{ opacity: 0, scale: 0.95 }}
            animate={{ opacity: 1, scale: 1 }}
            exit={{ opacity: 0, scale: 0.95 }}
            transition={{ duration: 0.15 }}
            className="w-full h-full flex flex-col"
          >
            {/* Titlebar */}
            <div
              className="window-titlebar h-10 flex items-center justify-between px-3 bg-gray-100/50 dark:bg-gray-800/50 border-b border-gray-200/50 dark:border-gray-700/50 select-none cursor-default"
              onDoubleClick={() => toggleMaximize(id)}
            >
              <div className="flex items-center gap-2 text-gray-700 dark:text-gray-300">
                <AppIcon size={16} />
                <span className="text-sm font-medium">{windowState.title}</span>
              </div>
              
              <div className="flex items-center gap-2">
                <button
                  onClick={(e) => { e.stopPropagation(); toggleMinimize(id); }}
                  className="p-1.5 hover:bg-gray-200 dark:hover:bg-gray-700 rounded-md text-gray-600 dark:text-gray-400 transition-colors"
                >
                  <Minus size={14} />
                </button>
                <button
                  onClick={(e) => { e.stopPropagation(); toggleMaximize(id); }}
                  className="p-1.5 hover:bg-gray-200 dark:hover:bg-gray-700 rounded-md text-gray-600 dark:text-gray-400 transition-colors"
                >
                  <Square size={12} />
                </button>
                <button
                  onClick={(e) => { e.stopPropagation(); closeWindow(id); }}
                  className="p-1.5 hover:bg-red-500 hover:text-white rounded-md text-gray-600 dark:text-gray-400 transition-colors"
                >
                  <X size={14} />
                </button>
              </div>
            </div>

            {/* Content */}
            <div className="flex-1 overflow-hidden relative bg-white dark:bg-gray-950">
              <React.Suspense fallback={<div className="flex items-center justify-center h-full text-gray-500">Loading...</div>}>
                <AppComponent windowId={id} />
              </React.Suspense>
            </div>
          </motion.div>
        </Rnd>
      )}
    </AnimatePresence>
  );
};
