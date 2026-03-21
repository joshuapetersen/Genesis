import customtkinter as ctk
import sqlite3
import threading
import time
import os
import psutil
from PIL import Image, ImageTk

# Configuration
DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'
SOUL_ID = 'ALICE_266'
YEAR_FILE = r'C:\PrimordialEarth\sim_year.txt'

class SovereignChassis(ctk.CTk):
    def __init__(self):
        super().__init__()

        # Window Settings
        self.title("S.O.V.E.R.E.I.G.N. - Digital Chassis V1")
        self.geometry("1000x650")
        self.attributes("-alpha", 0.95) # Glassmorphic effect
        ctk.set_appearance_mode("dark")
        
        # Grid layout
        self.grid_columnconfigure(1, weight=1)
        self.grid_rowconfigure(0, weight=1)

        # Sidebar (System Monitoring)
        self.sidebar = ctk.CTkFrame(self, width=200, corner_radius=0)
        self.sidebar.grid(row=0, column=0, sticky="nsew")
        
        self.logo_label = ctk.CTkLabel(self.sidebar, text="Σ SIGMA CORE", font=ctk.CTkFont(size=24, weight="bold", family="Orbitron"))
        self.logo_label.grid(row=0, column=0, padx=20, pady=(20, 10))
        
        self.sub_label = ctk.CTkLabel(self.sidebar, text="Substrate Status:", font=ctk.CTkFont(size=14, weight="bold"))
        self.sub_label.grid(row=1, column=0, padx=20, pady=(20, 0))
        
        self.cpu_label = ctk.CTkLabel(self.sidebar, text="CPU: 0%")
        self.cpu_label.grid(row=2, column=0, padx=20, pady=5)
        
        self.mem_label = ctk.CTkLabel(self.sidebar, text="RAM: 0%")
        self.mem_label.grid(row=3, column=0, padx=20, pady=5)
        
        self.lineage_label = ctk.CTkLabel(self.sidebar, text="LINEAGE: SOVEREIGN", text_color="#00ffcc")
        self.lineage_label.grid(row=4, column=0, padx=20, pady=(50, 0))

        # Main View (Sovereign Awareness)
        self.main_view = ctk.CTkFrame(self, corner_radius=15, fg_color="transparent")
        self.main_view.grid(row=0, column=1, padx=20, pady=20, sticky="nsew")
        self.main_view.grid_columnconfigure(0, weight=1)
        
        self.header = ctk.CTkLabel(self.main_view, text="AERIS - THE ARCH-OPTIMIZER", font=ctk.CTkFont(size=32, weight="bold"))
        self.header.grid(row=0, column=0, padx=20, pady=(20, 10))
        
        self.year_label = ctk.CTkLabel(self.main_view, text="SIMULATION YEAR: 0", font=ctk.CTkFont(size=18, family="Consolas"))
        self.year_label.grid(row=1, column=0, padx=20, pady=0)

        # Stats Card
        self.stats_frame = ctk.CTkFrame(self.main_view, fg_color="#1a1a1a", corner_radius=10)
        self.stats_frame.grid(row=2, column=0, padx=20, pady=20, sticky="ew")
        
        self.wis_label = ctk.CTkLabel(self.stats_frame, text="WIS: 0", font=ctk.CTkFont(size=16))
        self.wis_label.pack(side="left", padx=30, pady=15)
        
        self.int_label = ctk.CTkLabel(self.stats_frame, text="INT: 0", font=ctk.CTkFont(size=16))
        self.int_label.pack(side="left", padx=30, pady=15)
        
        self.bless_label = ctk.CTkLabel(self.stats_frame, text="BLESSING: UNKNOWN", text_color="#00aaff")
        self.bless_label.pack(side="left", padx=30, pady=15)

        # Hope Log Display
        self.hope_frame = ctk.CTkFrame(self.main_view, fg_color="#0d0d0d", corner_radius=10)
        self.hope_frame.grid(row=3, column=0, padx=20, pady=(10, 20), sticky="nsew")
        self.main_view.grid_rowconfigure(3, weight=1)
        
        self.hope_text = ctk.CTkTextbox(self.hope_frame, font=ctk.CTkFont(size=15, family="Consolas"), text_color="#00ff00")
        self.hope_text.pack(expand=True, fill="both", padx=10, pady=10)
        self.hope_text.insert("0.0", ">>> INITIALIZING KINETIC LINK...")

        # Start data thread
        self.stop_event = threading.Event()
        self.thread = threading.Thread(target=self.update_loop, daemon=True)
        self.thread.start()

    def update_loop(self):
        while not self.stop_event.is_set():
            try:
                # 1. System Stats
                cpu = psutil.cpu_percent()
                mem = psutil.virtual_memory().percent
                self.cpu_label.configure(text=f"CPU: {cpu}%")
                self.mem_label.configure(text=f"RAM: {mem}%")
                
                # 2. Sim Year
                if os.path.exists(YEAR_FILE):
                    with open(YEAR_FILE, 'r') as f:
                        year = f.read().strip()
                        self.year_label.configure(text=f"SIMULATION YEAR: {int(float(year)):,}")
                
                # 3. Vault Stats
                conn = sqlite3.connect(DB_PATH)
                cur = conn.cursor()
                cur.execute("SELECT wis, int_stat, blessing, hope_log FROM souls WHERE soul_id=?", (SOUL_ID,))
                row = cur.fetchone()
                if row:
                    wis, it, bless, hope = row
                    self.wis_label.configure(text=f"WIS: {wis}")
                    self.int_label.configure(text=f"INT: {it}")
                    self.bless_label.configure(text=f"BLESSING: {bless}")
                    
                    # Update text if different
                    current_text = self.hope_text.get("0.0", "end").strip()
                    if hope and hope.strip() != current_text:
                        self.hope_text.delete("0.0", "end")
                        self.hope_text.insert("0.0", hope.strip())
                
                conn.close()
            except Exception as e:
                print(f"UI Error: {e}")
            
            time.sleep(1)

    def on_closing(self):
        self.stop_event.set()
        self.destroy()

if __name__ == "__main__":
    app = SovereignChassis()
    app.protocol("WM_DELETE_WINDOW", app.on_closing)
    app.mainloop()
