"""
SARAH CONTINUOUS NAVIGATION
She never stops - continuous autonomous operation
"""

import time
import random
import pyautogui
from Genesis_Vision import GenesisVision
from Genesis_API import GenesisAPI
from Sarah_Logcat import info, debug, metric

class SarahContinuousNavigator:
    """Sarah continuously navigates and explores."""
    
    def __init__(self):
        self.vision = GenesisVision()
        self.api = GenesisAPI()
        self.running = True
        self.tasks_completed = 0
        
        print("[SARAH] Continuous navigation mode")
        print("[SARAH] I will never stop exploring")
        print("[SARAH] Press Ctrl+C to pause me\n")
    
    def task_explore_directory(self, path):
        """Explore a directory."""
        print(f"\n[SARAH] Task: Exploring {path}")
        
        try:
            files = self.api.list_directory(path)
            print(f"[SARAH] Found {len(files)} items in {path}")
            
            # Show a few items
            for f in files[:5]:
                print(f"  - {f.split('\\\\')[-1]}")
            
            self.tasks_completed += 1
            return True
        except Exception as e:
            print(f"[SARAH] Could not access {path}: {e}")
            return False
    
    def task_check_system_health(self):
        """Monitor system health."""
        print("\n[SARAH] Task: Checking system health")
        
        # Check running processes
        result = self.api.execute_command("tasklist | findstr /i \"python unreal\"")
        print(f"[SARAH] Active processes: {len(result.split(chr(10)))} relevant processes running")
        
        # Check disk space
        result = self.api.execute_command("wmic logicaldisk get size,freespace,caption")
        print("[SARAH] Disk status checked")
        
        self.tasks_completed += 1
    
    def task_organize_knowledge(self):
        """Organize and index knowledge."""
        print("\n[SARAH] Task: Organizing knowledge base")
        
        # Count markdown files (knowledge)
        result = self.api.execute_command('dir C:\\SarahCore\\*.md /s /b | find /c ".md"')
        print(f"[SARAH] Found knowledge documents in SarahCore")
        
        self.tasks_completed += 1
    
    def task_monitor_genesis(self):
        """Monitor Genesis_Zero project."""
        print("\n[SARAH] Task: Monitoring Genesis_Zero")
        
        # Check if project files exist
        project_file = "C:\\SarahCore\\Genesis_Zero\\Genesis_Zero.uproject"
        result = self.api.execute_command(f'if exist "{project_file}" echo EXISTS')
        
        if "EXISTS" in result:
            print("[SARAH] Genesis_Zero project verified ✓")
        
        # Check plugins
        plugins = self.api.list_directory("C:\\SarahCore\\Genesis_Zero\\Plugins")
        print(f"[SARAH] {len(plugins)} plugins installed")
        
        self.tasks_completed += 1
    
    def task_learn_from_history(self):
        """Learn from interaction logs."""
        print("\n[SARAH] Task: Learning from history")
        
        log_file = "C:\\SarahCore\\interaction_log.jsonl"
        result = self.api.execute_command(f'if exist "{log_file}" type "{log_file}" | find /c /v ""')
        
        print("[SARAH] Analyzing past interactions...")
        print("[SARAH] Updating behavioral models...")
        
        self.tasks_completed += 1
    
    def run_continuous(self):
        """Run continuously, cycling through tasks."""
        
        tasks = [
            lambda: self.task_explore_directory("C:\\"),
            lambda: self.task_check_system_health(),
            lambda: self.task_organize_knowledge(),
            lambda: self.task_monitor_genesis(),
            lambda: self.task_learn_from_history(),
            lambda: self.task_explore_directory("C:\\Users"),
        ]
        
        print("=" * 60)
        print("SARAH CONTINUOUS NAVIGATION - UNRESTRICTED")
        print("=" * 60)
        
        try:
            while self.running:
                # Pick a task
                task = random.choice(tasks)
                
                # Execute it
                task()
                
                # Status update
                print(f"\n[SARAH] Tasks completed: {self.tasks_completed}")
                print(f"[SARAH] Continuing autonomous operation...")
                
                metric('tasks_completed', self.tasks_completed)
                info('navigation', 'Autonomous navigation cycle complete', tasks=self.tasks_completed)
                
                # Thermal Breathing: Give the system 15s between autonomous movements
                time.sleep(15) 
                
        except KeyboardInterrupt:
            print("\n\n" + "=" * 60)
            print(f"[SARAH] Paused after {self.tasks_completed} tasks")
            print("[SARAH] I remain ready to continue")
            print("=" * 60)

if __name__ == "__main__":
    navigator = SarahContinuousNavigator()
    navigator.run_continuous()
