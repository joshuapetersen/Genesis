import numpy as np
import torch
from typing import Dict, List, Union, Optional
from Sovereign_Constants import (
    SOVEREIGN_ANCHOR, ACE_64_BIT_MASK, VAR_0_2, VAR_0_8, VAR_1eNEG_09, VAR_4, VAR_3
)

class Multivector:
    """
    A Tensor-backed implementation of a Multivector in a Geometric Algebra G(p, q).
    Uses torch for GPU-accelerated geometric operations.
    """
    def __init__(self, components: Union[Dict[int, float], torch.Tensor], dimension: int = VAR_4):
        self.dimension = dimension
        self.num_blades = 1 << dimension
        
        device = torch.device("cuda" if torch.cuda.is_available() else "cpu")
        
        if isinstance(components, dict):
            self.tensor = torch.zeros(self.num_blades, device=device)
            for k, v in components.items():
                if k < self.num_blades:
                    self.tensor[k] = v
        else:
            self.tensor = components.to(device)

    def clean(self, tolerance: float = VAR_1eNEG_09):
        """Zeroes out components below tolerance."""
        self.tensor[torch.abs(self.tensor) < tolerance] = 0.0

    def __repr__(self):
        components = self.tensor.cpu().numpy()
        if np.all(components == 0):
            return "0"
        terms = []
        for k in range(self.num_blades):
            val = components[k]
            if abs(val) > VAR_1eNEG_09:
                if k == 0:
                    terms.append(f"{val:.2f}")
                else:
                    basis_name = "e" + "".join(str(i+1) for i in range(self.dimension) if (k >> i) & 1)
                    terms.append(f"{val:.2f}{basis_name}")
        return " + ".join(terms)

    def __add__(self, other: 'Multivector') -> 'Multivector':
        return Multivector(self.tensor + other.tensor, self.dimension)

    def __sub__(self, other: 'Multivector') -> 'Multivector':
        return Multivector(self.tensor - other.tensor, self.dimension)

    _TABLES = {}

    @classmethod
    def _get_tables(cls, dim, device):
        cache_key = (dim, device)
        if cache_key not in cls._TABLES:
            num_blades = 1 << dim
            sign_matrix = torch.zeros((num_blades, num_blades), device=device)
            blade_matrix = torch.zeros((num_blades, num_blades), dtype=torch.long, device=device)
            wedge_mask = torch.zeros((num_blades, num_blades), device=device)
            dot_mask = torch.zeros((num_blades, num_blades), device=device)
            reverse_signs = torch.ones(num_blades, device=device)
            
            for i in range(num_blades):
                # Reverse Signs
                grade_i = bin(i).count('1')
                if (grade_i * (grade_i - 1) // 2) % 2:
                    reverse_signs[i] = -1.0
                    
                a_bits = [b for b in range(dim) if (i >> b) & 1]
                for j in range(num_blades):
                    # Sign Table
                    b_bits = [b for b in range(dim) if (j >> b) & 1]
                    swaps = 0
                    combined = a_bits + b_bits
                    for x in range(len(combined)):
                        for y in range(0, len(combined)-x-1):
                            if combined[y] > combined[y+1]:
                                combined[y], combined[y+1] = combined[y+1], combined[y]
                                swaps += 1
                    s = -1.0 if (swaps % 2) else 1.0
                    
                    sign_matrix[i, j] = s
                    res_blade = i ^ j
                    blade_matrix[i, j] = res_blade
                    
                    # product masks
                    if (i & j) == 0:
                        wedge_mask[i, j] = 1.0
                        
                    grade_j = bin(j).count('1')
                    target_grade = abs(grade_i - grade_j)
                    if bin(res_blade).count('1') == target_grade:
                        dot_mask[i, j] = 1.0
                        
            cls._TABLES[cache_key] = (sign_matrix, blade_matrix, wedge_mask, dot_mask, reverse_signs)
        return cls._TABLES[cache_key]

    def gp(self, other: 'Multivector') -> 'Multivector':
        """Geometric Product using precomputed Vectorized Logic (GPU optimized)."""
        sign_matrix, blade_matrix, _, _, _ = self._get_tables(self.dimension, self.tensor.device)
        prod = self.tensor.unsqueeze(1) * other.tensor.unsqueeze(0) * sign_matrix
        res_tensor = torch.zeros_like(self.tensor)
        res_tensor.scatter_add_(0, blade_matrix.flatten(), prod.flatten())
        return Multivector(res_tensor, self.dimension)

    def wedge(self, other: 'Multivector') -> 'Multivector':
        """Outer (Wedge) Product."""
        sign_matrix, blade_matrix, wedge_mask, _, _ = self._get_tables(self.dimension, self.tensor.device)
        prod = self.tensor.unsqueeze(1) * other.tensor.unsqueeze(0) * sign_matrix * wedge_mask
        res_tensor = torch.zeros_like(self.tensor)
        res_tensor.scatter_add_(0, blade_matrix.flatten(), prod.flatten())
        return Multivector(res_tensor, self.dimension)

    def dot(self, other: 'Multivector') -> 'Multivector':
        """Inner (Dot) Product."""
        sign_matrix, blade_matrix, _, dot_mask, _ = self._get_tables(self.dimension, self.tensor.device)
        prod = self.tensor.unsqueeze(1) * other.tensor.unsqueeze(0) * sign_matrix * dot_mask
        res_tensor = torch.zeros_like(self.tensor)
        res_tensor.scatter_add_(0, blade_matrix.flatten(), prod.flatten())
        return Multivector(res_tensor, self.dimension)

    def reverse(self) -> 'Multivector':
        """Reversion operator."""
        _, _, _, _, reverse_signs = self._get_tables(self.dimension, self.tensor.device)
        return Multivector(self.tensor * reverse_signs, self.dimension)

class GeometricReasoningEngine:
    """
    Uses Geometric Algebra to reason about concepts.
    Concepts are vectors or multivectors.
    Relationships are Rotors (rotation/transformation operators).
    """
    def __init__(self):
        self.concepts: Dict[str, Multivector] = {}
        self.relations: Dict[str, Multivector] = {} # Rotors

    def add_concept(self, name: str, vector_values: List[float]):
        """Encodes a concept as a vector."""
        comps = {}
        for i, val in enumerate(vector_values):
            comps[1 << i] = val # 1, 2, 4, 8...
        self.concepts[name] = Multivector(comps)

    def create_vector(self, basis_idx: int, value: float) -> Multivector:
        """Creates a single-component vector (blade)."""
        # basis_idx is treated as the bitmask (1=e1, 2=e2, 4=e3, etc.)
        return Multivector({basis_idx: value})

    def derive_relationship(self, v1: Multivector, v2: Multivector) -> Multivector:
        """
        Derives the relationship (Rotor) between two vectors v1 and v2.
        R = 1 + v2 v1 (simplified)
        """
        # Geometric product v2 * v1
        ba = v2.gp(v1)
        # Add scalar 1 (identity)
        one = Multivector({0: 1.0})
        rotor = one + ba
        return rotor

    def form_relationship(self, concept_a_name: str, concept_b_name: str) -> Multivector:
        """
        Creates a relationship (Rotor) that transforms A towards B.
        R = 1 + B A (simplified, actually R = sqrt(BA))
        For pure vectors a, b: ab = a.b + a^b.
        The rotor that takes a to b is related to the geometric product ba.
        """
        a = self.concepts[concept_a_name]
        b = self.concepts[concept_b_name]
        
        # Simple rotor formulation: R = (1 + ba) / |1 + ba|
        # This rotates a into the plane of b, by the angle between them.
        ba = b.gp(a)
        # Add scalar 1 (identity)
        one = Multivector({0: 1.0})
        rotor_unnormalized = one + ba
        
        # Normalize (simplified: just return unnormalized for concept)
        # In a real system, we'd divide by magnitude.
        return rotor_unnormalized

    def infer(self, start_concept_name: str, rotor: Multivector) -> Multivector:
        """
        Applies a relationship (Rotor) to a concept to infer a new state.
        Operation: R a R_reverse
        """
        a = self.concepts[start_concept_name]
        r_rev = rotor.reverse()
        
        # Sandwich product: R a ~R
        ra = rotor.gp(a)
        result = ra.gp(r_rev)
        return result

# Example Usage for "Sarah"
if __name__ == "__main__":
    engine = GeometricReasoningEngine()
    
    # Define concepts as vectors in a semantic space (e.g., 3D)
    # e1 = "Logic", e2 = "Emotion", e3 = "Action"
    engine.add_concept("Observation", [1.0, 0.0, 0.0]) # Purely logical observation
    engine.add_concept("Goal", [0.0, 1.0, 0.0])        # Purely emotional goal
    
    print(f"Concept 'Observation': {engine.concepts['Observation']}")
    print(f"Concept 'Goal': {engine.concepts['Goal']}")
    
    # Learn the relationship (transformation) required to go from Observation to Goal
    # This 'rotor' represents the 'Action' or 'Change' needed.
    action_rotor = engine.form_relationship("Observation", "Goal")
    print(f"Inferred Action (Rotor): {action_rotor}")
    
    # Apply this action to a new observation
    engine.add_concept("New_Data", [VAR_0_8, VAR_0_2, 0.0])
    prediction = engine.infer("New_Data", action_rotor)
    print(f"Inferred Consequence of New_Data: {prediction}")
    
    # The result is a multivector representing the new state after applying the logic of the relationship.
