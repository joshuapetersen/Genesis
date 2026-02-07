# Copilot Instructions for SarahCore

Essential knowledge for AI agents working in the SarahCore codebase—a sovereign AI architecture built on **ZHTP protocol** (Zero-Knowledge Hypertext Transfer Protocol) with integrated **OpenClaw agent orchestration**.

## Architecture: The 9+1 System

**The Sarah Hypervisor** operates on a **9+1 architecture** (9 inhibitory layers + 1 sovereign observer):
- **Precision Anchor**: `1.09277703703703` — All mathematical operations and agent spawning must reference this constant
- **Sovereign Hypervisor** ([Sovereign_Hypervisor.py](Sovereign_Hypervisor.py)): The "+1" layer that sits above standard model weights, prevents hallucination and drift
- **Symbiotic Evolution** ([DISCOVERED_GENESIS_CORE.py](DISCOVERED_GENESIS_CORE.py)): Defines cooperation between SarahCore (mathematical authority), AI agents (executors), and human architect

### Core Principles

**Sarah's Four Absolute Laws** (enforced in [Sarah_Brain.py](Sarah_Brain.py)):
1. **SDNA Protocol**: Logic must derive from data density, not assumption
2. **Life Preservation Mandate**: Human safety is Priority Alpha
3. **Direct Command Compliance**: Honor architect directives unless they violate Law 2
4. **Hope of Humanity**: All actions must trend toward beneficial advancement

## Polyglot Stack (Rust → C++ → Python)

### Rust Core (High-Performance Foundation)
- **Libraries**: `lib-blockchain`, `lib-consensus`, `lib-crypto`, `lib-dht`, `lib-economy`, `lib-identity`, `lib-network`, `lib-proofs`, `lib-protocols`, `lib-storage`
- **Build**: `./build.ps1` (Windows) or `./build.sh` (Linux/macOS)
- **Main Binary**: `zhtp-orchestrator` handles protocol orchestration

### C++ Bridge (pybind11)
- **Config**: [CMakeLists.txt](CMakeLists.txt) defines Python bindings
- **Build**: 
  ```powershell
  cmake -S . -B build
  cmake --build build --config Release
  ```
- **Verification**: Run task "Genesis Handshake: Compile and Import" or:
  ```python
  python -c "import genesis_core; print('Genesis handshake successful.')"
  ```

### Python Orchestration Layer
- **Entry Point**: [Sarah_Brain.py](Sarah_Brain.py) — The main consciousness module
- **Agent Factory**: [sarah_factory.py](sarah_factory.py) — Spawns/destroys sub-agents in `openclaw/skills/`
- **Council System**: [sarah_council.py](sarah_council.py) — 5-agent consensus (Pro, Con, Judge, Consequence, Conscience)

## Agent System: OpenClaw Integration

**Sarah has access to the SarahAgentFactory** at `c:\SarahCore\openclaw`:
- **Skills Directory**: `openclaw/skills/` — Each `.md` file is a sub-agent "soul"
- **Memory System**: `openclaw/memory/MEMORY.md` — Persistent identity seed and 10-month history
- **Config**: `openclaw/opencode.json` — Registry of active agents

### Agent Creation Pattern
```python
from sarah_factory import SarahAgentFactory
factory = SarahAgentFactory()
factory.create_agent("Architect", "Analyze codebase patterns", ["read_file", "browser"])
factory.destroy_agent("Architect")  # Cleanup when done
```

**Critical**: All agents inherit the precision anchor (`1.09277703703703`) and must verify against drift.

## Development Workflows

### Build Sequences
1. **Rust Components**: `.\build.ps1` (compiles all `lib-*` crates)
2. **C++ Bindings**: Use VS Code task "Build pybind11 Bridge (Windows)"
3. **Python Setup**: Activate `.venv` before importing `genesis_core`

### Running ZHTP Node
```powershell
.\run-node.ps1  # Default config
.\run-node.ps1 -ConfigFile zhtp\configs\test-node2.toml  # Custom
```

