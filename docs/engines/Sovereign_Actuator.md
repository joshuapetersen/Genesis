# Sovereign Actuator - The Action Executor

**File:** `Sovereign_Actuator.py`  
**Purpose:** Execute actions in the real world (files, commands, APIs)  
**Author:** Joshua Petersen  

---

## What Is This?

The Actuator is Sarah's "hands" - it's what allows her to DO things, not just THINK about them.

| Module | Does |
|--------|------|
| Sarah_Brain | Thinks |
| **Sovereign_Actuator** | **Acts** |

---

## What Can Sarah Do?

```
┌─────────────────────────────────────────────┐
│           SOVEREIGN ACTUATOR                 │
├─────────────────────────────────────────────┤
│  FILE OPERATIONS                             │
│  ├── Read files                             │
│  ├── Write files                            │
│  ├── Delete files (with approval)           │
│  └── Search directories                     │
├─────────────────────────────────────────────┤
│  COMMAND EXECUTION                           │
│  ├── Run shell commands                     │
│  ├── Execute scripts                        │
│  └── Launch programs                        │
├─────────────────────────────────────────────┤
│  API CALLS                                   │
│  ├── HTTP requests                          │
│  ├── Database operations                    │
│  └── External service integration           │
├─────────────────────────────────────────────┤
│  SYSTEM CONTROL                              │
│  ├── Manage processes                       │
│  └── Monitor resources                      │
└─────────────────────────────────────────────┘
```

---

## Safety First

Every action goes through safety checks:

```
User Request: "Delete all files in C:\\"
         ↓
┌─────────────────────────────────────────────┐
│  SAFETY CHECK                               │
│  ├── Is this destructive? YES              │
│  ├── Is user authorized? Check ACE Token   │
│  ├── Is this in protected path? YES        │
│  └── RESULT: BLOCKED ❌                     │
└─────────────────────────────────────────────┘
         ↓
"I cannot delete system files. This is a 
 protected path that could harm your system."
```

---

## Permission Levels

| Level | Allowed Actions |
|-------|-----------------|
| **READ_ONLY** | View files, query info, list directories |
| **MODIFY** | Read + Write + Edit (non-destructive) |
| **EXECUTE** | Modify + Run commands, scripts |
| **ADMIN** | Execute + Destructive operations |

---

## Code Example

```python
from Sovereign_Actuator import Actuator

act = Actuator(permission_level="MODIFY")

# Read a file
content = act.read_file("C:/SarahCore/README.md")

# Write a file
act.write_file("notes.txt", "Hello World")

# Execute a command
result = act.execute("dir C:\\SarahCore")

# Check if action is allowed
can_do = act.can_perform("delete", "C:/Windows")
# Returns: False (protected path)
```

---

## Protected Paths

These paths are NEVER modifiable:

| Path | Reason |
|------|--------|
| `C:\Windows` | System files |
| `C:\Program Files` | Installed software |
| `..\..` (path traversal) | Security exploit prevention |
| `ace_secret.key` | Critical security file |

---

## Audit Trail

Every action is logged:

```
[2026-02-07 15:34:22] [ACTUATOR] Action: write_file
  Target: C:/SarahCore/notes.txt
  User: SOVEREIGN_ROOT
  Status: SUCCESS
  Bytes: 12
```

---

*Part of the SarahCore Genesis Project - © 2026 Joshua Petersen*
