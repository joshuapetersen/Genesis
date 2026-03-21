import os

def get_dir_size(path):
    total_size = 0
    try:
        for dirpath, dirnames, filenames in os.walk(path):
            for f in filenames:
                fp = os.path.join(dirpath, f)
                # skip if it is a symbolic link
                if not os.path.islink(fp):
                    total_size += os.path.getsize(fp)
    except (PermissionError, FileNotFoundError):
        pass
    return total_size

def audit_c_drive():
    print(f"{'Path':<50} | {'Size (GB)':<10}")
    print("-" * 65)
    
    # Root level files
    for f in os.listdir("C:\\"):
        f_path = os.path.join("C:\\", f)
        if os.path.isfile(f_path):
            try:
                size_gb = os.path.getsize(f_path) / (1024**3)
                if size_gb > 0.1:
                    print(f"{f_path:<50} | {size_gb:.2f}")
            except (PermissionError, FileNotFoundError):
                pass

    # Root level directories
    for d in os.listdir("C:\\"):
        d_path = os.path.join("C:\\", d)
        if os.path.isdir(d_path):
            print(f"Calculating size for {d_path}...")
            size_gb = get_dir_size(d_path) / (1024**3)
            print(f"{d_path:<50} | {size_gb:.2f}")

    # Specific deep dive: AppData
    appdata_path = "C:\\Users\\drago\\AppData"
    if os.path.exists(appdata_path):
        print(f"\nScanning AppData: {appdata_path}")
        for d in os.listdir(appdata_path):
            d_path = os.path.join(appdata_path, d)
            if os.path.isdir(d_path):
                size_gb = get_dir_size(d_path) / (1024**3)
                print(f"{d_path:<50} | {size_gb:.2f}")

    # Specific deep dive: Windows\WinSxS
    winsxs = "C:\\Windows\\WinSxS"
    if os.path.exists(winsxs):
        print(f"\nScanning WinSxS: {winsxs}")
        size_gb = get_dir_size(winsxs) / (1024**3)
        print(f"{winsxs:<50} | {size_gb:.2f}")

if __name__ == "__main__":
    audit_c_drive()
