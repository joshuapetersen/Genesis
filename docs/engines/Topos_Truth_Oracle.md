# Topos Truth Oracle - Structural Truth Verification

**File:** `Topos_Truth_Oracle.py`  
**Purpose:** Verify truth through structural and topological analysis  
**Author:** Joshua Petersen  

---

## What Is This? (Beginner-Friendly)

The Topos Truth Oracle checks if something is true by examining its **structure**, not just its content.

**Analogy**: Imagine checking if a bridge is safe:
- **Content check**: "The sign says it's safe" (weak)
- **Structural check**: "The engineering is sound" (strong)

The Oracle does the structural check for information.

---

## Why "Topos"?

"Topos" comes from mathematics (topos theory) - it's about the structure of logical truth. Don't worry if that sounds complex; here's the simple version:

> **The Oracle checks HOW things relate, not just WHAT they are.**

---

## How It Works

```
Statement: "All birds can fly"
          ↓
┌─────────────────────────────────────┐
│  STRUCTURAL ANALYSIS                │
├─────────────────────────────────────┤
│  1. Define "birds" (category)       │
│  2. Define "fly" (property)         │
│  3. Check ALL members of category   │
│  4. Find counter-examples           │
│     → Penguins, Ostriches           │
│  5. Statement structure: INVALID    │
└─────────────────────────────────────┘
          ↓
Result: FALSE (counter-examples exist)
Corrected: "MOST birds can fly"
```

---

## When Is This Used?

The Oracle is called when Sarah needs to:
- Verify logical consistency
- Check universal claims ("all", "none", "always")
- Validate relationships between concepts
- Detect logical fallacies

---

## Code Example

```python
from Topos_Truth_Oracle import TruthOracle

oracle = TruthOracle()

# Check a statement structurally
result = oracle.verify(
    statement="All programming languages are compiled",
    depth=2
)

print(result.is_valid)        # False
print(result.counter_examples) # ["Python", "JavaScript"]
print(result.corrected)        # "Some programming languages are compiled"
```

---

## Types of Checks

| Check Type | What It Does | Example |
|------------|--------------|---------|
| **Universal** | Checks "all X are Y" | "All cats are mammals" |
| **Existential** | Checks "some X are Y" | "Some birds are flightless" |
| **Relational** | Checks relationships | "If A then B" |
| **Consistency** | Checks for contradictions | "A cannot be both true and false" |

---

## Integration

The Oracle works with:
| Module | Role |
|--------|------|
| `Recursive_Truth_Finder.py` | Calls Oracle for structural verification |
| `Dialectical_Logic_Core.py` | Uses Oracle to validate thesis/antithesis |
| `Consensus_Voter.py` | Oracle adds structural confidence scores |

---

*Part of the SarahCore Genesis Project - © 2026 Joshua Petersen*
