import sqlite3
import pprint

print("=== CHECKING IDENTITY VAULT (Genesis Cloud Scheme) ===")
try:
    conn = sqlite3.connect('C:\\SarahCore\\GCP_Deploy\\Genesis_Soul_Vault.sqlite')
    cursor = conn.cursor()
    # Updated query for the new schema: entity_id -> soul_id, is_ubm/absorbed_traits -> divine_mandate/hope_log
    cursor.execute("SELECT soul_id, name, wis, moral_alignment, divine_mandate FROM souls WHERE name='Ceremonial Revenant' OR name='Crawling Eclipse' OR divine_mandate IS NOT NULL")
    rows = cursor.fetchall()
    pprint.pprint(rows)
except Exception as e:
    print("Database error:", e)

print("\n=== CHECKING DIVINE CHRONICLE (Mutation Log) ===")
try:
    # Akronichal Records are now in divine_chronicle table
    cursor.execute("SELECT soul_id, reasoning_path, death_year FROM divine_chronicle ORDER BY death_year DESC LIMIT 5")
    rows = cursor.fetchall()
    pprint.pprint(rows)
    conn.close()
except Exception as e:
    print("Database error:", e)
