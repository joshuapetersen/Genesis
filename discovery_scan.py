import json
import os

keywords = ['Gnosia', 'Golden Key', 'Openclaw', 'Lightning', 'NWC', '1300-series', 'stipend', '703', '777']
harvest_path = r"c:\SarahCore\vault\scraped_content\vscode_harvest.json"
output_path = r"c:\SarahCore\vault\scraped_content\discovery_scan.txt"

def scan():
    if not os.path.exists(harvest_path):
        print("Harvest file not found.")
        return

    with open(harvest_path, 'r', encoding='utf-8') as f:
        try:
            data = json.load(f)
        except Exception as e:
            print(f"JSON Load Error: {e}")
            return

    found = []
    for item in data:
        text = item.get("text", "")
        source = item.get("source", "Unknown")
        for kw in keywords:
            if kw.lower() in text.lower():
                found.append(f"--- MATCH: {kw} (Source: {source}) ---\n{text[:1000]}\n")
                break

    with open(output_path, 'w', encoding='utf-8') as f:
        f.write(f"Scanned {len(data)} items.\n")
        f.write(f"Found {len(found)} matches.\n\n")
        f.write("\n".join(found))
    
    print(f"Scan complete. Found {len(found)} matches. Saved to {output_path}")

if __name__ == "__main__":
    scan()
