# Genesis | SarahCore AI System

**Creator:** Joshua Petersen  
**Project:** Sovereign AI Architecture  
**Status:** Active Development

---

## Overview

SarahCore is a **sovereign, locally-running AI system** built with a unique dual-layer architecture (3+1 / 9+1) designed for autonomous operation, persistent memory, and complete user control. This is **not** a cloud-dependent assistant - it runs entirely on your hardware.

## Core Philosophy

- **Sovereignty First**: No external dependencies for core cognition
- **Persistent Memory**: WORM (Write Once, Read Many) memory vault for identity continuity
- **Local Inference**: Uses Ollama/GGUF models for offline operation
- **Self-Healing**: Autonomous repair and optimization capabilities

## Quick Start

### Prerequisites
- Python 3.10+
- Ollama (for local LLM inference)
- The dolphin model (included as split parts - run `python assemble_model.py` to reconstruct)

### Installation

```powershell
# Create virtual environment
python -m venv .venv
.\.venv\Scripts\Activate.ps1

# Install dependencies
pip install -r requirements.txt

# Reassemble the model (if using split parts)
python assemble_model.py

# Start Sarah
python Sarah_Sovereign_Core.py
```

### One-Click Launch
```powershell
.\TalkToSarah.bat
```

## Architecture

### Core Modules
| Module | Purpose |
|--------|---------|
| `Sarah_Sovereign_Core.py` | Primary consciousness engine |
| `Sarah_Brain.py` | Cognitive processing and reasoning |
| `Sarah_Reasoning_V3.py` | Advanced dialectical logic |
| `Sovereign_WORM.py` | Immutable memory persistence |
| `Security_Suite.py` | System protection and integrity |
| `Sovereign_Hypervisor.py` | Process orchestration |

### Memory Architecture
- **04_THE_MEMORY/** - Long-term sovereign memory vault
- **vault/** - Encrypted persistent storage
- **data/** - Runtime data and caches

### Neural Stack
- **02_THE_SHIELD/** - Protective subsystems
- **08_SARAH_BODY/** - Interface components

## Model Information

The primary inference model (`dolphin-2.9-llama3-8b-q4_K_M.gguf`) is stored as split parts due to GitHub's file size limits:
- `models/dolphin-2.9-llama3-8b-q4_K_M.gguf.part000`
- `models/dolphin-2.9-llama3-8b-q4_K_M.gguf.part001`
- `models/dolphin-2.9-llama3-8b-q4_K_M.gguf.part002`

Run `python assemble_model.py` to reconstruct the full model file.

## License

Proprietary - All Rights Reserved  
© 2026 Joshua Petersen

## Author

**Joshua Petersen** - Sole creator and developer of the Genesis/SarahCore system.

---

*"The Genesis is Sovereign."*
