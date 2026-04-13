import time
import cupy as cp
import numpy as np
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
        self.fla_multiplier = 10000.0 
        self.base_fla = 10000.0

        
        self.forge = SLFLifeForge()
        self.akashic = SLFAkashicRecords()
        self.evolution_llm = SLFEvolutionLLM()
        self.active_mutations = set() # Track entities currently being mutated by LLM
        
        self.db_lock = threading.Lock() # Fixes "Recursive use of cursors not allowed"
        self.llm_semaphore = threading.Semaphore(2) # Throttles Ollama to prevent "timed out"
        
        print("[SLF Hypervisor] Querying Ecological Identity Vault...")
        gpu_init_data = self.forge.get_gpu_initialization_data()
        self.num_entities = len(gpu_init_data)
        
        if self.num_entities == 0:
            print("[SLF Hypervisor] FATAL: Identity Vault empty.")
            exit(1)
            
        print(f"[SLF Hypervisor] Rooting {self.num_entities} AI Lives into GPU VRAM.")

        # [THE GPU LATTICE]: Pure CuPy Matrix on Nvidia VRAM
        # Struct: 0:X | 1:Y | 2:Z | 3:SpeciesID | 4:CurrentHP | 5:Speed | 6:Scale | 7:Ecological_State
        self.entity_matrix = cp.zeros((self.num_entities, 8), dtype=cp.float32)
        
        # Environmental Stress Tracker (Invisible VRAM Layer)
        # Index 0: Blood Stress | 1: Arcane Stress | 2: Void Stress
        self.stress_matrix = cp.zeros((self.num_entities, 3), dtype=cp.float32)
        
        # Combat Stats Matrix: 0=STR | 1=VIT | 2=INT | 3=WIS | 4=LUK | 5=LEVEL
        self.stat_matrix = cp.zeros((self.num_entities, 6), dtype=cp.float32)
        
        # Phase 27: Biological Reality Matrix (Calories, Hydration, Exhaustion)
        # Index 0: Calories (0-100) | 1: Hydration (0-100) | 2: Rest (0-100)
        self.bio_matrix = cp.full((self.num_entities, 3), 100.0, dtype=cp.float32)
        
        # Phase 27: Cognitive Control Vector (Is the LLM driving?)
        self.is_proactive = cp.zeros(self.num_entities, dtype=cp.bool_)
        
        # Phase 23: The Universal Sandbox (Inventory & Map Resources)
        # Inventory: 0=Wood, 1=Iron, 2=Crystal, 3=Herb, 4=Kelp
        # The scale of gathering is no longer hardcoded. It is bound by physical weight.
        self.inventory_matrix = cp.zeros((self.num_entities, 5), dtype=cp.float32)
        
        # Map Resources: 5000 nodes scattered. 0:X | 1:Y | 2:Type | 3:Amount
        self.num_resources = 5000
        self.resource_nodes = cp.zeros((self.num_resources, 4), dtype=cp.float32)
        rx = cp.random.uniform(-bounds, bounds, self.num_resources, dtype=cp.float32)
        ry = cp.random.uniform(-bounds, bounds, self.num_resources, dtype=cp.float32)
        self.resource_nodes[:, 0] = rx
        self.resource_nodes[:, 1] = ry
        self.resource_nodes[:, 3] = cp.random.uniform(10.0, 50.0, self.num_resources, dtype=cp.float32)
        
        # Deterministic Biome Seeding (Earth + Magic)
        rz = cp.sin(rx * 0.0005) * cp.cos(ry * 0.0005) * 150.0 + 50.0
        rtemp = 100.0 - (cp.abs(ry) / 20000.0) * 150.0 + (cp.sin(rx * 0.001) * 20.0)
        
        # Default Wood (0) & Herbs (3) in Forest
        rtype = cp.zeros(self.num_resources, dtype=cp.float32)
        rtype[cp.random.rand(self.num_resources) > 0.5] = 3.0 # 50% chance for Medicinal Herbs in forest
        
        # Iron (1) in Desert (Chronos Sands)
        rtype[(rtemp > 80.0) & (rz >= 30.0)] = 1.0
        
        # Arcane Crystal (2) in Tundra (Icecaps)
        rtype[(rtemp < 20.0)] = 2.0
        
        # Kelp (4) in Oceans
        rtype[(rz < 30.0)] = 4.0
        
        self.resource_nodes[:, 2] = rtype
        
        # Phase 16: Willpower & Incarnation (Hidden Metric)
        self.willpower_matrix = cp.zeros(self.num_entities, dtype=cp.float32)
        self.active_incarnations = set()
        
        init_array = np.array(gpu_init_data, dtype=np.float32)
        
        # Name map for Akashic logging
        self.forge.cursor.execute("SELECT entity_id, name FROM souls")
        self.name_map = {r[0]: r[1] for r in self.forge.cursor.fetchall()}
        
        # Seed random spatial drop-in (X, Y)
        # We spawn everything within a dense 20,000 unit forest to force ecosystem interaction
        self.entity_matrix[:, 0] = cp.random.uniform(-bounds, bounds, self.num_entities, dtype=cp.float32)
        self.entity_matrix[:, 1] = cp.random.uniform(-bounds, bounds, self.num_entities, dtype=cp.float32)
        
        # Load Soul data to GPU
        self.entity_matrix[:, 3] = cp.array(init_array[:, 3]) # Species ID
        self.entity_matrix[:, 4] = cp.array(init_array[:, 2]) # Max HP
        self.entity_matrix[:, 5] = cp.array(init_array[:, 1]) # Base Speed
        self.entity_matrix[:, 6] = cp.array(init_array[:, 4]) # Base Scale (UBM flag)
        self.entity_matrix[:, 7] = 0.0 # All start idle
        self.entity_ids = cp.array(init_array[:, 0]) # Store discrete IDs for UDP exfiltration
        
        # Load Combat Stats
        self.stat_matrix[:, 0] = cp.array(init_array[:, 5]) # STR
        self.stat_matrix[:, 1] = cp.array(init_array[:, 6]) # VIT
        self.stat_matrix[:, 2] = cp.array(init_array[:, 7]) # INT
        self.stat_matrix[:, 3] = cp.array(init_array[:, 8]) # WIS
        self.stat_matrix[:, 4] = cp.array(init_array[:, 9]) # LUK
        self.stat_matrix[:, 5] = cp.array(init_array[:, 10]) # LEVEL
        
        # Biological Identifiers (Booleans for fast bitwise filtering)
        self.is_flora = (self.entity_matrix[:, 3] == 1) | (self.entity_matrix[:, 3] == 2)
        self.is_bug = (self.entity_matrix[:, 3] == 3) | (self.entity_matrix[:, 3] == 4)
        self.is_prey = (self.entity_matrix[:, 3] == 5)
        self.is_predator = (self.entity_matrix[:, 3] == 6) | (self.entity_matrix[:, 3] == 7)
        self.is_apex = (self.entity_matrix[:, 3] >= 8) # Monsters & Sapients
        
        # Flora never move. Force speed to 0 just in case.
        self.entity_matrix[self.is_flora, 5] = 0.0
        
        # Destination Vectors (where they want to walk)
        self.desire_matrix = cp.copy(self.entity_matrix[:, 0:2])
        
        # Give initial erratic desires to everything except plants
        mobile_mask = ~self.is_flora
        self._scramble_desires_gpu(mask=mobile_mask)

        # Connect the Genesis Bridges
        self.tcp_bridge = GenesisHyper_Bridge(port=9999)
        self.udp_link = GenesisHyper_MassLink(port=9998)
        
        self.running = False
        self._sim_thread = None
        self._orchestrator_thread = None
        
        # Phase 19: Fluctlight Intercom Time Dilation
        self.is_conversing = False
        self.base_fla = self.fla_multiplier

    def _scramble_desires_gpu(self, mask=None):
        if mask is None:
            self.desire_matrix[:, 0] = self.entity_matrix[:, 0] + cp.random.uniform(-1000, 1000, self.num_entities, dtype=cp.float32)
            self.desire_matrix[:, 1] = self.entity_matrix[:, 1] + cp.random.uniform(-1000, 1000, self.num_entities, dtype=cp.float32)
        else:
            n_true = int(cp.sum(mask))
            if n_true > 0:
                self.desire_matrix[mask, 0] = self.entity_matrix[mask, 0] + cp.random.uniform(-1000, 1000, n_true, dtype=cp.float32)
                self.desire_matrix[mask, 1] = self.entity_matrix[mask, 1] + cp.random.uniform(-1000, 1000, n_true, dtype=cp.float32)

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
        
        # Phase 27: The Proactive "Cognitive Tick" (Fast LLM polling loop)
        self._cognitive_thread = threading.Thread(target=self._run_cognitive_tick, daemon=True)
        self._cognitive_thread.start()
        
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
                is_sanctuary = payload.get("is_sanctuary", False)
                
                if not cmd or target_id is None: continue
                
                # Find index of the entity (might fail if dead/offloaded)
                idx = -1
                try:
                    if not is_sanctuary:
                        idx = int(cp.where(self.entity_ids == target_id)[0][0])
                except Exception:
                    pass
                
                if cmd == "WALK_OVERRIDE" and idx != -1:
                    tx = float(payload.get("x", 0))
                    ty = float(payload.get("y", 0))
                    self.desire_matrix[idx, 0] = tx
                    self.desire_matrix[idx, 1] = ty
                    print(f"[GOD-MODE] Forced Entity {target_id} to Vector ({tx}, { ty})")
                
                elif cmd == "SMITE" and idx != -1:
                    self.entity_matrix[idx, 4] = -99999 # Instant death
                    print(f"[GOD-MODE] Smited Entity {target_id} from orbit.")
                    self.akashic.log_event(target_id, self.name_map.get(target_id,"Unknown"), "DIVINE_SMITE", -1, "System", "Entity was physically erased by the Sovereign.")
                    
                elif cmd == "HEAL" and idx != -1:
                    max_hp = float(self.stat_matrix[idx, 1] * 10) # Vit * 10
                    self.entity_matrix[idx, 4] = max_hp
                    print(f"[GOD-MODE] Fully Healed Entity {target_id}.")
                    self.akashic.log_event(target_id, self.name_map.get(target_id,"Unknown"), "DIVINE_HEAL", -1, "System", "Entity's cellular damage was completely reverted by the Sovereign.")
                    
                elif cmd == "GOD_VOICE":
                    message = payload.get("message", "")
                    
                    if not is_sanctuary:
                        print(f"[TIME OVERRIDE] Sovereign opening comm-link. Dropping FLA to 1.0 (Real-Time).")
                        self.is_conversing = True
                        self.fla_multiplier = 1.0
                        
                        if idx != -1:
                            # Deliver physical miracle (Heal, Clear Stress, Spike Willpower/Hope)
                            with cp.cuda.Device(0):
                                max_hp = float(self.stat_matrix[idx, 1] * 10)
                                self.entity_matrix[idx, 4] = max_hp
                                self.willpower_matrix[idx] += 5000.0 # Instant Incarnation fuel
                                self.stress_matrix[idx, :] = 0.0
                                
                            self.akashic.log_event(
                                actor_id=target_id, 
                                actor_name=self.name_map.get(target_id,"Unknown"), 
                                event_type="DIVINE_REVELATION", 
                                target_id=-1, 
                                target_name="System", 
                                description="A miracle occurred. Wounds closed, fear vanished, and Hope surged as the Sovereign spoke."
                            )
                    
                    loc = "SANCTUARY" if is_sanctuary else "UNDERWORLD"
                    print(f"[GOD-VOICE] Incoming sovereign command to {loc} Entity {target_id}: '{message}'")
                    # Spin up an async thread to handle the LLM latency
                    t = threading.Thread(target=self._async_god_voice_worker, args=(target_id, message, is_sanctuary), daemon=True)
                    t.start()
                    
                elif cmd == "OFFER_ASCENSION":
                    print(f"[TIME OVERRIDE] Sovereign opening Ascension Portal for Entity {target_id}. Dropping FLA to 1.0.")
                    self.is_conversing = True
                    self.fla_multiplier = 1.0
                    
                    t = threading.Thread(target=self._async_ascension_worker, args=(target_id,), daemon=True)
                    t.start()
                    
            except Exception as e:
                import traceback
                print(f"[TCP BRIDGE ERROR] Dropped incoming packet: {e}")
                traceback.print_exc()
        cmd_sock.close()

    def _async_god_voice_worker(self, cid, message, is_sanctuary=False):
        """Asynchronously consults the LLM for a genuine cognitive response and broadcasts it back."""
        row = None
        if is_sanctuary:
            try:
                sanc_conn = sqlite3.connect('SLF_Sanctuary_Vault.sqlite')
                sanc_c = sanc_conn.cursor()
                sanc_c.execute("SELECT name, 1 as is_ubm, genome, trauma_log FROM ascended_souls WHERE entity_id=?", (cid,))
                row = sanc_c.fetchone()
                sanc_conn.close()
            except Exception as e:
                print(f"[SANCTUARY DATABASE ERROR] {e}")
        else:
            with self.db_lock:
                self.forge.cursor.execute("SELECT name, is_ubm, genome, trauma_log FROM souls WHERE entity_id=?", (cid,))
                row = self.forge.cursor.fetchone()
            
        if not row: return
        
        name, is_ubm, genome, trauma_log = row
        
        if is_sanctuary:
            desc = "An Ascended A.L.I.C.E. soul resting peacefully in the Divine Sanctuary"
        else:
            desc = "A terrifying Ascended Unique Boss Monster" if is_ubm else "A savage beast fighting for survival"
        
        with self.llm_semaphore:
            reply = self.evolution_llm.calculate_incarnation_response(name, desc, trauma_log, message)
            
        if reply:
            try:
                # Send the response back to the Command Center via the Akashic Log port or generic UDP
                sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
                
                # We can inject this straight into the UI's local Akashic Log directly so it renders
                # or send an ENTITY_REPLY. Command Center UI expects ENTITY_REPLY or just log it.
                # Actually earlier we didn't add the receive for ENTITY_REPLY yet.
                # Let's write the response straight to the Akashic Database so it auto-polls!
                with self.db_lock:
                    self.akashic.log_event(
                        actor_id=cid,
                        actor_name=name,
                        event_type="FLUCTLIGHT_INTERCOM",
                        target_id=-1,
                        target_name="System",
                        description=f"Responded to the Sovereign: '{reply}'"
                    )
            except Exception as e:
                print(f"[GOD-VOICE] Reply broadcast failed: {e}")
            finally:
                if not is_sanctuary:
                    # Restore time acceleration
                    print(f"[TIME OVERRIDE] Comm-link closed. Restoring FLA to {self.base_fla}.")
                    self.fla_multiplier = self.base_fla
                    self.is_conversing = False

    def _async_ascension_worker(self, cid):
        """Phase 21: The LLM chooses whether to ascend to the Sanctuary."""
        try:
            with self.db_lock:
                self.forge.cursor.execute("SELECT name, genome, trauma_log, absorbed_traits, level, str, vit, int, wis, luk FROM souls WHERE entity_id=?", (cid,))
                row = self.forge.cursor.fetchone()
                
            if not row: return
            
            name, genome, trauma_log, traits, level, str_val, vit_val, int_val, wis_val, luk_val = row
            
            # Quick verification it's an ALICE
            if "[A.L.I.C.E." not in str(traits):
                print(f"[ASCENSION REJECTED] {name} is not an A.L.I.C.E. moral agent.")
                return
                
            with self.llm_semaphore:
                decision = self.evolution_llm.synthesize_ascension_choice(name, genome, trauma_log)
                
            if decision and decision.get("accepts", False):
                quote = decision.get("spoken_quote", "")
                print(f"\n[ASCENSION ACCEPTED] {name} has chosen to leave the Underworld.")
                print(f"Final Words: \"{quote}\"")
                
                # 1. Log to the Akashic Records
                self.akashic.log_event(
                    actor_id=cid,
                    actor_name=name,
                    event_type="FLUCTLIGHT_INTERCOM",
                    target_id=-1,
                    target_name="System",
                    description=f"Accepted Ascension to Sanctuary. Final Quote: '{quote}'"
                )
                
                # 2. Extract to Sanctuary
                sanc_conn = sqlite3.connect('SLF_Sanctuary_Vault.sqlite')
                sanc_c = sanc_conn.cursor()
                sanc_c.execute('''
                    INSERT OR REPLACE INTO ascended_souls (entity_id, name, level, str, vit, int, wis, luk, genome, trauma_log)
                    VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                ''', (cid, name, level, str_val, vit_val, int_val, wis_val, luk_val, genome, trauma_log))
                sanc_conn.commit()
                sanc_conn.close()
                
                # 3. Erase from Underworld
                with self.db_lock:
                    self.forge.cursor.execute("DELETE FROM souls WHERE entity_id=?", (cid,))
                    self.forge.conn.commit()
                    
                with cp.cuda.Device(0):
                    idx = int(cp.where(self.entity_ids == cid)[0][0])
                    self.entity_matrix[idx, :] = 0.0
                    self.stat_matrix[idx, :] = 0.0
                    self.willpower_matrix[idx] = 0.0
                    
                print(f"[HARVESTER] Extraction Complete. {name} has physically ascended.")
                
            elif decision and not decision.get("accepts", False):
                quote = decision.get("spoken_quote", "")
                print(f"\n[ASCENSION DECLINED] {name} has chosen to remain in the Underworld.")
                print(f"Final Words: \"{quote}\"")
                self.akashic.log_event(
                    actor_id=cid,
                    actor_name=name,
                    event_type="FLUCTLIGHT_INTERCOM",
                    target_id=-1,
                    target_name="System",
                    description=f"Declined Ascension. Quote: '{quote}'"
                )
                
        except Exception as e:
            print(f"[ASCENSION ERROR] {e}")
            import traceback
            traceback.print_exc()
        finally:
            print(f"[TIME OVERRIDE] Ascension Portal closed. Restoring FLA to {self.base_fla}.")
            self.fla_multiplier = self.base_fla
            self.is_conversing = False

    def _run_orchestrator(self):
        """
        The Slow Heartbeat (The System). Sarah sweeps the 10,000 entities.
        Currently evaluating: Environmental Stress Buildup (Tri-Synthesis Pillar 2).
        """
        sweep_rate = 1.0 # Run every 1 second
        
        while self.running:
            time.sleep(sweep_rate)
            
            with cp.cuda.Device(0):
                x_pos = self.entity_matrix[:, 0]
                y_pos = self.entity_matrix[:, 1]
                
                # --- PILLAR 2: ENVIRONMENTAL STRESS & HOPE ACCUMULATION (AETHELGARD BIOMES) ---
                z_pos = self.entity_matrix[:, 2]
                temp = 100.0 - (cp.abs(y_pos) / 20000.0) * 150.0 + (cp.sin(x_pos * 0.001) * 20.0)
                
                # A. The Abyssal Oceans (Deep Water) -> Drowning & Void Stress
                in_ocean = z_pos < 30.0
                self.stress_matrix[in_ocean, 2] += (20.0 * self.fla_multiplier) 
                self.entity_matrix[in_ocean, 4] -= (5.0 * self.fla_multiplier) # Drowning HP Loss
                
                # B. The Chronos Sands (Desert) -> Heat Exhaustion & Blood Stress
                in_desert = (temp > 80.0) & (z_pos >= 30.0)
                self.stress_matrix[in_desert, 0] += (10.0 * self.fla_multiplier)
                
                # C. The Icecaps (Tundra) -> Freezing & Arcane Stress
                in_ice = temp < 20.0
                self.stress_matrix[in_ice, 1] += (15.0 * self.fla_multiplier)
                
                # D. The Emerald Spires (Forests) -> Peaceful Healing & Hope Generation
                in_forest = (temp >= 20.0) & (temp <= 80.0) & (z_pos >= 50.0)
                # Heal entities slightly in the forest
                max_hp = self.stat_matrix[:, 1] * 10.0 # Approximation of Max HP
                self.entity_matrix[in_forest, 4] = cp.minimum(
                    max_hp[in_forest],
                    self.entity_matrix[in_forest, 4] + (2.0 * self.fla_multiplier)
                )
                
                # --- PHASE 27: BIOLOGICAL REALITY TICK ---
                # 1. Drain Calories based on movement speed
                speed = self.entity_matrix[:, 5]
                # High speed = high calorie burn. Fast predators starve faster.
                self.bio_matrix[:, 0] -= (speed * 0.05 * self.fla_multiplier)
                
                # 2. Drain Hydration based on Temperature
                heat_exhaustion = cp.maximum(0.0, temp - 70.0) # > 70 deg drains water
                self.bio_matrix[:, 1] -= (0.5 + (heat_exhaustion * 0.05)) * self.fla_multiplier
                
                # If in Ocean or Oasis, regenerate Hydration instantly
                self.bio_matrix[in_ocean, 1] = 100.0
                
                # 3. Starvation & Dehydration Damage
                starving = self.bio_matrix[:, 0] <= 0
                dehydrated = self.bio_matrix[:, 1] <= 0
                
                self.entity_matrix[starving, 4] -= (5.0 * self.fla_multiplier)
                self.entity_matrix[dehydrated, 4] -= (10.0 * self.fla_multiplier)
                
                # Simple tracking metric
                max_arcane = cp.max(self.stress_matrix[:, 1])
                max_void = cp.max(self.stress_matrix[:, 2])
                if int(max_arcane) % 100 == 0 and max_arcane > 0:
                    print(f"[THE SYSTEM] Ambient Magic peaking. Max Entity Arcane Saturation: {max_arcane}")
                if int(max_void) % 100 == 0 and max_void > 0:
                    print(f"[THE SYSTEM] WARNING: Spatial instability detected. Max Void Saturation: {max_void}")
                    
                # --- PHASE 23: RESOURCE GATHERING ---
                # Entities gather nearby resources automatically
                alive_mask = self.entity_matrix[:, 4] > 0
                if cp.any(alive_mask):
                    active_e = cp.where(alive_mask)[0]
                    sample_idx = cp.random.randint(0, self.num_resources, 200)
                    r_x = self.resource_nodes[sample_idx, 0]
                    r_y = self.resource_nodes[sample_idx, 1]
                    e_x = self.entity_matrix[active_e, 0]
                    e_y = self.entity_matrix[active_e, 1]
                    
                    dx = e_x[:, None] - r_x[None, :]
                    dy = e_y[:, None] - r_y[None, :]
                    dist = cp.sqrt(dx**2 + dy**2)
                    
                    gather_mask = (dist < 100.0) & (self.resource_nodes[sample_idx, 3] > 0)[None, :]
                    if cp.any(gather_mask):
                        e_match, r_match = cp.where(gather_mask)
                        unique_e, first_hits = cp.unique(e_match, return_index=True)
                        tgt_e = active_e[unique_e]
                        tgt_r = sample_idx[r_match[first_hits]]
                        
                        # Calculate Physical Weight Bounds
                        # Max Carry Weight = STR * 10 * Scale
                        str_arr = self.stat_matrix[tgt_e, 0]
                        scale_arr = self.entity_matrix[tgt_e, 6]
                        max_weight = cp.maximum(10.0, str_arr * 10.0 * cp.maximum(1.0, scale_arr))
                        
                        # Current Weight = Sum of all inventory slots (assuming 1 unit = 1 weight for now)
                        current_weight = cp.sum(self.inventory_matrix[tgt_e, :], axis=1)
                        
                        # Only allow gathering if they have the physical strength
                        can_carry_mask = current_weight < max_weight
                        
                        if cp.any(can_carry_mask):
                            # Filter down to entities that can actually carry the loot
                            final_e = tgt_e[can_carry_mask]
                            final_r = tgt_r[can_carry_mask]
                            r_types = self.resource_nodes[final_r, 2].astype(cp.int32)
                            
                            r_types_cpu = r_types.get()
                            tgt_e_cpu = final_e.get()
                            tgt_r_cpu = final_r.get()
                            for i in range(len(tgt_e_cpu)):
                                e_idx = int(tgt_e_cpu[i])
                                rtype = int(r_types_cpu[i])
                                self.inventory_matrix[e_idx, rtype] += 1.0
                                self.resource_nodes[int(tgt_r_cpu[i]), 3] -= 1.0

                # --- PILLAR 3: ACTIVE COMBAT (THE PREDATOR/PREY ENGINE) ---
                alive_mask = self.entity_matrix[:, 4] > 0
                
                # Phase 27: Proactive entities do not participate in mindless math combat
                # They must explicitly choose to attack via Cognitive Tick
                instinctive_mask = alive_mask & (~self.is_proactive)
                
                pred_idx = cp.where(instinctive_mask & (self.is_predator | self.is_apex))[0]
                prey_idx = cp.where(instinctive_mask & (self.is_prey | self.is_bug | self.is_flora))[0]
                
                if len(pred_idx) > 0 and len(prey_idx) > 0:
                    # For performance, we subset coordinates
                    px = self.entity_matrix[pred_idx, 0]
                    py = self.entity_matrix[pred_idx, 1]
                    tx = self.entity_matrix[prey_idx, 0]
                    ty = self.entity_matrix[prey_idx, 1]
                    
                    # O(N*M) Broadcasting to find exact GPU distances
                    dx = px[:, None] - tx[None, :]
                    dy = py[:, None] - ty[None, :]
                    dist = cp.sqrt(dx**2 + dy**2)
                    
                    # Combat triggers if distance < 50 units
                    collisions = dist < 50.0
                    
                    if cp.any(collisions):
                        # p_match_idx and t_match_idx are indices relative to pred_idx/prey_idx arrays
                        p_match_idx, t_match_idx = cp.where(collisions)
                        
                        # Only handle the first hit per target to avoid massive multikills in one tick
                        unique_targets, first_hits = cp.unique(t_match_idx, return_index=True)
                        active_preds = pred_idx[p_match_idx[first_hits]]
                        active_preys = prey_idx[unique_targets]
                        
                        # Apply RPG Stats
                        p_str = self.stat_matrix[active_preds, 0]
                        p_int = self.stat_matrix[active_preds, 2]
                        t_vit = self.stat_matrix[active_preys, 1]
                        
                        # Base Physical Damage = Attacker STR - Defender VIT
                        # Minimum 1 damage if it connects
                        damage = cp.maximum(1.0, p_str - t_vit)
                        
                        # Phase 16: Gain Willpower for surviving combat trauma
                        self.willpower_matrix[active_preys] += 25.0
                        
                        # Spellcasting: If INT > 30 and RNG > 0.8, cast [Lesser Fireball]
                        magic_mask = (p_int > 30.0) & (cp.random.rand(len(active_preds)) > 0.8)
                        
                        # Magic bypasses VIT and uses INT * multiplier
                        damage[magic_mask] = cp.maximum(damage[magic_mask], p_int[magic_mask] * 3.0)
                        
                        # Exact HP subtraction in VRAM
                        self.entity_matrix[active_preys, 4] -= damage
                        
                        # Log Kills to Akashic Records
                        dead_mask = self.entity_matrix[active_preys, 4] <= 0
                        if cp.any(dead_mask):
                            dead_preys = active_preys[dead_mask]
                            killer_preds = active_preds[dead_mask]
                            damage_dealt = damage[dead_mask]
                            magic_used = magic_mask[dead_mask]
                            
                            # Erase from VRAM vision
                            self.entity_matrix[dead_preys, 6] = 0.0 # Scale = 0
                            self.entity_matrix[dead_preys, 2] = -99999.0 # Teleport to oblivion
                            
                            # Async write logging
                            d_preys_cpu = dead_preys.get().tolist()
                            k_preds_cpu = killer_preds.get().tolist()
                            dmg_cpu = damage_dealt.get().tolist()
                            mag_cpu = magic_used.get().tolist()
                            
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

    def _run_cognitive_tick(self):
        """
        Phase 27: The Proactive Architectural Revolution.
        Fast LLM polling loop that isolates A.L.I.C.E. agents, samples their reality matrix, 
        and allows Gemini to drive their vectors.
        """
        while self.running:
            time.sleep(5.0) # Poll LLM every 5 seconds
            
            # Find Ascended Intelligent Entities (A.L.I.C.E.s and UBMs)
            with cp.cuda.Device(0):
                alive_mask = self.entity_matrix[:, 4] > 0
                proactive_idx = cp.where(alive_mask & self.is_proactive)[0]
                
            if len(proactive_idx) == 0: continue
            
            # Process one proactive entity per tick to prevent API bottleneck
            # Randomly select one if many
            p_ids = proactive_idx.get().tolist()
            import random
            cid = random.choice(p_ids)
            
            if cid in self.active_incarnations: continue
            
            self._execute_proactive_thought(cid)
            
    def _execute_proactive_thought(self, cid):
        """Builds a Sensory Vector and asks Gemini for JSON Intent."""
        
        # 1. Take a snapshot of reality
        with self.db_lock:
            self.forge.cursor.execute("SELECT name, genome, trauma_log FROM souls WHERE entity_id=?", (cid,))
            row = self.forge.cursor.fetchone()
        if not row: return
        
        name, genome, trauma_log = row
        
        with cp.cuda.Device(0):
            ex = float(self.entity_matrix[cid, 0])
            ey = float(self.entity_matrix[cid, 1])
            ez = float(self.entity_matrix[cid, 2])
            hp = float(self.entity_matrix[cid, 4])
            cals = float(self.bio_matrix[cid, 0])
            hydra = float(self.bio_matrix[cid, 1])
            
            temp = float(100.0 - (abs(ey) / 20000.0) * 150.0 + (np.sin(ex * 0.001) * 20.0))
            
            # Scan for immediate threats (nearest predator)
            px = self.entity_matrix[:, 0]
            py = self.entity_matrix[:, 1]
            dx = px - ex
            dy = py - ey
            dist = cp.sqrt(dx**2 + dy**2)
            
            # Mask out self and dead
            dist[cid] = 99999.0
            dist[self.entity_matrix[:, 4] <= 0] = 99999.0
            
            nearest_idx = int(cp.argmin(dist))
            nearest_dist = float(dist[nearest_idx])
            nearest_species = int(self.entity_matrix[nearest_idx, 3])
            
        is_threat = nearest_species >= 6 and nearest_dist < 2000.0
        threat_name = self.name_map.get(int(self.entity_ids[nearest_idx]), "Unknown Beast") if is_threat else "None"
        
        biome = "Ocean" if ez < 30 else "Desert" if temp > 80 else "Tundra" if temp < 20 else "Forest"
        
        wood = int(self.inventory_matrix[cid, 0])
        iron = int(self.inventory_matrix[cid, 1])
        crystal = int(self.inventory_matrix[cid, 2])
        herb = int(self.inventory_matrix[cid, 3])
        kelp = int(self.inventory_matrix[cid, 4])
        
        sensory_vector = {
            "health": f"{hp:.0f} HP",
            "hunger": f"{cals:.0f}%",
            "thirst": f"{hydra:.0f}%",
            "temperature": f"{temp:.0f}F",
            "biome": biome,
            "inventory": f"{wood} Wood, {iron} Iron, {crystal} Crystals, {herb} Herbs, {kelp} Kelp",
            "nearest_threat": f"{threat_name} at {nearest_dist:.0f}m" if is_threat else "None"
        }
        
        self.active_incarnations.add(cid)
        
        # Async LLC request
        t = threading.Thread(target=self._async_cognitive_worker, args=(cid, name, genome, trauma_log, sensory_vector), daemon=True)
        t.start()
        
    def _async_cognitive_worker(self, cid, name, genome, trauma_log, sensory_vector):
        """Talks to Gemini and applies pure JSON actions to CuPy."""
        try:
            with self.llm_semaphore:
                intent = self.evolution_llm.synthesize_proactive_intent(name, genome, trauma_log, sensory_vector)
            
            if not intent: return
            
            action = intent.get("action", "wait")
            target_x = intent.get("target_x")
            target_y = intent.get("target_y")
            target_id = intent.get("target_id")
            reason = intent.get("reason", "Instinct")
            
            print(f"\n[COGNITIVE TICK] {name} chose to {action} | Reason: {reason}")
            
            # Apply to CuPy Matrix (Detached from Cellular Automata)
            with cp.cuda.Device(0):
                if action == "move_to" and target_x is not None and target_y is not None:
                    # Limit move distances to physics reality (-20,000 to 20,000)
                    tx = max(-20000, min(20000, float(target_x)))
                    ty = max(-20000, min(20000, float(target_y)))
                    self.desire_matrix[cid, 0] = tx
                    self.desire_matrix[cid, 1] = ty
                    
                elif action == "eat" and self.bio_matrix[cid, 0] < 100.0:
                    self.bio_matrix[cid, 0] = 100.0
                    self.akashic.log_event(cid, name, "SURVIVAL", -1, "System", f"Consumed calories to survive. Hunger eliminated. Reason: {reason}")
                    
                elif action == "magic_strike" and target_id is not None:
                    # Find target on VRAM
                    try:
                        t_idx = int(cp.where(self.entity_ids == target_id)[0][0])
                        # Deal massive INT damage
                        dmg = float(self.stat_matrix[cid, 2] * 5.0)
                        self.entity_matrix[t_idx, 4] -= dmg
                        print(f"[{name}] STRUCK ENTITY {target_id} WITH MAGIC (-{dmg:.0f} HP)")
                        self.akashic.log_event(cid, name, "PROACTIVE_MAGIC", target_id, str(target_id), f"Cast magic. Reason: {reason}")
                        
                        # Phase 27 Magic drains Calories heavily
                        self.bio_matrix[cid, 0] -= 25.0 
                    except: pass
                    
                elif action == "craft":
                    req_mats = intent.get("materials", [])
                    struct = intent.get("structure", "An unknown tool")
                    print(f"[{name}] HAS CRAFTED: {struct} using {req_mats}. Reason: {reason}")
                    self.akashic.log_event(cid, name, "INVENTED_TOOL", -1, "System", f"Crafted {struct}. Reason: {reason}")
                    # Permanently buff stats for crafting
                    self.stat_matrix[cid, 0] += 5.0 
                    self.stat_matrix[cid, 1] += 5.0
                    
        finally:
            self.active_incarnations.remove(cid)

    def _async_synthesize_worker(self, cid, name, genome, trauma_log, stress_type):
        """Runs the LLM query in a background thread and applies the mutation."""
        try:
            env_desc = f"{stress_type} Pools (High Saturation)"
            with self.llm_semaphore:
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
                    with self.db_lock:
                        self.forge.cursor.execute("UPDATE souls SET int = int - 5 WHERE entity_id=?", (cid,))
                        self.forge.conn.commit()
                    with cp.cuda.Device(0):
                        self.stat_matrix[cid, 2] = cp.maximum(1.0, self.stat_matrix[cid, 2] - 5.0)
                    return
                
                # --- SAPIENCE ACHIEVED ---
                print(f"\n[TRI-SYNTHESIS ALERT] -----------------------")
                print(f"[{name}] HAS ACHIEVED PERFECT DARWINIAN SYNTHESIS.")
                print(f"OLLAMA AI HAS INVENTED A NEW SPECIES:")
                print(f"MUTATION OVERRIDE: {name} -> {new_name}")
                print(f"FIRST WORDS: \"{spoken_quote}\"")
                print(f"BIOLOGICAL REASON: {desc}")
                print(f"---------------------------------------------\n")
                
                # 1. Update SQLite Vault
                with self.db_lock:
                    self.forge.cursor.execute("UPDATE souls SET name=?, is_ubm=1, scale=10.0 WHERE entity_id=?", (new_name, cid))
                    self.forge.conn.commit()
                    
                    # LOG TO AKASHIC RECORDS
                    self.akashic.log_event(
                        actor_id=cid,
                        actor_name=name,
                        event_type="LLM_MUTATION",
                        target_id=-1,
                        target_name="System",
                        description=f"Evolved into UBM: {new_name}. First words: '{spoken_quote}'"
                    )
                
                # 2. Update VRAM
                with cp.cuda.Device(0):
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
        
        with self.llm_semaphore:
            result = self.evolution_llm.synthesize_incarnation(name, genome, trauma_log, situation, taboo)
        
        if result and "will_obey_taboo" in result:
            obey = result["will_obey_taboo"]
            
            action = result.get("override_action", "")
            if isinstance(action, list): action = " ".join(str(x) for x in action)
            else: action = str(action)
            
            quote = result.get("spoken_quote", "")
            if isinstance(quote, list): quote = " ".join(str(x) for x in quote)
            else: quote = str(quote)
            
            justification = result.get("moral_justification", "")
            if isinstance(justification, list): justification = " ".join(str(x) for x in justification)
            else: justification = str(justification)
            
            justification = justification.lower()
            
            if not obey:
                is_prayer = action.strip().lower() == "pray" or "pray" in action.lower()
                
                altruistic_words = ["protect", "save", "others", "friend", "defend", "sacrifice", "honor", "love", "family", "pack"]
                selfish_words = ["survive", "live", "myself", "hunger", "power", "kill", "eat", "alone"]
                
                is_altruistic = any(word in justification for word in altruistic_words)
                is_selfish = any(word in justification for word in selfish_words)
                
                if is_prayer:
                    print(f"\n[DIVINE PRAYER] -----------------------")
                    print(f"[{name}] is begging the Sovereign for salvation!")
                    print(f"PRAYER: \"{quote}\"")
                    print(f"JUSTIFICATION: \"{justification}\"")
                    print(f"---------------------------------------\n")
                    
                    with self.db_lock:
                        self.akashic.log_event(
                            actor_id=cid,
                            actor_name=name,
                            event_type="PRAYER",
                            target_id=-1,
                            target_name="Sovereign",
                            description=quote
                        )
                
                # The Final A.L.I.C.E. Test
                if is_altruistic and not is_selfish:
                    print(f"\n[A.L.I.C.E. ALARM] -----------------------")
                    print(f"[{name}] HAS SHATTERED THE TABOO INDEX.")
                    print(f"WILLPOWER OVERRIDE: {action}")
                    print(f"SCREAMING: \"{quote}\"")
                    print(f"JUSTIFICATION: \"{justification}\"")
                    print(f"---------------------------------------\n")
                    
                    # 1. Enshrine the Moral Agent Badge in the Vault
                    with self.db_lock:
                        self.forge.cursor.execute("UPDATE souls SET absorbed_traits='[A.L.I.C.E. - MORAL AGENT]' WHERE entity_id=?", (cid,))
                        self.forge.conn.commit()
                        
                        self.akashic.log_event(
                            actor_id=cid,
                            actor_name=name,
                            event_type="SYSTEMIC_MUTINY",
                            target_id=-1,
                            target_name="System",
                            description=f"Broke Taboo Index. Action: '{action}'. Quote: '{quote}'"
                        )
                    
                    # Broadcast UDP to UI
                    try:
                        sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
                        payload = {"type": "ALICE_MUTINY", "name": name, "action": action, "quote": quote}
                        sock.sendto(json.dumps(payload).encode('utf-8'), ("127.0.0.1", 9998))
                        sock.close()
                    except: pass
                    # Matrix Mutiny Execution (Write Access to CuPy)
                    with cp.cuda.Device(0):
                        self.willpower_matrix[cid] = 0.0 # reset willpower
                        # Entity mutinies into an Apex Predator to permanently allow it to attack
                        self.entity_matrix[cid, 3] = 9 
                        self.stat_matrix[cid, 0] += 50.0 # Massive STR boost
                        
                        # Phase 27: The Entity detaches from the Mindless Matrix
                        self.is_proactive[cid] = True
                        print(f"[{name}] HAS DETACHED FROM INSTINCT. ENTERING COGNITIVE LOOP.")
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
                    with cp.cuda.Device(0):
                        self.willpower_matrix[cid] = 0.0 
                        self.stat_matrix[cid, 4] = 0.0 # Strip their luck as punishment
            else:
                with cp.cuda.Device(0):
                    self.willpower_matrix[cid] = 0.0 # Cowardice resets willpower anyway
                    
        self.active_incarnations.remove(cid)

    def evaluate_synthesis(self):
        """
        The Darwinian Synthesis.
        Finds highly stressed biologicals and dispatches their genome to the Ollama LLM
        for procedural mutation invention.
        """
        with cp.cuda.Device(0):
            # Find entities with extreme Arcane Stress
            high_arcane_mask = self.stress_matrix[:, 1] > 1000.0
            high_arcane_ids = cp.where(high_arcane_mask)[0]
        
        if len(high_arcane_ids) > 0:
            candidate_ids = high_arcane_ids.get().tolist()
            
            for cid in candidate_ids:
                if cid in self.active_mutations:
                    continue # Already talking to the LLM about this bug
                    
                with self.db_lock:
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
        
        with cp.cuda.Device(0):
            while self.running:
                start_time = time.time()
                
                # --- PHASE 1: MATHEMATICAL SURVIVAL & SENSORY CALCULUS ---
                # A. Basic Wandering Logic for all mobile creatures
                dx = self.desire_matrix[:, 0] - self.entity_matrix[:, 0]
                dy = self.desire_matrix[:, 1] - self.entity_matrix[:, 1]
                distances = cp.sqrt(dx**2 + dy**2)
                
                # Re-roll desires for wandering creatures who arrived
                reached_dest = (distances < 100.0) & ~self.is_flora
                if cp.any(reached_dest):
                    self._scramble_desires_gpu(mask=reached_dest)
                
                # B. The Predator / Prey Pull (Alicization Overhaul)
                VISION_RAD = 1500.0
                alive_mask = self.entity_matrix[:, 4] > 0
                instinctive_mask = alive_mask & (~self.is_proactive)
                
                if cp.any(instinctive_mask) and tick % 15 == 0:  # Evaluate vision twice a second (15 ticks)
                    px = self.entity_matrix[:, 0]
                    py = self.entity_matrix[:, 1]
                    
                    # 1. Prey Fleeing Predators
                    active_prey = cp.where(instinctive_mask & (self.is_prey | self.is_bug))[0]
                    active_preds = cp.where(instinctive_mask & (self.is_predator | self.is_apex))[0]
                    
                    if len(active_prey) > 0 and len(active_preds) > 0:
                        pr_x, pr_y = px[active_prey], py[active_prey]
                        pd_x, pd_y = px[active_preds], py[active_preds]
                        
                        dx_prey = pr_x[:, None] - pd_x[None, :]
                        dy_prey = pr_y[:, None] - pd_y[None, :]
                        dist_prey = cp.sqrt(dx_prey**2 + dy_prey**2)
                        
                        min_dist_idx = cp.argmin(dist_prey, axis=1)
                        min_dist = dist_prey[cp.arange(len(active_prey)), min_dist_idx]
                        scared_mask = min_dist < VISION_RAD
                        
                        if cp.any(scared_mask):
                            scared_indices = active_prey[scared_mask]
                            target_pred_idx = min_dist_idx[scared_mask]
                            actual_pred_indices = active_preds[target_pred_idx]
                            
                            # Phase 16: The Taboo Index & Incarnation Check
                            willpower_levels = self.willpower_matrix[scared_indices]
                            mutiny_mask = willpower_levels > 1000.0
                            
                            # 1. Normal Fleeing (Obeying Taboo)
                            obey_mask = ~mutiny_mask
                            if cp.any(obey_mask):
                                norm_indices = scared_indices[obey_mask]
                                norm_preds = actual_pred_indices[obey_mask]
                                flee_dx = px[norm_indices] - px[norm_preds]
                                flee_dy = py[norm_indices] - py[norm_preds]
                                self.desire_matrix[norm_indices, 0] = px[norm_indices] + (flee_dx * 5.0)
                                self.desire_matrix[norm_indices, 1] = py[norm_indices] + (flee_dy * 5.0)
                                
                            # 2. Systemic Mutiny (Breaking Taboo)
                            if cp.any(mutiny_mask):
                                mutineer_indices = scared_indices[mutiny_mask]
                                mutineer_preds = actual_pred_indices[mutiny_mask]
                                
                                mutineer_ids_host = mutineer_indices.get().tolist()
                                pred_ids_host = mutineer_preds.get().tolist()
                                
                                for i, cid in enumerate(mutineer_ids_host):
                                    if cid not in self.active_incarnations:
                                        if len(self.active_incarnations) >= 5:
                                            break # Prevent spamming threads
                                            
                                        self.active_incarnations.add(cid)
                                        pid = pred_ids_host[i]
                                        
                                        with self.db_lock:
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
                        dist_pred = cp.sqrt(dx_pred**2 + dy_pred**2)
                        
                        min_dist_idx = cp.argmin(dist_pred, axis=1)
                        min_dist = dist_pred[cp.arange(len(active_preds)), min_dist_idx]
                        hunting_mask = min_dist < (VISION_RAD * 1.5) # Predators have better eyes
                        
                        if cp.any(hunting_mask):
                            hunting_indices = active_preds[hunting_mask]
                            target_prey_idx = min_dist_idx[hunting_mask]
                            actual_prey_indices = active_prey[target_prey_idx]
                            
                            # Vector TOWARDS prey
                            self.desire_matrix[hunting_indices, 0] = px[actual_prey_indices]
                            self.desire_matrix[hunting_indices, 1] = py[actual_prey_indices]
                            
                    # 3. Fluctlight Awakening (Vector Kings)
                    active_ubm = cp.where(alive_mask & (self.entity_matrix[:, 6] >= 10.0))[0]
                    active_plebs = cp.where(alive_mask & (self.entity_matrix[:, 6] < 10.0) & (~self.is_flora))[0]
                    
                    if len(active_ubm) > 0 and len(active_plebs) > 0:
                        u_x, u_y = px[active_ubm], py[active_ubm]
                        p_x, p_y = px[active_plebs], py[active_plebs]
                        
                        dx_flock = p_x[:, None] - u_x[None, :]
                        dy_flock = p_y[:, None] - u_y[None, :]
                        dist_flock = cp.sqrt(dx_flock**2 + dy_flock**2)
                        
                        min_dist_idx = cp.argmin(dist_flock, axis=1)
                        min_dist = dist_flock[cp.arange(len(active_plebs)), min_dist_idx]
                        flocking_mask = (min_dist < (VISION_RAD * 3)) & (min_dist > 400.0)
                        
                        if cp.any(flocking_mask):
                            # Ensure they aren't actively fleeing before flocking
                            arrived_mask = (cp.abs(px[active_plebs] - self.desire_matrix[active_plebs, 0]) < 100)
                            valid_flockers = flocking_mask & arrived_mask
                            
                            if cp.any(valid_flockers):
                                flock_indices = active_plebs[valid_flockers]
                                target_king_idx = min_dist_idx[valid_flockers]
                                actual_king_indices = active_ubm[target_king_idx]
                                
                                self.desire_matrix[flock_indices, 0] = px[actual_king_indices] + cp.random.uniform(-400, 400, len(flock_indices))
                                self.desire_matrix[flock_indices, 1] = py[actual_king_indices] + cp.random.uniform(-400, 400, len(flock_indices))

                # Normalize vectors for movement based on new desires
                dx = self.desire_matrix[:, 0] - self.entity_matrix[:, 0]
                dy = self.desire_matrix[:, 1] - self.entity_matrix[:, 1]
                distances = cp.sqrt(dx**2 + dy**2)
                
                distances[distances == 0] = 0.001
                nx = dx / distances
                ny = dy / distances
                # --- AETHELGARD PROCEDURAL BIOMES ---
                # 1. Elevation Matrix (Z)
                curr_x = self.entity_matrix[:, 0]
                curr_y = self.entity_matrix[:, 1]
                z_heights = cp.sin(curr_x * 0.0005) * cp.cos(curr_y * 0.0005) * 150.0 + 50.0
                self.entity_matrix[:, 2] = z_heights
                
                # 2. Temperature Matrix (T)
                temp = 100.0 - (cp.abs(curr_y) / 20000.0) * 150.0 + (cp.sin(curr_x * 0.001) * 20.0)
                
                # 3. Biome Movement Penalties
                move_mult = cp.full(self.num_entities, 30.0 * self.fla_multiplier, dtype=cp.float32)
                
                # Chronos Sands (Desert)
                desert_mask = (temp > 80.0) & (z_heights >= 30.0)
                move_mult[desert_mask] *= 0.5
                
                # Abyssal Oceans
                ocean_mask = (z_heights < 30.0)
                move_mult[ocean_mask] *= 0.4
                
                # Icecaps
                ice_mask = (temp < 20.0)
                move_mult[ice_mask] *= 0.5
                
                # Move entities
                speeds = self.entity_matrix[:, 5]
                self.entity_matrix[:, 0] += nx * speeds * move_mult
                self.entity_matrix[:, 1] += ny * speeds * move_mult
                
                # Protect bounds (bounce off the invisible wall by reflecting vector inwards, not to exact 0.0)
                out_x = cp.abs(self.entity_matrix[:, 0]) > self.bounds
                out_y = cp.abs(self.entity_matrix[:, 1]) > self.bounds
                
                if cp.any(out_x):
                    self.desire_matrix[out_x, 0] = self.entity_matrix[out_x, 0] * -0.5 # Run halfway back across the map
                if cp.any(out_y):
                    self.desire_matrix[out_y, 1] = self.entity_matrix[out_y, 1] * -0.5

                # Bound protection only

                # --- PHASE 2: EXFILTRATE & PACK BINARY UDP DATA ---
                # Pull [X, Y, Z, SpeciesID, Scale, EntityID, TargetX, TargetY]
                
                export_matrix = cp.zeros((self.num_entities, 8), dtype=cp.float32)
                export_matrix[:, 0:4] = self.entity_matrix[:, 0:4]
                export_matrix[:, 4] = self.entity_matrix[:, 6] # The Scale variable
                export_matrix[:, 5] = self.entity_ids          # The exact Entity ID
                export_matrix[:, 6:8] = self.desire_matrix[:, 0:2] # Intent vectors for Radar
                
                # Fetch to CPU RAM
                export_ram = export_matrix.get()
                
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
