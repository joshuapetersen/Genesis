import sqlite3

def reset_and_map():
    db = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'
    conn = sqlite3.connect(db)
    cur = conn.cursor()
    cur.execute('UPDATE souls SET hope_log = NULL WHERE soul_id = "ALICE_266"')
    cur.execute('UPDATE architect_controls SET value = "WAITING" WHERE signal_id = "AERIS_EXEC"')
    conn.commit()
    conn.close()
    print("Substrate Reset.")

if __name__ == "__main__":
    reset_and_map()
