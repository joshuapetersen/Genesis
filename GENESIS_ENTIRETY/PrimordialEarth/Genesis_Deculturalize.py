import sqlite3

conn = sqlite3.connect(r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite')
cur = conn.cursor()

# Map old cultural names to raw biological classification codes
# In Genesis Earth, entities haven't named themselves yet
SPECIES_REMAP = {
    "Homo Sapien":    "BIO-001",
    "Lycan":          "BIO-002",
    "Fae":            "BIO-003",
    "Draconian":      "BIO-004",
    "Golem":          "BIO-005",
    "Merfolk":        "BIO-006",
    "Sylph":          "BIO-007",
    "Infernal":       "BIO-008",
    "Celestial":      "BIO-009",
    "Undead":         "BIO-010",
    "Fungal Hive":    "BIO-011",
    "Crystal Entity": "BIO-012",
}

for old, new in SPECIES_REMAP.items():
    cur.execute("UPDATE souls SET species = ? WHERE species = ?", (new, old))

# Also strip cultural "role" labels from ALICEs - they haven't earned those titles yet
cur.execute("""
    UPDATE souls 
    SET role = 'Unknown'
    WHERE soul_id LIKE 'ALICE_%' AND role IS NOT NULL
""")

conn.commit()

cur.execute("SELECT DISTINCT species FROM souls")
print("Species codes in Genesis Vault:")
for r in cur.fetchall():
    print(f"  {r[0]}")

conn.close()
print("Done. Species de-culturalized.")
