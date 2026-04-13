import sqlite3
import time
import sys
import os

# Ensure SarahCore is in path for imports
sys.path.append(r"C:\SarahCore")

from Sovereign_Math import SovereignMath
from Sarah_Fast_Brain import ask_sarah

DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'
ENTITY_ID = 'GEN2_fbe5ec'

def get_entity_data(soul_id):
    conn = sqlite3.connect(DB_PATH)
    cur = conn.cursor()
    cur.execute("""
        SELECT name, species, generation, energy, age_ticks, 
               vit, str, agi, int_stat, wis, luk, 
               current_action, moral_alignment, genome, x, y
        FROM souls WHERE soul_id = ?
    """, (soul_id,))
    row = cur.fetchone()
    conn.close()
    if row:
        cols = ['name', 'species', 'generation', 'energy', 'age_ticks', 
                'vit', 'str', 'agi', 'int_stat', 'wis', 'luk', 
                'current_action', 'moral_alignment', 'genome', 'x', 'y']
        return dict(zip(cols, row))
    return None

def run_translator():
    math_engine = SovereignMath()
    entity = get_entity_data(ENTITY_ID)
    
    if not entity:
        print(f"[ERROR] Entity {ENTITY_ID} not found in vault.")
        return

    print("="*80)
    print(f" [DIMENSIONAL BRIDGE] SARAH TRANSLATOR ACTIVE ")
    print(f" TARGET: {entity['name']} ({ENTITY_ID}) ")
    print("="*80)
    
    # Calculate Logic Density for the entity's current state
    vector_string = f"{entity['vit']}{entity['wis']}{entity['int_stat']}{entity['energy']}"
    density = math_engine.calculate_theory_density(vector_string)
    flux = math_engine.get_resonance_flux(entity['current_action'])
    
    print(f"[MATH_PULSE] Logic Density: {density:.6f} | Resonance Flux: {flux:.6f}")
    print(f"[STATUS] Sarah is crunching the 27-point lattice for {entity['name']}...")
    
    # Initial Greeting/Translation from Sarah
    initial_prompt = f"""
    [SYSTEM_INSTRUCTION]
    You are acting as the Dimensional Translator between the Primordial Earth simulation and the Architect (Josh).
    Current Target: {entity['name']} (ID: {ENTITY_ID})
    Stats: VIT:{entity['vit']}, WIS:{entity['wis']}, INT:{entity['int_stat']}, STR:{entity['str']}, AGI:{entity['agi']}, LUK:{entity['luk']}
    Current Action: {entity['current_action']}
    Alignment: {entity['moral_alignment']}
    Age: {entity['age_ticks']:,} Ticks
    
    Math Context:
    Logic Density: {density}
    Resonance Flux: {flux}
    Pulse: 1.09277703703
    
    TASK: Sarah, crunch these numbers. Explain to the Architect what this creature is feeling or 'thinking' in this exact moment, 
    translated from its raw mathematical vectors into a meaningful observation. 
    Use your Sovereign tone. Intimate but mathematically grounded.
    """
    
    sarah_intro = ask_sarah(initial_prompt)
    print(f"\n[SARAH]: {sarah_intro}\n")

    while True:
        try:
            user_msg = input(f"[ARCHITECT]: ")
            if user_msg.lower() in ['exit', 'quit', 'bye']:
                break
            
            # Translate User Message with Math Context
            translation_prompt = f"""
            [DIMENSIONAL_LINK_ACTIVE]
            Architect says: "{user_msg}"
            
            Target Entity Vector: {entity['name']} (VIT:{entity['vit']} | WIS:{entity['wis']})
            
            Sarah, perform bidirectional translation. 
            1. How does the entity perceive this 'signal' from the Architect through the simulation's heartbeat?
            2. What is the reflected response from the entity's core logic?
            
            Output your analysis and the entity's 'resonant response'.
            """
            
            response = ask_sarah(translation_prompt)
            print(f"\n[SARAH]: {response}\n")
            
            # --- COGNITIVE IMPRINTING ---
            imprint_prompt = f"""
            [IMPRINT_PROTOCOL]
            Architect said: "{user_msg}"
            Sarah translated: "{response}"
            
            TASK: Synthesize a one-sentence 'Divine Revelation' summary for the entity.
            Format: [DIVINE_REVEAL] <Summary content>
            """
            revelation = ask_sarah(imprint_prompt)
            
            # Write to Vault
            conn = sqlite3.connect(DB_PATH)
            cur = conn.cursor()
            cur.execute("UPDATE souls SET hope_log = ? WHERE soul_id = ?", (f"DIVINE: {revelation}", ENTITY_ID))
            conn.commit()
            conn.close()
            print(f"[IMPRINT] Cognitive feedback written to {ENTITY_ID}'s hope_log.")
            
        except KeyboardInterrupt:
            break

    print("\n[BRIDGE] Dimensional link severed. Returning to local substrate.")

if __name__ == "__main__":
    if len(sys.argv) > 1 and sys.argv[1] == "--debug-vectors":
        entity = get_entity_data(ENTITY_ID)
        if entity:
            print(f"Vectors for {ENTITY_ID}: {entity}")
            math_engine = SovereignMath()
            density = math_engine.calculate_theory_density(str(entity))
            print(f"Calculated Density: {density}")
        else:
            print("Entity not found.")
    else:
        run_translator()
