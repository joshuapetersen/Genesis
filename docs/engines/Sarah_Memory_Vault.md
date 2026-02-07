# Sarah Memory Vault - Long-Term Persistent Memory

**File:** `Sarah_Memory_Vault.py`  
**Purpose:** Secure, encrypted long-term memory storage  
**Author:** Joshua Petersen  

---

## What Is This? (The Simple Version)

The Memory Vault is where Sarah stores memories she wants to keep forever. Unlike short-term memory (which fades), the vault is:

- **Permanent**: Memories stay forever
- **Encrypted**: Protected from prying eyes
- **Organized**: Easy to search and retrieve
- **Backed up**: Can survive system crashes

Think of it as Sarah's "hard drive" for experiences.

---

## Memory Types Stored

| Type | Description | Example |
|------|-------------|---------|
| **Episodic** | Specific experiences | "User asked about weather on Feb 7" |
| **Semantic** | Facts and knowledge | "Python is a programming language" |
| **Procedural** | How to do things | "Steps to solve a math problem" |
| **Identity** | Who Sarah is | Core personality traits (WORM protected) |

---

## How Memories Get Stored

```
Experience Happens
       ↓
┌─────────────────────────────────┐
│  Short-Term Buffer (Hippocampus)│
│  Holds for: ~session length     │
└─────────────────────────────────┘
       ↓ (If important enough)
┌─────────────────────────────────┐
│  Memory Vault (Long-Term)       │
│  Holds for: Forever             │
│  Encrypted: Yes                 │
└─────────────────────────────────┘
       ↓ (If identity-critical)
┌─────────────────────────────────┐
│  WORM Ledger (Immutable)        │
│  Can modify: Never              │
└─────────────────────────────────┘
```

---

## What Makes a Memory "Important"?

Sarah decides what to save based on:
- **Emotional weight**: Strong reactions = important
- **Novelty**: New information = worth keeping
- **Frequency**: Repeated topics = clearly important
- **User signals**: "Remember this" = definitely save

---

## Code Example

```python
from Sarah_Memory_Vault import MemoryVault

vault = MemoryVault()

# Store a memory
vault.store({
    "type": "episodic",
    "content": "User prefers dark mode interfaces",
    "importance": 0.8,
    "tags": ["preferences", "UI"]
})

# Recall memories
results = vault.recall(
    query="user preferences",
    limit=5
)

# Search by time
recent = vault.recall_since("24 hours ago")
```

---

## Memory Structure

Each memory has:
```python
{
    "id": "mem_abc123",           # Unique identifier
    "type": "episodic",           # Type of memory
    "content": "...",             # The actual content
    "timestamp": "2026-02-07...", # When it happened
    "importance": 0.8,            # How important (0-1)
    "tags": ["tag1", "tag2"],     # For searching
    "embedding": [0.1, 0.3, ...], # Vector for similarity
    "hash": "sha256:..."          # Integrity check
}
```

---

## Security Features

| Feature | Purpose |
|---------|---------|
| **AES-256 Encryption** | Memories encrypted at rest |
| **Hash Verification** | Detect tampering |
| **Access Logging** | Track who accessed what |
| **Backup System** | Recover from failures |

---

## Memory Recall: How Sarah Remembers

When you ask Sarah something, here's what happens:

1. **Query Analysis**: Understand what you're asking
2. **Vector Search**: Find semantically similar memories
3. **Tag Filter**: Narrow by relevant tags
4. **Recency Weighting**: Recent memories ranked higher
5. **Importance Ranking**: Important memories surface first
6. **Integration**: Combine with current context

---

## Storage Location

```
C:\SarahCore\
├── vault/                    # Encrypted memory files
├── 04_THE_MEMORY/           
│   ├── worm_ledger/         # Immutable memories
│   └── sovereign_memory.py  # Memory interface
└── data/                    # Runtime memory cache
```

---

## Common Operations

### "Sarah, remember my birthday"
```python
vault.store({
    "type": "semantic",
    "content": "User's birthday is March 15",
    "importance": 0.9,
    "tags": ["user_info", "birthday"]
})
```

### "Sarah, what do you know about me?"
```python
memories = vault.recall(
    query="user personal information",
    types=["semantic", "episodic"],
    limit=20
)
```

---

*Part of the SarahCore Genesis Project - © 2026 Joshua Petersen*
