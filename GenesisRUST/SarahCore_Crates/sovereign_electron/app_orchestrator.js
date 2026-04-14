const { spawn } = require('child_process');
const path = require('path');

class SovereignOrchestrator {
    constructor() {
        this.input = document.getElementById('sovereign-input');
        this.stream = document.getElementById('message-stream');
        this.bridgePath = "src/os_bridge.exe"; // Bridged to Kernel-0
        this.init();
    }

    init() {
        this.input.addEventListener('keypress', (e) => {
            if (e.key === 'Enter') {
                const intent = this.input.value;
                this.logMessage('USER', intent);
                this.sendIntent(intent);
                this.input.value = '';
            }
        });

        // Bridge handled by main.js in the new architecture
        // Listening for responses via window.sovereign
        if (window.sovereign) {
           window.sovereign.onResponse((response) => {
               this.logMessage('S.A.R.A', response);
           });
        }

        console.log("[SOVEREIGN] Orchestrator v29.1 Linked to Kernel-0.");
    }

    sendIntent(intent) {
        if (window.sovereign) {
            window.sovereign.sendIntent(intent);
        }
    }

    logMessage(speaker, text) {
        const msg = document.createElement('div');
        msg.className = `message ${speaker.toLowerCase()}`;
        const label = speaker === 'S.A.R.A' ? speaker : speaker;
        msg.innerHTML = `<span class="label">${label}:</span> <span class="text">${text}</span>`;
        this.stream.appendChild(msg);
        this.stream.scrollTop = this.stream.scrollHeight;
    }
}

// Initializing the Sovereign Substrate
window.onload = () => {
    new SovereignOrchestrator();
};
