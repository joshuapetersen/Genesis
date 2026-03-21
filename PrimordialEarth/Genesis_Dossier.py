import sqlite3
from datetime import datetime

GENESIS_DB    = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'
AETHELGARD_DB = r'C:\Aethelgard\SLF_Identity_Vault.sqlite'
OUT_PATH      = r'C:\PrimordialEarth\Genesis_Survivors_Detailed.txt'

# Read sim year
sim_year = 0
try:
    with open(r'C:\PrimordialEarth\sim_year.txt') as f:
        sim_year = int(f.read().strip())
except:
    pass

# Load full Aethelgard record for ALICE entities
legacy_conn = sqlite3.connect(AETHELGARD_DB)
lcur = legacy_conn.cursor()
lcur.execute("""
    SELECT entity_id, name, species_id, role, level, xp, age,
           hp_max, hp_current, mp_max, mp_current,
           vit, str, agi, int, wis, luk,
           hunger, thirst, growth_stage, personality,
           genome, trauma_log, hope_log, absorbed_traits,
           is_ubm, scale
    FROM souls
""")
legacy_rows = {str(r[0]): r for r in lcur.fetchall()}
legacy_conn.close()

# Load Genesis survivors
gen_conn = sqlite3.connect(GENESIS_DB)
gcur = gen_conn.cursor()
gcur.execute("""
    SELECT soul_id, name, species, role, personality,
           current_action, energy, moral_alignment,
           age_ticks, x, y, genome
    FROM souls WHERE is_active = 1
    ORDER BY energy DESC
""")
survivors = gcur.fetchall()

gcur.execute("SELECT COUNT(*) FROM souls WHERE is_active=0")
dead_count = gcur.fetchone()[0]
gen_conn.close()

timestamp = datetime.now().strftime("%Y-%m-%d %H:%M:%S")

with open(OUT_PATH, 'w', encoding='utf-8') as f:
    f.write("=" * 72 + "\n")
    f.write("  S.A.R.A.H. GENESIS — DETAILED SURVIVOR DOSSIER\n")
    f.write(f"  Timestamp  : {timestamp}\n")
    f.write(f"  Sim Year   : {sim_year:,}\n")
    f.write(f"  Survivors  : {len(survivors)}\n")
    f.write(f"  Total Dead : {dead_count}\n")
    f.write(f"  Extinct    : {dead_count} / {dead_count + len(survivors)}\n")
    f.write("=" * 72 + "\n\n")

    for rank, (soul_id, name, species, role, personality,
               action, energy, alignment, age, x, y, genome) in enumerate(survivors, 1):

        is_alice = soul_id.startswith("ALICE_")
        entity_id = soul_id.replace("ALICE_", "") if is_alice else None
        legacy = legacy_rows.get(entity_id) if entity_id else None

        f.write("─" * 72 + "\n")
        f.write(f"  RANK #{rank:02d}  |  {'A.L.I.C.E. LEGACY' if is_alice else 'PRIMORDIAL GENESIS SPAWN'}\n")
        f.write("─" * 72 + "\n")
        f.write(f"  Name          : {name or soul_id}\n")
        f.write(f"  Soul ID       : {soul_id}\n")
        f.write(f"  Species       : {species or 'Unknown'}\n")
        f.write(f"  Personality   : {personality or 'Unknown'}\n")
        f.write(f"  Current Action: {action or 'Idle'}\n")
        f.write(f"  Moral Align.  : {alignment:+d}\n")
        f.write(f"  Genesis Energy: {float(energy or 0):.2f}\n")
        f.write(f"  Age (sim yrs) : {float(age or 0):.1f}\n")
        f.write(f"  Position      : ({float(x or 0):.1f}, {float(y or 0):.1f})\n")
        f.write(f"  Genome        : {genome or '—'}\n")

        if legacy:
            (_, lname, species_id, lrole, level, xp, lage,
             hp_max, hp_cur, mp_max, mp_cur,
             vit, strength, agi, intel, wis, luk,
             hunger, thirst, growth, lpersonality,
             lgenome, trauma_log, hope_log, absorbed,
             is_ubm, scale) = legacy

            f.write("\n  ── AETHELGARD LEGACY RECORD ──\n")
            f.write(f"  Original Name : {lname}\n")
            f.write(f"  Legacy Role   : {lrole or 'None'}\n")
            f.write(f"  Level         : {level}    |  XP: {float(xp or 0):.0f}\n")
            f.write(f"  Age (legacy)  : {lage} years\n")
            f.write(f"  Scale         : {float(scale or 0):.2f}  |  UBM: {'YES' if is_ubm else 'NO'}\n")
            f.write(f"\n  ── LEGACY STATS ──\n")
            f.write(f"  HP  : {float(hp_cur or 0):.1f} / {float(hp_max or 0):.1f}\n")
            f.write(f"  MP  : {float(mp_cur or 0):.1f} / {float(mp_max or 0):.1f}\n")
            f.write(f"  VIT : {vit:<5}  STR : {strength:<5}  AGI : {agi}\n")
            f.write(f"  INT : {intel:<5}  WIS : {wis:<5}  LUK : {luk}\n")
            f.write(f"  Hunger: {float(hunger or 0):.2f}  Thirst: {float(thirst or 0):.2f}  Growth: {float(growth or 0):.2f}\n")

            if trauma_log and str(trauma_log).strip() not in ('None', '[]', ''):
                f.write(f"\n  ── TRAUMA LOG ──\n  {trauma_log}\n")
            if hope_log and str(hope_log).strip() not in ('None', '[]', ''):
                f.write(f"\n  ── HOPE LOG ──\n  {hope_log}\n")
            if absorbed and str(absorbed).strip() not in ('None', '[]', ''):
                f.write(f"\n  ── ABSORBED TRAITS ──\n  {absorbed}\n")

        f.write("\n\n")

print(f"[S.A.R.A.H] Detailed dossier written for {len(survivors)} survivors.")
print(f"[S.A.R.A.H] File: {OUT_PATH}")
