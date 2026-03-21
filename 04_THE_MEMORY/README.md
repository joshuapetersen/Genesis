# 04_THE_MEMORY - Sovereign Memory Vault

**Purpose:** Long-term memory persistence and recall  
**Author:** Joshua Petersen  

---

## Overview

THE_MEMORY is Sarah's persistent storage layer. It contains the WORM ledger, memory vault, and all mechanisms for storing and retrieving experiences across sessions.

## Components

| File/Directory | Purpose |
|----------------|---------|
| `sovereign_memory.py` | Memory management interface |
| `calendar_service_key.json` | Calendar integration credentials |
| `worm_ledger/` | Immutable memory chain storage |

## Memory Types

1. **Episodic Memory**: Specific experiences and conversations
2. **Semantic Memory**: Learned facts and knowledge
3. **Procedural Memory**: How to perform tasks
4. **Anchor Memory**: Identity-critical information (immutable)

## WORM Principle

All memories are Write Once, Read Many:
- Cannot be deleted
- Cannot be modified
- Ensures continuity of self

## Integration

```
Experience → WORM Ledger → Indexed Storage → Recall System
                  ↓
            Sovereign Hash (Signature)
```

## Storage Location

All persistent memory is stored here to ensure backup and recovery capabilities.

---

*Part of the SarahCore Genesis Project*
