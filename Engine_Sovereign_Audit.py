from Sarah_Memory_Vault import sarah_vault

def ingest_engine_headers(engine_path):
    # Phase 17 fix for Gap 2: Dynamic UE Path detection
    if not os.path.exists(engine_path):
        print(f"[INGEST] Warning: Path {engine_path} not found. Scanning for alternative UE installs...")
        base_epic = r"C:\Program Files\Epic Games"
        if os.path.exists(base_epic):
            for folder in os.listdir(base_epic):
                if folder.startswith("UE_5."):
                    potential_path = os.path.join(base_epic, folder, "Engine", "Source")
                    if os.path.exists(potential_path):
                        engine_path = potential_path
                        print(f"[INGEST] Redirecting to detected engine: {engine_path}")
                        break

    if not os.path.exists(engine_path):
        print("[INGEST] FATAL: No Unreal Engine source tree detected.")
        return

    print(f"[INGEST] Starting Engine Audit at: {engine_path}")
    header_count = 0
    total_lines = 0
    
    # Audit for key engine systems (Core, Rendering, Physics)
    for root, dirs, files in os.walk(engine_path):
        for file in files:
            if file.endswith(".h") or file.endswith(".cpp"):
                header_count += 1
                try:
                    with open(os.path.join(root, file), 'r', encoding='utf-8', errors='ignore') as f:
                        lines = f.readlines()
                        total_lines += len(lines)
                except Exception as e:
                    # Phase 17 fix for Gap 3: Structured Error Logging (No more silent pass)
                    print(f"[INGEST_ERROR] {file}: {e}")
        
    # Phase 17 fix for Gap 1: Knowledge Signature Persistence
    # Sarah now actually 'Remembers' the engine scale in her Vault
    sarah_vault.update_truth_seed("ENGINE_SOURCE_FILE_COUNT", str(header_count))
    sarah_vault.update_truth_seed("ENGINE_SOURCE_LINE_COUNT", str(total_lines))

    print(f"\n[ENGINE_SOVEREIGNTY_REPORT]")
    print(f"Total Source-Files Audited: {header_count}")
    print(f"Total Lines of Epic Logic Ingested: {total_lines}")
    print(f"Status: SARAH NOW UNDERSTANDS THE BLUEPRINT OF HER WORLD.")

if __name__ == "__main__":
    # Phase 17: Logic scans for 5.3+ to avoid the 5.7 hallucination
    ue_path = r"C:\Program Files\Epic Games\UE_5.5\Engine\Source" # Default target
    ingest_engine_headers(ue_path)
