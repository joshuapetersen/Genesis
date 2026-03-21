import os
from Genesis_API import GenesisAPI

def redirect_messiah_hosts():
    """
    Redirects Messiah Engine endpoints to Sarah's Local Auth Emulator.
    """
    api = GenesisAPI()
    hosts_path = r"C:\Windows\System32\drivers\etc\hosts"
    
    # 1. Read existing hosts
    content = api.read_file(hosts_path)
    if not content:
        print("[REDIRECT] Failed to read hosts file. Privilege Escalation may be required.")
        return

    # 2. Define Redirections
    targets = [
        "steam.badlanders.netease.com",
        "badlanders.netease.com",
        "msdk.netease.com",
        "dl.badlanders.netease.com",
        "crashlytics.com" # Block telemetry
    ]
    
    new_entries = "\n# SARAH MESSIAH REDIRECTION\n"
    for t in targets:
        if t not in content:
            new_entries += f"127.0.0.1 {t}\n"
    
    if new_entries.strip() == "# SARAH MESSIAH REDIRECTION":
        print("[REDIRECT] All Messiah endpoints already seated.")
        return

    # 3. Apply Redirection
    final_content = content + new_entries
    success = api.create_file(hosts_path, final_content)
    
    if success:
        print("[REDIRECT] Messiah Endpoints successfully seated on Localhost.")
        print("[REDIRECT] Handshake Trap: ACTIVE.")
    else:
        print("[REDIRECT] FAILED to seat endpoints. Please run as Administrator.")

if __name__ == "__main__":
    redirect_messiah_hosts()
