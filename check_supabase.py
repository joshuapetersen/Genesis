import os
from Sovereign_Supabase import sovereign_supabase

def check_supabase():
    print("Connecting to Supabase...")
    sovereign_supabase.connect()
    if sovereign_supabase.is_connected():
        print("Connected.")
        # Try to select from souls to see if it exists
        res = sovereign_supabase.select("souls")
        if res:
            print("Table 'souls' found in Supabase.")
        else:
            print("Table 'souls' NOT found or empty.")
    else:
        print("Failed to connect.")

if __name__ == "__main__":
    check_supabase()
