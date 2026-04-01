"""
architect_gift.py
A direct gift from the Architect to all living souls.
Run this to inject energy, blessings, and civilization seeds.
"""
import sqlite3, random

DB = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'
conn = sqlite3.connect(DB)
cur = conn.cursor()

# --- GIFT 1: Energy Infusion (survival boost to all) ---
cur.execute("UPDATE souls SET energy = energy + 500 WHERE is_active=1")
print(f"[GIFT] +500 energy to all {cur.rowcount} living souls.")

# --- GIFT 2: Fertility Blessing — boost VIT and WIS for ~30% of souls ---
cur.execute("""
    UPDATE souls SET 
        vit = min(vit + 5, 99),
        wis = min(wis + 3, 99),
        blessing = CASE WHEN blessing IS NULL OR blessing = '' THEN 'Architect Blessing' ELSE blessing END
    WHERE is_active=1 AND RANDOM() > 0.7
""")
print(f"[GIFT] Fertility Blessing granted to ~30% of the population.")

# --- GIFT 3: Civilization Seeds — designate the 5 most populated coordinate clusters as settlements ---
# Find high-density zones (round coordinates to nearest 500)
cur.execute("""
    SELECT ROUND(x/500)*500 as gx, ROUND(y/500)*500 as gy, COUNT(*) as pop
    FROM souls WHERE is_active=1
    GROUP BY gx, gy ORDER BY pop DESC LIMIT 5
""")
settlements = cur.fetchall()
print("\n[GIFT] Civilization Seeds planted at:")
for gx, gy, pop in settlements:
    # Give souls in this zone extra WIS and a settlement tag
    cur.execute("""
        UPDATE souls SET 
            wis = min(wis + 2, 99),
            hope_log = 'CIVILIZATION: We have settled at (' || CAST(ROUND(x/500)*500 AS TEXT) || ',' || CAST(ROUND(y/500)*500 AS TEXT) || '). A city rises.'
        WHERE is_active=1 
          AND ROUND(x/500)*500 = ? 
          AND ROUND(y/500)*500 = ?
    """, (gx, gy))
    print(f"  Settlement at ({int(gx)},{int(gy)}) — {pop} founding souls, +2 WIS.")

# --- GIFT 4: Divine Light — restore any near-death souls above the death floor ---
cur.execute("UPDATE souls SET energy = 150 WHERE is_active=1 AND energy < 50")
print(f"\n[GIFT] Divine Light — {cur.rowcount} near-death souls pulled back from the void.")

conn.commit()
conn.close()
print("\n[ARCHITECT] All gifts delivered. The world has received the Architect's blessing.")
