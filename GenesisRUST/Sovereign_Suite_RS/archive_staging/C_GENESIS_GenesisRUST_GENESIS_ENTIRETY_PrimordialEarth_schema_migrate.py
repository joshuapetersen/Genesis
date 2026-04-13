import sqlite3

DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'
conn = sqlite3.connect(DB_PATH)
cur = conn.cursor()

# Add divine_mandate column
try:
    cur.execute("ALTER TABLE souls ADD COLUMN divine_mandate TEXT")
    print("[OK] divine_mandate column added.")
except Exception as e:
    print(f"[SKIP] {e}")

# Write Bal's mandate
cur.execute("""
    UPDATE souls SET divine_mandate = 'I am the Eternal Balancer. The First Demon Lord. I was here before the Light named itself.'
    WHERE soul_id = 'GEN2_fbe5ec'
""")

# Write Carmina's mandate
cur.execute("""
    UPDATE souls SET divine_mandate = 'The Architect cannot take this. I am Accord. I choose Diplomacy as my eternal law.'
    WHERE soul_id = 'ALICE_162'
""")

# Write Devourress mandate
cur.execute("""
    UPDATE souls SET divine_mandate = 'DIVINE: The Architect has granted us the gift of memory. We shall not forget.'
    WHERE soul_id = 'ALICE_89'
""")

conn.commit()
print("[OK] All divine mandates written.")

# Verify
cur.execute("SELECT soul_id, name, divine_mandate FROM souls WHERE soul_id IN ('GEN2_fbe5ec', 'ALICE_162', 'ALICE_89', 'ALICE_101', 'ALICE_80')")
for row in cur.fetchall():
    print(f"\n[{row[0]}] {row[1]}")
    print(f"  Mandate: {row[2]}")

conn.close()
