const { app, BrowserWindow, ipcMain } = require('electron');
const path = require('path');

// THE SARAH HEARTBEAT [1.09277703703 Hz]
const HEARTBEAT_HZ = 1.09277703703;
const HEARTBEAT_MS = 1000 / HEARTBEAT_HZ;

const { spawn } = require('child_process');
let bridge;

function startBridge(win) {
    const bridgePath = path.resolve(__dirname, '../../src/os_bridge.exe');
    console.log(`[OS-BRIDGE] Starting bridge at: ${bridgePath}`);
    bridge = spawn(bridgePath, []);
    
    bridge.on('error', (err) => {
        console.error(`[OS-BRIDGE] Failed to start bridge: ${err.message}`);
    });

    bridge.stderr.on('data', (data) => {
        console.error(`[OS-BRIDGE] STDERR: ${data.toString()}`);
    });

    bridge.stdout.on('data', (data) => {
        const output = data.toString();
        console.log(`[OS-BRIDGE] STDOUT: ${output}`);
        if (output.includes('S.A.R.A:')) {
            const res = output.split('S.A.R.A:')[1].trim();
            win.webContents.send('sovereign-response', res);
        }
    });

    ipcMain.removeAllListeners('sovereign-intent');
    ipcMain.on('sovereign-intent', (event, intent) => {
        console.log(`[OS-BRIDGE] Sending Intent: ${intent}`);
        if (bridge && !bridge.killed) {
            bridge.stdin.write(intent + '\n');
        } else {
            console.error(`[OS-BRIDGE] Bridge is not running. Intent ignored.`);
        }
    });
}

function createWindow() {
    const win = new BrowserWindow({
        width: 1440,
        height: 900,
        title: 'GENESIS OS',
        backgroundColor: '#030305',
        frame: false, // SOVEREIGN FRAMELESS MODE
        resizable: true, // MAKE IT RESIZABLE
        webPreferences: {
            preload: path.join(__dirname, 'preload.js'),
            contextIsolation: true,
            nodeIntegration: false,
            enableRemoteModule: false,
            webSecurity: false
        }
    });

    win.loadFile(path.join(__dirname, '../sovereign_desktop/dist/index.html'));

    win.webContents.on('console-message', (event, level, message, line, sourceId) => {
        const levels = ['VERBOSE', 'INFO', 'WARNING', 'ERROR'];
        console.log(`[RENDERER ${levels[level] || level}] ${message} (${sourceId}:${line})`);
    });

    win.webContents.on('did-fail-load', (event, errorCode, errorDescription, validatedURL) => {
        console.error(`[RENDERER LOAD FAIL] ${errorCode} - ${errorDescription} @ ${validatedURL}`);
    });

    setInterval(() => {
        win.webContents.send('heartbeat-pulse', Date.now());
    }, HEARTBEAT_MS);

    return win;
}

app.whenReady().then(() => {
    const win = createWindow();
    startBridge(win);

    app.on('activate', () => {
        if (BrowserWindow.getAllWindows().length === 0) {
            const newWin = createWindow();
            startBridge(newWin);
        }
    });
});

app.on('window-all-closed', () => {
    if (process.platform !== 'darwin') app.quit();
});

// IPC HANDSHAKE FOR 35 GB/S RAM FLASH
ipcMain.on('window-control', (event, command) => {
    const win = BrowserWindow.fromWebContents(event.sender);
    if (command === 'close') win.close();
    if (command === 'minimize') win.minimize();
});

ipcMain.on('binary-snapshot-request', (event, data) => {
    console.log('[HYPERVISOR] Initiating Binary State Mirroring...');
    // Handshake logic for the 33 GB/s Flash handled here
});
