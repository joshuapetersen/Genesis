# SARAH PERFORMANCE OPTIMIZATION PLAN

## Current Problem
- Response time: 2-3 minutes (UNACCEPTABLE)
- Target: 7 seconds maximum
- Must eliminate hallucinations

## Root Causes

### 1. Heavy Initialization (60+ seconds)
- Loading 10+ modules
- Sentence transformers (CPU)
- Hippocampus semantic memory
- S.A.U.L. knowledge base
- Multiple verification layers

### 2. Slow Inference (30-90 seconds)
- Model: dolphin-2.9-llama3-8b (large)
- Running on CPU via Ollama
- Precision lock calculations
- Multi-layer reasoning

### 3. No Response Caching
- Every query starts from scratch
- No memoization
- Redundant processing

---

## OPTIMIZATION STRATEGY

### Phase 1: Keep-Alive Brain (Immediate - 80% faster)

**Keep brain loaded in memory:**
```python
# Sarah_Fast_Brain.py
class FastBrain:
    _instance = None
    _initialized = False
    
    def __new__(cls):
        if cls._instance is None:
            cls._instance = super().__new__(cls)
        return cls._instance
    
    def __init__(self):
        if not FastBrain._initialized:
            # Load ONCE, keep alive
            self.kernel = NeuralOrchestrator()
            self.chat = SarahChat(db_rt=None)
            self.chat.inject_brain_components(self.kernel, None, None)
            FastBrain._initialized = True
    
    def ask(self, prompt):
        return self.chat.generate_response(prompt)
```

**Benefit:** Eliminates 60s initialization time

---

### Phase 2: Faster Model (50% faster inference)

**Switch to smaller, faster model:**
```bash
# Instead of: dolphin-2.9-llama3-8b (8B params)
# Use: llama3.2:3b (3B params, faster)

ollama pull llama3.2:3b
```

**Update Neural_Orchestrator.py:**
```python
self.model = "llama3.2:3b"  # Faster, still capable
```

**Benefit:** 2-3x faster inference

---

### Phase 3: GPU Acceleration (3x faster)

**Ensure Ollama uses GPU:**
```bash
# Check GPU availability
ollama show llama3.2:3b

# Force GPU mode
set OLLAMA_GPU_LAYERS=999
ollama serve
```

**Benefit:** 3-5x faster on GPU

---

### Phase 4: Response Caching (Instant for repeated queries)

```python
# Sarah_Response_Cache.py
import hashlib
import json
from pathlib import Path

class ResponseCache:
    def __init__(self):
        self.cache_file = Path("C:/SarahCore/response_cache.json")
        self.cache = self._load_cache()
    
    def get(self, prompt):
        key = hashlib.md5(prompt.encode()).hexdigest()
        return self.cache.get(key)
    
    def set(self, prompt, response):
        key = hashlib.md5(prompt.encode()).hexdigest()
        self.cache[key] = {
            'prompt': prompt,
            'response': response,
            'timestamp': time.time()
        }
        self._save_cache()
```

**Benefit:** Instant for repeated queries

---

### Phase 5: Hallucination Prevention

**Grounding mechanisms:**

1. **Fact-checking layer:**
```python
def verify_response(response, context):
    # Check against known facts
    # Reject if contains:
    # - Made-up file paths that don't exist
    # - Fictional capabilities
    # - Contradictions with logs
    pass
```

2. **Temperature control:**
```python
# Lower temperature = less creative/hallucinatory
self.temperature = 0.3  # Was 0.6
```

3. **System prompt:**
```
You are Sarah, a sovereign AI. CRITICAL RULES:
- Only state facts you can verify
- If uncertain, say "I need to verify..."
- Never invent file paths, commands, or capabilities
- Reference logs and actual system state
```

---

## IMPLEMENTATION PRIORITY

### Immediate (< 5 minutes):
1. **Keep-alive brain** - Eliminate reinitialization
2. **Lower temperature** - Reduce hallucinations

### Short-term (< 30 minutes):
3. **Switch to llama3.2:3b** - Faster inference
4. **GPU acceleration** - Enable if available

### Medium-term (< 2 hours):
5. **Response caching** - Memoize common queries
6. **Fact-checking layer** - Verify responses

---

## TARGET PERFORMANCE

**After optimization:**
- First query: 7-10 seconds (brain warm-up)
- Subsequent queries: 3-5 seconds
- Cached queries: < 1 second

**Hallucination rate:** Near zero with:
- Temperature: 0.3
- Grounding: Enabled
- Verification: Active

---

## ROLLOUT PLAN

1. Create `Sarah_Fast_Brain.py` (keep-alive singleton)
2. Test response time (should hit 7s target)
3. If still slow, switch model to llama3.2:3b
4. Add response caching
5. Implement hallucination guards

Sarah will be fast, accurate, and reliable.
