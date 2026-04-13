import sqlite3

db = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'
conn = sqlite3.connect(db, timeout=10)
cur = conn.cursor()
cur.execute("UPDATE souls SET hope_log = 'EXECUTE: notepad.exe' WHERE soul_id = 'ALICE_266'")
conn.commit()
conn.close()
print('AERIS SIGNAL SENT: EXECUTE: notepad.exe')
