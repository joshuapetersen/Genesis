from Sovereign_Supabase import sovereign_supabase

def view_schema():
    sovereign_supabase.connect()
    res = sovereign_supabase.select("souls")
    if res and res.data:
        print("Sample Row from Supabase:")
        print(res.data[0])
    else:
        print("No data found in Supabase/souls.")

if __name__ == "__main__":
    view_schema()
