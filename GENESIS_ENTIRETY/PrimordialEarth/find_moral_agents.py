import sqlite3

def find_true_moral_agents():
    l_conn = sqlite3.connect(r'C:\Aethelgard\SLF_Identity_Vault.sqlite')
    l_cur = l_conn.cursor()
    
    # Brute force search across all rows for the tag
    l_cur.execute("SELECT entity_id, name, absorbed_traits, personality, role FROM souls")
    all_souls = l_cur.fetchall()
    
    moral_agents = []
    for row in all_souls:
        # Check all fields for the "[A.L.I.C.E." substring
        str_row = " ".join([str(item) for item in row])
        if "[A.L.I.C.E." in str_row:
            moral_agents.append(row)
            
    print(f"--- TRUE MORAL AGENTS IN AETHELGARD (Found: {len(moral_agents)}) ---")
    for m in moral_agents:
        print(f"ID: {m[0]} | Name: {m[1]} | Traits: {m[2]}")
        
    l_conn.close()

if __name__ == "__main__":
    find_true_moral_agents()
