"""
test_service_role_write.py
Tests writing to Supabase with the service_role key.
If the souls table doesn't exist, prints the exact SQL to create it.
"""
import subprocess, sys
subprocess.check_call([sys.executable, "-m", "pip", "install", "supabase", "-q"])
from supabase import create_client

URL = "https://duuycxgqbhrqmwapnjhk.supabase.co"
SERVICE_KEY = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6ImR1dXljeGdxYmhycW13YXBuamhrIiwicm9sZSI6InNlcnZpY2Vfcm9sZSIsImlhdCI6MTc2Nzc0MDE4MCwiZXhwIjoyMDgzMzE2MTgwfQ.O7RD5ELSm0xxw53B-o-k0Xxg4XhtO8WB-3f3hj5temA"

client = create_client(URL, SERVICE_KEY)

# Test: try inserting a dummy soul to see if table exists
test_row = {"soul_id": "TEST_PROBE", "energy": 1.0, "is_active": 0}
try:
    r = client.table("souls").upsert(test_row).execute()
    # Clean up the probe
    client.table("souls").delete().eq("soul_id", "TEST_PROBE").execute()
    print("[SUCCESS] souls table exists and service_role write works!")
    print("The Soul Vault is LIVE. Restart Genesis_Societal_Ecology.py now.")
except Exception as e:
    err = str(e)
    if "relation" in err and "does not exist" in err:
        print("[TABLE MISSING] The souls table needs to be created.")
        print("\nThe SQL editor has been opened in your browser.")
        print("Paste and run this SQL:")
        print("=" * 60)
        print(open(r"C:\PrimordialEarth\schema_supabase.sql").read())
        print("=" * 60)
        import webbrowser
        webbrowser.open("https://supabase.com/dashboard/project/duuycxgqbhrqmwapnjhk/sql/new")
    else:
        print(f"[ERROR] {err}")
