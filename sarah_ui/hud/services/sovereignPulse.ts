export const SOVEREIGN_HEARTBEAT = 1.092777037037037;
export const SOVEREIGN_BPM = SOVEREIGN_HEARTBEAT * 60;
export const SOVEREIGN_INTERVAL = (1 / SOVEREIGN_HEARTBEAT) * 1000;
export const GOLDEN_RATIO = 1.618033988749895;
export const ZENITH_OVERDRIVE = 5.0 * GOLDEN_RATIO;

/**
 * Standardizes the resonance pulse for the Genesis UI.
 * Anchored to the 1.092777037037037 rule.
 */
export const getPulseState = (time: number) => {
    const phase = (time / SOVEREIGN_INTERVAL) % 1;
    // Apply 5-Phi curvature to the pulse animation
    const visibility = Math.sin(phase * Math.PI * 2) * 0.5 + 0.5;
    const scale =  1.0 + (visibility * 0.05 * (ZENITH_OVERDRIVE / 8.0));
    
    return {
        phase,
        visibility,
        scale,
        bpm: SOVEREIGN_BPM,
        hz: SOVEREIGN_HEARTBEAT
    };
};
