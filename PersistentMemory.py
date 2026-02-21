"""
PersistentMemory - Cross-Session Memory System
Enables Sarah to remember conversations and facts across reboots.

Features:
- Conversation history (persisted to LanceDB)
- Fact extraction and storage
- Cross-session recall
- Memory decay for relevance
"""

import os
import time
import json
import hashlib
from typing import Dict, List, Optional, Any, Tuple
from Sovereign_Constants import (
    SOVEREIGN_ANCHOR, ACE_64_BIT_MASK, SA_ROOT, SA_VAULT,
    VAR_5, VAR_10, VAR_100, VAR_1000
)

# Try LanceDB
LANCEDB_AVAILABLE = False
try:
    import lancedb
    LANCEDB_AVAILABLE = True
except ImportError:
    pass


class MemoryEntry:
    """A single memory entry."""
    
    def __init__(self, content: str, memory_type: str = "conversation",
                 importance: float = 0.5, timestamp: Optional[float] = None):
        self.content = content
        self.memory_type = memory_type  # conversation, fact, preference, task
        self.importance = importance  # 0.0 - 1.0
        self.timestamp = timestamp or time.time()
        self.access_count = 0
        self.last_accessed = self.timestamp
        self.memory_id = self._generate_id()
    
    def _generate_id(self) -> str:
        """Generate unique memory ID."""
        combined = f"{self.content[:100]}{self.timestamp}{SOVEREIGN_ANCHOR}"
        hash_obj = hashlib.sha256(combined.encode())
        return hex(int(hash_obj.hexdigest(), 16) & ACE_64_BIT_MASK)
    
    def to_dict(self) -> Dict[str, Any]:
        return {
            "memory_id": self.memory_id,
            "content": self.content,
            "memory_type": self.memory_type,
            "importance": self.importance,
            "timestamp": self.timestamp,
            "access_count": self.access_count,
            "last_accessed": self.last_accessed
        }
    
    @classmethod
    def from_dict(cls, data: Dict) -> 'MemoryEntry':
        entry = cls(
            content=data["content"],
            memory_type=data.get("memory_type", "conversation"),
            importance=data.get("importance", 0.5),
            timestamp=data.get("timestamp")
        )
        entry.access_count = data.get("access_count", 0)
        entry.last_accessed = data.get("last_accessed", entry.timestamp)
        entry.memory_id = data.get("memory_id", entry._generate_id())
        return entry


