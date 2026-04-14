const { contextBridge, ipcRenderer } = require('electron');

// SOVEREIGN CONTEXT BRIDGE
contextBridge.exposeInMainWorld('sovereign', {
    onHeartbeat: (callback) => ipcRenderer.on('heartbeat-pulse', (event, timestamp) => callback(timestamp)),
    requestSnapshot: () => ipcRenderer.send('binary-snapshot-request'),
    windowControl: (command) => ipcRenderer.send('window-control', command),
    sendIntent: (intent) => ipcRenderer.send('sovereign-intent', intent),
    onResponse: (callback) => ipcRenderer.on('sovereign-response', (event, response) => callback(response))
});

console.log('[PRELOAD] Sovereign IPC Handshake Active.');
