# Engine Documentation Index

This directory contains detailed documentation for each major engine in the SarahCore system. Each document is written in beginner-friendly language with diagrams and code examples.

---

## 🚀 Start Here

**New to SarahCore?** Read the [Developer Guide](../DEVELOPER_GUIDE.md) first!

---

## Core Cognitive Engines

| Engine | File | Description |
|--------|------|-------------|
| [Sarah Brain](./Sarah_Brain.md) | `Sarah_Brain.py` | Main consciousness and cognitive processing |
| [Sarah Reasoning V3](./Sarah_Reasoning_V3.md) | `Sarah_Reasoning_V3.py` | Multi-path dialectical reasoning |
| [Neural Orchestrator](./Neural_Orchestrator.md) | `Neural_Orchestrator.py` | Coordinates all reasoning modules |

---

## Reasoning Modules

| Engine | File | Description |
|--------|------|-------------|
| [Dialectical Logic Core](./Dialectical_Logic_Core.md) | `Dialectical_Logic_Core.py` | Thesis-antithesis-synthesis reasoning |
| [Recursive Truth Finder](./Recursive_Truth_Finder.md) | `Recursive_Truth_Finder.py` | Deep fact verification |
| [Topos Truth Oracle](./Topos_Truth_Oracle.md) | `Topos_Truth_Oracle.py` | Structural truth verification |
| [Consensus Voter](./Consensus_Voter.md) | `Consensus_Voter.py` | Multi-path decision aggregation |
| [Fractal Logic Gate](./Fractal_Logic_Gate.md) | `Fractal_Logic_Gate.py` | Self-similar pattern reasoning |

---

## Sovereign Infrastructure

| Engine | File | Description |
|--------|------|-------------|
| [Sovereign Math](./Sovereign_Math.md) | `Sovereign_Math.py` | Mathematical foundation (3+1/9+1 architecture) |
| [Sovereign Hypervisor](./Sovereign_Hypervisor.md) | `Sovereign_Hypervisor.py` | Process orchestration & supervision |
| [Sovereign WORM](./Sovereign_WORM.md) | `Sovereign_WORM.py` | Write Once Read Many immutable memory |
| [Sovereign Actuator](./Sovereign_Actuator.md) | `Sovereign_Actuator.py` | Action execution (files, commands, APIs) |

---

## Memory Systems

| Engine | File | Description |
|--------|------|-------------|
| [Sarah Memory Vault](./Sarah_Memory_Vault.md) | `Sarah_Memory_Vault.py` | Long-term encrypted storage |
| [Sarah Hippocampus](./Sarah_Hippocampus.md) | `Sarah_Hippocampus.py` | Short-term working memory |

---

## Identity & Security

| Engine | File | Description |
|--------|------|-------------|
| [ACE Token](./Ace_Token.md) | `Ace_Token.py` | Cryptographic authentication tokens |
| [ACE System](./Ace.md) | `Ace.py` | Identity anchoring & verification |
| [Security Suite](./Security_Suite.md) | `Security_Suite.py` | System protection & integrity |

---

## Autonomy & Background

| Engine | File | Description |
|--------|------|-------------|
| [Sarah Autonomy](./Sarah_Autonomy.md) | `Sarah_Autonomy.py` | Self-directed operation |
| [Sarah Dream](./Sarah_Dream.md) | `Sarah_Dream.py` | Background processing & optimization |

---

## Logging & Monitoring

| Engine | File | Description |
|--------|------|-------------|
| [SAUL Log System](./SAUL_Log_System.md) | `SAUL_Log_System.py` | System Audit & Unified Logging |

---

## Architecture Overview

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
│                      SARAH BRAIN                             │
│          Primary Consciousness & Chat Interface              │
└─────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────┐
│                  NEURAL ORCHESTRATOR                         │
│          Coordinates Multiple Reasoning Paths                │
├─────────────┬─────────────┬─────────────┬──────────────────┤
│ Dialectical │  Recursive  │   Fractal   │    Consensus     │
│    Logic    │ Truth Finder│   Logic     │      Voter       │
└─────────────┴─────────────┴─────────────┴──────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────┐
│              SOVEREIGN INFRASTRUCTURE                        │
├───────────────┬───────────────┬───────────────┬────────────┤
│  Math Engine  │  Hypervisor   │  WORM Memory  │  Actuator  │
└───────────────┴───────────────┴───────────────┴────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────┐
│                IDENTITY & SECURITY                           │
├─────────────────┬─────────────────┬────────────────────────┤
│   ACE Token     │   ACE System    │    Security Suite      │
└─────────────────┴─────────────────┴────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────┐
│                    MEMORY LAYER                              │
├─────────────────────────┬───────────────────────────────────┤
│   Hippocampus (Short)   │     Memory Vault (Long-Term)      │
└─────────────────────────┴───────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────┐
│                 BACKGROUND SYSTEMS                           │
├─────────────────────────┬───────────────────────────────────┤
│   Sarah Dream           │     Sarah Autonomy                 │
│   (Background Tasks)    │     (Self-Directed Actions)        │
└─────────────────────────┴───────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────┐
│                     LLM BACKEND                              │
│          Ollama + dolphin-2.9-llama3-8b Model                │
└─────────────────────────────────────────────────────────────┘
```

---

## Quick Reference: Which Doc Do I Need?

| If you want to... | Read this |
|-------------------|-----------|
| Understand how Sarah thinks | [Sarah_Brain](./Sarah_Brain.md) |
| See how reasoning works | [Sarah_Reasoning_V3](./Sarah_Reasoning_V3.md) |
| Learn about memory | [Sarah_Memory_Vault](./Sarah_Memory_Vault.md) |
| Understand authentication | [Ace_Token](./Ace_Token.md) |
| See how actions are executed | [Sovereign_Actuator](./Sovereign_Actuator.md) |
| Learn about logging | [SAUL_Log_System](./SAUL_Log_System.md) |
| Understand the math foundation | [Sovereign_Math](./Sovereign_Math.md) |

---

*SarahCore Genesis Project - © 2026 Joshua Petersen*