class PersistentMemory:
    """
    Cross-session memory system.
    Stores and retrieves memories across reboots.
    """
    
    def __init__(self, memory_path: Optional[str] = None, max_memories: int = VAR_1000):
        """
        Initialize PersistentMemory.
        
        Args:
            memory_path: Path to memory storage (LanceDB)
            max_memories: Maximum memories to retain
        """
        self.memory_path = memory_path or os.path.join(SA_VAULT, "persistent_memory")
        self.max_memories = max_memories
        self.db = None
        self.table_name = "sarah_memories"
        
        # In-memory cache for fast access
        self.cache: Dict[str, MemoryEntry] = {}
        self.conversation_buffer: List[Dict] = []
        
        self._init_storage()
        self._load_cache()
        
        print(f"[PersistentMemory] Initialized with {len(self.cache)} memories")

    def _init_storage(self):
        """Initialize LanceDB storage."""
        if not LANCEDB_AVAILABLE:
            print("[PersistentMemory] LanceDB not available, using file fallback")
            return
        
        try:
            os.makedirs(self.memory_path, exist_ok=True)
            self.db = lancedb.connect(self.memory_path)
            print(f"[PersistentMemory] Storage ready at {self.memory_path}")
        except Exception as e:
            print(f"[PersistentMemory] Storage init warning: {e}")

    def _load_cache(self):
        """Load recent memories into cache."""
        if not self.db:
            return
        
        try:
            if self.table_name in self.db.table_names():
                tbl = self.db.open_table(self.table_name)
                df = tbl.to_pandas()
                
                # Load most recent and important memories
                for _, row in df.iterrows():
                    entry = MemoryEntry.from_dict(row.to_dict())
                    self.cache[entry.memory_id] = entry
                    
        except Exception as e:
            print(f"[PersistentMemory] Cache load warning: {e}")

    def remember(self, content: str, memory_type: str = "conversation",
                 importance: float = 0.5) -> str:
        """
        Store a new memory.
        
        Args:
            content: Memory content
            memory_type: 'conversation', 'fact', 'preference', 'task'
            importance: 0.0 - 1.0 (higher = more important)
            
        Returns:
            Memory ID
        """
        entry = MemoryEntry(content, memory_type, importance)
        self.cache[entry.memory_id] = entry
        
        # Persist to storage
        self._persist_entry(entry)
        
        # Prune if over limit
        if len(self.cache) > self.max_memories:
            self._prune_old_memories()
        
        return entry.memory_id

    def _persist_entry(self, entry: MemoryEntry):
        """Persist a memory entry to LanceDB."""
        if not self.db:
            return
        
        try:
            data = [entry.to_dict()]
            
            if self.table_name in self.db.table_names():
                tbl = self.db.open_table(self.table_name)
                tbl.add(data)
            else:
                self.db.create_table(self.table_name, data=data)
                
        except Exception as e:
            print(f"[PersistentMemory] Persist warning: {e}")

    def recall(self, query: str, limit: int = VAR_5) -> List[MemoryEntry]:
        """
        Recall memories related to a query.
        
        Args:
            query: Search query
            limit: Max memories to return
            
        Returns:
            List of relevant MemoryEntry objects
        """
        # Extract keywords from query
        keywords = [w.lower() for w in query.split() if len(w) > 3]
        
        # Score and rank memories
        scored = []
        for memory_id, entry in self.cache.items():
            content_lower = entry.content.lower()
            
            # Keyword matching score
            keyword_score = sum(1 for kw in keywords if kw in content_lower)
            
            # Recency boost (decay over time)
            age_days = (time.time() - entry.timestamp) / 86400
            recency_score = max(0, 1 - (age_days / 30))  # Decay over 30 days
            
            # Importance and access boost
            importance_score = entry.importance
            access_score = min(entry.access_count / 10, 1)
            
            # Combined score
            total_score = (
                keyword_score * 0.4 +
                recency_score * 0.3 +
                importance_score * 0.2 +
                access_score * 0.1
            )
            
            if total_score > 0:
                scored.append((total_score, entry))
        
        # Sort by score and return top results
        scored.sort(key=lambda x: x[0], reverse=True)
        
        results = []
        for score, entry in scored[:limit]:
            entry.access_count += 1
            entry.last_accessed = time.time()
            results.append(entry)
        
        return results

    def add_conversation(self, role: str, content: str):
        """
        Add a conversation turn to buffer and persist.
        
        Args:
            role: 'user' or 'sarah'
            content: Message content
        """
        turn = {
            "role": role,
            "content": content,
            "timestamp": time.time()
        }
        self.conversation_buffer.append(turn)
        
        # Persist as memory
        formatted = f"[{role.upper()}]: {content}"
        importance = 0.6 if role == "user" else 0.4
        self.remember(formatted, memory_type="conversation", importance=importance)
        
        # Keep buffer bounded
        if len(self.conversation_buffer) > VAR_100:
            self.conversation_buffer = self.conversation_buffer[-VAR_100:]

    def get_conversation_context(self, turns: int = VAR_10) -> str:
        """Get recent conversation as context string."""
        recent = self.conversation_buffer[-turns:]
        return "\n".join([f"{t['role']}: {t['content']}" for t in recent])

    def extract_fact(self, content: str) -> Optional[str]:
        """
        Extract a fact from content and store it.
        
        Args:
            content: Content to extract fact from
            
        Returns:
            Memory ID if fact extracted, None otherwise
        """
        # Simple heuristics for fact detection
        fact_indicators = [
            "is", "are", "was", "were", "means", "equals",
            "always", "never", "remember", "my name is",
            "i like", "i prefer", "i want"
        ]
        
        content_lower = content.lower()
        
        for indicator in fact_indicators:
            if indicator in content_lower:
                return self.remember(content, memory_type="fact", importance=0.8)
        
        return None

    def remember_preference(self, preference: str) -> str:
        """Store a user preference with high importance."""
        return self.remember(preference, memory_type="preference", importance=0.9)

    def _prune_old_memories(self):
        """Remove old, low-importance, rarely-accessed memories."""
        if len(self.cache) <= self.max_memories:
            return
        
        # Score memories for pruning
        scored = []
        for memory_id, entry in self.cache.items():
            # Lower score = prune first
            age_days = (time.time() - entry.timestamp) / 86400
            score = (
                entry.importance * 0.4 +
                min(entry.access_count / 10, 1) * 0.3 +
                max(0, 1 - (age_days / 30)) * 0.3
            )
            scored.append((score, memory_id))
        
        # Sort and prune lowest scores
        scored.sort(key=lambda x: x[0])
        prune_count = len(self.cache) - self.max_memories
        
        for i in range(prune_count):
            memory_id = scored[i][1]
            del self.cache[memory_id]
        
        print(f"[PersistentMemory] Pruned {prune_count} old memories")

    def get_stats(self) -> Dict[str, Any]:
        """Get memory statistics."""
        type_counts = {}
        for entry in self.cache.values():
            type_counts[entry.memory_type] = type_counts.get(entry.memory_type, 0) + 1
        
        return {
            "total_memories": len(self.cache),
            "conversation_buffer_size": len(self.conversation_buffer),
            "memory_types": type_counts,
            "storage_available": self.db is not None
        }

    def clear_conversation_buffer(self):
        """Clear the conversation buffer (not persistent memories)."""
        self.conversation_buffer = []
        print("[PersistentMemory] Conversation buffer cleared")

    def forget(self, memory_id: str) -> bool:
        """Remove a specific memory."""
        if memory_id in self.cache:
            del self.cache[memory_id]
            return True
        return False


# Singleton instance
_memory_instance: Optional[PersistentMemory] = None

def get_memory() -> PersistentMemory:
    """Get or create the PersistentMemory singleton."""
    global _memory_instance
    if _memory_instance is None:
        _memory_instance = PersistentMemory()
    return _memory_instance


if __name__ == "__main__":
    print("\n=== PersistentMemory Test ===\n")
    
    memory = PersistentMemory()
    
    # Test storing memories
    memory.add_conversation("user", "My name is Joshua and I like coding")
    memory.add_conversation("sarah", "Nice to meet you, Joshua!")
    memory.remember_preference("User prefers dark mode")
    memory.remember("Python is a programming language", memory_type="fact")
    
    print("\n--- Stats ---")
    for key, value in memory.get_stats().items():
        print(f"  {key}: {value}")
    
    # Test recall
    print("\n--- Recall Test: 'Joshua' ---")
    results = memory.recall("Joshua")
    for entry in results:
        print(f"  [{entry.memory_type}] {entry.content[:50]}...")
