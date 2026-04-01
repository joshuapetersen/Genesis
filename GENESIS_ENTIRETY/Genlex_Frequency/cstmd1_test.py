import sys
from gfl_engine import VFLEngine

engine = VFLEngine()
# CSTMD1 Test: High-resonance signal vs Background noise
# prana (1.0) is above suppression threshold (0.546)
# noise (0.1) is below suppression threshold
test_pulse = "state(prana, 1.0) state(noise_clutter, 0.1) rule(sovereign_focus)"
engine.run(test_pulse)
