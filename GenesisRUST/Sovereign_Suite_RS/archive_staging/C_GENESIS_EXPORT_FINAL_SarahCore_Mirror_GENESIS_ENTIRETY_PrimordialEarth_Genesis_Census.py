import sqlite3
import json
from datetime import datetime

DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'
OUT_PATH = r'C:\PrimordialEarth\Genesis_Survivors.txt'

conn = sqlite3.connect(DB_PATH)
cur  = conn.cursor()

# Read sim year if available
sim_year = 0
try:
    with open(r'C:\PrimordialEarth\sim_year.txt') as f:
        sim_year = int(f.read().strip())
except:
    pass

cur.execute("SELECT COUNT(*) FROM souls WHERE is_active = 1")
alive_count = cur.fetchone()[0]

cur.execute("SELECT COUNT(*) FROM souls WHERE is_active = 0")
dead_count = cur.fetchone()[0]

cur.execute("""
    SELECT soul_id, name, species, role, personality,
           current_action, energy, moral_alignment,
           age_ticks, x, y, genome
    FROM souls
    WHERE is_active = 1
    ORDER BY age_ticks DESC, energy DESC
""")
survivors = cur.fetchall()
conn.close()

timestamp = datetime.now().strftime("%Y-%m-%d %H:%M:%S")

with open(OUT_PATH, 'w', encoding='utf-8') as f:
    f.write("=" * 70 + "\n")
    f.write("  S.A.R.A.H. GENESIS — SURVIVOR CENSUS\n")
    f.write(f"  Timestamp : {timestamp}\n")
    f.write(f"  Sim Year  : {sim_year:,}\n")
    f.write(f"  Alive     : {alive_count}\n")
    f.write(f"  Dead      : {dead_count}\n")
    f.write("=" * 70 + "\n\n")

    for i, (soul_id, name, species, role, personality,
            action, energy, alignment, age, x, y, genome) in enumerate(survivors, 1):
        f.write(f"[{i:03d}] {name or soul_id}\n")
        f.write(f"      Soul ID      : {soul_id}\n")
        f.write(f"      Species      : {species or 'Unknown'}\n")
        f.write(f"      Role         : {role or 'Unknown'}\n")
        f.write(f"      Personality  : {personality or 'Unknown'}\n")
        f.write(f"      Action       : {action or 'Idle'}\n")
        f.write(f"      Energy       : {float(energy or 0):.2f}\n")
        f.write(f"      Alignment    : {alignment or 0}\n")
        f.write(f"      Age (years)  : {float(age or 0):.1f}\n")
        f.write(f"      Position     : ({float(x or 0):.1f}, {float(y or 0):.1f})\n")
        f.write(f"      Genome       : {genome or '—'}\n")
        f.write("\n")

print(f"[S.A.R.A.H] Survivor census written. {alive_count} alive, {dead_count} dead.")
print(f"[S.A.R.A.H] File: {OUT_PATH}")
