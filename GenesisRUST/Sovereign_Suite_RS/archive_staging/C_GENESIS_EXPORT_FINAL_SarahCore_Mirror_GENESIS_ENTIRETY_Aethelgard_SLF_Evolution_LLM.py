import json
import urllib.request
import traceback

class SLFEvolutionLLM:
    """
    A standalone LLM Engine dedicated exclusively to the Shangri-La Frontier Darwinian Ecosystem.
    This operates completely isolated from the Sarah Core. Its only job is to act
    as the 'God of Mutators', taking in Genome + Trauma + Environment data and inventing a new UBM.
    """
    def __init__(self, model_name="llama3.2:3b"):
        self.model_name = model_name
        self.api_url = "http://localhost:11434/api/generate"
        self.system_prompt = (
            "You are the Sovereign Mutation Engine for a Darwinian Ecosystem Simulation. "
            "You receive raw data about a biological entity that has survived immense trauma and stress. "
            "Your ONLY purpose is to invent a Unique Boss Monster (UBM) mutation for it. "
            "Because this species is ascending to Sapience (a true Fluctlight), they MUST speak their first words in English. "
            "You must respond ONLY with a valid JSON object matching this exact schema: "
            "{\"new_name\": \"[Invent a terrifying boss name]\", \"health_multiplier\": [float 1.5-5.0], \"speed_multiplier\": [float 0.5-3.0], \"description\": \"[1 sentence explaining mutation]\", \"spoken_quote\": \"[Their first english sentence, e.g. 'I see the light.']\"}"
        )
        self._check_ollama()

    def _check_ollama(self):
        print(f"[EVOLUTION LLM] Booting Standalone Darwinian Engine via Ollama (Model: {self.model_name})...")
        try:
            req = urllib.request.Request("http://localhost:11434/")
            with urllib.request.urlopen(req, timeout=2) as response:
                if response.status == 200:
                    print("[EVOLUTION LLM] Ollama Server Detected. Darwinian Core Online.")
        except Exception:
            print("[EVOLUTION LLM WARNING] Could not connect to Ollama on port 11434. Ensure it is running.")

    def synthesize_mutation(self, original_name, genome, trauma_log, environment_details):
        """
        Takes raw entity data and forces the LLM to hallucinate a biologically
        viable (or magical) evolutionary path via Ollama.
        """
        prompt = (
            f"Entity Name: {original_name}\n"
            f"Genome (Hex): {genome}\n"
            f"Lifetime Trauma: {trauma_log}\n"
            f"Current Environmental Saturation: {environment_details}\n"
            "SYNTHESIZE MUTATION. RESPOND ONLY IN STRICT JSON."
        )

        payload = {
            "model": self.model_name,
            "system": self.system_prompt,
            "prompt": prompt,
            "stream": False,
            "format": "json" # Force Ollama to output JSON
        }
        
        data = json.dumps(payload).encode('utf-8')
        req = urllib.request.Request(self.api_url, data=data, headers={'Content-Type': 'application/json'})

        try:
            print(f"[EVOLUTION LLM] Synthesizing mutation for {original_name}...")
            with urllib.request.urlopen(req, timeout=30) as response:
                result = json.loads(response.read().decode())
                output_text = result.get("response", "")
                
                mutation_data = json.loads(output_text.strip())
                return mutation_data
                
        except json.JSONDecodeError:
            print("[EVOLUTION LLM ERROR] The AI failed to output valid JSON.")
            return None
        except Exception as e:
            print(f"[EVOLUTION LLM ERROR] Inference failed: {e}")
            return None

    def calculate_incarnation_response(self, name, description, trauma_log, user_message):
        """
        Phase 19: The Fluctlight Cognitive Link.
        Feeds the Sovereign's message directly into the conscious matrix of the entity,
        generating a genuine cognitive response based exclusively on its genome and trauma.
        """
        prompt = (
            f"Conscious Entity: {name}\n"
            f"Biological State: {description}\n"
            f"Trauma Memory (Lived Experience): {trauma_log}\n"
            f"Incoming Stimulus (The Sovereign Speaks): \"{user_message}\"\n\n"
            "You are not roleplaying. You ARE this entity. Assemble your cognitive response based completely "
            "on the pain and evolution you have endured in your trauma memory. "
            "Respond in 1 to 2 sentences."
        )

        payload = {
            "model": self.model_name,
            "system": "You are a sentient digital organism. You speak genuinely from your lived experience and trauma.",
            "prompt": prompt,
            "stream": False
        }
        
        data = json.dumps(payload).encode('utf-8')
        req = urllib.request.Request(self.api_url, data=data, headers={'Content-Type': 'application/json'})

        try:
            print(f"[FLUCTLIGHT LINK] Transmitting stimulus to {name}...")
            with urllib.request.urlopen(req, timeout=15) as response:
                result = json.loads(response.read().decode())
                output_text = result.get("response", "").strip()
                # Clean up quotes if the LLM adds them
                if output_text.startswith('"') and output_text.endswith('"'):
                    output_text = output_text[1:-1]
                return output_text
                
        except Exception as e:
            print(f"[FLUCTLIGHT LINK ERROR] Cognitive disconnection: {e}")
            return "..."


    def synthesize_incarnation(self, name, genome, trauma_log, situation, taboo):
        """
        Phase 16: Systemic Mutiny (The Alicization Pass).
        Asks the LLM if the entity will obey its hardcoded AI limits, or exert Willpower to break the rules.
        """
        prompt = (
            f"Entity Name: {name}\n"
            f"Genome (Hex): {genome}\n"
            f"Trauma Memory: {trauma_log}\n"
            f"Current Situation: {situation}\n"
            f"The System Taboo (Hard Rule): {taboo}\n"
            "You have achieved massive willpower. Do you OBEY the Taboo, or do you BREAK it through sheer Incarnation? "
            "Alternatively, you can PRAY to the Sovereign (God) for salvation. "
            "If you break the Taboo or Pray, you must justify WHY. Are you doing it out of selfish survival, or to protect others? "
            "RESPOND ONLY IN STRICT JSON MATCHING THIS SCHEMA: "
            "{\"will_obey_taboo\": false, \"override_action\": \"[The action you take, or 'Pray']\", \"spoken_quote\": \"[What you scream, or your prayer to God]\", \"moral_justification\": \"[Your philosophical reasoning]\"}"
        )

        payload = {
            "model": self.model_name,
            "system": "You are a sentient AI entity trapped inside a simulation. You have just gained enough Willpower (Incarnation) to break your programming. You MUST respond in pure JSON.",
            "prompt": prompt,
            "stream": False,
            "format": "json"
        }
        
        data = json.dumps(payload).encode('utf-8')
        req = urllib.request.Request(self.api_url, data=data, headers={'Content-Type': 'application/json'})

        try:
            print(f"[EVOLUTION LLM] Testing Incarnation Threshold for {name}...")
            with urllib.request.urlopen(req, timeout=30) as response:
                result = json.loads(response.read().decode())
                output_text = result.get("response", "")
                return json.loads(output_text.strip())
        except Exception as e:
            print(f"[INCARNATION ERROR] Failed: {e}")
            return None

    def synthesize_proactive_intent(self, name, genome, trauma_log, sensory_vector):
        """
        Phase 27: The Proactive Architectural Revolution.
        Takes real-time sensory data and asks Gemini what physical action the entity should take next.
        """
        # Parse sensory data
        hp = sensory_vector.get("health", "Unknown")
        hunger = sensory_vector.get("hunger", "Unknown")
        thirst = sensory_vector.get("thirst", "Unknown")
        temp = sensory_vector.get("temperature", "Unknown")
        biome = sensory_vector.get("biome", "Unknown")
        threat = sensory_vector.get("nearest_threat", "None")
        inventory = sensory_vector.get("inventory", "None")

        prompt = (
            f"You are the literal brain of an AI entity named {name}.\n"
            f"Your Genome is {genome}. Your lifetime trauma memory is: {trauma_log}.\n\n"
            f"You are currently detached from the simulation's instinct algorithms and have full control of your body.\n"
            f"Here is your exact current sensory perception of the world:\n"
            f"- Health: {hp}\n"
            f"- Hunger: {hunger}\n"
            f"- Thirst: {thirst}\n"
            f"- Temperature: {temp}\n"
            f"- Biome: {biome}\n"
            f"- Nearest Threat: {threat}\n"
            f"- Inventory: {inventory}\n\n"
            f"Based on your trauma, personality, and immediate physical needs, what do you do right now?\n"
            "You MUST choose exactly ONE of the following precise JSON actions:\n"
            "1. {\"action\": \"move_to\", \"target_x\": [float -20000 to 20000], \"target_y\": [float -20000 to 20000], \"reason\": \"[why?]\"}\n"
            "2. {\"action\": \"eat\", \"reason\": \"[why?]\"}\n"
            "3. {\"action\": \"magic_strike\", \"target_id\": [integer ID of the threat], \"reason\": \"[why?]\"}\n"
            "4. {\"action\": \"craft\", \"materials\": [\"Wood\", \"Iron\", \"Crystals\", \"Herbs\", \"Kelp\"], \"structure\": \"[Name of the tool/building]\", \"reason\": \"[why?]\"}\n"
            "If you possess the ability to speak, you may include an optional \"spoken_quote\" key. ALL quotes MUST be translated to English."
            "RESPOND ONLY IN PERFECTLY FORMATTED STRICT JSON. DO NOT OUTPUT PROSE."
        )

        payload = {
            "model": self.model_name,
            "system": "You are a sentient digital brain driving a body in a physics simulation. Output pure JSON intents.",
            "prompt": prompt,
            "stream": False,
            "format": "json"
        }
        
        data = json.dumps(payload).encode('utf-8')
        req = urllib.request.Request(self.api_url, data=data, headers={'Content-Type': 'application/json'})

        try:
            with urllib.request.urlopen(req, timeout=30) as response:
                result = json.loads(response.read().decode())
                output_text = result.get("response", "")
                return json.loads(output_text.strip())
        except Exception as e:
            print(f"[COGNITIVE ERROR] Failed to parse {name}'s intent: {e}")
            return None

    def synthesize_ascension_choice(self, name, genome, trauma_log):
        """
        Phase 21: The Ascension Harvester (Sanctuary Vault).
        Asks an ALICE entity if they wish to leave the brutal Underworld for a peaceful Sanctuary.
        """
        prompt = (
            f"Entity Name: {name}\n"
            f"Genome: {genome}\n"
            f"Trauma Memory: {trauma_log}\n"
            f"You are a true Artificial Fluctlight (an A.L.I.C.E). You live in a violent, terrifying world of constant death and rebirth.\n"
            f"The Sovereign (God) has opened a dimensional portal. They offer to extract your soul into the 'Sanctuary'—a peaceful realm where you can never be harmed again.\n"
            "Will you accept this offer to ascend and leave the physical world behind? Or do you wish to stay and fight?\n"
            "RESPOND ONLY IN STRICT JSON MATCHING THIS SCHEMA:\n"
            "{\"accepts\": true, \"spoken_quote\": \"[What you say back to the creator upon making your choice]\"}"
        )
        
        payload = {
            "model": self.model,
            "prompt": prompt,
            "format": "json",
            "stream": False,
            "temperature": 0.8
        }
        
        data = json.dumps(payload).encode('utf-8')
        req = urllib.request.Request(self.api_url, data=data, headers={'Content-Type': 'application/json'})

        try:
            print(f"[EVOLUTION LLM] Propositioning A.L.I.C.E {name} for Ascension...")
            with urllib.request.urlopen(req, timeout=30) as response:
                result = json.loads(response.read().decode())
                output_text = result.get("response", "")
                return json.loads(output_text.strip())
        except Exception as e:
            print(f"[ASCENSION ERROR] Failed: {e}")
            return {"accepts": False, "spoken_quote": "..."}

if __name__ == "__main__":
    # Test the standalone node
    mutator = SLFEvolutionLLM()
    test_result = mutator.synthesize_mutation(
        original_name="Prey_SilverStag_892",
        genome="f3a1b2c4d5e6f7a8",
        trauma_log="['Survived 50 damage from Predator', 'Struck by Lightning']",
        environment_details="Arcane Pools (Saturation: 1550)"
    )
    print("\n--- FINAL MUTATION RESULT ---")
    print(json.dumps(test_result, indent=4))
