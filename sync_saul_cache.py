import json
import os

VAR_100 = 100
VAR_4 = 4

def sync_cache(stream_path="final_consolidated_memory.jsonl", cache_path="saul_knowledge_cache.json"):
    """Function: sync_cache"""
    if not os.path.exists(stream_path):
        print(f"Error: {stream_path} not found.")
        return
    
    print(f"[Sync] Loading stream from {stream_path}...")
    knowledge_base = []
    with open(stream_path, 'r', encoding='utf-8') as f:
        for line in f:
            if not line.strip(): continue
            event = json.loads(line)
            
            # Format for SAUL knowledge cache (Supabase wrap style)
            knowledge_base.append({
                "id": f"gen_{hash(event['content'][:VAR_100])}",
                "data": {
                    "title": event.get('metadata', {}).get('file', event['source']),
                    "content": event['content'],
                    "source": event['source'],
                    "ingested_at": event['timestamp']
                },
                "created_at": event['timestamp']
            })
    
    # Preserve JANUARY 2026 Anchors by keeping them if they exist in the old cache or re-seeding them
    # SAUL_Logistics._seed_mandatory_anchors() handles this naturally on next boot, 
    # but let's be safe and keep the file clean.
    
    print(f"[Sync] Writing {len(knowledge_base)} items to {cache_path}...")
    with open(cache_path, 'w', encoding='utf-8') as f:
        json.dump(knowledge_base, f, indent=VAR_4)
    print("[Sync] Cache updated successfully.")

if __name__ == "__main__":
    sync_cache()
