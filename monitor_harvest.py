import os
import json
import glob

def monitor_harvest():
    base_dir = os.path.dirname(os.path.abspath(__file__))
    discovery_path = os.path.join(base_dir, "vault", "scraped_content", "discovery_map.json")
    history_dir = os.path.join(base_dir, "vault", "scraped_content", "chat_history")
    
    print("\n" + "="*50)
    print(" [SARAH_CORE] CHAT HARVEST MONITORING")
    print("="*50)
    
    # 1. Load Discovery Data
    if not os.path.exists(discovery_path):
        print("[Error] discovery_map.json not found.")
        return
        
    with open(discovery_path, 'r', encoding='utf-8') as f:
        discovery_data = json.load(f)
        
    discovered_ids = {item['id'] for item in discovery_data}
    total_discovered = max(len(discovered_ids), 600) # Baseline based on wide discovery
    
    # 2. Scan Harvested Files
    harvested_files = glob.glob(os.path.join(history_dir, "*.json"))
    harvested_ids = set()
    total_messages = 0
    
    for fpath in harvested_files:
        try:
            with open(fpath, 'r', encoding='utf-8') as f:
                data = json.load(f)
                tid = data.get('thread_id')
                if tid:
                    harvested_ids.add(tid)
                    total_messages += len(data.get('history', []))
        except:
            pass
            
    # 3. Stats calculation
    harvested_count = len(harvested_ids)
    remaining_count = total_discovered - harvested_count
    percent = (harvested_count / total_discovered * 100) if total_discovered > 0 else 0
    
    # 4. Progress Bar
    bar_width = 30
    filled = int(bar_width * percent / 100)
    bar = "█" * filled + "░" * (bar_width - filled)
    
    print(f"\n Progress: [{bar}] {percent:.2f}%")
    print(f" Discovered: {total_discovered} threads")
    print(f" Harvested:  {harvested_count} threads")
    print(f" Remaining:  {remaining_count} threads")
    print(f" Total Msgs: {total_messages} (Extracted & Indexed)")
    
    if harvested_count > 0:
        avg_msgs = total_messages / harvested_count
        print(f" Avg Length:  {avg_msgs:.1f} messages / thread")
    
    print("\n Lately Harvested (Batch 5 Highlights):")
    # Sort by mtime to see latest
    harvested_files.sort(key=os.path.getmtime, reverse=True)
    for fpath in harvested_files[:5]:
        with open(fpath, 'r', encoding='utf-8') as f:
            data = json.load(f)
            msgs = len(data.get('history', []))
            print(f" - {data.get('thread_id')}: {msgs} messages")
    
    print("\n" + "="*50 + "\n")

if __name__ == "__main__":
    monitor_harvest()
