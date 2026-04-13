import sqlite3
import random
import os
import json
import time

class SLFLifeForge:
    """
    Shangri-La Frontier (SLF) - The Ecological Vault (The Soul).
    Generates 10,000 AI lives across the entire ecological spectrum:
    Plants, Bugs, Beasts, Monsters, and Sapients. Every single one is a living AI node.
    """
    def __init__(self, db_path="SLF_Identity_Vault.sqlite", total_souls=10000):
        self.db_path = db_path
        self.total_souls = total_souls
        self.conn = sqlite3.connect(self.db_path, check_same_thread=False)
        self.cursor = self.conn.cursor()
        
        self.cursor.execute("PRAGMA journal_mode=WAL;")
        self.cursor.execute("PRAGMA synchronous=NORMAL;")
        
        # 10 Core Species covering the entire ecosystem
        self.species_types = {
            1: {"name": "Flora_AncientOak", "base_hp": 500, "base_spd": 0.0, "type": "Plant"},
            2: {"name": "Flora_ManaFern", "base_hp": 20, "base_spd": 0.0, "type": "Plant"},
            3: {"name": "Insect_Scarab", "base_hp": 5, "base_spd": 1.0, "type": "Bug"},
            4: {"name": "Insect_GoliathBeetle", "base_hp": 30, "base_spd": 0.8, "type": "Bug"},
            5: {"name": "Prey_SilverStag", "base_hp": 80, "base_spd": 5.0, "type": "Beast"},
            6: {"name": "Predator_DireWolf", "base_hp": 150, "base_spd": 4.5, "type": "Beast"},
            7: {"name": "Avian_StormHawk", "base_hp": 40, "base_spd": 6.0, "type": "Bird"},
            8: {"name": "Monster_Goblin", "base_hp": 100, "base_spd": 2.5, "type": "Monster"},
            9: {"name": "Sapient_Human", "base_hp": 120, "base_spd": 2.0, "type": "Sapient"},
            10: {"name": "Sapient_Elf", "base_hp": 100, "base_spd": 2.2, "type": "Sapient"}
        }
        
        # Ecological distribution (Weights out of 100)
        # 40% Plants, 25% Bugs, 15% Beasts/Birds, 15% Monsters, 5% Sapients
        self.species_weights = [25, 15, 15, 10, 10, 5, 5, 10, 3, 2]
        
        self.ecosystem_roles = ["Producer", "Scavenger", "Prey", "Predator", "Apex", "Builder"]
        self.personality_traits = ["Aggressive", "Docile", "Territorial", "Nomadic", "Symbiotic", "Parasitic", "Curious"]

        self._initialize_schema()

    def _initialize_schema(self):
        self.cursor.executescript("""
            CREATE TABLE IF NOT EXISTS souls (
                entity_id INTEGER PRIMARY KEY,
                name TEXT,
                species_id INTEGER,
                role TEXT,
                level INTEGER,
                xp REAL,
                age INTEGER,
                hp_max REAL,
                hp_current REAL,
                mp_max REAL,
                mp_current REAL,
                vit INTEGER,
                str INTEGER,
                agi INTEGER,
                int INTEGER,
                wis INTEGER,
                luk INTEGER,
                hunger REAL,
                thirst REAL,
                growth_stage REAL,
                personality TEXT,
                genome TEXT,
                trauma_log TEXT,
                absorbed_traits TEXT,
                is_ubm INTEGER,
                scale REAL
            );
            
            CREATE INDEX IF NOT EXISTS idx_species ON souls(species_id);
            CREATE INDEX IF NOT EXISTS idx_role ON souls(role);
        """)
        self.conn.commit()

    def _generate_identifier(self, species_id, entity_id):
        base_name = self.species_types[species_id]["name"]
        if self.species_types[species_id]["type"] == "Sapient":
            first = ["Aer", "Bal", "Cor", "Daz", "El", "Fen", "Gor", "Hul", "Il", "Jor", "Kul", "Lor", "Mor"]
            last = ["a", "ar", "dor", "eth", "is", "la", "os", "ra", "th", "us"]
            return (random.choice(first) + random.choice(last)).title()
        elif self.species_types[species_id]["type"] == "Monster":
            return "Gruk" if random.random() > 0.5 else "Snaga" + f"_{entity_id}"
        else:
            return f"{base_name}_{entity_id}"

    def construct_world(self, wipe_existing=False):
        if wipe_existing:
            print("[Life Forge] Wiping existing Identity Vault...")
            self.cursor.execute("DELETE FROM souls")
            self.conn.commit()
            
        self.cursor.execute("SELECT COUNT(*) FROM souls")
        count = self.cursor.fetchone()[0]
        
        if count >= self.total_souls:
            print(f"[Life Forge] Vault is already full. {count} AI Lives ready.")
            return

        print(f"[Life Forge] Seeding {self.total_souls - count} new AI Lives into the Ecosystem...")
        start_time = time.time()
        
        batch = []
        for i in range(count, self.total_souls):
            species_id = random.choices(list(self.species_types.keys()), weights=self.species_weights)[0]
            base_stats = self.species_types[species_id]
            stype = base_stats["type"]
            
            age = random.randint(1, 100) if stype == "Plant" else random.randint(1, 15)
            
            # --- The 6 Pillars of RPG Stats ---
            vit, str_stat, agi, int_stat, wis, luk = 1, 1, 1, 1, 1, 1
            
            if stype == "Plant":
                role = "Producer"
                vit, str_stat, agi, int_stat, wis, luk = random.randint(50, 100), random.randint(1, 5), 0, random.randint(10, 50), random.randint(20, 60), random.randint(1, 10)
            elif stype == "Bug":
                role = "Scavenger"
                vit, str_stat, agi, int_stat, wis, luk = random.randint(1, 5), random.randint(1, 5), random.randint(20, 50), 1, 1, random.randint(1, 100)
            elif stype == "Beast":
                role = "Predator" if "DireWolf" in base_stats["name"] else "Prey"
                vit, str_stat, agi, int_stat, wis, luk = random.randint(20, 40), random.randint(15, 30), random.randint(20, 40), random.randint(2, 8), random.randint(5, 15), random.randint(5, 20)
            elif stype == "Bird":
                role = "Predator"
                vit, str_stat, agi, int_stat, wis, luk = random.randint(10, 20), random.randint(5, 10), random.randint(40, 60), random.randint(5, 10), random.randint(5, 20), random.randint(10, 30)
            else: # Sapient / Monster
                role = "Builder" if stype == "Sapient" else "Apex"
                vit, str_stat, agi, int_stat, wis, luk = random.randint(20, 50), random.randint(10, 30), random.randint(10, 30), random.randint(15, 40), random.randint(15, 40), random.randint(10, 50)
                
            hp_max = base_stats["base_hp"] + (vit * 5)
            mp_max = int_stat * 10.0
            
            personality = random.choice(self.personality_traits)
            if stype == "Plant": personality = "Docile"
                
            # --- The Tri-Synthesis Genetic Block ---
            # 64-bit Hexadecimal Genome (16 characters)
            genome = format(random.getrandbits(64), '016x')
            
            trauma_log = json.dumps([]) # Empty list of lifetime traumas
            absorbed_traits = json.dumps({}) # Dictionary of DNA harvested from eating
            
            batch.append((
                i, # entity_id
                self._generate_identifier(species_id, i), # name
                species_id,
                role,
                1, # level
                0.0, # xp
                age,
                hp_max,
                hp_max, # current hp
                mp_max,
                mp_max, # current mp
                vit,
                str_stat,
                agi,
                int_stat,
                wis,
                luk,
                0.0, # hunger
                0.0, # thirst
                0.0, # growth_stage
                personality,
                genome,
                trauma_log,
                absorbed_traits,
                0,   # is_ubm (False)
                1.0  # scale (Normal size)
            ))
            
            if len(batch) >= 1000:
                self.cursor.executemany(
                    "INSERT INTO souls VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)", 
                    batch
                )
                self.conn.commit()
                batch = []
                
        if batch:
            self.cursor.executemany("INSERT INTO souls VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)", batch)
            self.conn.commit()
            
        print(f"[Life Forge] Forged {self.total_souls} AI Lives in {time.time() - start_time:.2f} seconds.")

    def get_gpu_initialization_data(self):
        self.cursor.execute("SELECT entity_id, species_id, hp_max, is_ubm, str, vit, int, wis, luk, level FROM souls ORDER BY entity_id ASC")
        rows = self.cursor.fetchall()
        
        gpu_data = []
        for r in rows:
            speed = self.species_types[r[1]]["base_spd"]
            scale = 10.0 if r[3] == 1 else 1.0 # If UBM trigger is true, start at 10x size default.
            
            # Additional combat stats
            combat_stats = [
                r[4], # STR
                r[5], # VIT
                r[6], # INT
                r[7], # WIS
                r[8], # LUK
                r[9]  # LEVEL
            ]
            
            gpu_data.append([
                r[0], # Entity ID [0]
                speed, # Base Speed [1]
                r[2], # Max HP [2]
                r[1], # Species ID [3]
                scale # Current Scale [4]
            ] + combat_stats)
            
        return gpu_data

    def close(self):
        self.conn.close()

if __name__ == "__main__":
    print("Initiating Sovereign Ecological Forge...")
    forge = SLFLifeForge(total_souls=10000)
    forge.construct_world(wipe_existing=True)
    forge.close()
