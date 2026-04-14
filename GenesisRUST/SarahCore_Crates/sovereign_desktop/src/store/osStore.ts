import { create } from 'zustand';

export interface WindowState {
  id: string;
  appId: string;
  title: string;
  isOpen: boolean;
  isMinimized: boolean;
  isMaximized: boolean;
  zIndex: number;
  position: { x: number; y: number };
  size: { width: number | string; height: number | string };
}

export interface OSConfig {
  wallpaper: string;
  accentColor: string;
  fontFamily: string;
  snapToGrid: boolean;
  showThoughts: boolean;
  voiceEnabled: boolean;
}

interface OSStore {
  windows: WindowState[];
  activeWindowId: string | null;
  highestZIndex: number;
  openApp: (appId: string, title: string, defaultWidth?: number, defaultHeight?: number) => void;
  closeWindow: (id: string) => void;
  toggleMinimize: (id: string) => void;
  toggleMaximize: (id: string) => void;
  focusWindow: (id: string) => void;
  updateWindowPosition: (id: string, position: { x: number; y: number }) => void;
  updateWindowSize: (id: string, size: { width: number | string; height: number | string }) => void;
  
  // Customization
  systemConfig: OSConfig;
  updateConfig: (updates: Partial<OSConfig>) => void;
  iconPositions: Record<string, { x: number; y: number }>;
  updateIconPosition: (appId: string, x: number, y: number) => void;
}

export const useOSStore = create<OSStore>((set, get) => ({
  windows: [],
  activeWindowId: null,
  highestZIndex: 10,

  openApp: (appId, title, defaultWidth = 600, defaultHeight = 400) => {
    const { windows, highestZIndex } = get();
    const existingWindow = windows.find((w) => w.appId === appId);

    if (existingWindow) {
      if (existingWindow.isMinimized) {
        get().toggleMinimize(existingWindow.id);
      }
      get().focusWindow(existingWindow.id);
      return;
    }

    const newZIndex = highestZIndex + 1;
    const newWindow: WindowState = {
      id: `${appId}-${Date.now()}`,
      appId,
      title,
      isOpen: true,
      isMinimized: false,
      isMaximized: false,
      zIndex: newZIndex,
      position: { x: 50 + windows.length * 30, y: 50 + windows.length * 30 },
      size: { width: defaultWidth, height: defaultHeight },
    };

    set({
      windows: [...windows, newWindow],
      activeWindowId: newWindow.id,
      highestZIndex: newZIndex,
    });
  },

  closeWindow: (id) => {
    set((state) => ({
      windows: state.windows.filter((w) => w.id !== id),
      activeWindowId: state.activeWindowId === id ? null : state.activeWindowId,
    }));
  },

  toggleMinimize: (id) => {
    set((state) => {
      const windows = state.windows.map((w) => {
        if (w.id === id) {
          const isMinimizing = !w.isMinimized;
          return { ...w, isMinimized: isMinimizing };
        }
        return w;
      });
      
      const activeWindowId = state.activeWindowId === id ? null : state.activeWindowId;
      return { windows, activeWindowId };
    });
  },

  toggleMaximize: (id) => {
    set((state) => ({
      windows: state.windows.map((w) =>
        w.id === id ? { ...w, isMaximized: !w.isMaximized } : w
      ),
    }));
    get().focusWindow(id);
  },

  focusWindow: (id) => {
    const { activeWindowId, highestZIndex } = get();
    if (activeWindowId === id) return;

    const newZIndex = highestZIndex + 1;
    set((state) => ({
      windows: state.windows.map((w) =>
        w.id === id ? { ...w, zIndex: newZIndex, isMinimized: false } : w
      ),
      activeWindowId: id,
      highestZIndex: newZIndex,
    }));
  },

  updateWindowPosition: (id, position) => {
    set((state) => ({
      windows: state.windows.map((w) =>
        w.id === id ? { ...w, position } : w
      ),
    }));
  },

  updateWindowSize: (id, size) => {
    set((state) => ({
      windows: state.windows.map((w) =>
        w.id === id ? { ...w, size } : w
      ),
    }));
  },

  // Customization
  systemConfig: {
    wallpaper: '',
    accentColor: '#3b82f6', // Default blue
    fontFamily: 'system-ui, -apple-system, sans-serif',
    snapToGrid: false,
    showThoughts: true,
    voiceEnabled: true,
  },
  
  updateConfig: (updates) => {
    set((state) => ({
      systemConfig: { ...state.systemConfig, ...updates }
    }));
  },

  iconPositions: {
    'aichat': { x: 20, y: 20 },
    'terminal': { x: 20, y: 120 },
    'sysmon': { x: 20, y: 220 },
    'notepad': { x: 20, y: 320 },
    'calculator': { x: 20, y: 420 },
    'browser': { x: 20, y: 520 },
  },

  updateIconPosition: (appId, x, y) => {
    set((state) => ({
      iconPositions: { ...state.iconPositions, [appId]: { x, y } }
    }));
  },
}));
