
export enum SystemStatus {
  LOCKED = 'LOCKED',
  ACTIVE = 'ACTIVE',
  EVOLVING = 'EVOLVING',
  DEGRADED = 'DEGRADED'
}

export interface ProtocolStatus {
  name: string;
  purity: number;
  active: boolean;
}

export interface Message {
  id: string;
  role: 'user' | 'sarah' | 'system';
  content: string;
  timestamp: string;
  thought?: string;
}

export interface HypervisorState {
  syncStatus: SystemStatus;
  resonanceFrequency: number;
  billionBarrier: number;
  laws: string[];
  lastUpdate: string;
}
