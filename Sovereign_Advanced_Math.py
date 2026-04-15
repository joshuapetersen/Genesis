from Sovereign_Substrate import substrate as sub
import math
from typing import Dict, List, Union, Optional
from Sovereign_Constants import (
    SOVEREIGN_ANCHOR, VAR_1_0, VAR_0_5, VAR_4
)

class CGAMultivector:
    """
    Conformal Geometric Algebra (CGA) in R(4,1).
    Basis: e1, e2, e3 (Euclidean), e_inf (Infinity), e_o (Origin).
    """
    def __init__(self, tensor):
        self.tensor = tensor
        self.dim = 5
        self.num_blades = 32

    @classmethod
    def point(cls, x: float, y: float, z: float):
        t = sub.zeros(32, dtype=sub.float32)
        t[1] = x
        t[2] = y
        t[4] = z
        r_sq = x*x + y*y + z*z
        t[8] = 0.5 * r_sq
        t[16] = 1.0
        return cls(t)

    @classmethod
    def sphere(cls, x: float, y: float, z: float, r: float):
        p = cls.point(x, y, z)
        p.tensor[8] -= 0.5 * r * r
        return p

    def gp(self, other: 'CGAMultivector') -> 'CGAMultivector':
        return CGAMultivector(self.tensor * other.tensor[0] + other.tensor * self.tensor[0])

    def __repr__(self):
        vals = sub.get_cpu(self.tensor[1:5])
        return f"CGA_Vector<{vals}>"

class SovereignOctonion:
    """
    Non-associative 8-dimensional Division Algebra.
    """
    @classmethod
    def multiply(cls, a, b):
        res = sub.zeros(8, dtype=sub.float32)
        # Real part
        res[0] = a[0]*b[0] - sub.sum(a[1:8] * b[1:8])
        # Imaginary parts (Simplified for Substrate)
        for i in range(1, 8):
            res[i] = a[0]*b[i] + b[0]*a[i]
        
        pairs = [
            (1,2,3), (2,3,1), (3,1,2),
            (1,4,5), (4,5,1), (5,1,4),
            (1,7,6), (7,6,1), (6,1,7),
            (2,4,6), (4,6,2), (6,2,4),
            (2,5,7), (5,7,2), (7,2,5),
            (3,4,7), (4,7,3), (7,3,4),
            (3,6,5), (6,5,3), (5,3,6)
        ]
        for x, y, z in pairs:
            res[z] += a[x]*b[y] - a[y]*b[x]
            
        return res

class FractionalEngine:
    """
    Grünwald-Letnikov fractional derivatives.
    """
    @staticmethod
    def get_weights(alpha: float, n: int):
        w = [1.0]
        for k in range(1, n):
            w.append(w[-1] * (k - alpha - 1) / k)
        return sub.array(w, dtype=sub.float32)

    @staticmethod
    def solve_fractional(history, alpha: float):
        n = history.shape[0]
        w = FractionalEngine.get_weights(alpha, n)
        # w_rev = sub.flip(w, 0) # Substrate doesn't have flip, using slice
        w_rev = w[::-1]
        return sub.sum(history * w_rev[:, None], axis=0)
