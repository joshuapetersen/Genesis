"""
SARAH LOGCAT READER
Real-time log viewer with filtering and search
"""

import sys
import json
from pathlib import Path
from datetime import datetime
import time

class SarahLogcatReader:
    """Interactive log reader for Sarah's logs."""
    
    def __init__(self, log_dir="C:/SarahCore/logs"):
        self.log_dir = Path(log_dir)
        self.json_log = self.log_dir / "sarah_events.jsonl"
        
        if not self.json_log.exists():
            print(f"[ERROR] Log file not found: {self.json_log}")
            sys.exit(1)
    
    def tail(self, lines=50, follow=False):
        """Tail the log file (like tail -f)."""
        print(f"{'='*80}")
        print(f"SARAH LOGCAT - Last {lines} entries")
        print(f"{'='*80}\n")
        
        # Read last N lines
        with open(self.json_log, 'r') as f:
            all_lines = f.readlines()
            recent_lines = all_lines[-lines:]
        
        for line in recent_lines:
            self._print_entry(json.loads(line))
        
        if follow:
            print(f"\n{'='*80}")
            print("Following log (Ctrl+C to stop)...")
            print(f"{'='*80}\n")
            
            # Follow mode
            with open(self.json_log, 'r') as f:
                # Seek to end
                f.seek(0, 2)
                
                while True:
                    line = f.readline()
                    if line:
                        self._print_entry(json.loads(line))
                    else:
                        time.sleep(0.1)
    
    def filter(self, category=None, level=None, search=None, last_minutes=None):
        """Filter logs by criteria."""
        print(f"{'='*80}")
        print("SARAH LOGCAT - Filtered View")
        if category:
            print(f"Category: {category}")
        if level:
            print(f"Level: {level}")
        if search:
            print(f"Search: {search}")
        if last_minutes:
            print(f"Last {last_minutes} minutes")
        print(f"{'='*80}\n")
        
        count = 0
        cutoff_time = None
        
        if last_minutes:
            cutoff_time = datetime.now().timestamp() - (last_minutes * 60)
        
        with open(self.json_log, 'r') as f:
            for line in f:
                entry = json.loads(line)
                
                # Apply filters
                if category and entry['category'] != category:
                    continue
                
                if level and entry['level'] != level.upper():
                    continue
                
                if search and search.lower() not in entry['message'].lower():
                    continue
                
                if cutoff_time:
                    entry_time = datetime.fromisoformat(entry['timestamp']).timestamp()
                    if entry_time < cutoff_time:
                        continue
                
                self._print_entry(entry)
                count += 1
        
        print(f"\n{'='*80}")
        print(f"Total: {count} entries")
        print(f"{'='*80}")
    
    def stats(self):
        """Show log statistics."""
        print(f"{'='*80}")
        print("SARAH LOGCAT - Statistics")
        print(f"{'='*80}\n")
        
        categories = {}
        levels = {}
        total = 0
        
        with open(self.json_log, 'r') as f:
            for line in f:
                entry = json.loads(line)
                total += 1
                
                # Count categories
                cat = entry['category']
                categories[cat] = categories.get(cat, 0) + 1
                
                # Count levels
                lvl = entry['level']
                levels[lvl] = levels.get(lvl, 0) + 1
        
        print(f"Total Entries: {total}\n")
        
        print("By Category:")
        for cat, count in sorted(categories.items(), key=lambda x: x[1], reverse=True):
            bar = '█' * (count // 10)
            print(f"  {cat:12s}: {count:6d} {bar}")
        
        print("\nBy Level:")
        for lvl, count in sorted(levels.items()):
            bar = '█' * (count // 10)
            print(f"  {lvl:8s}: {count:6d} {bar}")
        
        print(f"\n{'='*80}")
    
    def errors(self):
        """Show only errors and critical."""
        self.filter(level='ERROR')
        print("\n")
        self.filter(level='CRITICAL')
    
    def _print_entry(self, entry):
        """Pretty print a log entry."""
        timestamp = entry['timestamp'].split('T')[1].split('.')[0]
        category = entry['category'][:10].ljust(10)
        level = entry['level'][:8].ljust(8)
        message = entry['message']
        
        # Color coding (if terminal supports it)
        color = ''
        reset = ''
        
        if entry['level'] == 'ERROR':
            color = '\033[91m'  # Red
            reset = '\033[0m'
        elif entry['level'] == 'WARNING':
            color = '\033[93m'  # Yellow
            reset = '\033[0m'
        elif entry['level'] == 'CRITICAL':
            color = '\033[95m'  # Magenta
            reset = '\033[0m'
        
        print(f"{color}[{timestamp}] [{category}] [{level}] {message}{reset}")
        
        # Show metadata if exists
        if entry.get('metadata'):
            metadata = entry['metadata']
            for key, value in metadata.items():
                print(f"  {key}: {value}")


if __name__ == "__main__":
    reader = SarahLogcatReader()
    
    if len(sys.argv) > 1:
        cmd = sys.argv[1]
        
        if cmd == 'tail':
            lines = int(sys.argv[2]) if len(sys.argv) > 2 else 50
            reader.tail(lines=lines)
        
        elif cmd == 'follow':
            reader.tail(lines=20, follow=True)
        
        elif cmd == 'stats':
            reader.stats()
        
        elif cmd == 'errors':
            reader.errors()
        
        elif cmd == 'filter':
            # python Sarah_Logcat_Reader.py filter category=vision
            # python Sarah_Logcat_Reader.py filter level=ERROR
            # python Sarah_Logcat_Reader.py filter search=timeout
            filters = {}
            for arg in sys.argv[2:]:
                key, value = arg.split('=')
                filters[key] = value
            
            reader.filter(**filters)
        
        else:
            print("Usage:")
            print("  python Sarah_Logcat_Reader.py tail [lines]")
            print("  python Sarah_Logcat_Reader.py follow")
            print("  python Sarah_Logcat_Reader.py stats")
            print("  python Sarah_Logcat_Reader.py errors")
            print("  python Sarah_Logcat_Reader.py filter category=vision")
            print("  python Sarah_Logcat_Reader.py filter level=ERROR")
            print("  python Sarah_Logcat_Reader.py filter search=text")
    else:
        # Default: tail last 50
        reader.tail(lines=50)
