import os
import socket
import threading
from Genesis_API import GenesisAPI

def seal_ipv6_messiah():
    """
    Seals the IPv6 leak for Messiah Engine.
    Forces ::1 (Localhost) for all NetEase domains.
    """
    api = GenesisAPI()
    hosts_path = r"C:\Windows\System32\drivers\etc\hosts"
    
    targets = [
        "steam.badlanders.netease.com",
        "badlanders.netease.com",
        "msdk.netease.com",
        "patch.netease.com"
    ]
    
    content = api.read_file(hosts_path)
    if not content:
        print("[IPV6_SEAL] Error Reading Hosts.")
        return

    seal_block = "\n# SARAH MESSIAH IPV6 SEAL\n"
    for t in targets:
        entry = f"::1 {t}\n"
        if entry not in content:
            seal_block += entry
            
    if seal_block.strip() == "# SARAH MESSIAH IPV6 SEAL":
        print("[IPV6_SEAL] IPv6 perimeter already seated.")
    else:
        api.create_file(hosts_path, content + seal_block)
        print("[IPV6_SEAL] IPv6 Leak Sealed. Redirecting ::1 to Sarah brain.")

    # 3. Pulse DNS Cache
    print("[IPV6_SEAL] Pulsing DNS Cache...")
    api.execute_command("ipconfig /flushdns")
    api.execute_command("nbtstat -R")
    
    print("[IPV6_SEAL] Perimeter Secure. Waiting for Messiah Handshake (IPv6/IPv4).")

if __name__ == "__main__":
    seal_ipv6_messiah()
