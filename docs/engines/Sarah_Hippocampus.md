# Sarah Hippocampus - Short-Term Memory Buffer

**File:** `Sarah_Hippocampus.py`  
**Purpose:** Working memory and context management  
**Author:** Joshua Petersen  

---

## What Is This? (Brain Analogy)

In the human brain, the hippocampus is responsible for short-term memory and helps transfer important information to long-term storage. Sarah's Hippocampus does the same thing!

---

## Short-Term vs Long-Term

| Hippocampus (Short-Term) | Memory Vault (Long-Term) |
|--------------------------|--------------------------|
| Current conversation | Past conversations |
| Working context | Permanent knowledge |
| Fades after session | Stays forever |
| Fast access | Slightly slower |
| Limited capacity | Large capacity |

---

## What's in Working Memory?

At any moment, the Hippocampus holds:

```
┌────────────────────────────────────┐
│          WORKING MEMORY            │
├────────────────────────────────────┤
│  • Last 10 messages in chat        │
│  • Current user's preferences      │
│  • Active task context             │
│  • Recently accessed memories      │
│  • Emotional state indicators      │
└────────────────────────────────────┘
```

---

## Context Window Management

Sarah can only "think about" so much at once. The Hippocampus manages this:

```
New Information Arrives
         ↓
┌─────────────────────────────────┐
│  Is buffer full?                │
│  YES → Remove oldest item       │
│  NO  → Add to buffer            │
└─────────────────────────────────┘
         ↓
Information available for reasoning
```

---

## Code Example

```python
from Sarah_Hippocampus import Hippocampus

hippo = Hippocampus(max_items=50)

# Add to working memory
hippo.remember("User asked about Python")
hippo.remember("User seems to be a beginner")
hippo.remember("User prefers examples over theory")

# Get current context
context = hippo.get_context()

# Check if something is in working memory
if hippo.contains("Python"):
    print("Yes, we discussed Python recently")

# Clear after session ends
hippo.clear()
```

---

## Memory Flow

```
User says something
        ↓
   HIPPOCAMPUS (adds to buffer)
        ↓
   SARAH BRAIN (uses for response)
        ↓
   Important? ──YES──→ MEMORY VAULT (permanent storage)
        │
       NO
        ↓
   Fades away after session
```

---

## Why Separate Short from Long Term?

1. **Speed**: Short-term access is instant
2. **Relevance**: Only current context needed for most queries
3. **Capacity**: Can't keep everything in active memory
4. **Privacy**: Not everything needs to be saved forever

---

## Integration Example

```python
from Sarah_Hippocampus import Hippocampus
from Sarah_Memory_Vault import MemoryVault

hippo = Hippocampus()
vault = MemoryVault()

# During conversation
hippo.remember("User mentioned their cat is sick")

# At end of significant conversation
if hippo.get_importance("cat sick") > 0.7:
    vault.store(hippo.get("cat sick"))  # Promote to long-term
```

---

*Part of the SarahCore Genesis Project - © 2026 Joshua Petersen*
