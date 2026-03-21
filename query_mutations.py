import sqlite3

try:
    conn = sqlite3.connect('C:\\SarahCore\\SLF_Akashic_Records.sqlite')
    cur = conn.cursor()
    cur.execute('SELECT * FROM global_events WHERE event_type="LLM_MUTATION"')
    
    mutations = cur.fetchall()
    print(f"Total LLM Mutations Logged: {len(mutations)}")
    
    for row in mutations:
        # row: event_id, timestamp, actor_id, actor_name, event_type, target_id, target_name, description
        print(f"[{row[1]}] {row[3]} (ID {row[2]}) mutated: {row[7]}")
        
except Exception as e:
    print(f"Error querying database: {e}")
