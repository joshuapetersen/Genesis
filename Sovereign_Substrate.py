import os
import sys

# Detection
GPU_AVAILABLE = False
try:
    import cupy as cp
    # Check if a device is actually available to avoid runtime errors
    if cp.cuda.runtime.getDeviceCount() > 0:
        GPU_AVAILABLE = True
    else:
        GPU_AVAILABLE = False
except Exception:
    GPU_AVAILABLE = False

import numpy as np

class SovereignSubstrate:
    """
    Unified Substrate for SarahCore.
    Transparently handles GPU/CPU math logic.
    """
    def __init__(self):
        self.gpu_active = GPU_AVAILABLE
        self.backend = cp if self.gpu_active else np
        self.mode = "GPU_ACCELERATED" if self.gpu_active else "CPU_STABILIZED"
        
        # Expose all top-level backend functions
        self.array = self.backend.array
        self.zeros = self.backend.zeros
        self.ones = self.backend.ones
        self.linspace = self.backend.linspace
        self.copy = self.backend.copy
        self.where = self.backend.where
        self.abs = self.backend.abs
        self.sin = self.backend.sin
        self.cos = self.backend.cos
        self.mean = self.backend.mean
        self.sqrt = self.backend.sqrt
        self.maximum = self.backend.maximum
        self.minimum = self.backend.minimum
        self.sum = self.backend.sum
        self.arange = self.backend.arange
        self.unique = self.backend.unique
        self.random = self.backend.random
        self.argmax = self.backend.argmax
        self.argmin = self.backend.argmin
        self.power = self.backend.power
        
        # Types
        self.float32 = self.backend.float32
        self.float64 = self.backend.float64
        self.int32 = self.backend.int32

    def get_cpu(self, array):
        """Standardizes fetching data to host RAM."""
        if self.gpu_active and hasattr(array, 'get'):
            return array.get()
        return array

    def sync(self, device_id=0):
        """Forces a hardware sync point."""
        if self.gpu_active:
            try:
                cp.cuda.Device(device_id).synchronize()
            except:
                pass

    def get_report(self):
        return {
            "mode": self.mode,
            "backend": str(self.backend.__name__),
            "vram_management": "ACTIVE" if self.gpu_active else "VIRTUAL"
        }

    @property
    def cuda(self):
        """Mock cuda attribute for compatibility."""
        class MockDevice:
            def __init__(self, id): pass
            def __enter__(self): pass
            def __exit__(self, *args): pass
            def synchronize(self): pass
        
        class MockCuda:
            def Device(self, id): return MockDevice(id)
            
        if self.gpu_active:
            return cp.cuda
        return MockCuda()

# Singleton instance
substrate = SovereignSubstrate()
