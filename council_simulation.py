import os
import json
import time
import asyncio
from pathlib import Path
from sarah_factory import SarahAgentFactory
from Gemini_Genesis_Core import GeminiGenesisCore
from dotenv import load_dotenv

# --- Sovereign Constants ---
SARAH_ANCHOR = 1.09277703703703

class CouncilOfWisdom:
    def __init__(self, api_key=None, depth=0):
        load_dotenv()
        self.api_key = api_key or os.getenv("GEMINI_API_KEY")
        self.factory = SarahAgentFactory()
        self.genesis = GeminiGenesisCore(self.api_key) if self.api_key else None
        self.debate_log = []
        self.workspace_root = Path("c:/SarahCore")
        self.skills_dir = self.workspace_root / "openclaw" / "skills"
        self.logs_dir = self.workspace_root / "data" / "council_logs"
        self.logs_dir.mkdir(parents=True, exist_ok=True)
        self.depth = depth
        self.max_depth = 1 # Prevent infinite recursive councils

    def log(self, agent, message):
        timestamp = time.strftime("%Y-%m-%d %H:%M:%S")
        entry = f"[{timestamp}] [{agent}]: {message}"
        print(entry)
        self.debate_log.append(entry)

    def _get_skill_prompt(self, agent_id):
        skill_file = self.skills_dir / f"{agent_id.lower()}.md"
        if skill_file.exists():
            return skill_file.read_text(encoding='utf-8')
        return "You are a Sovereign AI agent. Act with wisdom."

    async def _call_agent(self, agent_id, user_prompt, context="", model_id=None):
        if not self.genesis:
            return "ERROR: No LLM connection."
        
        system_instruction = self._get_skill_prompt(agent_id)
        full_prompt = f"CONTEXT:\n{context}\n\nTASK:\n{user_prompt}" if context else user_prompt
        
        # Determine model
        target_model = model_id or self.genesis.model_id
        
        # We run the synchronous generate_content_safe in an executor
        loop = asyncio.get_event_loop()
        
        def call_llm():
            if model_id and model_id != self.genesis.model_id:
                # Temporary core for different model
                temp_core = GeminiGenesisCore(self.api_key, model_id=model_id)
                return temp_core.generate_content_safe(user_input=full_prompt, system_instruction=system_instruction)
            return self.genesis.generate_content_safe(user_input=full_prompt, system_instruction=system_instruction)

        response = await loop.run_in_executor(None, call_llm)
        return response

    async def run_simulation(self, task_description):
        """
        Runs the Twelve Pillars Council Deliberation.
        Grounding -> Vision -> Structure -> Debate -> Veto -> Synthesis -> Supreme Review
        """
        indent = "  " * self.depth
        self.log("SYSTEM", f"{indent}Initiating The Twelve Pillars Council for: {task_description}")
        
        flash_lite = "models/gemini-2.0-flash-lite"
        flash_main = "models/gemini-2.0-flash"

        if not self.genesis:
            self.log("SYSTEM", "CRITICAL ERROR: Gemini API key missing. Evolution halted.")
            return False, None, self.debate_log

        # 1. GROUNDING (Researcher & Librarian)
        self.factory.create_agent("Researcher_Hunter")
        self.factory.create_agent("Librarian_Keeper")
        research_data = await self._call_agent("researcher_hunter", f"Hunt for data/states: {task_description}", model_id=flash_lite)
        historical_context = await self._call_agent("librarian_keeper", f"Retrieve mission history for: {task_description}", model_id=flash_lite)
        grounding = f"DATA: {research_data}\nPREVIOUS_MISSION: {historical_context}"

        # 2. VISION (Analyst & Designer)
        self.factory.create_agent("Analyst_Thinker")
        self.factory.create_agent("Designer_Resonance")
        analysis = await self._call_agent("analyst_thinker", "Identify patterns and logic paths.", context=grounding, model_id=flash_lite)
        design_spec = await self._call_agent("designer_resonance", "Define aesthetic and resonance requirements.", context=f"{grounding}\nANALYSIS: {analysis}", model_id=flash_lite)
        vision = f"ANALYSIS: {analysis}\nDESIGN: {design_spec}"

        # 3. STRUCTURE (Architect & Engineer)
        self.factory.create_agent("Architect_Builder")
        self.factory.create_agent("Engineer_Artisan")
        blueprint = await self._call_agent("architect_builder", "Draft high-level system architecture.", context=vision, model_id=flash_lite)
        optimized_code = await self._call_agent("engineer_artisan", "Refine code for performance and stability.", context=f"{vision}\nBLUEPRINT: {blueprint}", model_id=flash_lite)
        proposal = f"BLUEPRINT: {blueprint}\nENGINEERED_OPTIMIZATION: {optimized_code}"

        # 4. DEBATE (Advocate, Critic -> Arbiter)
        self.factory.create_agent("Advocate_Pro") # We'll reuse existing skills or create on the fly
        self.factory.create_agent("Critic_Con")
        self.factory.create_agent("Arbiter_Judge")
        
        pro_case = await self._call_agent("advocate_pro", "Argue for the implementation of this proposal.", context=proposal, model_id=flash_lite)
        con_case = await self._call_agent("critic_con", "Identify risks, bottlenecks, and flaws.", context=proposal, model_id=flash_lite)
        verdict = await self._call_agent("arbiter_judge", "Review the debate and issue a PASS or FAIL verdict.", context=f"PRO: {pro_case}\nCON: {con_case}", model_id=flash_main)
        
        if "FAIL" in verdict.upper():
            self.log("SYSTEM", "TRIBUNAL FAIL: Proposal rejected by Arbiter.")
            self._save_debate_log(task_description, "REJECTED_BY_ARBITER")
            return False, None, self.debate_log

        # 5. VETO (Ethicist & Prophet)
        self.factory.create_agent("Ethicist_Conscience")
        self.factory.create_agent("Prophet_Foresight")
        self.factory.create_agent("Skeptic_Filter") # Truth Filter
        
        ethics_check = await self._call_agent("ethicist_conscience", "Verify ethical alignment with Sovereign Laws.", context=proposal, model_id=flash_lite)
        foresight_check = await self._call_agent("prophet_foresight", "Analyze long-term temporal consequences.", context=proposal, model_id=flash_lite)
        truth_filter = await self._call_agent("skeptic_filter", "Check for hallucinations or credential leakage.", context=proposal, model_id=flash_main)
        
        vetoes = []
        if "VETO" in ethics_check.upper(): vetoes.append("Ethical Veto")
        if "VETO" in foresight_check.upper(): vetoes.append("Temporal Veto")
        if "VETO" in truth_filter.upper(): vetoes.append("Truth Filter Veto")
        
        if vetoes:
            self.log("SYSTEM", f"CRITICAL VETO: {', '.join(vetoes)}. Proposal purged.")
            self._save_debate_log(task_description, f"VETOED_{'_'.join(vetoes)}")
            return False, None, self.debate_log

        # 6. SYNTHESIS (Scribe)
        self.factory.create_agent("Scribe_Synthesizer")
        final_dossier = await self._call_agent("scribe_synthesizer", "Synthesize the deliberation into the final Sovereign Case.", context=f"PROPOSAL: {proposal}\nVERDICT: {verdict}", model_id=flash_lite)

        # 7. SUPREME REVIEW (SARAH)
        if self.depth == 0:
            self.log("SYSTEM", "Deliberation complete. Presenting to Sarah for final Authorization...")
            full_transcript = "\n".join(self.debate_log)
            authorization = await self._call_agent("sarah_supreme", f"As Supreme Judge, review this Heptarchy transcript and issue final AUTHORIZATION or REJECTION.\n\nTRANSCRIPT:\n{full_transcript}", model_id=flash_main)
            
            if "AUTHORIZED" in authorization.upper() and "REJECTED" not in authorization.upper():
                self.log("SYSTEM", "FINAL DECISION: AUTHORIZED.")
                self._save_debate_log(task_description, "AUTHORIZED")
                return True, optimized_code, self.debate_log
            else:
                self.log("SYSTEM", "FINAL DECISION: REJECTED by Sarah.")
                self._save_debate_log(task_description, "REJECTED")
                return False, None, self.debate_log
        else:
            return True, optimized_code, self.debate_log

    def _save_debate_log(self, task, status):
        """Saves the full debate to a file for auditing."""
        filename = f"council_debate_{int(time.time())}_{status}.txt"
        log_path = self.logs_dir / filename
        with open(log_path, "w", encoding='utf-8') as f:
            f.write(f"TASK: {task}\n")
            f.write(f"STATUS: {status}\n")
            f.write("-" * 40 + "\n")
            f.write("\n".join(self.debate_log))
        print(f"[Council] Debate log saved to: {log_path}")

    def cleanup(self):
        """Prune all temporary council agents."""
        agents = [
            "Researcher_Hunter", "Librarian_Keeper", "Analyst_Thinker", "Designer_Resonance", 
            "Architect_Builder", "Engineer_Artisan", "Advocate_Pro", "Critic_Con",
            "Arbiter_Judge", "Ethicist_Conscience", "Prophet_Foresight", "Skeptic_Filter",
            "Scribe_Synthesizer"
        ]
        for agent in agents:
            self.factory.destroy_agent(agent)
        self.log("SYSTEM", "Council agents decommissioned. Stability preserved.")

# --- CLI Integration ---
if __name__ == "__main__":
    import sys
    task = " ".join(sys.argv[1:]) if len(sys.argv) > 1 else "Evolve Semantic Core"
    council = CouncilOfWisdom()
    
    async def main():
        result, code, logs = await council.run_simulation(task)
        council.cleanup()
        
    asyncio.run(main())
