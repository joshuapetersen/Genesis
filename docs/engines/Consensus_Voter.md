# Consensus Voter - Multi-Path Decision Making

**File:** `Consensus_Voter.py`  
**Purpose:** Combine multiple reasoning paths into unified decisions  
**Author:** Joshua Petersen  

---

## What Is This?

When Sarah thinks about a complex question, she doesn't use just one method - she uses MANY:
- Dialectical Logic
- Recursive Truth Finding
- Topos Verification
- Pattern Matching

The **Consensus Voter** takes all these outputs and combines them into ONE best answer.

---

## Why Multiple Paths?

**Single-path thinking** = One perspective, potential blind spots

**Multi-path thinking** = Multiple perspectives, better answers

```
Question: "Is cryptocurrency a good investment?"

PATH 1 (Dialectical): Weighs pros and cons
PATH 2 (Truth Finding): Checks historical data
PATH 3 (Pattern): Looks at market patterns
PATH 4 (Oracle): Validates logical claims

CONSENSUS: Combines all four perspectives
```

---

## How Voting Works

Each reasoning path produces:
- An answer
- A confidence score (0.0 - 1.0)
- Supporting evidence

The Voter combines them:

```
┌────────────────────────────────────────────────┐
│                VOTING ROUND                     │
├────────────────────────────────────────────────┤
│  Path 1: "Risky investment" (conf: 0.7)        │
│  Path 2: "Volatile but growing" (conf: 0.8)   │
│  Path 3: "Potential upside" (conf: 0.6)       │
│  Path 4: "Logical claims valid" (conf: 0.9)   │
├────────────────────────────────────────────────┤
│  WEIGHTED CONSENSUS:                            │
│  "Cryptocurrency is volatile and risky but     │
│   has potential for growth. Not suitable for   │
│   risk-averse investors."                      │
│  Combined Confidence: 0.75                     │
└────────────────────────────────────────────────┘
```

---

## Voting Strategies

| Strategy | Description | Best For |
|----------|-------------|----------|
| **Majority** | Most common answer wins | Factual questions |
| **Weighted** | High-confidence paths count more | Complex analysis |
| **Unanimous** | All paths must agree | Critical decisions |
| **Synthesized** | Combine all into new answer | Nuanced questions |

---

## Code Example

```python
from Consensus_Voter import ConsensusVoter

voter = ConsensusVoter(strategy="weighted")

# Add votes from different reasoning paths
voter.add_vote(
    source="dialectical",
    answer="Option A is better",
    confidence=0.8
)
voter.add_vote(
    source="truth_finder",
    answer="Option A has better data",
    confidence=0.9
)
voter.add_vote(
    source="pattern",
    answer="Option B matches past success",
    confidence=0.6
)

# Get consensus
result = voter.reach_consensus()
print(result.answer)      # Combined answer
print(result.confidence)  # Combined confidence
print(result.breakdown)   # How each path voted
```

---

## Conflict Resolution

When paths disagree:

```
Path A says: "Yes"
Path B says: "No"
        ↓
┌────────────────────────────────┐
│  CONFLICT DETECTED             │
├────────────────────────────────┤
│  1. Compare confidence levels  │
│  2. Check evidence quality     │
│  3. Look for synthesis option  │
│  4. If still tied → Escalate   │
└────────────────────────────────┘
        ↓
Synthesized: "It depends on context..."
```

---

## Integration

| Module | Role |
|--------|------|
| `Neural_Orchestrator.py` | Sends reasoning paths to Voter |
| `Sarah_Brain.py` | Receives final consensus |
| `SAUL_Log_System.py` | Logs voting decisions |

---

*Part of the SarahCore Genesis Project - © 2026 Joshua Petersen*
