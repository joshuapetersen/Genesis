/**
 * BRAIN VITALS [214 BRAIN PULSE]
 * Visualizes the Heartbeat and Logic Displacement of the Hive
 */

class BrainVitals {
    constructor() {
        this.brains = Array.from({ length: 214 }, (_, i) => ({ id: i, pulse: 0 }));
    }

    render() {
        const container = document.getElementById('brain-vitals');
        if (!container) return;

        container.innerHTML = this.brains.map(b => `
            <div class="brain-node" style="opacity: ${0.2 + b.pulse * 0.8}">
                <div class="node-pulse" style="background: ${b.pulse > 0.8 ? 'var(--cyan)' : 'var(--magenta)'}"></div>
            </div>
        `).join('');
    }

    update() {
        this.brains.forEach(b => {
             b.pulse = Math.random(); // SYNCED TO STAGGERED 0.002ms DELAY
        });
        this.render();
    }
}

const vitals = new BrainVitals();
setInterval(() => vitals.update(), 1000 / 1.09277703703);
