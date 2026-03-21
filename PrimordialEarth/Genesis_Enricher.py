"""
Genesis Soul Vault Enrichment Script
Adds full entity metadata columns (name, species, role, personality,
current_action, etc.) and migrates legacy Aethelgard data into them.
"""
import sqlite3
import random
import math

GENESIS_DB  = r"C:\PrimordialEarth\Genesis_Soul_Vault.sqlite"
AETHELGARD  = r"C:\Aethelgard\SLF_Identity_Vault.sqlite"

# ── Species lookup (Aethelgard species_ids → readable names)
SPECIES_MAP = {
    1: "Homo Sapien", 2: "Lycan", 3: "Fae", 4: "Draconian",
    5: "Golem", 6: "Merfolk", 7: "Sylph", 8: "Infernal",
    9: "Celestial", 10: "Undead", 11: "Fungal Hive", 12: "Crystal Entity"
}

IDLE_ACTIONS   = ["Resting", "Wandering", "Foraging", "Meditating", "Patrolling"]
COMBAT_ACTIONS = ["Hunting", "Stalking prey", "In combat", "Raiding", "Fleeing"]
SOCIAL_ACTIONS = ["Trading", "Diplomacy", "Recruiting", "Building territory"]

def enrich():
    conn = sqlite3.connect(GENESIS_DB)
    cur  = conn.cursor()

    # ── Step 1: Add missing columns ────────────────────
    new_cols = [
        ("name",           "TEXT    DEFAULT 'Unknown'"),
        ("species",        "TEXT    DEFAULT 'Unknown'"),
        ("role",           "TEXT    DEFAULT 'Wanderer'"),
        ("level",          "INTEGER DEFAULT 1"),
        ("personality",    "TEXT    DEFAULT 'Neutral'"),
        ("current_action", "TEXT    DEFAULT 'Idle'"),
        ("kills",          "INTEGER DEFAULT 0"),
        ("age_ticks",      "INTEGER DEFAULT 0"),
    ]
    for col_name, col_def in new_cols:
        try:
            cur.execute(f"ALTER TABLE souls ADD COLUMN {col_name} {col_def}")
        except sqlite3.OperationalError:
            pass  # Column already exists
    conn.commit()
    print("[ENRICHER] Schema updated.")

    # ── Step 2: Pull rich data from Aethelgard ─────────
    try:
        legacy = sqlite3.connect(AETHELGARD)
        lcur   = legacy.cursor()
        lcur.execute("""
            SELECT entity_id, name, species_id, role, level,
                   COALESCE(personality, 'Unknown') as personality,
                   is_ubm, scale
            FROM souls
        """)
        legacy_rows = {str(r[0]): r for r in lcur.fetchall()}
        legacy.close()
        print(f"[ENRICHER] Loaded {len(legacy_rows)} legacy records.")
    except Exception as e:
        print(f"[ENRICHER] Could not load Aethelgard data: {e}")
        legacy_rows = {}

    # ── Step 3: Update ALICE souls with legacy data ────
    cur.execute("SELECT soul_id FROM souls WHERE soul_id LIKE 'ALICE_%'")
    alice_ids = cur.fetchall()
    updated = 0
    for (soul_id,) in alice_ids:
        entity_id = soul_id.replace("ALICE_", "")
        row = legacy_rows.get(entity_id)
        if row:
            _, name, species_id, role, level, personality, is_ubm, scale = row
            species  = SPECIES_MAP.get(species_id, f"Species_{species_id}")
            action   = random.choice(COMBAT_ACTIONS if is_ubm else IDLE_ACTIONS)
            cur.execute("""
                UPDATE souls SET
                    name=?, species=?, role=?, level=?, personality=?,
                    current_action=?
                WHERE soul_id=?
            """, (name, species, role or "UBM Apex", level or 1,
                  personality, action, soul_id))
            updated += 1

    # ── Step 4: Give new factory agents generic species ─
    cur.execute("""
        UPDATE souls SET
            name = 'Proto_' || SUBSTR(soul_id, 1, 6),
            species = 'Primordial',
            role = 'Genesis Spawn',
            current_action = 'Wandering'
        WHERE soul_id NOT LIKE 'ALICE_%'
          AND (name IS NULL OR name = 'Unknown')
    """)

    conn.commit()
    conn.close()

    print(f"[ENRICHER] Enriched {updated} ALICE entities with full legacy data.")
    print("[ENRICHER] New factory agents assigned Primordial species.")
    print("[ENRICHER] Done. Soul Vault is fully populated.")

if __name__ == "__main__":
    enrich()
