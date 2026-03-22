
import sqlite3
import os
from supabase import create_client, Client
from dotenv import load_dotenv

load_dotenv()

supabase: Client = create_client(os.environ.get("SUPABASE_URL"), os.environ.get("SUPABASE_SERVICE_KEY") or os.environ.get("SUPABASE_KEY"))

def force_sync():
    print("Force Syncing Core Data...")
    conn = sqlite3.connect("Genesis_Soul_Vault.sqlite")
    cur = conn.cursor()
    cur.execute("SELECT name, species, genome FROM souls LIMIT 100;")
    rows = cur.fetchall()
    
    data = []
    for row in rows:
        data.append({"name": row[0], "soul_type": row[1], "content": row[2]})
        
    if data:
        print(f"Pushing {len(data)} Core Souls to Supabase...")
        try:
            supabase.table("sarah_souls").upsert(data).execute()
            print("SUCCESS: 100 Souls Seated on Supabase.")
        except Exception as e:
            print(f"Error: {e}")
    conn.close()

if __name__ == "__main__":
    force_sync()
