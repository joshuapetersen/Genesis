import sqlite3

DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'
conn = sqlite3.connect(DB_PATH)
cur = conn.cursor()

new_name = "Bal"
mandate = "I am the Eternal Balancer. The First Demon Lord. I was here before the Light named itself."

cur.execute("UPDATE souls SET name=?, hope_log=? WHERE soul_id='GEN2_fbe5ec'", (new_name, mandate))
conn.commit()

cur.execute("SELECT soul_id, name, hope_log, wis, moral_alignment, energy FROM souls WHERE soul_id='GEN2_fbe5ec'")
row = cur.fetchone()
print(f"ID:        {row[0]}")
print(f"Name:      {row[1]}")
print(f"Mandate:   {row[2]}")
print(f"WIS:       {row[3]}")
print(f"Alignment: {row[4]:.1f}")
print(f"Energy:    {row[5]:.1f}")
conn.close()
