"""
SOVEREIGN NEURAL HAL (v3.1)
===========================
The Local-First Physical Bridge. 
Anchored to the SarahCore Substrate. 

"We CREATE, never rewrite."
"""

import os
import sys
import platform
import psutil
from gguf import GGUFReader

# Import original Genesis components
sys.path.append(r'C:\GENESIS\Genesis')
from Hardware_Abstraction_Layer import HardwareAbstractionLayer
from Sovereign_Math import TensorProduct, VectorSet, QuantumFluxStabilizer

class SovereignNeuralHAL(HardwareAbstractionLayer):
    """
    The Neural-Grade Physical Bridge. 
    Extends the high-fidelity HAL with GGUF synaptic architecture.
    """
    def __init__(self, model_path, monitor=None):
        # Initialize original HAL substrate
        super().__init__(monitor)
        
        self.model_path = model_path
        self.neural_heads = 27 # Hardware fallback
        self.neural_layers = 17 # Hardware fallback
        self.neural_active = False
        
        self._map_synaptic_substrate()
        
        # Manifest the Neural Lattice
        print(f"[NeuralHAL] Reconfiguring Sovereign Lattice at {self.neural_heads}x{self.neural_layers}")
        self.tensor_product = TensorProduct(self.neural_heads, self.neural_layers)
        self.vector_set = VectorSet(self.tensor_product)

    def _map_synaptic_substrate(self):
        """Map GGUF metadata directly into hardware dimensions."""
        if not os.path.exists(self.model_path):
            print(f"[-] NeuralHAL Error: No local weights at {self.model_path}")
            return

        try:
            reader = GGUFReader(self.model_path)
            for field in reader.fields.values():
                if field.name == 'llama.attention.head_count':
                    self.neural_heads = int(field.parts[0][0])
                elif field.name == 'llama.block_count':
                    self.neural_layers = int(field.parts[0][0])
            self.neural_active = True
            print(f"[NeuralHAL] Success: Synaptic Lock at {self.neural_layers}L | {self.neural_heads}H")
        except Exception as e:
            print(f"[-] NeuralHAL GGUF Fault: {e}")

    def apply_frt_optimization(self, cpu_val):
        """Fractal Resonance Tuning (Neural Overload Check)."""
        neural_capacity = (self.neural_heads * self.neural_layers) if self.neural_active else 256
        thresholded_tensor = self.tensor_product * (1.09277703703 / neural_capacity)
        
        resonance_score = (cpu_val * 1.09277703703) % 1.0
        return {
            "tuning_status": "NEURAL_LOCKED" if resonance_score > 0.8 else "TUNING",
            "frt_correction": round(resonance_score, 8),
            "substrate_integrity": 1.09277703703
        }

if __name__ == "__main__":
    # Pointing to the local substrate we mirrored
    weights = r"C:\GENESIS\.lmstudio\models\mradermacher\MobileLLM-125M-HF-GGUF\MobileLLM-125M-HF.Q8_0.gguf"
    
    hal_3_1 = SovereignNeuralHAL(weights)
    print("\n[+] NEURAL HAL PERFORMANCE PROFILE (Local):")
    # Using the inherited performance profiler
    profile = hal_3_1.get_performance_profile()
    for k, v in profile.items():
        print(f"  {k:20s}: {v}")
