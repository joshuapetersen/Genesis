# Recursive Truth Finder - Deep Knowledge Discovery

**File:** `Recursive_Truth_Finder.py`  
**Purpose:** Multi-level truth exploration and fact verification  
**Author:** Joshua Petersen  

---

## What Is This? (Simple Explanation)

When you ask Sarah a factual question, she doesn't just give you the first answer she finds. She digs deeper - and then deeper again - until she's confident in the truth.

Think of it like peeling an onion:
- **Layer 1**: Surface-level answer
- **Layer 2**: Supporting evidence
- **Layer 3**: Sources of that evidence
- **Layer 4**: Verification of sources
- ...and so on until confident

---

## Why "Recursive"?

"Recursive" means something that calls itself. The Truth Finder works like this:

```
find_truth("Is water wet?")
    ├── Check known facts → Still uncertain?
    │       ↓
    │   find_truth("What does 'wet' mean?")
    │       ├── Check definitions → Still uncertain?
    │       │       ↓
    │       │   find_truth("Is water a liquid?")
    │       │       └── YES (confirmed)
    │       └── "Wet = liquid touching surface"
    └── Final: "Water makes things wet, but isn't 'wet' itself"
```

Each layer calls the next until we hit solid ground.

---

## How It Works

### Step 1: Receive Query
```python
query = "Did Einstein fail math?"
```

### Step 2: Initial Search
```python
initial_result = "Some sources say yes, some say no"
confidence = 0.4  # Too low! Keep digging.
```

### Step 3: Recursive Deep Dive
```python
# Check primary sources
school_records = search("Einstein school transcripts")
# Result: He got top grades in math

# Verify the source
zurich_archives = verify("ETH Zurich records")
# Result: Confirmed authentic

confidence = 0.95  # High enough!
```

### Step 4: Return Truth
```python
answer = "No, Einstein excelled at math. This is a myth."
```

---

## Visual Flow

```
Question
    ↓
┌─────────────────────────────────┐
│  Level 1: Quick Search          │
│  Confidence: 40%                │
│  Status: INSUFFICIENT           │
└─────────────────────────────────┘
    ↓
┌─────────────────────────────────┐
│  Level 2: Deep Search           │
│  Confidence: 70%                │
│  Status: NEEDS VERIFICATION     │
└─────────────────────────────────┘
    ↓
┌─────────────────────────────────┐
│  Level 3: Source Verification   │
│  Confidence: 95%                │
│  Status: VERIFIED ✓             │
└─────────────────────────────────┘
    ↓
Confident Answer
```

---

## Code Example

```python
from Recursive_Truth_Finder import TruthFinder

finder = TruthFinder()

result = finder.find(
    query="What year was the moon landing?",
    min_confidence=0.9,  # Don't stop until 90% confident
    max_depth=5          # Don't go more than 5 levels deep
)

print(f"Answer: {result.answer}")
print(f"Confidence: {result.confidence}")
print(f"Sources: {result.sources}")
print(f"Depth reached: {result.depth}")
```

---

## Confidence Thresholds

| Level | Meaning | Action |
|-------|---------|--------|
| 0.95+ | Verified fact | Return answer |
| 0.80-0.95 | High confidence | Return with minor caveats |
| 0.60-0.80 | Moderate confidence | Note uncertainty |
| 0.40-0.60 | Low confidence | Keep digging or ask user |
| <0.40 | Unknown | Admit "I don't know" |

---

## Why This Matters

Without the Truth Finder, Sarah might:
- Repeat common myths as facts
- Give confident-sounding wrong answers
- Miss important nuances

WITH the Truth Finder, Sarah:
- Verifies claims before asserting them
- Cites sources when possible
- Admits uncertainty when appropriate

---

*Part of the SarahCore Genesis Project - © 2026 Joshua Petersen*
