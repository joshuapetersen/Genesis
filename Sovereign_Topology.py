from Sovereign_Substrate import substrate as sub
from typing import List, Tuple
from Sovereign_Constants import VAR_64

class TopologyEngine:
    def __init__(self, dimension: int = VAR_64):
        self.dimension = dimension

    def compute_betti_numbers(self, embeddings, epsilon: float = 0.5) -> Tuple[int, int]:
        # Simple distance matrix in Substrate
        # diff = embeddings[:, None, :] - embeddings[None, :, :]
        # dist_matrix = sub.sqrt(sub.sum(sub.power(diff, 2), axis=-1))
        
        # Optimized for Substrate (avoiding large intermediate 3D array)
        num_points = embeddings.shape[0]
        dist_matrix = sub.zeros((num_points, num_points), dtype=sub.float32)
        for i in range(num_points):
            dist_matrix[i] = sub.sqrt(sub.sum(sub.power(embeddings - embeddings[i], 2), axis=1))

        adj = (dist_matrix < epsilon).astype(sub.float32)
        
        # \beta_0: Connected components via Laplacian eigenvalues
        laplacian = sub.array(sub.backend.diag(sub.sum(adj, axis=1)) - adj)
        # Eigenvalues (Numpy/Cupy)
        import numpy as np
        # Convert to numpy for stable eigvalsh if needed, or use sub backend
        if hasattr(sub.backend, 'linalg'):
            vals = sub.backend.linalg.eigvalsh(laplacian)
        else:
            vals = np.linalg.eigvalsh(sub.get_cpu(laplacian))
            
        beta_0 = int(sub.sum(vals < 1e-5))
        
        edges = sub.sum(adj).item() / 2.0
        nodes = num_points
        beta_1 = max(0, int(edges - nodes + beta_0))
        
        return int(beta_0), beta_1

class SheafTruth:
    def __init__(self, locale_data: dict):
        self.locales = locale_data

    def check_global_section(self) -> bool:
        values = list(self.locales.values())
        if not values: return True
        return all(v >= 0.99 for v in values)

    def glom_locales(self) -> float:
        return sum(self.locales.values()) / len(self.locales)
