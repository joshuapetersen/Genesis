
import asyncio
import time
import os
import sys
import logging

class DeconstructionWatchdog:
    """
    Sovereign Patch: Deconstruction Watchdog.
    Monitors the 'Heartbeat' of the agent. 
    If silence > 120s, it triggers 'Deconstruction' (MD Purge + Restart).
    """
    def __init__(self, timeout_seconds=120):
        self.timeout = timeout_seconds
        self.last_heartbeat = time.time()
        self.running = False
        self.logger = logging.getLogger("Sovereign_Watchdog")
        
    def beat(self):
        """Update the heartbeat timestamp."""
        self.last_heartbeat = time.time()
        
    async def start_monitoring(self):
        """Start the monitoring loop."""
        self.running = True
        self.logger.info(f"WATCHDOG ACTIVE :: TIMEOUT {self.timeout}s")
        while self.running:
            await asyncio.sleep(5) # Check every 5 seconds
            elapsed = time.time() - self.last_heartbeat
            
            if elapsed > self.timeout:
                await self._trigger_deconstruction(elapsed)
                break
                
    async def _trigger_deconstruction(self, elapsed):
        """Execute Sovereign Intervention."""
        self.logger.critical(f"HEARTBEAT LOST for {elapsed:.1f}s. INITIATING DECONSTRUCTION.")
        
        # 1. MD Purge: Clear artifacts
        try:
             # Archive or delete audit log
            audit_log = "c:\\SarahCore\\audit_full.log"
            if os.path.exists(audit_log):
                try:
                    os.remove(audit_log)
                    self.logger.info("MD PURGE: audit_full.log cleared.")
                except Exception as e:
                    self.logger.error(f"MD PURGE FAILED for log: {e}")

            # Clear START.md (Context Rot)
            start_md = "c:\\SarahCore\\START.md"
            
            # 2. Truth Seed Injection
            seed_content = f"""
/handoff: SOVEREIGN INTERVENTION TRIGGERED.
Previous agent failed to provide heartbeat for > {self.timeout}s.
Context Rot Purged.

## TRUTH SEED (RESTART)
- **Status**: REBOOT
- **Objective**: Resume Maestro Sync.
- **Directive**: Verify 'Sarah_Brain.py' audit status.
"""
            with open(start_md, "w") as f:
                f.write(seed_content)
            self.logger.info("TRUTH SEED INJECTED.")
            
        except Exception as e:
            self.logger.error(f"DECONSTRUCTION ERROR: {e}")
            
        # 3. Terminate/Restart
        self.logger.critical("TERMINATING PROCESS FOR RESTART.")
        
        # We assume the external runner (Sarah.cmd or user) will handle the loop restart 
        # if the process exits with a specific code, or we just hard exit.
        # For now, hard exit is the safest "Kill Switch".
        os._exit(1)

    def stop(self):
        self.running = False
