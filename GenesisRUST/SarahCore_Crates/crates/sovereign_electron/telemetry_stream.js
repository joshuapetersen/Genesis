/**
 * TELEMETRY STREAM [PORT 8080]
 * Pulls the raw Logic Manifest from the C++/Node Bridge
 */

class TelemetryStream {
    constructor(port = 8080) {
        this.port = port;
        this.logs = [];
    }

    pushLog(msg) {
        const container = document.getElementById('log-stream');
        if (!container) return;

        const logLine = document.createElement('div');
        logLine.className = 'log-line';
        logLine.innerText = `[${new Date().toLocaleTimeString()}] ${msg}`;
        container.appendChild(logLine);

        if (container.childNodes.length > 100) container.removeChild(container.firstChild);
        container.scrollTop = container.scrollHeight;
    }
}

const telemetry = new TelemetryStream();
// SIMULATE 8080 STREAM
setInterval(() => {
    telemetry.pushLog('Substrate Displacement: 33.41 GiB/s | C3-Lattice-Sync-Success');
}, 500);
