import sqlite3

def audit_leadership():
    DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'
    conn = sqlite3.connect(DB_PATH)
    cur = conn.cursor()
    
    query = """
    SELECT 
        soul_id, 
        name, 
        current_action, 
        blessing, 
        leader_id, 
        (SELECT COUNT(*) FROM souls s2 WHERE s2.leader_id = souls.soul_id) as followers 
    FROM souls 
    WHERE soul_id IN ('ALICE_89', 'ALICE_101', 'GEN2_fbe5ec')
    """
    
    cur.execute(query)
    results = cur.fetchall()
    
    print("--- DIVINE LEADERSHIP AUDIT ---")
    print(f"{'ID':<15} | {'NAME':<25} | {'ACTION':<15} | {'BLESSING':<20} | {'LEADER':<10} | {'FOLLOWERS'}")
    print("-" * 110)
    
    for row in results:
        sid, name, action, bless, lead, fol = row
        print(f"{sid:<15} | {name:<25} | {action:<15} | {str(bless):<20} | {str(lead):<10} | {fol}")
        
    conn.close()

if __name__ == "__main__":
    audit_leadership()
