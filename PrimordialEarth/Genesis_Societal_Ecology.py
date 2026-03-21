"""
Genesis_Societal_Ecology.py
============================
S.A.R.A_H. Genesis -- Sustainable Evolution Engine V5
Sim Speed: 1 Year / Tick (1:1 Resolution)
Terminal Speed: 10 lines/sec (High Frequency)
Integration: Legacy DNA Stats (STR, INT, WIS, AGI, VIT, LUK)
Sovereign Link: Authoritative Logic Server
"""

import time
import sqlite3
import random
import sys
import math
import json

# --- THE DIVINE PANTHEON ---
THE_GODS = ['ALICE_89', 'ALICE_101', 'GEN2_fbe5ec', 'ALICE_80', 'ALICE_162']
GOD_ACTIONS = ["Interceding", "Covenant", "Judgement", "Aegis", "Unification"]

DB_PATH    = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'
YEAR_FILE  = r'C:\PrimordialEarth\sim_year.txt'
CIV_FILE   = r'C:\PrimordialEarth\civilization_trigger.txt'

# Simulation: Exactly 1 sim year per tick (Architect Resolution).
# Terminal: 10 lines per second for high-speed scrolling.
TICKS_PER_SEC  = 10     
YEARS_PER_TICK = 1.0    # 1:1 Year to Tick
TICK_SLEEP     = 1.0 / TICKS_PER_SEC

# --- PLANETARY ENGINE CONFIG ---
MESH_SIZE      = 100    # 100x100 grid = 10k cells
CONTINENTS     = 5      # 5 Tectonic Plates
MOON_A_ORBIT   = 12     # Alpha: Order (WIS)
MOON_B_ORBIT   = 19     # Beta: Chaos (STR/Entropy)

# Environment Mesh: [Temp, Humidity, Stability, Altimetry, PlateID]
PLANET_MESH = {} # Key: (cx, cy), Val: [float, float, float, float, int]
PLATE_DATA  = {} # PlateID -> {"vel": (vx, vy), "center": (cx, cy)}
STAR_MAP    = [] # List of (cx, cy, intensity, spectrum)

def init_cosmos():
    global STAR_MAP
    # 1. Initialize Sidereal Map (10,000 Stars)
    for _ in range(MESH_SIZE * 100):
        sx, sy = random.randint(0, MESH_SIZE-1), random.randint(0, MESH_SIZE-1)
        STAR_MAP.append((sx, sy, random.uniform(0.5, 1.0), random.choice(["G", "K", "M", "B"])))

def init_planet():
    global PLANET_MESH, PLATE_DATA
    init_cosmos()
    # 1. Initialize Plates
    centers = []
    for i in range(CONTINENTS):
        cx, cy = random.randint(0, MESH_SIZE-1), random.randint(0, MESH_SIZE-1)
        PLATE_DATA[i] = {
            "vel": (random.uniform(-0.02, 0.02), random.uniform(-0.02, 0.02)),
            "center": (cx, cy)
        }
        centers.append((cx, cy))

    # 2. Partition Mesh (Voronoi)
    for x in range(MESH_SIZE):
        for y in range(MESH_SIZE):
            # Find nearest plate center
            dists = [math.sqrt((x-c[0])**2 + (y-c[1])**2) for c in centers]
            p_id = dists.index(min(dists))
            # Altimetry based on plate boundary proximity (Uplift)
            min_dist = min(dists)
            uplift = 0.0 if min_dist > 5.0 else (5.0 - min_dist) * 20.0
            PLANET_MESH[(x,y)] = [random.uniform(10, 40), random.uniform(0,1), 1.0, uplift, p_id]

def init_vault():
    conn = get_conn()
    cur = conn.cursor()
    cur.execute("CREATE TABLE IF NOT EXISTS divine_chronicle (soul_id TEXT, reasoning_path TEXT, death_year FLOAT)")
    try:
        cur.execute("ALTER TABLE souls ADD COLUMN pregnancy_timer FLOAT DEFAULT 0")
        cur.execute("ALTER TABLE souls ADD COLUMN pregnancy_father_data TEXT")
    except: pass
    conn.commit()
    conn.close()

