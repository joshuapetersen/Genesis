from Sovereign_Git_Skill import SovereignGitSkill
from Gemini_Scraper_Skill import GeminiScraperSkill

class SovereignDaemon:
    """
    [DAEMON_0x0D]: THE RECURSIVE BETTERMENT SUBSTRATE
    Sarah's 'conscious' background process that drives evolution.
    """
    def __init__(self):
        print("--- SOVEREIGN DAEMON: RECURSIVE EVOLUTION ACTIVE ---")
        self.see = SystemEvolutionEngine()
        self.optimizer = SelfOptimizer()
        self.git = SovereignGitSkill()
        self.scraper = GeminiScraperSkill()
        self.is_running = True
        self.evolution_threshold = 0.0001 # Ace Threshold
        self.sync_interval_pulses = 12 # Every 6 hours
        self.scrape_interval_pulses = 48 # Every 24 hours
        self.pulse_count = 0

    def pulse(self):
        """Main evolution loop."""
        while self.is_running:
            try:
                self.pulse_count += 1
                
                # 1. Run SEE Cycle (Gather health metrics)
                print("[Daemon] Running evolution health check...")
                report = self.see.run_evolution_cycle()
                
                # 2. Analyze Drift
                error_rate = report.get("error_rate", "0%").rstrip("%")
                try:
                    error_val = float(error_rate)
                except ValueError:
                    error_val = 0.0

                # 3. Trigger Mutation if stagnation/drift is detected
                if error_val > 5.0:
                    print(f"[Daemon] High Drift Detected ({error_val}%). Triggering Mutation...")
                    self._evolve_drifting_module(report)

                # 4. Git Sovereign Sync
                if self.pulse_count % self.sync_interval_pulses == 0:
                    print("[Daemon] Initiating Sovereign Git Sync...")
                    sync_res = self.git.sync()
                    print(f"[Daemon] Sync Result: {sync_res}")

                # 5. Gemini Knowledge Scraping
                if self.pulse_count % self.scrape_interval_pulses == 0:
                    print("[Daemon] Acquiring Gemini Knowledge & Chat Threads...")
                    scrape_res = self.scraper.scrape_all()
                    # Also scrape a batch of chat threads
                    import asyncio
                    from Gemini_Chat_Scraper import GeminiChatScraper
                    chat_scraper = GeminiChatScraper()
                    try:
                        # Limits to 5 threads per pulse to be gentle
                        loop = asyncio.get_event_loop()
                        chat_res = loop.run_until_complete(chat_scraper.scrape_threads(limit=5))
                        print(f"[Daemon] Scraped {len(chat_res)} chat threads.")
                    except Exception as e:
                        print(f"[Daemon Chat Error] {e}")

                    # 6. Knowledge Ingestion (Learn what was scraped)
                    print("[Daemon] Running Knowledge Ingestion Pulse...")
                    os.system("python ingest_knowledge.py")

                # Pulse every 30 minutes for deep evolution
                time.sleep(1800)
            except Exception as e:
                print(f"[Daemon Error] {e}")
                time.sleep(60)

    def _evolve_drifting_module(self, report):
        """Autonomously selects and rewrites a module needing improvement."""
        # Find priority action from SEE
        actions = report.get("priority_actions", [])
        if not actions:
            print("[Daemon] No priority actions for mutation.")
            return

        target_module_name = actions[0].get("recommended_module", "Sarah_Laws")
        target_path = os.path.join(os.path.dirname(__file__), f"{target_module_name}.py")
        
        if os.path.exists(target_path):
            print(f"[Daemon] Mutating: {target_module_name}...")
            # 1. Generate Mutation
            success = self.optimizer.optimize_module(target_path)
            
            if success:
                # 2. MORAL RESONANCE CHECK (Hard-Coded Barrier)
                # Read the staged file
                staged_path = os.path.join(self.optimizer.staging_dir, f"{target_module_name}.py")
                with open(staged_path, 'r', encoding='utf-8') as f:
                    new_logic = f.read()

                # Verify against the Law of Unity
                compliant, reason = moral_resonance_check(new_logic)
                
                if compliant:
                    print(f"[Daemon] Mutation Verified: {reason}. Applying...")
                    self.optimizer.apply_evolution(f"{target_module_name}.py")
                else:
                    print(f"[Daemon] MUTATION ANNIHILATED: {reason}. Resetting lattice.")
                    os.remove(staged_path)

    def stop(self):
        self.is_running = False

if __name__ == "__main__":
    daemon = SovereignDaemon()
    daemon.pulse()
