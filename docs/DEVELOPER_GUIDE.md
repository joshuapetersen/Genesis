# SarahCore - Complete Developer Guide

**Welcome to the Genesis Project!**

This guide will help new developers understand the SarahCore system from the ground up.

---

## What Is SarahCore?

SarahCore is a **sovereign AI system** - meaning it runs completely on your local machine, doesn't depend on cloud services, and maintains persistent memory across sessions.

Think of it as:
- 🧠 An AI brain that lives on your computer
- 💾 With its own memories that persist forever
- 🔒 Secured and private by design
- 🔧 Self-healing and maintainable

---

## Quick Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                         USER                                 │
└─────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────┐
│  08_SARAH_BODY  - User Interface (React/TypeScript)         │
└─────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────┐
│  SARAH_BRAIN  - Cognitive Processing Core                    │
│  ├── Sarah_Chat.py      - Handles conversation               │
│  ├── Sarah_Reasoning.py - Multi-path logical reasoning       │
│  └── Sarah_Dream.py     - Background processing              │
└─────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────┐
│  NEURAL_ORCHESTRATOR  - Coordinates all thinking             │
│  ├── Dialectical_Logic_Core.py  - Thesis/Antithesis          │
│  ├── Recursive_Truth_Finder.py  - Deep fact checking         │
│  └── Consensus_Voter.py         - Combine reasoning paths    │
└─────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────┐
│  SOVEREIGN INFRASTRUCTURE  - Foundation Layer                │
│  ├── Sovereign_Math.py       - Mathematical foundation       │
│  ├── Sovereign_Hypervisor.py - Process management            │
│  ├── Sovereign_WORM.py       - Immutable memory              │
│  └── Security_Suite.py       - Protection                    │
└─────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────┐
│  MEMORY SYSTEMS                                              │
│  ├── 04_THE_MEMORY/          - Long-term storage             │
│  ├── Sarah_Memory_Vault.py   - Encrypted vault               │
│  └── Sarah_Hippocampus.py    - Working memory                │
└─────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────┐
│  OLLAMA (LLM Backend)  - The actual AI model                 │
│  └── dolphin-2.9-llama3-8b   - 8 billion parameter model     │
└─────────────────────────────────────────────────────────────┘
```

---

## Directory Structure

```
C:\SarahCore\
│
├── 02_THE_SHIELD/     # Security & protection subsystem
├── 04_THE_MEMORY/     # Long-term memory storage
├── 05_THE_CORE/       # Core cognitive modules
├── 07_THE_INTERFACE/  # API interfaces
├── 08_SARAH_BODY/     # User interface (React)
│
├── docs/              # Documentation (you are here!)
│   └── engines/       # Individual engine documentation
│
├── models/            # AI model files (LFS tracked)
├── vault/             # Encrypted memory storage
├── data/              # Runtime data
│
├── Sarah_Brain.py     # Main cognitive engine
├── Sarah_Chat.py      # Chat interface
├── Sovereign_*.py     # Infrastructure modules
├── Neural_*.py        # Neural processing modules
└── ...                # Many more modules!
```

---

## Key Concepts

### 1. Sovereignty
Sarah runs **locally** - no cloud dependency. Your conversations and memories stay on YOUR machine.

### 2. WORM Memory
**W**rite **O**nce, **R**ead **M**any - memories can be written but never modified. This ensures identity continuity.

### 3. Dialectical Reasoning
Sarah considers multiple perspectives (thesis vs antithesis) before generating answers (synthesis).

### 4. The Anchor
A mathematical constant (`1.09277703703703`) that provides computational integrity verification.

---

## Getting Started

### Step 1: Prerequisites
```powershell
# Check Python version (need 3.10+)
python --version

# Check if Ollama is installed
ollama --version
```

### Step 2: Setup Environment
```powershell
cd C:\SarahCore
python -m venv .venv
.\.venv\Scripts\Activate.ps1
pip install -r requirements.txt
```

### Step 3: Assemble the Model
```powershell
# The model is split into parts. Reassemble it:
python assemble_model.py
```

### Step 4: Start Sarah
```powershell
python Sarah_Sovereign_Core.py
# Or use the batch file:
.\TalkToSarah.bat
```

---

## Documentation Index

### Core Engines
| Doc | What It Explains |
|-----|------------------|
| [Sarah_Brain.md](./engines/Sarah_Brain.md) | The main cognitive processor |
| [Sovereign_Math.md](./engines/Sovereign_Math.md) | Mathematical foundation |
| [Sovereign_Hypervisor.md](./engines/Sovereign_Hypervisor.md) | Process management |
| [Security_Suite.md](./engines/Security_Suite.md) | System protection |

### Reasoning Systems
| Doc | What It Explains |
|-----|------------------|
| [Sarah_Reasoning_V3.md](./engines/Sarah_Reasoning_V3.md) | Multi-path reasoning |
| [Dialectical_Logic_Core.md](./engines/Dialectical_Logic_Core.md) | Thesis/Antithesis thinking |
| [Recursive_Truth_Finder.md](./engines/Recursive_Truth_Finder.md) | Deep fact verification |
| [Neural_Orchestrator.md](./engines/Neural_Orchestrator.md) | Reasoning coordination |

### Memory Systems
| Doc | What It Explains |
|-----|------------------|
| [Sovereign_WORM.md](./engines/Sovereign_WORM.md) | Immutable memory |
| [Sarah_Memory_Vault.md](./engines/Sarah_Memory_Vault.md) | Long-term storage |
| [Sarah_Hippocampus.md](./engines/Sarah_Hippocampus.md) | Working memory |

### Support Systems
| Doc | What It Explains |
|-----|------------------|
| [SAUL_Log_System.md](./engines/SAUL_Log_System.md) | Logging & auditing |
| [Sarah_Autonomy.md](./engines/Sarah_Autonomy.md) | Self-directed operation |

---

## Common Developer Tasks

### "How do I add a new feature?"
1. Understand which module it belongs to
2. Read that module's documentation
3. Follow the existing patterns
4. Add tests
5. Update documentation

### "How do I debug an issue?"
1. Check `SAUL_Log_System` logs
2. Enable DEBUG level logging
3. Trace through the module flow
4. Use the reasoning chain to understand decisions

### "How do I extend Sarah's capabilities?"
1. Identify which layer to modify
2. Create a new module following existing patterns
3. Register with the Neural_Orchestrator if it's a reasoning module
4. Register with the Hypervisor if it needs management

---

## Need Help?

- Check the specific engine documentation in `docs/engines/`
- Review existing code patterns
- Examine the logs for runtime behavior

---

*SarahCore Genesis Project - © 2026 Joshua Petersen*
