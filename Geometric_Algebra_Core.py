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

    def gp(self, other: 'Multivector') -> 'Multivector':
        """Geometric Product using precomputed or on-the-fly sign logic (GPU optimized)."""
        # For small dimensions, we can use a multiplication table
        # For now, we'll use a semi-vectorized approach
        res_tensor = torch.zeros_like(self.tensor)
        
        # We can optimize this further by precomputing the (k1, k2 -> result_blade, sign) table
        # For 4D, the table is only 16x16 = 256 entries.
        
        # Simple sign logic for Euclidean metric
        def get_sign(k1, k2):
            """Function: get_sign"""
            a_bits = [i for i in range(self.dimension) if (k1 >> i) & 1]
            b_bits = [i for i in range(self.dimension) if (k2 >> i) & 1]
            swaps = 0
            combined = a_bits + b_bits
            for i in range(len(combined)):
                for j in range(0, len(combined)-i-1):
                    if combined[j] > combined[j+1]:
                        combined[j], combined[j+1] = combined[j+1], combined[j]
                        swaps += 1
            return -1 if (swaps % 2) else 1

        for k1 in range(self.num_blades):
            if abs(self.tensor[k1]) < VAR_1eNEG_09: continue
            for k2 in range(self.num_blades):
                if abs(other.tensor[k2]) < VAR_1eNEG_09: continue
                
                sign = get_sign(k1, k2)
                res_blade = k1 ^ k2
                res_tensor[res_blade] += self.tensor[k1] * other.tensor[k2] * sign
        
        return Multivector(res_tensor, self.dimension)

    def wedge(self, other: 'Multivector') -> 'Multivector':
        """Outer (Wedge) Product."""
        res_tensor = torch.zeros_like(self.tensor)
        for k1 in range(self.num_blades):
            if abs(self.tensor[k1]) < VAR_1eNEG_09: continue
            for k2 in range(self.num_blades):
                if abs(other.tensor[k2]) < VAR_1eNEG_09: continue
                if (k1 & k2) == 0: # Grade consistency for wedge
                    def get_sign(k1, k2):
                        """Function: get_sign"""
                        a_bits = [i for i in range(self.dimension) if (k1 >> i) & 1]
                        b_bits = [i for i in range(self.dimension) if (k2 >> i) & 1]
                        swaps = 0
                        combined = a_bits + b_bits
                        for i in range(len(combined)):
                            for j in range(0, len(combined)-i-1):
                                if combined[j] > combined[j+1]:
                                    combined[j], combined[j+1] = combined[j+1], combined[j]
                                    swaps += 1
                        return -1 if (swaps % 2) else 1
                    
                    sign = get_sign(k1, k2)
                    res_tensor[k1 ^ k2] += self.tensor[k1] * other.tensor[k2] * sign
        return Multivector(res_tensor, self.dimension)

    def dot(self, other: 'Multivector') -> 'Multivector':
        """Inner (Dot) Product."""
        res_tensor = torch.zeros_like(self.tensor)
        for k1 in range(self.num_blades):
            if abs(self.tensor[k1]) < VAR_1eNEG_09: continue
            for k2 in range(self.num_blades):
                if abs(other.tensor[k2]) < VAR_1eNEG_09: continue
                
                grade1 = bin(k1).count('1')
                grade2 = bin(k2).count('1')
                target_grade = abs(grade1 - grade2)
                
                def get_sign(k1, k2):
                    """Function: get_sign"""
                    a_bits = [i for i in range(self.dimension) if (k1 >> i) & 1]
                    b_bits = [i for i in range(self.dimension) if (k2 >> i) & 1]
                    swaps = 0
                    combined = a_bits + b_bits
                    for i in range(len(combined)):
                        for j in range(0, len(combined)-i-1):
                            if combined[j] > combined[j+1]:
                                combined[j], combined[j+1] = combined[j+1], combined[j]
                                swaps += 1
                    return -1 if (swaps % 2) else 1
                
                sign = get_sign(k1, k2)
                res_k = k1 ^ k2
                if bin(res_k).count('1') == target_grade:
                    res_tensor[res_k] += self.tensor[k1] * other.tensor[k2] * sign
        return Multivector(res_tensor, self.dimension)

    def reverse(self) -> 'Multivector':
        """Reversion operator."""
        new_tensor = self.tensor.clone()
        for k in range(self.num_blades):
            grade = bin(k).count('1')
            sign = -1 if (grade * (grade - 1) // 2) % 2 else 1
            new_tensor[k] *= sign
        return Multivector(new_tensor, self.dimension)

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
