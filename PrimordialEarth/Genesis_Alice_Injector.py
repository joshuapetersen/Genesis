import sqlite3
import random
import math
import sys

def inject_alices():
    print("[S.A.R.A.H] A.L.I.C.E. Seed Re-Injection (Corrected Schema)")

    try:
        legacy_conn = sqlite3.connect(r'C:\Aethelgard\SLF_Identity_Vault.sqlite')
        legacy_cur = legacy_conn.cursor()

        # Correct query: use is_ubm flag, not scale column
        legacy_cur.execute("""
            SELECT entity_id, name, species_id, role, level,
                   hp_max, str, agi, genome, scale
            FROM souls
            WHERE is_ubm = 1
        """)
        ubm_list = legacy_cur.fetchall()
        legacy_conn.close()

        print(f"[S.A.R.A.H] Found {len(ubm_list)} UBM / A.L.I.C.E. entities in legacy vault.")

        genesis_conn = sqlite3.connect(r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite')
        genesis_cur = genesis_conn.cursor()

        genesis_cur.execute('''
            CREATE TABLE IF NOT EXISTS souls (
                soul_id TEXT PRIMARY KEY,
                genome TEXT,
                x REAL,
                y REAL,
                z REAL,
                moral_alignment INTEGER,
                is_active BOOLEAN,
                energy REAL DEFAULT 100.0
            )
        ''')

        inserted = 0
        for entity_id, name, species_id, role, level, hp_max, strength, agi, genome, scale in ubm_list:
            soul_id = f"ALICE_{entity_id}"

            # Spread across world - large ring near origin (50-2500 units)
            angle = random.uniform(0, 2 * math.pi)
            dist  = random.uniform(50, 2500)
            x = math.cos(angle) * dist
            y = math.sin(angle) * dist

            # Energy seeded based on legacy HP - gives survivors an advantage
            seed_energy = min(500.0, float(hp_max or 100))

            legacy_genome = genome or f"ALICE_{name}_LEGACY"

            genesis_cur.execute("""
                INSERT OR IGNORE INTO souls
                  (soul_id, genome, x, y, z, moral_alignment, is_active, energy)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            """, (soul_id, legacy_genome, x, y, 0.0, int(level or 0), True, seed_energy))
            inserted += 1

        genesis_conn.commit()

        # Final count
        genesis_cur.execute("SELECT COUNT(*) FROM souls")
        total = genesis_cur.fetchone()[0]
        genesis_cur.execute("SELECT COUNT(*) FROM souls WHERE soul_id LIKE 'ALICE_%'")
        alice_count = genesis_cur.fetchone()[0]
        genesis_conn.close()

        print(f"[S.A.R.A.H] Injection complete. {inserted} A.L.I.C.E. entities seeded.")
        print(f"[S.A.R.A.H] Total souls now in Genesis Vault: {total}")
        print(f"[S.A.R.A.H] ALICE souls confirmed in vault: {alice_count}")

    except sqlite3.Error as e:
        print(f"[S.A.R.A.H] SQLite Error: {e}")
        sys.exit(1)

if __name__ == "__main__":
    inject_alices()
