import socket
import json
import sqlite3
import os
import sys

try:
    from colorama import init, Fore, Style
    init(convert=True, autoreset=True)
except ImportError:
    class Fore:
        GREEN = ''
        YELLOW = ''
        RED = ''
        CYAN = ''
        MAGENTA = ''
    class Style:
        RESET_ALL = ''

def get_entity_list(filter_type=None):
    try:
        conn = sqlite3.connect("C:/SarahCore/SLF_Identity_Vault.sqlite")
        cursor = conn.cursor()
        
        query = "SELECT entity_id, name, level, is_ubm FROM souls WHERE hp_current > 0"
        if filter_type == "ubm":
            query += " AND is_ubm = 1"
        query += " ORDER BY RANDOM() LIMIT 15"
        
        cursor.execute(query)
        rows = cursor.fetchall()
        
        print(f"\n{Fore.CYAN}--- LIVE ENTITIES WATCHLIST ---{Style.RESET_ALL}")
        for r in rows:
            eid, name, lvl, is_ubm = r
            tag = f"{Fore.MAGENTA}[UBM]{Style.RESET_ALL}" if is_ubm else f"{Fore.GREEN}[Normal]{Style.RESET_ALL}"
            print(f"ID: {eid} | Name: {name} | Level: {lvl} {tag}")
        print(f"{Fore.CYAN}-------------------------------{Style.RESET_ALL}\n")
        conn.close()
    except Exception as e:
        print(f"{Fore.RED}[DB ERROR] Could not fetch entities from Identity Vault: {e}{Style.RESET_ALL}")

def set_focus(entity_id):
    state = {"focus_id": entity_id}
    with open("C:/SarahCore/focus.json", "w") as f:
        json.dump(state, f)
    if entity_id:
        print(f"{Fore.GREEN}[CHRONICLER] Now focusing exclusively on Entity {entity_id}.{Style.RESET_ALL}")
    else:
        print(f"{Fore.GREEN}[CHRONICLER] Returning to Global Aethelgard Stream.{Style.RESET_ALL}")

def send_revelation(target_id, message):
    try:
        sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
        payload = {
            "cmd": "GOD_VOICE",
            "target_id": int(target_id),
            "message": message,
            "is_sanctuary": False
        }
        sock.sendto(json.dumps(payload).encode('utf-8'), ("127.0.0.1", 9999))
        print(f"{Fore.CYAN}[REVELATION SENT] The heavens part and your words descend upon Entity {target_id}.{Style.RESET_ALL}")
    except Exception as e:
        print(f"{Fore.RED}[REVELATION FAILED] Could not connect to Hypervisor Command Bridge: {e}{Style.RESET_ALL}")

def resolve_entity(input_str):
    if input_str.isdigit():
        eid = int(input_str)
        try:
            conn = sqlite3.connect("C:/SarahCore/SLF_Identity_Vault.sqlite")
            cursor = conn.cursor()
            cursor.execute("SELECT name FROM souls WHERE entity_id=?", (eid,))
            row = cursor.fetchone()
            conn.close()
            if row: return eid, row[0]
        except: pass
    else:
        try:
            conn = sqlite3.connect("C:/SarahCore/SLF_Identity_Vault.sqlite")
            cursor = conn.cursor()
            cursor.execute("SELECT entity_id, name FROM souls WHERE name LIKE ?", (f"%{input_str}%",))
            row = cursor.fetchone()
            conn.close()
            if row: return row[0], row[1]
        except: pass
    return None, None

if __name__ == "__main__":
    print(f"\n{Fore.MAGENTA}=== THE DIVINE INPUT TERMINAL ==={Style.RESET_ALL}")
    print(f"Welcome, Sovereign. The system has been simplified for you.")
    
    set_focus(None)
    current_id = None
    current_name = None
    
    while True:
        try:
            if current_id is None:
                print(f"\n{Fore.YELLOW}Who do you want to speak to?{Style.RESET_ALL}")
                print("Type a Name (e.g. 'Flora'), an ID (e.g. '35'), 'list', or 'quit'.")
                cmd = input(f"{Fore.CYAN}SELECT TARGET > {Style.RESET_ALL}").strip()
                
                if not cmd: continue
                if cmd.lower() in ("quit", "exit"): break
                if cmd.lower().startswith("list"):
                    filter_type = "ubm" if "ubm" in cmd.lower() else "all"
                    get_entity_list(filter_type=filter_type)
                    continue
                    
                eid, ename = resolve_entity(cmd)
                if eid is not None:
                    current_id = eid
                    current_name = ename
                    set_focus(eid)
                    print(f"\n{Fore.GREEN}=== COGNITIVE LINK ESTABLISHED ==={Style.RESET_ALL}")
                    print(f"You are now peering directly into the mind of {current_name} (ID: {eid}).")
                    print(f"Everything you type will be sent to them as a Divine Revelation.")
                    print(f"Type 'back' to disconnect.")
                else:
                    print(f"{Fore.RED}Could not find an entity matching '{cmd}'. Try 'list'.{Style.RESET_ALL}")
                    
            else:
                msg = input(f"{Fore.MAGENTA}Speak to {current_name} > {Style.RESET_ALL}").strip()
                if not msg: continue
                if msg.lower() in ('back', 'unfocus', 'exit', 'quit', 'disconnect'):
                    current_id = None
                    current_name = None
                    set_focus(None)
                    continue
                
                send_revelation(current_id, msg)
                
        except (KeyboardInterrupt, EOFError):
            break
