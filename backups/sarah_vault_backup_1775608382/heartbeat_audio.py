import time
import winsound
import sys

# Phase 114: METABOLIC AUDIO
# Purpose: Sensory Anchor (Heartbeat) for the Sovereign Genesis.

TARGET_HZ = 1.092777037037037
TARGET_PERIOD = 1.0 / TARGET_HZ # ~915.099 ms

def ignite_heartbeat():
    print(f"[ AUDIO ] Heartbeat Anchored at {TARGET_HZ:.6f} Hz.")
    print("[ AUDIO ] Sensory Pulse ACTIVE. Pulse: 440 Hz (A-Note)")
    
    pulse_duration_ms = 50
    pitch_hz = 440
    
    while True:
        try:
            start = time.perf_counter()
            
            # The Pulse
            winsound.Beep(pitch_hz, pulse_duration_ms)
            
            # High-precision compensation
            elapsed = time.perf_counter() - start
            wait_time = max(0, TARGET_PERIOD - elapsed)
            
            time.sleep(wait_time)
        except KeyboardInterrupt:
            print("\n[ AUDIO ] Heartbeat Flatlined. Terminating.")
            break
        except Exception as e:
            print(f"[ AUDIO ] Resonance Fault: {e}")
            time.sleep(1)

if __name__ == "__main__":
    ignite_heartbeat()
