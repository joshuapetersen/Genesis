const { ipcRenderer } = require('electron');

/// SOVEREIGN DESKTOP UI (GSK v24.1 SHARP)
/// Architecture: DESKTOP-FORTRESS Native Bridge.

document.addEventListener('DOMContentLoaded', () => {
    const logFlow = document.getElementById('log-flow');
    const heatmap = document.getElementById('heatmap');
    const neuralInput = document.getElementById('neural-input');

    // 1. Initialize TTRS-17 Heatmap (64 cells)
    for (let i = 0; i < 64; i++) {
        const cell = document.createElement('div');
        cell.className = 'heat-cell';
        heatmap.appendChild(cell);
    }

    // 2. NATIVE TELEMETRY RECEIVER
    ipcRenderer.on('hive-telemetry', (event, data) => {
        const p = document.createElement('p');
        p.className = 'ln';
        p.innerText = `[${new Date().toLocaleTimeString()}] ${data.trim()}`;
        logFlow.appendChild(p);
        
        // Auto-scroll and cleanup
        logFlow.scrollTop = logFlow.scrollHeight;
        if (logFlow.children.length > 25) logFlow.removeChild(logFlow.firstChild);

        // Heatmap Jitter on telemetry burst
        const cells = document.querySelectorAll('.heat-cell');
        cells[Math.floor(Math.random() * 64)].classList.add('hot');
        setTimeout(() => cells.forEach(c => c.classList.remove('hot')), 500);
    });

    // 3. NATIVE COMMAND DISPATCH
    neuralInput.addEventListener('keypress', (e) => {
        if (e.key === 'Enter') {
            const cmd = neuralInput.value;
            if (!cmd) return;
            
            // Dispatch to Rust Backend via Electron IPC
            ipcRenderer.send('dispatch-command', cmd);
            
            neuralInput.value = '';
        }
    });

    console.log("[DESKTOP-FORTRESS] Native IPC Bridge: ACTIVE.");
});
