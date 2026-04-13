const { app, BrowserWindow, ipcMain } = require('electron');
const { spawn } = require('child_process');
const path = require('path');

let hiveProcess;

function createWindow() {
    const win = new BrowserWindow({
        width: 1400,
        height: 900,
        titleBarStyle: 'hidden',
        backgroundColor: '#030305',
        webPreferences: {
            nodeIntegration: true,
            contextIsolation: false
        }
    });

    win.loadFile('index.html');

    // NATIVE IPC BRIDGE (SOVEREIGN SKILL: DESKTOP_FORTRESS)
    // Launching the Rust Orchestrator as a Native Subprocess
    hiveProcess = spawn('cargo', ['run', '-p', 'sovereign_orchestrator', '--quiet']);

    hiveProcess.stdout.on('data', (data) => {
        // Send real-time hive logs and stats to the UI
        win.webContents.send('hive-telemetry', data.toString());
    });

    hiveProcess.stderr.on('data', (data) => {
        console.error(`[RUST-HIVE-ERR]: ${data}`);
    });
}

ipcMain.on('dispatch-command', (event, cmd) => {
    console.log(`[DESKTOP-FORTRESS] Dispatching: ${cmd}`);
    // Direct IPC write to Rust stdin
    if (hiveProcess && hiveProcess.stdin) {
        hiveProcess.stdin.write(`${cmd}\n`);
    }
});

app.whenReady().then(createWindow);

app.on('window-all-closed', () => {
    if (hiveProcess) hiveProcess.kill();
    if (process.platform !== 'darwin') app.quit();
});
