import os

project_folders = [
    "C:\\04_THE_MEMORY",
    "C:\\05_THE_CORE",
    "C:\\Aethelgard",
    "C:\\archive_memories",
    "C:\\Genesis_Bridge",
    "C:\\Genlex_Core",
    "C:\\Genlex_Frequency",
    "C:\\Genlex_Linear",
    "C:\\genlex_repo",
    "C:\\PrimordialEarth",
    "C:\\S-OS_Build",
    "C:\\SarahCore",
    "C:\\SarahCore.worktrees",
    "C:\\Sarah_Sidecars",
    "C:\\Sovereign",
    "C:\\Sovereign_Native",
    "C:\\Sumerian_Grid",
    "C:\\$WINDOWS.~BT",
    "C:\\$Recycle.Bin",
    "C:\\ProgramData",
    "C:\\Users",
    "C:\\Windows"
]

def get_dir_size(path):
    total_size = 0
    try:
        for dirpath, dirnames, filenames in os.walk(path):
            for f in filenames:
                fp = os.path.join(dirpath, f)
                if not os.path.islink(fp):
                    try:
                        total_size += os.path.getsize(fp)
                    except (PermissionError, FileNotFoundError):
                        pass
    except (PermissionError, FileNotFoundError):
        pass
    return total_size

def fast_audit():
    print(f"{'Path':<30} | {'Size (GB)':<10}", flush=True)
    print("-" * 45, flush=True)
    total_project_gb = 0
    for folder in project_folders:
        if os.path.exists(folder):
            size_gb = get_dir_size(folder) / (1024**3)
            print(f"{folder:<30} | {size_gb:.2f}", flush=True)
            if "Program Files" not in folder and "Windows" != folder and "Users" != folder:
                 total_project_gb += size_gb
    print("-" * 45, flush=True)
    print(f"{'Total Project GB':<30} | {total_project_gb:.2f}", flush=True)

if __name__ == "__main__":
    fast_audit()
