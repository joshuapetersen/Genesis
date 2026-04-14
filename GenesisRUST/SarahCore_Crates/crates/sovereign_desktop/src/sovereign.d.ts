export {};

declare global {
  interface Window {
    sovereign: {
      onHeartbeat: (callback: (timestamp: number) => void) => void;
      requestSnapshot: () => void;
      windowControl: (command: 'minimize' | 'close') => void;
      sendIntent: (intent: string) => void;
      onResponse: (callback: (response: string) => void) => void;
    };
  }
}
