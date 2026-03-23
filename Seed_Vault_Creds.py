import sys
import os

# Add SarahCore to path
sys.path.append("C:/SarahCore")

try:
    from Sarah_Memory_Vault import sarah_vault
    print("[VAULT] Linked to Sarah Memory Vault.")
except ImportError:
    print("[ERROR] Sarah_Memory_Vault.py not found.")
    sys.exit(1)

def seed_wifi_creds(ssid, password):
    """
    Seeds the Wi-Fi credentials into the truth_seeds table.
    """
    print(f"[VAULT] Seeding credentials for SSID: {ssid}...")
    sarah_vault.update_truth_seed("WIFI_SSID", ssid)
    sarah_vault.update_truth_seed("WIFI_PASSWORD", password)
    print("[VAULT] Credentials sealed in Vault.")

if __name__ == "__main__":
    if len(sys.argv) < 3:
        print("Usage: python Seed_Vault_Creds.py <SSID> <PASSWORD>")
    else:
        seed_wifi_creds(sys.argv[1], sys.argv[2])
