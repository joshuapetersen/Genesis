
import { UI_COLORS } from '../constants';

export interface DeviceSpecs {
  id: string;
  type: 'IOT' | 'MOBILE' | 'WORKSTATION' | 'SERVER';
  cores: number;
  memory: string;
  architecture: string;
  resonanceStability: number;
}

export class HardwareBridge {
  static detectSpecs(): DeviceSpecs {
    const ua = navigator.userAgent;
    const isMobile = /iPhone|iPad|iPod|Android/i.test(ua);
    const cores = navigator.hardwareConcurrency || 4;
    
    // Simulated specs based on real browser capabilities
    return {
      id: `NODE-${Math.random().toString(36).substring(2, 8).toUpperCase()}`,
      type: isMobile ? 'MOBILE' : (cores > 8 ? 'WORKSTATION' : 'SERVER'),
      cores: cores,
      memory: `${(navigator as any).deviceMemory || 8}GB`,
      architecture: ua.includes('Win64') || ua.includes('x86_64') ? 'x86_64' : 'ARM64',
      resonanceStability: 0.95 + Math.random() * 0.04
    };
  }

  static getOptimizationFlag(specs: DeviceSpecs): string {
    if (specs.type === 'MOBILE') return '--power-save --high-latency-buffer';
    if (specs.type === 'WORKSTATION') return '--volumetric-c3-priority --full-mesh';
    return '--balanced-load';
  }
}
