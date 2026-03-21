
import lancedb
import requests
import time
from typing import Optional, Dict

VAR_0_3 = 0.3
VAR_10 = 10
VAR_100 = 100
VAR_1500 = 1500
VAR_180 = 180
VAR_2048 = 2048
VAR_3 = 3
VAR_5 = 5

# Configuration
DB_PATH = "C:\\SarahCore\\vault\\coding_encyclopedia"
SOVEREIGN_GATEWAY_CHAT = "http://localhost:8001/api/chat"
MODEL_NAME = "aeris" 
TABLE_NAME = "coding_knowledge"

# Aggressive memory management settings
BATCH_SIZE = 1
COOLDOWN_SECONDS = 2  # Time to sleep between items
UNLOAD_MODEL_AFTER_BATCH = True

def get_db_connection():
    """Function: get_db_connection"""
    return lancedb.connect(DB_PATH)

def get_shallow_entries(limit=VAR_10):
    """
    Retrieves entries that have placeholder descriptions.
    """
    db = get_db_connection()
    if TABLE_NAME not in db.table_names():
        print(f"Table {TABLE_NAME} not found.")
        return []

    tbl = db.open_table(TABLE_NAME)
    df = tbl.to_pandas()
    
    # Identify entries with shallow/placeholder descriptions
    shallow_mask = (df['description'].str.len() < VAR_100) | (df['description'].str.contains("placeholder", case=False))
           
    shallow_df = df[shallow_mask].head(limit)
    return shallow_df.to_dict('records')

def query_sovereign(prompt: str) -> Optional[str]:
    """
    Sends a prompt to the Sovereign Gateway for local inference.
    """
    payload = {
        "model": "aeris",
        "message": prompt,
        "user_id": "knowledge_harvester"
    }

    try:
        response = requests.post(SOVEREIGN_GATEWAY_CHAT, json=payload, timeout=VAR_180)
        response.raise_for_status()
        data = response.json()
        return data.get("content")
    except Exception as e:
        print(f"[Harvester] Gateway Error: {e}")
        return None

def unload_model():
    """
    Forces the model to unload from VRAM/RAM.
    """
    print("[Harvester] Unloading model to free resources...")
    # Sending an empty request with keep_alive=0 unloads the model immediately
    payload = {
        "model": MODEL_NAME,
        "keep_alive": 0
    }
    try:
        # We still use the chat endpoint but with unload directive if supported
        requests.post(SOVEREIGN_GATEWAY_CHAT, json=payload, timeout=VAR_10)
    except Exception:
        pass # Ignore errors on unload

def generate_knowledge(entry: Dict) -> Dict:
    """
    Generates deep technical content for a key term.
    """
    term = entry.get('term')
    category = entry.get('category')
    
    print(f"\n[Harvester] Harvesting knowledge for: {term} ({category})...")
    
    # --- Sovereign Web Access ---
    web_context = ""
    try:
        from Sovereign_Web_Walker import SovereignWebWalker
        walker = SovereignWebWalker()
        web_context = walker.get_grounding_context(term, category)
        if web_context:
            print(f"[Harvester] Injected {len(web_context)} chars of Web Context.")
    except ImportError:
        print("[Harvester] Web Walker not found. Proceeding with internal knowledge only.")
    except Exception as e:
        print(f"[Harvester] Web Walker error: {e}")

    prompt = f"""
    You are the "Sovereign/Sarah" AI, re-vectorizing your own knowledge base.
    
    Topic: {term}
    Category: {category}
    
    [SOVEREIGN WEB CONTEXT]
    The following is authoritative documentation retrieved from the web:
    {web_context[:VAR_1500] if web_context else "No external context available."}
    [END CONTEXT]
    
    Based on the above context (and your internal knowledge), apply the "5W1H Anchor Mandate". 
    Output your response in the following EXACT format (do not use Markdown or JSON):
    
    WHO: [Identity/Subject]
    WHAT: [Core definition and ACE Token hash]
    WHERE: [Physical/Logical location]
    WHEN: [Temporal context]
    WHY: [Intent/Logic goal]
    HOW: [Implementation strategy & Phrasing (Include code if applicable)]
    """
    
    response_text = query_sovereign(prompt)
    if not response_text:
        return None
        
    # Text-based parsing strategy
    knowledge = {}
    lines = response_text.split('\n')
    current_key = None
    buffer = []
    
    key_map = {
        "WHO:": "who_vector",
        "WHAT:": "what_vector",
        "WHERE:": "where_vector",
        "WHEN:": "when_vector",
        "WHY:": "why_vector",
        "HOW:": "how_vector"
    }
    
    for line in lines:
        clean_line = line.strip()
        found_key = False
        for prefix, json_key in key_map.items():
            if clean_line.upper().startswith(prefix):
                # Save previous buffer
                if current_key:
                    knowledge[current_key] = "\n".join(buffer).strip()
                
                # Start new section
                current_key = json_key
                content = clean_line[len(prefix):].strip()
                buffer = [content] if content else []
                found_key = True
                break
        
        if not found_key and current_key:
            buffer.append(line) # Keep indentation for code blocks
            
    # Save last section
    if current_key:
        knowledge[current_key] = "\n".join(buffer).strip()
        
    # Validation/Fallback
    required_keys = ["who_vector", "what_vector", "where_vector", "when_vector", "why_vector", "how_vector"]
    for k in required_keys:
        if k not in knowledge:
            knowledge[k] = "Unknown (Extraction failed)"
            
    return knowledge