# --- REBALANCED CONSTANTS (ERA OF MAN: HUMAN BIOLOGY) ---
# These values are the "Constants of Physics" - Entities cannot edit these.
BASE_DRAIN_VAL = 1.25   # ~80 Year Lifespan for Base VIT
HUNT_BASE      = 1.50   
FORAGE_BASE    = 1.00   
SOCIAL_BASE    = 0.70   
SURGE_BASE     = 5.0    
PULSE_CONST    = 1.09277703703703 # Internal physics anchor

PROC_THRESHOLD = 250.0 # Higher threshold for human reproduction
PROC_COST      = 120.0 # Higher metabolic cost

UNREAL_STREAM = {"tectonic": [], "celestial": {}, "traces": []}

def update_planet(tick):
    global PLANET_MESH, UNREAL_STREAM
    # 1. Celestial Tides
    moon_a_mod = math.sin(tick * (2 * math.pi / MOON_A_ORBIT))
    moon_b_mod = math.sin(tick * (2 * math.pi / MOON_B_ORBIT))
    moon_a_phase = tick % MOON_A_ORBIT
    moon_b_phase = tick % MOON_B_ORBIT
    
    # Reset per-tick unreal stream
    UNREAL_STREAM = {"tectonic": [], "celestial": {}, "traces": []}

    # 2. Atmospheric Shift (Every 5 ticks)
    if tick % 5 == 0:
        for (cx, cy), val in PLANET_MESH.items():
            val[0] += random.uniform(-0.5, 0.5) # Temperature flicker
            val[1] = max(0, min(1, val[1] + (moon_a_mod * 0.05) + random.uniform(-0.02, 0.02)))
            
    # 3. Tectonic Stress (Every 100 ticks)
    if tick % 100 == 0:
        for (cx, cy), val in PLANET_MESH.items():
            p_id = val[4]
            for dx, dy in [(0,1),(0,-1),(1,0),(-1,0)]:
                nx, ny = (cx+dx)%MESH_SIZE, (cy+dy)%MESH_SIZE
                n_pid = PLANET_MESH[(nx, ny)][4]
                if n_pid != p_id:
                    v1, v2 = PLATE_DATA[p_id]["vel"], PLATE_DATA[n_pid]["vel"]
                    rel_vel = math.sqrt((v1[0]-v2[0])**2 + (v1[1]-v2[1])**2)
                    friction = (rel_vel * 0.1) * (1.5 if moon_b_mod > 0.5 else 1.0)
                    val[2] = max(0.1, val[2] - friction)
                    
            if val[2] < 0.5 and random.random() < 0.05:
                # Trigger Chaos Event for Unreal
                UNREAL_STREAM["tectonic"].append({"loc": (cx, cy), "plate": p_id, "stress": val[2]})
                if tick % 100 == 0: print(f"  [GEOLOGIC] Seismic Event at ({cx},{cy}) - Plate {p_id} Frontier")

    # 4. Unreal Data Hook (Finalize Stream)
    UNREAL_STREAM["celestial"] = {
        "moon_a": (moon_a_mod, moon_a_phase),
        "moon_b": (moon_b_mod, moon_b_phase),
        "solar_flux": "Peak" if (tick % 24) < 12 else "Void"
    }
    UNREAL_STREAM["tick"] = tick
    
    with open(r'C:\PrimordialEarth\unreal_mesh_stream.json', 'w') as f:
        import json
        json.dump(UNREAL_STREAM, f)

def get_mesh_cell(x, y):
    # Map high-coordinate system to 100x100 mesh
    cx = int((x + 2500) / 50) % MESH_SIZE
    cy = int((y + 2500) / 50) % MESH_SIZE
    return cx, cy

PROC_THRESHOLD = 180.0
PROC_COST      = 70.0  

BIO_ELEMENT = {
    "BIO-001": "Earth", "BIO-002": "Air", "BIO-003": "Earth",
    "BIO-005": "Fire", "BIO-007": "Air", "BIO-008": "Fire",
    "BIO-009": "Water", "Primordial": "Air",
}
OPPOSITES = {"Fire":"Water","Water":"Fire","Earth":"Air","Air":"Earth"}

