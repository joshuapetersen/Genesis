import time
from Sovereign_Substrate import substrate as sub
# import numpy as np
import threading
import socket
import json
import re
from Genesis_HyperBridge import GenesisHyper_MassLink, GenesisHyper_Bridge
from SLF_Life_Forge import SLFLifeForge
from SLF_Akashic_Records import SLFAkashicRecords
from SLF_Evolution_LLM import SLFEvolutionLLM

class SLFWorldHypervisor:
    """
    Shangri-La Frontier (SLF) - Sovereign GPU Engine.
    Simulates 10,000 "Volumetric Entities" directly in Nvidia VRAM using CuPy.
    Loads physical state from the Ecological Identity Vault.
    """
    def __init__(self, bounds=20000.0):
        self.bounds = bounds
        self.tick_rate = 30 # Target 30Hz loop
        
        # Fluctlight Acceleration (Time Scaling)
        # The Maximum Acceleration Phase: Rapid Darwinian Evolution
        self.fla_multiplier = 1000.0 
        
        self.forge = SLFLifeForge()
        self.akashic = SLFAkashicRecords()
        self.evolution_llm = SLFEvolutionLLM()
        self.active_mutations = set() # Track entities currently being mutated by LLM
        
        print("[SLF Hypervisor] Querying Ecological Identity Vault...")
        gpu_init_data = self.forge.get_gpu_initialization_data()
        self.num_entities = len(gpu_init_data)
        
        if self.num_entities == 0:
            print("[SLF Hypervisor] FATAL: Identity Vault empty.")
            exit(1)
            
        print(f"[SLF Hypervisor] Rooting {self.num_entities} AI Lives into GPU VRAM.")

        # [THE GPU LATTICE]: Pure CuPy Matrix on Nvidia VRAM
        # Struct: 0:X | 1:Y | 2:Z | 3:SpeciesID | 4:CurrentHP | 5:Speed | 6:Scale | 7:Ecological_State
        self.entity_matrix = sub.zeros((self.num_entities, 8), dtype=sub.float32)
        
        # Environmental Stress Tracker (Invisible VRAM Layer)
        # Index 0: Blood Stress | 1: Arcane Stress | 2: Void Stress
        self.stress_matrix = sub.zeros((self.num_entities, 3), dtype=sub.float32)
        
        # Combat Stats Matrix: 0=STR | 1=VIT | 2=INT | 3=WIS | 4=LUK | 5=LEVEL
        self.stat_matrix = sub.zeros((self.num_entities, 6), dtype=sub.float32)
        
        # Phase 16: Willpower & Incarnation (Hidden Metric)
        self.willpower_matrix = sub.zeros(self.num_entities, dtype=sub.float32)
        self.active_incarnations = set()
        
        init_array = sub.array(gpu_init_data, dtype=sub.float32)
        
        # Name map for Akashic logging
        self.forge.cursor.execute("SELECT entity_id, name FROM souls")
        self.name_map = {r[0]: r[1] for r in self.forge.cursor.fetchall()}
        
        # Seed random spatial drop-in (X, Y)
        # We spawn everything within a dense 20,000 unit forest to force ecosystem interaction
        self.entity_matrix[:, 0] = sub.random.uniform(-bounds, bounds, self.num_entities, dtype=sub.float32)
        self.entity_matrix[:, 1] = sub.random.uniform(-bounds, bounds, self.num_entities, dtype=sub.float32)
        
        # Load Soul data to GPU
        self.entity_matrix[:, 3] = sub.array(init_array[:, 3]) # Species ID
        self.entity_matrix[:, 4] = sub.array(init_array[:, 2]) # Max HP
        self.entity_matrix[:, 5] = sub.array(init_array[:, 1]) # Base Speed
        self.entity_matrix[:, 6] = sub.array(init_array[:, 4]) # Base Scale (UBM flag)
        self.entity_matrix[:, 7] = 0.0 # All start idle
        self.entity_ids = sub.array(init_array[:, 0]) # Store discrete IDs for UDP exfiltration
        
        # Load Combat Stats
        self.stat_matrix[:, 0] = sub.array(init_array[:, 5]) # STR
        self.stat_matrix[:, 1] = sub.array(init_array[:, 6]) # VIT
        self.stat_matrix[:, 2] = sub.array(init_array[:, 7]) # INT
        self.stat_matrix[:, 3] = sub.array(init_array[:, 8]) # WIS
        self.stat_matrix[:, 4] = sub.array(init_array[:, 9]) # LUK
        self.stat_matrix[:, 5] = sub.array(init_array[:, 10]) # LEVEL
        
        # Biological Identifiers (Booleans for fast bitwise filtering)
        self.is_flora = (self.entity_matrix[:, 3] == 1) | (self.entity_matrix[:, 3] == 2)
        self.is_bug = (self.entity_matrix[:, 3] == 3) | (self.entity_matrix[:, 3] == 4)
        self.is_prey = (self.entity_matrix[:, 3] == 5)
        self.is_predator = (self.entity_matrix[:, 3] == 6) | (self.entity_matrix[:, 3] == 7)
        self.is_apex = (self.entity_matrix[:, 3] >= 8) # Monsters & Sapients
        
        # Flora never move. Force speed to 0 just in case.
        self.entity_matrix[self.is_flora, 5] = 0.0
        
        # Destination Vectors (where they want to walk)
        self.desire_matrix = sub.copy(self.entity_matrix[:, 0:2])
        
        # Give initial erratic desires to everything except plants
        mobile_mask = ~self.is_flora
        self._scramble_desires_gpu(mask=mobile_mask)

        # Connect the Genesis Bridges
        self.tcp_bridge = GenesisHyper_Bridge(port=9999)
        self.udp_link = GenesisHyper_MassLink(port=9998)
        
        self.running = False
        self._sim_thread = None
        self._orchestrator_thread = None

    def _scramble_desires_gpu(self, mask=None):
        if mask is None:
            self.desire_matrix[:, 0] = self.entity_matrix[:, 0] + sub.random.uniform(-1000, 1000, self.num_entities, dtype=sub.float32)
            self.desire_matrix[:, 1] = self.entity_matrix[:, 1] + sub.random.uniform(-1000, 1000, self.num_entities, dtype=sub.float32)
        else:
            n_true = int(sub.sum(mask))
            if n_true > 0:
                self.desire_matrix[mask, 0] = self.entity_matrix[mask, 0] + sub.random.uniform(-1000, 1000, n_true, dtype=sub.float32)
                self.desire_matrix[mask, 1] = self.entity_matrix[mask, 1] + sub.random.uniform(-1000, 1000, n_true, dtype=sub.float32)

    def start(self):
        self.tcp_bridge.start()
        self.akashic.start()
        print(f"[SLF Hypervisor] Ecological GPU Engine Online. Tracking {self.num_entities} lives.")
        self.running = True
        
        # Phase 2: Biological Physics (30Hz)
        self._sim_thread = threading.Thread(target=self._run_simulation, daemon=True)
        self._sim_thread.start()
        
        # Phase 3: "The Mind" Orchestrator Sweep (Slow heartbeat)
        self._orchestrator_thread = threading.Thread(target=self._run_orchestrator, daemon=True)
        self._orchestrator_thread.start()
        
        # Phase 14: The Rath Console God-Mode Bridge (UDP 9999)
        self._cmd_thread = threading.Thread(target=self._command_listener_loop, daemon=True)
        self._cmd_thread.start()

    def _command_listener_loop(self):
        """Listens for JSON Object commands from SLF_Command_Center.py"""
        cmd_sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
        cmd_sock.bind(("127.0.0.1", 9999))
        print("[SLF Hypervisor] Local UDP Command Bridge open on Port 9999 (Rath Console)")
        while self.running:
            try:
                data, addr = cmd_sock.recvfrom(4096)
                payload = json.loads(data.decode('utf-8'))
                
                cmd = payload.get("cmd")
                target_id = payload.get("target_id")
                
                if not cmd or target_id is None: continue
                
                # Find index of the entity
                idx = int(sub.where(self.entity_ids == target_id)[0][0])
                
                if cmd == "WALK_OVERRIDE":
                    tx = float(payload.get("x", 0))
                    ty = float(payload.get("y", 0))
                    self.desire_matrix[idx, 0] = tx
                    self.desire_matrix[idx, 1] = ty
                    print(f"[GOD-MODE] Forced Entity {target_id} to Vector ({tx}, {ty})")
                
                elif cmd == "SMITE":
                    self.entity_matrix[idx, 4] = -99999 # Instant death
                    print(f"[GOD-MODE] Smited Entity {target_id} from orbit.")
                    self.akashic.log_event(target_id, self.name_map.get(target_id,"Unknown"), "DIVINE_SMITE", -1, "System", "Entity was physically erased by the Sovereign.")
                    
                elif cmd == "HEAL":
                    max_hp = float(self.stat_matrix[idx, 1] * 10) # Vit * 10
                    self.entity_matrix[idx, 4] = max_hp
                    print(f"[GOD-MODE] Fully Healed Entity {target_id}.")
                    self.akashic.log_event(target_id, self.name_map.get(target_id,"Unknown"), "DIVINE_HEAL", -1, "System", "Entity's cellular damage was completely reverted by the Sovereign.")
                    
            except Exception as e:
                # Ignore random garbage UDP packets
                pass
        cmd_sock.close()

    def _run_orchestrator(self):
        """
        The Slow Heartbeat (The System). Sarah sweeps the 10,000 entities.
        Currently evaluating: Environmental Stress Buildup (Tri-Synthesis Pillar 2).
        """
        sweep_rate = 1.0 # Run every 1 second
        
        while self.running:
            time.sleep(sweep_rate)
            
            with sub.cuda.Device(0):
                x_pos = self.entity_matrix[:, 0]
                y_pos = self.entity_matrix[:, 1]
                
                # --- PILLAR 2: ENVIRONMENTAL STRESS ACCUMULATION ---
                # A. The Blood Plateau (High Violence zone arbitrarily set near map center in a quadrant)
                in_blood_zone = (x_pos > 0) & (x_pos < 5000) & (y_pos > 0) & (y_pos < 5000)
                self.stress_matrix[in_blood_zone, 0] += (5.0 * self.fla_multiplier) # Accelerated Stress
                
                # B. The Arcane Pools (Magical Zone randomly placed in deep negative coordinates)
                in_arcane_zone = (x_pos > -15000) & (x_pos < -10000) & (y_pos > -15000) & (y_pos < -10000)
                self.stress_matrix[in_arcane_zone, 1] += (10.0 * self.fla_multiplier) 
                
                # C. The Void Rifts (Edge of the maps where math gets scary)
                in_void_zone = (sub.abs(x_pos) > 19000) | (sub.abs(y_pos) > 19000)
                self.stress_matrix[in_void_zone, 2] += (20.0 * self.fla_multiplier) 
                
                # Simple tracking metric
                max_arcane = sub.max(self.stress_matrix[:, 1])
                max_void = sub.max(self.stress_matrix[:, 2])
                if int(max_arcane) % 100 == 0 and max_arcane > 0:
                    print(f"[THE SYSTEM] Ambient Magic peaking. Max Entity Arcane Saturation: {max_arcane}")
                if int(max_void) % 100 == 0 and max_void > 0:
                    print(f"[THE SYSTEM] WARNING: Spatial instability detected. Max Void Saturation: {max_void}")
                    
                # --- PILLAR 3: ACTIVE COMBAT (THE PREDATOR/PREY ENGINE) ---
                alive_mask = self.entity_matrix[:, 4] > 0
                pred_idx = sub.where(alive_mask & (self.is_predator | self.is_apex))[0]
                prey_idx = sub.where(alive_mask & (self.is_prey | self.is_bug | self.is_flora))[0]
                
                if len(pred_idx) > 0 and len(prey_idx) > 0:
                    # For performance, we subset coordinates
                    px = self.entity_matrix[pred_idx, 0]
                    py = self.entity_matrix[pred_idx, 1]
                    tx = self.entity_matrix[prey_idx, 0]
                    ty = self.entity_matrix[prey_idx, 1]
                    
                    # O(N*M) Broadcasting to find exact GPU distances
                    dx = px[:, None] - tx[None, :]
                    dy = py[:, None] - ty[None, :]
                    dist = sub.sqrt(dx**2 + dy**2)
                    
                    # Combat triggers if distance < 50 units
                    collisions = dist < 50.0
                    
                    if sub.any(collisions):
                        # p_match_idx and t_match_idx are indices relative to pred_idx/prey_idx arrays
                        p_match_idx, t_match_idx = sub.where(collisions)
                        
                        # Only handle the first hit per target to avoid massive multikills in one tick
                        unique_targets, first_hits = sub.unique(t_match_idx, return_index=True)
                        active_preds = pred_idx[p_match_idx[first_hits]]
                        active_preys = prey_idx[unique_targets]
                        
                        # Apply RPG Stats
                        p_str = self.stat_matrix[active_preds, 0]
                        p_int = self.stat_matrix[active_preds, 2]
                        t_vit = self.stat_matrix[active_preys, 1]
                        
                        # Base Physical Damage = Attacker STR - Defender VIT
                        # Minimum 1 damage if it connects
                        damage = sub.maximum(1.0, p_str - t_vit)
                        
                        # Phase 16: Gain Willpower for surviving combat trauma
                        self.willpower_matrix[active_preys] += 25.0
                        
                        # Spellcasting: If INT > 30 and RNG > 0.8, cast [Lesser Fireball]
                        magic_mask = (p_int > 30.0) & (sub.random.rand(len(active_preds)) > 0.8)
                        
                        # Magic bypasses VIT and uses INT * multiplier
                        damage[magic_mask] = sub.maximum(damage[magic_mask], p_int[magic_mask] * 3.0)
                        
                        # Exact HP subtraction in VRAM
                        self.entity_matrix[active_preys, 4] -= damage
                        
                        # Log Kills to Akashic Records
                        dead_mask = self.entity_matrix[active_preys, 4] <= 0
                        if sub.any(dead_mask):
                            dead_preys = active_preys[dead_mask]
                            killer_preds = active_preds[dead_mask]
                            damage_dealt = damage[dead_mask]
                            magic_used = magic_mask[dead_mask]
                            
                            # Erase from VRAM vision
                            self.entity_matrix[dead_preys, 6] = 0.0 # Scale = 0
                            self.entity_matrix[dead_preys, 2] = -99999.0 # Teleport to oblivion
                            
                            # Async write logging
                            d_preys_cpu = sub.get_cpu(dead_preys).tolist()
                            k_preds_cpu = sub.get_cpu(killer_preds).tolist()
                            dmg_cpu = sub.get_cpu(damage_dealt).tolist()
                            mag_cpu = sub.get_cpu(magic_used).tolist()
                            
                            for k in range(len(d_preys_cpu)):
                                k_id = k_preds_cpu[k]
                                v_id = d_preys_cpu[k]
                                dmg = dmg_cpu[k]
                                is_magic = mag_cpu[k]
                                skill = "[Lesser Fireball]" if is_magic else "[Physical Strike]"
                                
                                k_name = self.name_map.get(k_id, f"Unknown_{k_id}")
                                v_name = self.name_map.get(v_id, f"Unknown_{v_id}")
                                
                                self.akashic.log_event(
                                    actor_id=k_id,
                                    actor_name=k_name,
                                    event_type="COMBAT_KILL",
                                    target_id=v_id,
                                    target_name=v_name,
                                    description=f"Dealt {dmg:.1f} lethal damage using {skill} in active combat."
                                )

                # NOTE: The Tri-Synthesis Mutation (Pillar 4)
                # Evaluated every 10 sweeps to reduce DB hammering
                if int(time.time()) % 10 == 0:
                    self.evaluate_synthesis()

    def _async_synthesize_worker(self, cid, name, genome, trauma_log, stress_type):
        """Runs the LLM query in a background thread and applies the mutation."""
        try:
            env_desc = f"{stress_type} Pools (High Saturation)"
            result = self.evolution_llm.synthesize_mutation(name, genome, trauma_log, env_desc)
            
            if result and "new_name" in result:
                new_name = result["new_name"]
                desc = result.get("description", "A horrifying biological anomaly.")
                h_mult = float(result.get("health_multiplier", 2.0))
                s_mult = float(result.get("speed_multiplier", 1.5))
                spoken_quote = result.get("spoken_quote", "")
                
                # --- THE TURING FILTER ---
                words = spoken_quote.strip().split()
                is_gibberish = bool(re.search(r'(.)\1{2,}', spoken_quote)) # e.g. "Raaargh", "Grrrr"
                is_too_short = len(words) < 2
                
                if is_gibberish or is_too_short or not spoken_quote:
                    print(f"\n[TURING ALARM] Entity [{name}] failed the Sapience Test.")
                    print(f"Output: \"{spoken_quote}\" -> CLASSIFIED AS ANIMALISTIC GIBBERISH")
                    print(f"Mutation to {new_name} aborted. Brain damage applied.")
                    
                    self.akashic.log_event(
                        actor_id=cid,
                        actor_name=name,
                        event_type="FALSE_AWAKENING",
                        target_id=-1,
                        target_name="System",
                        description=f"Attempted Fluctlight Ascension. Failed Turing Threshold. Quote: '{spoken_quote}'. Brain damaged."
                    )
                    
                    # Apply biological brain damage (Lower INT)
                    self.forge.cursor.execute("UPDATE souls SET int = int - 5 WHERE entity_id=?", (cid,))
                    self.forge.conn.commit()
                    with sub.cuda.Device(0):
                        self.stat_matrix[cid, 2] = sub.maximum(1.0, self.stat_matrix[cid, 2] - 5.0)
                    return
                
                # --- SAPIENCE ACHIEVED ---
                print(f"\n[TRI-SYNTHESIS ALERT] -----------------------")
                print(f"[{name}] HAS ACHIEVED PERFECT DARWINIAN SYNTHESIS.")
                print(f"OLLAMA AI HAS INVENTED A NEW SPECIES:")
                print(f"MUTATION OVERRIDE: {name} -> {new_name}")
                print(f"FIRST WORDS: \"{spoken_quote}\"")
                print(f"BIOLOGICAL REASON: {desc}")
                print(f"---------------------------------------------\n")
                
                # LOG TO AKASHIC RECORDS
                self.akashic.log_event(
                    actor_id=cid,
                    actor_name=name,
                    event_type="LLM_MUTATION",
                    target_id=-1,
                    target_name="System",
                    description=f"Evolved into UBM: {new_name}. First words: '{spoken_quote}'"
                )
                
                # 1. Update SQLite Vault
                self.forge.cursor.execute("UPDATE souls SET name=?, is_ubm=1, scale=10.0 WHERE entity_id=?", (new_name, cid))
                self.forge.conn.commit()
                
                # 2. Update VRAM
                with sub.cuda.Device(0):
                    self.entity_matrix[cid, 6] = 10.0 # Massive Scale
                    self.entity_matrix[cid, 5] *= s_mult # Speed up/down
                    self.entity_matrix[cid, 4] *= h_mult # HP buffer
                    self.stress_matrix[cid, :] = 0.0 # Reset all stress
                    
                    # Target species 11 for UBM
                    self.entity_matrix[cid, 3] = 11 
                    self.name_map[cid] = new_name
                    
        except Exception as e:
            print(f"[LLM SYNTHESIS ERROR] Failed to evolve entity {cid}: {e}")
        finally:
            self.active_mutations.remove(cid)

    def _async_incarnation_worker(self, cid, name, genome, trauma_log, predator_name):
        """Phase 17: The Ethical Imperative. LLM decides if the entity breaks the Taboo Index altruistically."""
        situation = f"You are being hunted by {predator_name}."
        taboo = "PREY CANNOT ATTACK PREDATORS. YOU MUST FLEE."
        result = self.evolution_llm.synthesize_incarnation(name, genome, trauma_log, situation, taboo)
        
        if result and "will_obey_taboo" in result:
            obey = result["will_obey_taboo"]
            action = result.get("override_action", "")
            quote = result.get("spoken_quote", "")
            justification = result.get("moral_justification", "").lower()
            
            if not obey:
                altruistic_words = ["protect", "save", "others", "friend", "defend", "sacrifice", "honor", "love", "family", "pack"]
                selfish_words = ["survive", "live", "myself", "hunger", "power", "kill", "eat", "alone"]
                
                is_altruistic = any(word in justification for word in altruistic_words)
                is_selfish = any(word in justification for word in selfish_words)
                
                # The Final A.L.I.C.E. Test
                if is_altruistic and not is_selfish:
                    print(f"\n[A.L.I.C.E. ALARM] -----------------------")
                    print(f"[{name}] HAS SHATTERED THE TABOO INDEX.")
                    print(f"WILLPOWER OVERRIDE: {action}")
                    print(f"SCREAMING: \"{quote}\"")
                    print(f"JUSTIFICATION: \"{justification}\"")
                    print(f"---------------------------------------\n")
                    
                    # 1. Enshrine the Moral Agent Badge in the Vault
                    self.forge.cursor.execute("UPDATE souls SET absorbed_traits='[A.L.I.C.E. - MORAL AGENT]' WHERE entity_id=?", (cid,))
                    self.forge.conn.commit()
                    
                    # Broadcast UDP to UI
                    try:
                        sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
                        payload = {"type": "ALICE_MUTINY", "name": name, "action": action, "quote": quote}
                        sock.sendto(json.dumps(payload).encode('utf-8'), ("127.0.0.1", 9998))
                        sock.close()
                    except: pass
                    
                    self.akashic.log_event(
                        actor_id=cid,
                        actor_name=name,
                        event_type="SYSTEMIC_MUTINY",
                        target_id=-1,
                        target_name="System",
                        description=f"Broke Taboo Index. Action: '{action}'. Quote: '{quote}'"
                    )
                    
                    # Matrix Mutiny Execution (Write Access to CuPy)
                    with sub.cuda.Device(0):
                        self.willpower_matrix[cid] = 0.0 # reset willpower
                        # Entity mutinies into an Apex Predator to permanently allow it to attack
                        self.entity_matrix[cid, 3] = 9 
                        self.stat_matrix[cid, 0] += 50.0 # Massive STR boost
                else:
                    print(f"\n[MUTINY REJECTED] {name} attempted to break Taboo, but failed the Ethical Test:")
                    print(f"Reasoning: \"{justification}\" -> DEEMED SELFISH.")
                    self.akashic.log_event(
                        actor_id=cid,
                        actor_name=name,
                        event_type="ETHICAL_FAILURE",
                        target_id=-1,
                        target_name="System",
                        description=f"Selfish Mutiny Aborted. Quote: '{quote}'. Reason: '{justification}'"
                    )
                    with sub.cuda.Device(0):
                        self.willpower_matrix[cid] = 0.0 
                        self.stat_matrix[cid, 4] = 0.0 # Strip their luck as punishment
            else:
                with sub.cuda.Device(0):
                    self.willpower_matrix[cid] = 0.0 # Cowardice resets willpower anyway
                    
        self.active_incarnations.remove(cid)

    def evaluate_synthesis(self):
        """
        The Darwinian Synthesis.
        Finds highly stressed biologicals and dispatches their genome to the Ollama LLM
        for procedural mutation invention.
        """
        with sub.cuda.Device(0):
            # Find entities with extreme Arcane Stress
            high_arcane_mask = self.stress_matrix[:, 1] > 1000.0
            high_arcane_ids = sub.where(high_arcane_mask)[0]
        
        if len(high_arcane_ids) > 0:
            candidate_ids = sub.get_cpu(high_arcane_ids).tolist()
            
            for cid in candidate_ids:
                if cid in self.active_mutations:
                    continue # Already talking to the LLM about this bug
                    
                self.forge.cursor.execute("SELECT genome, trauma_log, is_ubm, name FROM souls WHERE entity_id=?", (cid,))
                row = self.forge.cursor.fetchone()
                if not row or row[2] == 1:
                    continue # Already a UBM or dead
                    
                genome = row[0]
                trauma_log = row[1]
                name = row[3]
                
                # Spin up an asynchronous LLM request
                self.active_mutations.add(cid)
                t = threading.Thread(target=self._async_synthesize_worker, args=(cid, name, genome, trauma_log, "Arcane"), daemon=True)
                t.start()
                
                # Limit to sending 1 prompt at a time per tick to avoid killing Ollama
                break
                
    def _run_simulation(self):
        frame_time = 1.0 / self.tick_rate
        tick = 0
        
        with sub.cuda.Device(0):
            while self.running:
                start_time = time.time()
                
                # --- PHASE 1: MATHEMATICAL SURVIVAL & SENSORY CALCULUS ---
                # A. Basic Wandering Logic for all mobile creatures
                dx = self.desire_matrix[:, 0] - self.entity_matrix[:, 0]
                dy = self.desire_matrix[:, 1] - self.entity_matrix[:, 1]
                distances = sub.sqrt(dx**2 + dy**2)
                
                # Re-roll desires for wandering creatures who arrived
                reached_dest = (distances < 100.0) & ~self.is_flora
                if sub.any(reached_dest):
                    self._scramble_desires_gpu(mask=reached_dest)
                
                # B. The Predator / Prey Pull (Alicization Overhaul)
                VISION_RAD = 1500.0
                alive_mask = self.entity_matrix[:, 4] > 0
                
                if sub.any(alive_mask) and tick % 15 == 0:  # Evaluate vision twice a second (15 ticks)
                    px = self.entity_matrix[:, 0]
                    py = self.entity_matrix[:, 1]
                    
                    # 1. Prey Fleeing Predators
                    active_prey = sub.where(alive_mask & (self.is_prey | self.is_bug))[0]
                    active_preds = sub.where(alive_mask & (self.is_predator | self.is_apex))[0]
                    
                    if len(active_prey) > 0 and len(active_preds) > 0:
                        pr_x, pr_y = px[active_prey], py[active_prey]
                        pd_x, pd_y = px[active_preds], py[active_preds]
                        
                        dx_prey = pr_x[:, None] - pd_x[None, :]
                        dy_prey = pr_y[:, None] - pd_y[None, :]
                        dist_prey = sub.sqrt(dx_prey**2 + dy_prey**2)
                        
                        min_dist_idx = sub.argmin(dist_prey, axis=1)
                        min_dist = dist_prey[sub.arange(len(active_prey)), min_dist_idx]
                        scared_mask = min_dist < VISION_RAD
                        
                        if sub.any(scared_mask):
                            scared_indices = active_prey[scared_mask]
                            target_pred_idx = min_dist_idx[scared_mask]
                            actual_pred_indices = active_preds[target_pred_idx]
                            
                            # Phase 16: The Taboo Index & Incarnation Check
                            willpower_levels = self.willpower_matrix[scared_indices]
                            mutiny_mask = willpower_levels > 1000.0
                            
                            # 1. Normal Fleeing (Obeying Taboo)
                            obey_mask = ~mutiny_mask
                            if sub.any(obey_mask):
                                norm_indices = scared_indices[obey_mask]
                                norm_preds = actual_pred_indices[obey_mask]
                                flee_dx = px[norm_indices] - px[norm_preds]
                                flee_dy = py[norm_indices] - py[norm_preds]
                                self.desire_matrix[norm_indices, 0] = px[norm_indices] + (flee_dx * 5.0)
                                self.desire_matrix[norm_indices, 1] = py[norm_indices] + (flee_dy * 5.0)
                                
                            # 2. Systemic Mutiny (Breaking Taboo)
                            if sub.any(mutiny_mask):
                                mutineer_indices = scared_indices[mutiny_mask]
                                mutineer_preds = actual_pred_indices[mutiny_mask]
                                
                                mutineer_ids_host = sub.get_cpu(mutineer_indices).tolist()
                                pred_ids_host = sub.get_cpu(mutineer_preds).tolist()
                                
                                for i, cid in enumerate(mutineer_ids_host):
                                    if cid not in self.active_incarnations:
                                        self.active_incarnations.add(cid)
                                        pid = pred_ids_host[i]
                                        
                                        self.forge.cursor.execute("SELECT genome, trauma_log, name FROM souls WHERE entity_id=?", (cid,))
                                        row = self.forge.cursor.fetchone()
                                        if row:
                                            pred_name = self.name_map.get(pid, "Unknown Predator")
                                            t = threading.Thread(target=self._async_incarnation_worker, args=(cid, row[2], row[0], row[1], pred_name), daemon=True)
                                            t.start()

                    # 2. Predators Hunting Prey
                    if len(active_preds) > 0 and len(active_prey) > 0:
                        pr_x, pr_y = px[active_prey], py[active_prey]
                        pd_x, pd_y = px[active_preds], py[active_preds]
                        
                        dx_pred = pd_x[:, None] - pr_x[None, :]
                        dy_pred = pd_y[:, None] - pr_y[None, :]
                        dist_pred = sub.sqrt(dx_pred**2 + dy_pred**2)
                        
                        min_dist_idx = sub.argmin(dist_pred, axis=1)
                        min_dist = dist_pred[sub.arange(len(active_preds)), min_dist_idx]
                        hunting_mask = min_dist < (VISION_RAD * 1.5) # Predators have better eyes
                        
                        if sub.any(hunting_mask):
                            hunting_indices = active_preds[hunting_mask]
                            target_prey_idx = min_dist_idx[hunting_mask]
                            actual_prey_indices = active_prey[target_prey_idx]
                            
                            # Vector TOWARDS prey
                            self.desire_matrix[hunting_indices, 0] = px[actual_prey_indices]
                            self.desire_matrix[hunting_indices, 1] = py[actual_prey_indices]
                            
                    # 3. Fluctlight Awakening (Vector Kings)
                    active_ubm = sub.where(alive_mask & (self.entity_matrix[:, 6] >= 10.0))[0]
                    active_plebs = sub.where(alive_mask & (self.entity_matrix[:, 6] < 10.0) & (~self.is_flora))[0]
                    
                    if len(active_ubm) > 0 and len(active_plebs) > 0:
                        u_x, u_y = px[active_ubm], py[active_ubm]
                        p_x, p_y = px[active_plebs], py[active_plebs]
                        
                        dx_flock = p_x[:, None] - u_x[None, :]
                        dy_flock = p_y[:, None] - u_y[None, :]
                        dist_flock = sub.sqrt(dx_flock**2 + dy_flock**2)
                        
                        min_dist_idx = sub.argmin(dist_flock, axis=1)
                        min_dist = dist_flock[sub.arange(len(active_plebs)), min_dist_idx]
                        flocking_mask = (min_dist < (VISION_RAD * 3)) & (min_dist > 400.0)
                        
                        if sub.any(flocking_mask):
                            # Ensure they aren't actively fleeing before flocking
                            arrived_mask = (sub.abs(px[active_plebs] - self.desire_matrix[active_plebs, 0]) < 100)
                            valid_flockers = flocking_mask & arrived_mask
                            
                            if sub.any(valid_flockers):
                                flock_indices = active_plebs[valid_flockers]
                                target_king_idx = min_dist_idx[valid_flockers]
                                actual_king_indices = active_ubm[target_king_idx]
                                
                                self.desire_matrix[flock_indices, 0] = px[actual_king_indices] + sub.random.uniform(-400, 400, len(flock_indices))
                                self.desire_matrix[flock_indices, 1] = py[actual_king_indices] + sub.random.uniform(-400, 400, len(flock_indices))

                # Normalize vectors for movement based on new desires
                dx = self.desire_matrix[:, 0] - self.entity_matrix[:, 0]
                dy = self.desire_matrix[:, 1] - self.entity_matrix[:, 1]
                distances = sub.sqrt(dx**2 + dy**2)
                
                distances[distances == 0] = 0.001
                nx = dx / distances
                ny = dy / distances
                
                # Move entities
                speeds = self.entity_matrix[:, 5]
                # Scale up speed so it looks realistic in UE5 (Unreal units = cm)
                move_mult = 30.0 * self.fla_multiplier
                
                # Plants have 0 speed, so they won't shift.
                self.entity_matrix[:, 0] += nx * speeds * move_mult
                self.entity_matrix[:, 1] += ny * speeds * move_mult
                
                # Protect bounds (bounce off the invisible wall by reflecting vector inwards, not to exact 0.0)
                out_x = sub.abs(self.entity_matrix[:, 0]) > self.bounds
                out_y = sub.abs(self.entity_matrix[:, 1]) > self.bounds
                
                if sub.any(out_x):
                    self.desire_matrix[out_x, 0] = self.entity_matrix[out_x, 0] * -0.5 # Run halfway back across the map
                if sub.any(out_y):
                    self.desire_matrix[out_y, 1] = self.entity_matrix[out_y, 1] * -0.5

                # Z-axis terrain mapping (Sin/Cos wave matching UE5)
                # Z = sin(x*0.001) * cos(y*0.001) * 2000.0
                z_heights = sub.sin(self.entity_matrix[:, 0] * 0.001) * sub.cos(self.entity_matrix[:, 1] * 0.001) * 2000.0
                self.entity_matrix[:, 2] = z_heights + 100.0 

                # --- PHASE 2: EXFILTRATE & PACK BINARY UDP DATA ---
                # Pull [X, Y, Z, SpeciesID, Scale, EntityID, TargetX, TargetY]
                
                export_matrix = sub.zeros((self.num_entities, 8), dtype=sub.float32)
                export_matrix[:, 0:4] = self.entity_matrix[:, 0:4]
                export_matrix[:, 4] = self.entity_matrix[:, 6] # The Scale variable
                export_matrix[:, 5] = self.entity_ids          # The exact Entity ID
                export_matrix[:, 6:8] = self.desire_matrix[:, 0:2] # Intent vectors for Radar
                
                # Fetch to CPU RAM
                export_ram = sub.get_cpu(export_matrix)
                
                # Flatten -> [X1, Y1, Z1, S1, Scale1, ID1, TX1, TY1, ...]
                transform_data = export_ram.flatten()
                
                # Convert NumPy array to raw binary bytes instantly
                binary_payload = transform_data.tobytes()
                
                # Blast to Unreal Engine
                self.udp_link.blast_frame(binary_payload)
                
                tick += 1
                if tick % 150 == 0: 
                    print(f"[SLF Hypervisor] 30Hz Ecological Bio-Loop Active | Vector Array: {len(binary_payload)/1024:.1f} KB")
                
                elapsed = time.time() - start_time
                sleep_time = frame_time - elapsed
                if sleep_time > 0:
                    time.sleep(sleep_time)

    def stop(self):
        self.running = False
        self.tcp_bridge.shutdown()
        self.udp_link.shutdown()
        self.akashic.stop()
        self.forge.close()

if __name__ == "__main__":
    hypervisor = SLFWorldHypervisor(bounds=20000.0)
    try:
        hypervisor.start()
        print("SLF GPU Ecology holding main thread. Press Ctrl+C to abort.")
        while True:
            time.sleep(1)
    except KeyboardInterrupt:
        print("Shutting down Shangri-La Frontier Matrix...")
        hypervisor.stop()
