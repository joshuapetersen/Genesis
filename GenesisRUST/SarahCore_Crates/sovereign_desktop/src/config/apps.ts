import React from 'react';
import { Terminal, FileText, Calculator, Globe, Settings, Image as ImageIcon, Activity, Bot, Mic } from 'lucide-react';

export interface AppDefinition {
  id: string;
  title: string;
  icon: React.ElementType;
  component: React.LazyExoticComponent<React.FC<any>> | React.FC<any>;
  defaultWidth?: number;
  defaultHeight?: number;
}

// We will lazy load the apps to simulate a real OS loading programs
export const apps: Record<string, AppDefinition> = {
  aichat: {
    id: 'aichat',
    title: 'GENESIS OS',
    icon: Bot,
    component: React.lazy(() => import('../components/apps/AIChatApp')),
    defaultWidth: 800,
    defaultHeight: 600,
  },
  terminal: {
    id: 'terminal',
    title: 'Terminal',
    icon: Terminal,
    component: React.lazy(() => import('../components/apps/TerminalApp')),
    defaultWidth: 600,
    defaultHeight: 400,
  },
  sysmon: {
    id: 'sysmon',
    title: 'System Monitor',
    icon: Activity,
    component: React.lazy(() => import('../components/apps/SystemMonitorApp')),
    defaultWidth: 600,
    defaultHeight: 400,
  },
  notepad: {
    id: 'notepad',
    title: 'Notepad',
    icon: FileText,
    component: React.lazy(() => import('../components/apps/NotepadApp')),
    defaultWidth: 500,
    defaultHeight: 500,
  },
  calculator: {
    id: 'calculator',
    title: 'Calculator',
    icon: Calculator,
    component: React.lazy(() => import('../components/apps/CalculatorApp')),
    defaultWidth: 320,
    defaultHeight: 480,
  },
  browser: {
    id: 'browser',
    title: 'Web Browser',
    icon: Globe,
    component: React.lazy(() => import('../components/apps/BrowserApp')),
    defaultWidth: 1024,
    defaultHeight: 768,
  },
  voice: {
    id: 'voice',
    title: 'Sovereign Voice',
    icon: Mic,
    component: React.lazy(() => import('../components/apps/VoiceApp')),
    defaultWidth: 600,
    defaultHeight: 500,
  },
};
