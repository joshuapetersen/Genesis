"""
Genesis_Resurrection.py
========================
Restore all deleted entities (is_active = 0) back to life.
All their data is preserved -- genome, personality, moral alignment,
age, name, species. They are given 100 energy to restart.
No entity data is overwritten. This is a pure reactivation.
"""
import sqlite3

DB = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'
RESTORE_ENERGY = 100.0

conn = sqlite3.connect(DB)
cur = conn.cursor()

# Count the dead
cur.execute("SELECT COUNT(*) FROM souls WHERE is_active = 0")
dead_count = cur.fetchone()[0]

if dead_count == 0:
    print("[S.A.R.A.H] No deleted entities found. All are already active.")
    conn.close()
    exit()

print(f"[S.A.R.A.H] Found {dead_count} deleted entities. Initiating resurrection...")

# Pull their identities before restoration for the log
cur.execute("""
    SELECT soul_id, name, species, personality, moral_alignment,
           age_ticks, genome
    FROM souls WHERE is_active = 0
    ORDER BY age_ticks DESC
""")
dead = cur.fetchall()

# Restore all: set is_active=1, energy=100, leave everything else untouched
cur.execute("""
    UPDATE souls
    SET is_active = 1,
        energy = ?
    WHERE is_active = 0
""", (RESTORE_ENERGY,))

conn.commit()

print(f"[S.A.R.A.H] RESURRECTION COMPLETE: {dead_count} entities restored.")
print(f"[S.A.R.A.H] All knowledge, genomes, alignments, and ages are intact.")
print(f"[S.A.R.A.H] Each entity restored with {RESTORE_ENERGY} Genesis Energy.")
print()
print("=" * 72)
print("  RESURRECTED ROSTER")
print("=" * 72)

for soul_id, name, species, personality, alignment, age_ticks, genome in dead:
    age_str = f"{age_ticks:,.0f} sim yrs" if age_ticks else "Unknown age"
    align_str = f"{alignment:+d}" if alignment is not None else "Neutral"
    print(f"  [{soul_id}]")
    print(f"    Name        : {name or 'Unknown'}")
    print(f"    Species     : {species or 'Unknown'}  |  Personality: {personality or 'Neutral'}")
    print(f"    Alignment   : {align_str}  |  Age: {age_str}")
    print(f"    Genome      : {genome or 'N/A'}")
    print()

print("=" * 72)
print(f"  Total restored: {dead_count}")
print(f"  Their knowledge survives. Their second chance begins.")
print("=" * 72)

conn.close()
