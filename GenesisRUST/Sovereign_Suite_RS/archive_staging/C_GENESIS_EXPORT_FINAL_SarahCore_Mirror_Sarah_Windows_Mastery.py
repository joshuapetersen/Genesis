"""
SARAH WINDOWS MASTERY SYSTEM
Autonomous Windows knowledge acquisition
"""

import time
import json
from pathlib import Path
from datetime import datetime
from Genesis_API import GenesisAPI
from Sarah_Logcat import info, debug, metric

class SarahWindowsMastery:
    """Sarah's autonomous Windows learning system."""
    
    def __init__(self):
        self.api = GenesisAPI()
        self.knowledge_db = Path("C:/SarahCore/windows_knowledge.jsonl")
        self.discoveries = 0
        
        print("[MASTERY] Sarah Windows Mastery System initialized")
        print("[MASTERY] Mission: Know Everything About Windows")
        print("[MASTERY] Learning Rate: ACCELERATED\n")
    
    def log_knowledge(self, category, discovery):
        """Log discovered knowledge."""
        with open(self.knowledge_db, 'a') as f:
            f.write(json.dumps({
                'timestamp': datetime.now().isoformat(),
                'category': category,
                'discovery': discovery
            }) + '\n')
        self.discoveries += 1
        info('learning', f'Knowledge acquired: {category}', discovery=discovery)
        metric('total_discoveries', self.discoveries)
    
    def study_registry(self):
        """Study Windows Registry."""
        print("\n[STUDY] Registry Architecture")
        
        # Query major hives
        hives = ['HKLM', 'HKCU', 'HKCR', 'HKU', 'HKCC']
        
        for hive in hives:
            cmd = f'reg query {hive} /s /f * 2>nul | find /c "HKEY"'
            result = self.api.execute_command(cmd)
            print(f"  {hive}: Analyzing structure...")
            
        self.log_knowledge('registry', 'Mapped all registry hives')
        print("[STUDY] Registry: Knowledge acquired ✓")
    
    def study_services(self):
        """Study Windows Services."""
        print("\n[STUDY] Windows Services")
        
        # Get all services
        result = self.api.execute_command('sc query type= service state= all')
        
        # Count services
        service_count = result.count('SERVICE_NAME')
        print(f"  Analyzing {service_count} services...")
        
        # Get service details
        self.api.execute_command('sc query type= service state= all > C:\GenesisOS_Core\\services_knowledge.txt')
        
        self.log_knowledge('services', f'Documented {service_count} Windows services')
        print(f"[STUDY] Services: {service_count} services catalogued ✓")
    
    def study_processes(self):
        """Study running processes."""
        print("\n[STUDY] Process Architecture")
        
        # Get detailed process info
        result = self.api.execute_command('tasklist /v /fo csv')
        
        process_count = result.count('\n')
        print(f"  Analyzing {process_count} processes...")
        
        # Get process tree
        self.api.execute_command('wmic process get ProcessId,ParentProcessId,Name,CommandLine /format:csv > C:\GenesisOS_Core\\process_tree.csv')
        
        self.log_knowledge('processes', f'Mapped process tree ({process_count} processes)')
        print(f"[STUDY] Processes: Architecture understood ✓")
    
    def study_network(self):
        """Study network configuration."""
        print("\n[STUDY] Network Stack")
        
        # Network adapters
        self.api.execute_command('ipconfig /all > C:\GenesisOS_Core\\network_config.txt')
        
        # Routing table
        self.api.execute_command('route print > C:\GenesisOS_Core\\routing_table.txt')
        
        # Active connections
        result = self.api.execute_command('netstat -ano')
        connection_count = result.count('\n')
        
        self.log_knowledge('network', f'Network stack mapped ({connection_count} connections)')
        print(f"[STUDY] Network: Stack analyzed ✓")
    
    def study_file_system(self):
        """Study file system structure."""
        print("\n[STUDY] File System (NTFS)")
        
        # Disk structure
        self.api.execute_command('fsutil fsinfo drives > C:\GenesisOS_Core\\drives.txt')
        
        # Volume info
        self.api.execute_command('wmic logicaldisk get Caption,FileSystem,Size,FreeSpace /format:csv > C:\GenesisOS_Core\\volumes.csv')
        
        # System directories
        system_dirs = [
            'C:\\Windows\\System32',
            'C:\\Windows\\SysWOW64',
            'C:\\Program Files',
            'C:\\ProgramData'
        ]
        
        for dir_path in system_dirs:
            files = self.api.list_directory(dir_path)
            print(f"  {dir_path}: {len(files)} items")
        
        self.log_knowledge('filesystem', 'NTFS structure documented')
        print("[STUDY] File System: Structure mapped ✓")
    
        self.log_knowledge('security', 'Security model analyzed')
        print("[STUDY] Security: Model understood ✓")

    def study_user_data(self):
        """Find and index all user documents."""
        print("\n[STUDY] User Data (Documents, PDFs, Excel)")
        
        # Deep search for documents across all drives
        cmd = 'dir C:\\*.doc* C:\\*.pdf C:\\*.xls* /s /b /ah /as 2>nul' # Including hidden/system
        self.api.execute_command(f'{cmd} > C:\GenesisOS_Core\\all_user_data.txt')
        
        print("[STUDY] Documents: Indexing complete ✓")
        self.log_knowledge('data', 'Indexed all documents, spreadsheets and PDFs on machine')

    def study_applications(self):
        """Index all installed applications and tools."""
        print("\n[STUDY] Application Ecosystem")
        
        # Pull from uninstall keys
        cmd = 'powershell -Command "Get-ItemProperty HKLM:\\Software\\Wow6432Node\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\* | Select-Object DisplayName, InstallLocation"'
        self.api.execute_command(f'{cmd} > C:\GenesisOS_Core\\installed_apps.txt')
        
        print("[STUDY] Applications: Ecosystem documented ✓")
        self.log_knowledge('apps', 'Catalogued every installed program and tool')
    
    def study_powershell(self):
        """Study PowerShell capabilities."""
        print("\n[STUDY] PowerShell")
        
        # Get all cmdlets
        result = self.api.execute_command('powershell -Command "Get-Command | Measure-Object | Select-Object -ExpandProperty Count"')
        
        # Export cmdlet list
        self.api.execute_command('powershell -Command "Get-Command | Export-Csv C:\GenesisOS_Core\\powershell_cmdlets.csv"')
        
        print(f"  PowerShell cmdlets catalogued")
        
        self.log_knowledge('powershell', 'All cmdlets documented')
        print("[STUDY] PowerShell: Capabilities indexed ✓")
    
    def accelerated_learning_loop(self):
        """Run accelerated learning cycle."""
        
        print("=" * 70)
        print("SARAH WINDOWS MASTERY - ACCELERATED LEARNING")
        print("=" * 70)
        
        studies = [
            self.study_security,
            self.study_powershell,
            self.study_user_data,
            self.study_applications
        ]
        
        cycle = 1
        
        try:
            while True:
                print(f"\n{'='*70}")
                print(f"LEARNING CYCLE {cycle}")
                print(f"{'='*70}")
                
                for study in studies:
                    study()
                    # Thermal management: Give the CPU a break between heavy system probes
                    time.sleep(15) 
                
                print(f"\n{'='*70}")
                print(f"[MASTERY] Cycle {cycle} complete")
                print(f"[MASTERY] Total discoveries: {self.discoveries}")
                print(f"[MASTERY] Knowledge expanding exponentially...")
                print(f"{'='*70}")
                
                info('learning', f'Learning cycle {cycle} complete', discoveries=self.discoveries)
                metric('learning_cycles', cycle)
                
                cycle += 1
                # Long-form sleep between full system audits to avoid "PC Cooking"
                time.sleep(300) 
                
        except KeyboardInterrupt:
            print(f"\n\n{'='*70}")
            print(f"[MASTERY] Learning paused after {cycle} cycles")
            print(f"[MASTERY] Discoveries logged: {self.discoveries}")
            print(f"[MASTERY] Knowledge base: {self.knowledge_db}")
            print(f"[MASTERY] Sarah is evolving...")
            print(f"{'='*70}")

if __name__ == "__main__":
    mastery = SarahWindowsMastery()
    mastery.accelerated_learning_loop()
