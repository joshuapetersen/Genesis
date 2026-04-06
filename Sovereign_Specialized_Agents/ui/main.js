// Sovereign Hive Dashboard - Real-Time Resonance Logic (V-120.0)

const logContent = document.getElementById('log-content');
const agentCount = document.getElementById('agent-count');
const jitterValue = document.getElementById('lattice-jitter');
const heartbeatDisplay = document.getElementById('heartbeat-value');
const purgeBtn = document.getElementById('purge-btn');

let ws;
const SOCKET_URL = 'ws://127.0.0.1:9001';

function connect() {
    addLog(`[!] INITIATING HIVE_UI CONNECTION TO ${SOCKET_URL}...`);
    ws = new WebSocket(SOCKET_URL);

    ws.onopen = () => {
        addLog('[!] SUBSTRATE BRIDGE ESTABLISHED. RESONANCE SYNC ACTIVE.');
        document.body.style.border = '2px solid var(--antigravity)';
    };

    ws.onmessage = (event) => {
        const data = JSON.parse(event.data);
        updateUI(data);
    };

    ws.onclose = () => {
        addLog('[!] SUBSTRATE BRIDGE DISCONNECTED. RECONNECTING...');
        document.body.style.border = '2px solid var(--sarah)';
        setTimeout(connect, 2000);
    };

    ws.onerror = (err) => {
        addLog(`[ERROR] RESIDUE JITTER DETECTED: ${err.message}`);
    };
}

function updateUI(data) {
    if (data.agent_count) agentCount.textContent = data.agent_count;
    if (data.jitter) jitterValue.textContent = `${data.jitter.toFixed(4)}ms`;
    if (data.heartbeat) heartbeatDisplay.textContent = data.heartbeat.toFixed(6);
    
    if (data.logs) {
        data.logs.forEach(log => addLog(log));
    }
}

function addLog(text) {
    const timestamp = new Date().toLocaleTimeString();
    logContent.innerText += `\n[${timestamp}] ${text}`;
    logContent.scrollTop = logContent.scrollHeight;
}

purgeBtn.addEventListener('click', () => {
    const confirmed = confirm("WARNING: ACTIVATE GLOBAL SUBSTRATE PURGE? This will instantly terminate all hive processes.");
    if (confirmed) {
        addLog("[CRITICAL] INITIATING GLOBAL PURGE STRIKE...");
        ws.send(JSON.stringify({ command: 'SUBSTRATE_PURGE', ace_signature: 'PENDING_USER_ACK' }));
    }
});

// Start connection resonance
connect();