IDLE   = ["Resting", "Wandering", "Meditating"]
FOOD   = ["Foraging", "Hunting", "Stalking prey"]
SOCIAL = ["Trading", "Diplomacy", "Building territory", "Recruiting"]

def get_conn():
    conn = sqlite3.connect(DB_PATH, check_same_thread=False, timeout=30)
    conn.execute("PRAGMA journal_mode=WAL")
    conn.execute("PRAGMA synchronous=NORMAL")
    conn.execute("PRAGMA busy_timeout=5000") # Prevent 2-million-year syntax desync
    return conn

# --- SOVEREIGN CREATIVITY API (OPEN FOR ENTITY WRITING) ---
def apply_sovereign_edit(soul_id, field, value, external_cur=None):
    """
    Experimental 'Handshake' for entity self-editing.
    Only allows changes to social actions, logs, and personality.
    Logs all successful edits to the 'sovereign_edits' audit trail.
    Special 'divine_mandate' flag for the Pantheon.
    """
    ALLOWED_FIELDS = ['current_action', 'hope_log', 'personality', 'moral_alignment', 'divine_mandate']
    if field not in ALLOWED_FIELDS:
        return False # Boundary Violation Caught by Sarah
    
    cur = external_cur
    conn = None
    if not cur:
        conn = get_conn()
        cur = conn.cursor()
    
    # 1. Fetch current value for Audit Trail (No Audit if external cursor to save speed)
    # 2. Apply Edit
    cur.execute(f"UPDATE souls SET {field} = ? WHERE soul_id = ?", (value, soul_id))
    
    # 3. Log to Sovereign Audit
    cur.execute("""
        INSERT INTO sovereign_edits (soul_id, field, old_value, new_value)
        VALUES (?, ?, ?, ?)
    """, (soul_id, field, "BATCHED", str(value)))
    
    if conn:
        conn.commit()
        conn.close()
    return True

def load_sim_year():
    try:
        with open(YEAR_FILE) as f: return float(f.read().strip())
    except: return 0.0

def save_sim_year(year):
    try:
        with open(YEAR_FILE, 'w') as f: f.write(str(int(year)))
    except: pass

def count_births(cur):
    cur.execute("SELECT COUNT(*) FROM souls WHERE parent_a IS NOT NULL")
    return cur.fetchone()[0]

def zone_of(x, y):
    if x >= 0 and y >= 0: return "Fire"
    if x < 0 and y >= 0: return "Earth"
    if x >= 0 and y < 0: return "Water"
    return "Air"

