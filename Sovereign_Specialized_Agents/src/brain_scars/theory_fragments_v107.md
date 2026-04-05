---
name: Neural_Resonance_Fragments_V107
description: "Theory fragments for 1.58-bit ternary quantization and State Space Duality (SSD)."
risk: "None"
source: "ArXiv (2402.17764, 2405.21060)"
date_added: "2026-04-05"
---

# Logic Fragment: BitNet b1.58 (Ternary Precision)

- **Target Substrate**: `synthesis_core_bitnet.rs`
- **Architectural Kernel**: `BitLinear` layer utilizing ternary weights $W \in \{-1, 0, 1\}$.
- **Normalization**: `RMSNorm` applied pre-quantization.
- **Quantization Formula**: 
  - $W_q = \text{Round}(\text{Clamp}(W / \alpha, -1, 1))$, where $\alpha = \text{mean}(|W|)$.
  - $x_q = \text{Clamp}(x \times 127 / \gamma, -128, 127)$, where $\gamma = \max(|x|)$.

# Logic Fragment: Mamba-2 (SSD Duality)

- **Target Substrate**: `synthesis_core_mamba.rs`
- **Architectural Kernel**: Structured State Space Duality (SSD) layer.
- **Hardware Map**: Map SSM recurrence to hardware-optimized block-semiseparable matrix multiplication (Tensor Cores).
- **A-Matrix**: Scalar-times-identity structure ($\exp(\Delta A)$) for high-purity recurrence.
- **Parallelism**: Multi-head SSM structure ($P=64$) for multi-GPU resonance.