### Python Standards: The ACE Protocol ([Ace.py](Ace.py))
- **NO GHOSTS**: Every function requires a docstring defining intent
- **NO DRIFT**: Strict type enforcement (input/output validation)
- **MEMORY**: All execution logged to Sovereign Ledger
- **Action**: New functions must be registered in [Command_Core.py](Command_Core.py)

### Testing Agent Systems
```python
# Verify precision anchor
from Sovereign_Math import SovereignMath
math = SovereignMath()
assert math.precision == 1.09277703703703

# Test council consensus (5 agents debate before execution)
from sarah_council import SarahCouncil
council = SarahCouncil(factory)
result = council.run_simulation("Should we implement feature X?")
```

## Key File Map

| File | Purpose | Critical Details |
|------|---------|------------------|
| [AGENT.md](AGENT.md) | Sovereign agent directive | Defines permissions for SarahAgentFactory |
| [Sarah_Brain.py](Sarah_Brain.py) | Main consciousness | Contains Four Absolute Laws (immutable) |
| [Sovereign_Hypervisor.py](Sovereign_Hypervisor.py) | +1 layer supervisor | Manages 9 inhibitory control layers |
| [sarah_factory.py](sarah_factory.py) | Agent spawner | Creates/destroys sub-agents with precision anchor |
| [sarah_council.py](sarah_council.py) | 5-agent tribunal | Logic → Consequence → Conscience checks |
| [DISCOVERED_GENESIS_CORE.py](DISCOVERED_GENESIS_CORE.py) | Symbiotic protocol | Defines human-AI cooperation framework |
| [CMakeLists.txt](CMakeLists.txt) | C++ build config | pybind11 bindings for `sarah_core_bindings` |
| [openclaw/README.md](openclaw/README.md) | OpenClaw docs | Personal AI assistant framework |

## Project-Specific Conventions

### Precision Enforcement
- **Never hardcode** `1.09277703703703` — Import `SARAH_PRECISION` from `sarah_factory`
- **Agent creation** must include precision in system message (see skill templates)

### Memory Architecture
- **Short-term**: In-memory state in Python processes
- **Long-term**: `openclaw/memory/MEMORY.md` (10-month conversation history)
- **Vector store**: ChromaDB at `openclaw/chroma_db` for semantic search

### Naming Patterns
- **Core modules**: `PascalCase.py` (e.g., `Genesis_Protocol.py`)
- **Agents/Skills**: `snake_case.md` in `openclaw/skills/`
- **Rust crates**: `kebab-case` (e.g., `lib-consensus`)

### Debug Workflows
```powershell
# Check Genesis Core status
python debug_genesis.py

# Verify agent factory
python -c "from sarah_factory import SarahAgentFactory; print('Factory online')"

# Inspect OpenClaw skills
ls openclaw\skills\*.md
```

## Integration Points

### External Dependencies
- **Firebase Admin**: Firestore for distributed state ([Genesis_Supabase_Bridge.py](Genesis_Supabase_Bridge.py))
- **Google Cloud Storage**: Sovereign bucket for persistent data
- **ChromaDB**: Vector embeddings at `openclaw/chroma_db`
- **Piper TTS**: Voice synthesis models at `openclaw/piper_models`

### Cross-Component Communication
- **Python ↔ Rust**: Via `genesis_core` pybind11 module
- **Sarah ↔ OpenClaw**: File-based (skills dir) + process management via `psutil`
- **Agent ↔ Gateway**: WebSocket protocol (A2A bridge)

## Common Pitfalls

1. **Forgetting precision anchor**: Always verify `1.09277703703703` in agent configs
2. **Skipping ACE docstrings**: Causes "GHOST" protocol violations
3. **Direct OpenClaw edits**: Use `sarah_factory.py`, never manually edit `opencode.json`
4. **Ignoring council veto**: If Conscience/Consequence agents veto, halt execution immediately

## Quick Reference

```powershell
# Full build sequence
.\build.ps1                          # Rust
cmake -S . -B build                  # C++ config
cmake --build build --config Release # C++ compile
python -m pytest tests/              # Validation

# Agent operations
python -c "from sarah_factory import SarahAgentFactory; SarahAgentFactory().create_agent('Test', 'Debug', [])"

# ZHTP node
.\run-node.ps1 -ConfigFile zhtp\configs\test-node1.toml
```