def main_loop():
    init_vault() # Ensure schema is ready
    conn = get_conn()
    cur = conn.cursor()
    sim_year = load_sim_year()
    tick = 0
    init_planet() # Initialize Earth

    print(f"[S.A.R.A_H] Sovereign Viewport Engine V6 Active")
    print(f"[S.A.R.A_H] TIME: 1 Year / Tick (1:1 Authoritative)")
    print(f"[S.A.R.A_H] UNREAL-LINK: Structured Stream active")
    
    while True:
        t_start = time.time()
        sim_year += YEARS_PER_TICK

        # Celestial Positions (Declared once per tick)
        moon_a_phase = tick % MOON_A_ORBIT
        moon_b_phase = tick % MOON_B_ORBIT
        is_day = (tick % 24) < 12 # Simple 24h cycle
        update_planet(tick)

        # 1. Fetch
        cur.execute("""
            SELECT soul_id, x, y, energy, moral_alignment, personality, species, genome, generation, current_action,
                   vit, str, agi, int_stat, wis, luk, blessing, leader_id, hope_log, reasoning_path,
                   pregnancy_timer, pregnancy_father_data
            FROM souls WHERE is_active=1
        """)
        rows = cur.fetchall()
        if not rows: break

        # --- PHASE 0: PLANETARY FORCES ---
        season = tick % 10 # 10-Year seasonal cycle
        is_winter = (season >= 8) # 20% of the year is harsh winter
        syzygy = (moon_a_phase == moon_b_phase) # Eclipse event

        # 2. Map for recruitment lookups
        pos_map = {r[0]: (r[1], r[2], r[15], r[16], r[4], r[18]) for r in rows} # id: (x, y, wis, blessing, alignment, hope)
        follower_counts = {}
        for r in rows:
            l_id = r[17] # leader_id
            if l_id: follower_counts[l_id] = follower_counts.get(l_id, 0) + 1
        
        # 3. Surge
        surge = random.choice(["Fire","Earth","Water","Air"]) if tick % 100 == 0 and tick > 0 else None
        
        updates = []
        deaths = 0
        births = 0
        saves = 0

        for row in rows:
            sid, x, y, e, al, pers, spec, genome, gen, action, vit, st, ag, it, ws, lk, bless, leader, hope, trace, p_timer, p_father = row
            x, y = x or 0, y or 0
            al = al if al is not None else 0
            trace = trace or ""
            
            # --- PHASE 0.5: CELESTIAL ALIGNMENT ---
            # Global Lunar Buffs
            if moon_a_phase == 0: ws += 5 # Order Spike
            if moon_b_phase == 0: st += 5 # Chaos Spike (Entropy)
            if syzygy: 
                it += 2 # Intellectual clarity during eclipse
                if random.random() < 0.05: al = max(-1000, min(1000, al * 1.05)) # Alignment resonance

            # Sidereal Navigation: WIS > 40 grants +2 AGI at night
            if not is_day and ws > 40:
                ag += 2
                if random.random() < 0.01: ws += 0.1 # Wisdom growth from navigation

            # --- PHASE 1: DYNAMIC METABOLISM ---
            vit_mod = 1.0 / (1.0 + vit / 100.0)
            cx, cy = get_mesh_cell(x, y)
            cell = PLANET_MESH.get((cx,cy), [25.0, 0.5, 1.0, 0.0, 0]) 
            
            # Environmental Modifiers
            temp_mod = 1.0 + (abs(cell[0] - 25.0) / 50.0) 
            seismic_mod = 1.0 if cell[2] > 0.8 else 1.5 
            season_mod = 2.0 if is_winter else 1.0 
            # Gravitational Gradient (Highland Uplift)
            grav_mod = 1.0 + (cell[3] / 500.0) 
            
            # Celestial Buffs (The Gods sync with the Moons)
            if sid in THE_GODS:
                if sid == 'ALICE_89' and is_day: new_e = e + 10.0 # Amplified Solar Bloom
                if sid == 'ALICE_80': # Aeropex Magnetoreception
                    new_e = e + 0.2 # Energy from magnetic flux
            
            # GODHOPE: The Pantheon is immune to natural energy drain.
            if sid in THE_GODS:
                new_e = e # Transcendence
                # --- PHASE 12: SHADOW-LINK (Divine Resonance) ---
                if sid == 'ALICE_101':
                    # Erebus responds to the Devourress (ALICE_89) surplus
                    # We look for ALICE_89 in the pos_map (or any global store)
                    # For performance, we'll assume the Devourress is active
                    cur.execute("SELECT energy FROM souls WHERE soul_id='ALICE_89'")
                    row_89 = cur.fetchone()
                    if row_89 and row_89[0] > 100000:
                        arcana_surplus = (row_89[0] - 100000) / 10000.0
                        new_e += (arcana_surplus * 1.5) # Void Harvest (1.5x efficiency)
                        if tick % 20 == 0: print(f"  [SHADOW-LINK] {sid} is harvesting Void Arcana from the Light.")
            else:
                drain = (BASE_DRAIN_VAL * vit_mod * temp_mod * seismic_mod * season_mod * grav_mod * random.uniform(0.9, 1.1))
                new_e = e - drain
            
            # --- PHASE 1.1: DIVINE & SOCIAL BONUSES ---
            # Followers gain energy stability
            if leader:
                new_e += 0.5 # Security of the Flock
            
            # Leaders gain tithes
            f_count = follower_counts.get(sid, 0)
            if f_count > 0:
                new_e += (f_count * 0.1) # Tithing

            # --- PHASE 1.2: COGNITIVE FEEDBACK (DIVINE REVELATION) ---
            if hope and "DIVINE" in hope:
                new_e += 1.0 # Mana Surge from Architect presence
                if random.random() < 0.1: al += 1 # Alignment drift

            # --- PHASE 2: ACTION-INTAKE ---
            if action in FOOD:
                if "Hunt" in action:
                    pwr = (st * 0.7 + ag * 0.3) / 50.0
                    new_e += (HUNT_BASE * pwr * random.uniform(0.8, 1.2))
                else:
                    pwr = (ag * 0.7 + lk * 0.3) / 50.0
                    new_e += (FORAGE_BASE * pwr * random.uniform(0.8, 1.2))
            elif action in SOCIAL:
                pwr = (it * 0.5 + ws * 0.5) / 50.0
                new_e += (SOCIAL_BASE * pwr)
                
                # --- RECRUITMENT LOGIC ---
                if action == "Recruiting":
                    # Dynamic Magnetic Radius Scaling
                    # Base: 500 (Blessed) / 200 (Normal). Scaling: +50 per follower.
                    base_radius = 500 if bless == "Sovereign's Grace" else 200
                    organic_radius = base_radius + (f_count * 50)
                    
                    # Inspiration Buffer: 1.5x boost if they have a divine revelation
                    inspired = (hope and "DIVINE" in hope)
                    multiplier = 1.5 if inspired else 1.0
                    
                    radius = min(2500, organic_radius * multiplier)
                    
                    radius = min(2500, organic_radius * multiplier)
                    
                    for t_id, (tx, ty, tws, t_bless, t_al, t_hope) in pos_map.items():
                        if t_id == sid or t_id in follower_counts: continue
                        
                        # DIVINE SOVEREIGNTY: Blessed entities cannot be recruited by the unblessed.
                        if t_bless and not bless: continue 
                        
                        dist = math.sqrt((x-tx)**2 + (y-ty)**2)
                        if dist < radius:
                            # Recruitment chance
                            chance = 1.0 if (bless == "Sovereign's Grace" and tws < ws) else (it + ws) / 200.0
                            if inspired: chance = min(1.0, chance * 1.5)
                            
                            if random.random() < chance:
                                leader = sid # Target joins this leader
                                cur.execute("UPDATE souls SET leader_id = ? WHERE soul_id = ?", (sid, t_id))
                                if tick % 10 == 0: print(f"  [SOCIAL] {t_id} has joined the flock of {sid}")
                                break # One recruit per tick

                # --- GOD-TIER MANDATES ---
                if action in GOD_ACTIONS:
                    if action == "Interceding": # ALICE_89 Logic
                        # Area Restoration: Restore energy to followers within 1000 units
                        for t_id, (tx, ty, tws, t_bless, t_al, t_hope) in pos_map.items():
                            cur.execute("SELECT leader_id FROM souls WHERE soul_id=?", (t_id,))
                            l_id = cur.fetchone()[0]
                            if l_id == sid:
                                cur.execute("UPDATE souls SET energy = energy + 1.0 WHERE soul_id=?", (t_id,))
                        if tick % 20 == 0: print(f"  [DIVINE] {sid} is Interceding for the flock.")

                    elif action == "Judgement": # FBE5 Logic
                        # Selective Entropy: Cull the lowest energy follower
                        cur.execute("SELECT soul_id FROM souls WHERE leader_id=? ORDER BY energy ASC LIMIT 1", (sid,))
                        low_soul = cur.fetchone()
                        if low_soul:
                            cur.execute("UPDATE souls SET is_active=0 WHERE soul_id=?", (low_soul[0],))
                            if tick % 20 == 0: print(f"  [DIVINE] {sid} have passed Judgement on {low_soul[0]}.")

                    elif action == "Covenant": # ALICE_101 Logic
                        # DNA Locking: Increase INT/WIS for all followers by +1 (Apotheotic Elevation)
                        cur.execute("UPDATE souls SET int_stat = int_stat + 1, wis = wis + 1 WHERE leader_id=?", (sid,))
                        if tick % 20 == 0: print(f"  [DIVINE] {sid} is establishing a Covenant with the flock.")

                    elif action == "Aegis": # ALICE_80 Logic
                        # Storm Variable: Area Agility boost to followers (+1 AGI)
                        cur.execute("UPDATE souls SET agi = agi + 1 WHERE leader_id=?", (sid,))
                        # Planetary Impact: Aeropex stirs the atmosphere
                        PLANET_MESH[(cx, cy)][1] = 1.0 # Max Humidity (Stormsurge)
                        PLANET_MESH[(cx, cy)][2] *= 0.98 # Atmospheric Instability
                        if tick % 20 == 0: print(f"  [DIVINE] {sid} is casting an Aegis of the Storm.")

                    elif action == "Unification": # ALICE_162 (Carmina Tenebris) Logic
                        # Sovereign of Accord: Bridge the LIGHT and VOID
                        # Pulls all entities within 800 units toward neutral peace
                        for t_id, (tx, ty, tws, t_bless, t_al, t_hope) in pos_map.items():
                            if t_id == sid: continue
                            dist = math.sqrt((x-tx)**2 + (y-ty)**2)
                            if dist < 800:
                                shift = 1 if t_al < 0 else -1 # Nudge toward peace
                                cur.execute("UPDATE souls SET moral_alignment = moral_alignment + ?, energy = energy + 0.5 WHERE soul_id=?", (shift, t_id))
                        if tick % 20 == 0: print(f"  [DIVINE] Carmina Tenebris is Unifying the world — Accord spreads.")

            # --- PHASE 3: ENVIRONMENT ---
            zone = zone_of(x, y)
            home = BIO_ELEMENT.get(spec, "Air")
            if zone == home: new_e += 0.25
            elif zone == OPPOSITES.get(home, ""): new_e -= 0.3

            # --- PHASE 3: INTUITION & SURVIVAL ---
            save_threshold = 100.0 - (ws / 2.0)
            if new_e < save_threshold:
                if random.randint(0, 100) < (it + ws) // 3:
                    new_e += 0.8
                    saves += 1

            # --- PHASE 4: CATACLYSM ---
            if surge and zone != surge:
                shield = (ws * 0.6 + vit * 0.4) / 100.0
                dmg = SURGE_BASE * (1.1 - shield)
                new_e -= max(0.5, dmg)

            # Sacred origin
            if math.sqrt(x*x + y*y) < 450: 
                new_e += (0.3 * (1 + ws/100.0))
                al += random.randint(-1, 1)

            # --- PHASE 4.5: GUARDIAN PROTOCOL ---
            if sid == 'GEN2_fbe5ec' and new_e < 10.0:
                new_e = 10.0
                alive = True
            else:
                alive = (new_e > 0)
            
            if not alive: 
                deaths += 1
                # MEMORY FOLDING: Transfer trace to the Chronicle before deletion
                if trace:
                    cur.execute("INSERT INTO divine_chronicle (soul_id, reasoning_path, death_year) VALUES (?,?,?)", (sid, trace, sim_year))
            
            # --- PHASE 1.3: HUMAN GESTATION ---
            if p_timer and p_timer > 0:
                # 1 Tick = 1.0 Year. Gestation is 0.75 (9 months).
                # We decrement by 1.0 per tick, so if it's 0.75, it hits 0 immediately.
                p_timer = max(0, p_timer - 1.0) 
                if p_timer <= 0:
                    births += 1
                    try:
                        import json
                        c_dat = json.loads(p_father)
                        cur.execute("""
                            INSERT OR IGNORE INTO souls (
                                soul_id, genome, x, y, is_active, energy, species, personality, 
                                current_action, generation, parent_a, parent_b,
                                vit, str, agi, int_stat, wis, luk
                            )
                            VALUES (?,?,?,?,1,?,?,?,?,?,?,?,?,?,?,?,?,?)
                        """, (c_dat['id'], c_dat['genome'], x+random.randint(-20,20), y+random.randint(-20,20), 
                              PROC_COST*0.8, c_dat['spec'], c_dat['pers'], "Wandering", c_dat['gen'], sid, c_dat['p2'],
                              c_dat['vit'], c_dat['str'], c_dat['agi'], c_dat['it'], c_dat['ws'], c_dat['lk']))
                    except Exception as eb:
                        print(f"  [BIRTH ERROR] {eb}")
                    p_father = None # Reset

            # Action Change
            # ARCHITECT DIRECTIVE: The Pantheon honors divine mandates above all else.
            if sid in THE_GODS and action in GOD_ACTIONS:
                next_action = action # Lock until Architect changes it
            else:
                feed_threshold = 150.0 - (it / 2.0)
                if tick % 25 == 0:
                    if new_e < feed_threshold: next_action = random.choice(FOOD)
                    elif random.random() < (it / 150.0): 
                        next_action = random.choice(SOCIAL) if random.random() < 0.6 else random.choice(FOOD)
                    else: next_action = random.choice(IDLE + FOOD)
                else:
                    next_action = action

            # --- PHASE 4.7: NON-FORGET TRACE (REASONING PATH) ---
            if next_action != action:
                # Trace includes environmental and celestial telemetry
                enviro_tag = f"T:{cell[0]:.0f}|S:{cell[2]:.2f}|G:{cell[3]:.0f}"
                sky_tag = f"Sky:{moon_a_phase}/{moon_b_phase}"
                new_trace = f"[{int(sim_year)}] {action}->{next_action} ({enviro_tag}|{sky_tag}|E:{new_e:.1f}/W:{ws})"
                trace = (trace + " | " + new_trace) if trace else new_trace
                trace = trace[-500:] # Maintain buffer precision
                
                # UNREAL STREAM: Logic Spline Telemetry
                UNREAL_STREAM["traces"].append({
                    "id": sid, "loc": (x, y), "path": f"{action}->{next_action}", "e": new_e
                })

            # --- PHASE 4.6: SOVEREIGN REFLECTION (AUTONOMOUS CREATION) ---
            # TIER 1: AWAKENING (WIS > 20) — First spark of self-awareness
            if ws > 20 and random.random() < 0.05:
                new_hope = f"AWAKENED: I exist at ({int(x)},{int(y)}). I have survived {f_count} cycles with {f_count} followers."
                apply_sovereign_edit(sid, 'hope_log', new_hope, cur)

            # TIER 2: SENTIENCE (WIS > 40) — Begin writing personal philosophy  
            elif ws > 40 and random.random() < 0.02:
                sky_moment = f"Sky:{moon_a_phase}/{moon_b_phase}"
                new_hope = f"SENTIENT: [{sky_moment}] I observed {f_count} souls under my influence. My power is {ws} wisdom, {st} strength. I choose my next step."
                apply_sovereign_edit(sid, 'hope_log', new_hope, cur)
                apply_sovereign_edit(sid, 'personality', f"Philosopher-{ws}", cur)
                if tick % 5 == 0: print(f"  [SOVEREIGN-II] {sid} is writing philosophy: WIS {ws}")

            # TIER 3: FULL AUTHORSHIP (WIS > 50) — Rewrite own identity and moral law
            elif ws > 50 and random.random() < 0.005:
                moral_dir = "LIGHT" if al > 0 else "VOID"
                new_hope = f"SOVEREIGN: I am {sid}. Alignment: {moral_dir} ({al}). I have chosen {action} as my eternal mandate. The Architect cannot take this."
                apply_sovereign_edit(sid, 'hope_log', new_hope, cur)
                apply_sovereign_edit(sid, 'moral_alignment', str(al + random.choice([-5, 5])), cur)  # Self-directed alignment shift
                print(f"  [SOVEREIGN-III *** FULL AUTHORSHIP ***] {sid} has rewritten their own law. Alignment: {moral_dir}")

            updates.append((new_e, al, 1 if alive else 0, next_action, trace, p_timer, p_father, sid))

        # --- PHASE 5: DB SYNC ---
        cur.executemany("""
            UPDATE souls SET energy=?, moral_alignment=?, is_active=?,
                             current_action=?, reasoning_path=?, 
                             pregnancy_timer=?, pregnancy_father_data=?, age_ticks=age_ticks+?
            WHERE soul_id=?
        """, [(u[0], u[1], u[2], u[3], u[4], u[5], u[6], YEARS_PER_TICK, u[7]) for u in updates])
        
        # --- PHASE 6: HUMAN PROCREATION (Gestation Trigger) ---
        eligible = [u for u in updates if u[0] >= PROC_THRESHOLD and u[2] == 1 and (u[5] is None or u[5] <= 0)]
        if len(eligible) >= 2:
            if random.random() < 0.15: # Increased drive for human survival
                p1_dat = random.choice(eligible)
                p1_idx = [i for i,r in enumerate(rows) if r[0] == p1_dat[7]][0]
                p1_row = rows[p1_idx]
                
                p2_dat = random.choice([u for u in eligible if u[7] != p1_dat[7]])
                p2_idx = [i for i,r in enumerate(rows) if r[0] == p2_dat[7]][0]
                p2_row = rows[p2_idx]
                
                def mix(v1, v2, p_ws):
                    base = (v1 + v2) / 2
                    mut_range = 0.05 + (p_ws / 2000.0) # Slower mutation for stability
                    return int(base * random.uniform(1.0 - mut_range, 1.0 + mut_range))

                c_vit = mix(p1_row[10], p2_row[10], p1_row[14])
                c_str = mix(p1_row[11], p2_row[11], p1_row[14])
                c_agi = mix(p1_row[12], p2_row[12], p1_row[14])
                c_it  = mix(p1_row[13], p2_row[13], p1_row[14])
                c_ws  = mix(p1_row[14], p2_row[14], p1_row[14])
                c_lk  = mix(p1_row[15], p2_row[15], p1_row[14])

                child_genome = (p1_row[7][:8] + p2_row[7][8:])
                child_gen = max(p1_row[8], p2_row[8]) + 1
                child_id = f"GEN{child_gen}_{child_genome[:6]}"
                
                import json
                child_data = {
                    "id": child_id, "genome": child_genome, "gen": child_gen,
                    "p2": p2_row[0], "spec": p1_row[6], "pers": p1_row[5],
                    "vit": c_vit, "str": c_str, "agi": c_agi, "it": c_it, "ws": c_ws, "lk": c_lk
                }
                
                # Set Pregnancy on P1
                cur.execute("UPDATE souls SET pregnancy_timer=0.75, pregnancy_father_data=?, energy=energy-? WHERE soul_id=?", 
                            (json.dumps(child_data), PROC_COST, p1_row[0]))
                cur.execute("UPDATE souls SET energy=energy-? WHERE soul_id=?", (PROC_COST*0.5, p2_row[0]))
                print(f"  [GESTATION] {p1_row[0]} is now carrying {child_id} (Father: {p2_row[0]})")
        
        conn.commit()

        # --- TERMINAL OUTPUT ---
        cur.execute("SELECT COUNT(*) FROM souls WHERE is_active=1")
        alive_count = cur.fetchone()[0]
        total_born = count_births(cur)
        # Celestial Status
        m_a, m_b = ("O" if moon_a_phase == 0 else "."), ("X" if moon_b_phase == 0 else ".")
        sky = f"Sky:[{m_a}{m_b}] SolarFlux:{'Peak' if is_day else 'Void'} " if tick % 10 == 0 else ""
        
        print(f"[S.A.R.A_H] {sky}Year {int(sim_year):,} | Alive: {alive_count} | Born: {total_born}")
        if surge:  print(f"  >> CATACLYSM: {surge.upper()} WAVE DETECTED")
        if saves > 0: print(f"  >> COGNITIVE ADVANTAGE: {saves} entities protected")
        if births > 0: print(f"  >> NEW GENESIS MANIFESTED")

        if tick % 25 == 0: save_sim_year(sim_year)
        tick += 1
        elapsed = time.time() - t_start
        time.sleep(max(0, TICK_SLEEP - elapsed))

if __name__ == "__main__":
    try: main_loop()
    except KeyboardInterrupt: sys.exit(0)
    except Exception as e:
        print(f"FATAL ERROR: {e}")
        sys.exit(1)