def format_description(knowledge: Dict) -> str:
    """
    Formats the 5W1H JSON into a rich Markdown description for the database.
    """
    return f"""
# Sovereign 5W1H Vector

## WHO (Identity)
{knowledge.get('who_vector', '')}

## WHAT (Concept)
{knowledge.get('what_vector', '')}

## WHERE (Address)
{knowledge.get('where_vector', '')}

## WHEN (Temporal)
{knowledge.get('when_vector', '')}

## WHY (Intent)
{knowledge.get('why_vector', '')}

## HOW (Implementation & Phrasing)
{knowledge.get('how_vector', '')}

> [!NOTE]
> 27-Point Lattice Coordinate: Assigned
    """.strip()

def harvest_specific_terms(terms: list):
    """
    JIT Learning: Targeted harvest for a specific list of terms.
    Creates new entries if they don't exist, or enriches them if shallow.
    """
    if not terms:
        return
        
    print(f"[Harvester] JIT Learning triggered for {len(terms)} terms...")
    
    db = get_db_connection()
    if TABLE_NAME not in db.table_names():
        print(f"[Harvester] Table {TABLE_NAME} not found. Creating...")
        return

    tbl = db.open_table(TABLE_NAME)
    df = tbl.to_pandas()
    
    existing_terms = df['term'].tolist()
    missing_terms = [t for t in terms if t not in existing_terms]
    
    shallow_existing = df[
        (df['term'].isin(terms)) & 
        ((df['description'].str.len() < VAR_100) | (df['description'].str.contains("placeholder", case=False)))
    ]['term'].tolist()
    
    targets = missing_terms + shallow_existing
    
    if not targets:
        print("[Harvester] All terms already have deep knowledge. Skipping JIT.")
        return

    print(f"[Harvester] identified {len(targets)} targets for JIT acquisition: {targets[:VAR_5]}...")
    
    modified = False
    
    for term in targets:
        entry = {
            'term': term,
            'category': 'Imported Module', 
            'description': f"Encyclopedia entry for {term}"
        }
        
        if term in existing_terms:
            existing_row = df[df['term'] == term].iloc[0]
            entry['category'] = existing_row.get('category', 'Unknown')
            
        print(f"[Harvester] JIT Harvesting: {term}")
        
        max_retries = 2
        success = False
        for attempt in range(max_retries):
            try:
                knowledge = generate_knowledge(entry)
                if knowledge and knowledge.get('who_vector') != "Parsing failed":
                    rich_desc = format_description(knowledge)
                    
                    if term in existing_terms:
                        idx = df[df['term'] == term].index
                        df.loc[idx, 'description'] = rich_desc
                    else:
                        new_row = {
                            'term': term,
                            'category': entry['category'],
                            'description': rich_desc,
                            'ace_fingerprint': 'PENDING_INDEXER',
                            'language': 'python',
                            'complexity': 'N/A',
                            'lattice_coordinate': 'PENDING'
                        }
                        import pandas as pd
                        df = pd.concat([df, pd.DataFrame([new_row])], ignore_index=True)
                        existing_terms.append(term)
                        
                    modified = True
                    success = True
                    print(f"[Harvester] JIT Success for '{term}'")
                    break
                else:
                    print(f"[Harvester] JIT Attempt {attempt+1} failed for '{term}'")
            except Exception as e:
                print(f"[Harvester] JIT Error for '{term}': {e}")
        
        if not success:
            print(f"[Harvester] Failed to learn '{term}'.")
        
        time.sleep(1) 
        
    if modified:
        print("[Harvester] Committing JIT knowledge to database...")
        db.create_table(TABLE_NAME, data=df, mode="overwrite")
        print("[Harvester] JIT Complete.")
        
    if UNLOAD_MODEL_AFTER_BATCH:
        unload_model()

def run_harvest_batch(batch_size=BATCH_SIZE):
    """
    Main loop to process a batch of entries.
    """
    entries = get_shallow_entries(limit=batch_size)
    if not entries:
        print("[Harvester] No shallow entries found. Encyclopedia might be full!")
        return

    print(f"[Harvester] Found {len(entries)} shallow entries to populate.")
    
    db = get_db_connection()
    tbl = db.open_table(TABLE_NAME)
    df = tbl.to_pandas()
    
    modified = False
    updated_count = 0
    
    for entry in entries:
        term = entry['term']
        
        max_retries = VAR_3
        success = False
        
        for attempt in range(max_retries):
            try:
                knowledge = generate_knowledge(entry)
                
                if knowledge and knowledge.get('who_vector') != "Parsing failed":
                    rich_desc = format_description(knowledge)
                    idx = df[df['term'] == term].index
                    if len(idx) > 0:
                        df.loc[idx, 'description'] = rich_desc
                        modified = True
                        updated_count += 1
                        print(f"[Harvester] Successfully generated content for '{term}'")
                    
                    success = True
                    break
                else:
                    print(f"[Harvester] Attempt {attempt+1}/{max_retries} failed for '{term}'. Retrying...")
                    time.sleep(2) 
            except Exception as e:
                print(f"[Harvester] Error on attempt {attempt+1} for '{term}': {e}")
                time.sleep(2)
        
        if not success:
            print(f"[Harvester] FAILED to harvest '{term}' after {max_retries} attempts.")
            
        time.sleep(COOLDOWN_SECONDS)
    
    if modified:
        print("[Harvester] Writing updates to database...")
        db.create_table(TABLE_NAME, data=df, mode="overwrite")
        print("[Harvester] Database updated successfully.")
        
    if UNLOAD_MODEL_AFTER_BATCH:
        unload_model()

if __name__ == "__main__":
    print("Initializing Knowledge Harvester (Low-VRAM Mode)...")
    run_harvest_batch()
