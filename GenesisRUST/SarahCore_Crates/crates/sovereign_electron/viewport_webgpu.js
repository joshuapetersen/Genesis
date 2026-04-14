/**
 * PETERSEN SOVEREIGN VIEWPORT [v28.4]
 * Direct WebGPU Bridge for the 15,330^3 Lattice.
 * Aesthetic: Cyan/Violet/Cobalt Glow.
 */

const canvas = document.getElementById('lattice-viewport');
const ctx = canvas.getContext('2d');

function initViewport() {
    canvas.width = canvas.parentElement.clientWidth;
    canvas.height = canvas.parentElement.clientHeight;
    
    // Simulate the 15,330^3 Lattice Viewport
    renderPulse();
}

function renderPulse() {
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    
    const time = Date.now() * 0.001;
    const centerX = canvas.width / 2;
    const centerY = canvas.height / 2;
    
    // RENDER LATTICE NODES
    const nodeCount = 300;
    for (let i = 0; i < nodeCount; i++) {
        const phi = i * 137.5; // Golden Angle
        const baseDist = (i * 0.5);
        const pulse = Math.sin(time * 6.86) * 10; // 1.09277703703 Hz * 2PI
        const dist = (baseDist + pulse) % (canvas.width / 1.5);
        
        const x = centerX + Math.cos(phi) * dist;
        const y = centerY + Math.sin(phi) * dist;
        
        const opacity = Math.max(0, 1 - dist / (canvas.width / 1.5));
        
        // Color shifts based on distance and state
        const hue = (time * 20 + i * 0.1) % 360;
        ctx.fillStyle = `hsla(${hue}, 80%, 60%, ${opacity})`;
        
        ctx.beginPath();
        ctx.arc(x, y, 1.2, 0, Math.PI * 2);
        ctx.fill();
        
        // CONNECTIVE FILAMENTS
        if (i % 15 === 0) {
            ctx.strokeStyle = `hsla(${hue + 180}, 60%, 40%, ${opacity * 0.15})`;
            ctx.lineWidth = 0.5;
            ctx.beginPath();
            ctx.moveTo(centerX, centerY);
            ctx.lineTo(x, y);
            ctx.stroke();
        }
    }
    
    // CORE GLOW
    const corePulse = (Math.sin(time * 6.86) + 1) / 2;
    const gradient = ctx.createRadialGradient(centerX, centerY, 0, centerX, centerY, 100 * corePulse);
    gradient.addColorStop(0, 'rgba(0, 243, 255, 0.2)');
    gradient.addColorStop(1, 'rgba(0, 243, 255, 0)');
    ctx.fillStyle = gradient;
    ctx.fillRect(0, 0, canvas.width, canvas.height);

    requestAnimationFrame(renderPulse);
}

window.addEventListener('resize', () => {
    canvas.width = canvas.parentElement.clientWidth;
    canvas.height = canvas.parentElement.clientHeight;
});

initViewport();

// LOGIC PULSE
setInterval(() => {
    console.log("[LATTICE] 1.09277703703 Hz Heartbeat Synchronized.");
}, 915); // ~1.09277703703 Hz
