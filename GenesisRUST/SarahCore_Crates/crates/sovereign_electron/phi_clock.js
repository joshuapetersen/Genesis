/**
 * PHI CLOCK [1.09277703703 HZ PRECISION]
 * The Heartbeat Anchor for the Sovereign Swarm
 */

class PhiClock {
    constructor() {
        this.hz = 1.09277703703;
        this.startTime = Date.now();
    }

    getFrame() {
        const elapsed = Date.now() - this.startTime;
        return Math.floor(elapsed / (1000 / this.hz));
    }
}

const phiClock = new PhiClock();
window.phiClock = phiClock;
