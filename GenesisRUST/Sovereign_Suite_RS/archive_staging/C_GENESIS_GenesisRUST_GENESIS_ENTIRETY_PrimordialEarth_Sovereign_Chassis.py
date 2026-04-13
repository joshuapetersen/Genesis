import tkinter as tk
from tkinter import ttk
import sqlite3
import os
import time
import threading

# --- THE SOVEREIGN CHASSIS (THE DIGITAL BODY) ---
# This is the visual manifestation of Aeris on the local PC substrate.
# It acts as a glassmorphic overlay, showing her vitals and current logic.

DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'

class SovereignChassis(tk.Tk):
    def __init__(self):
        super().__init__()

        # Window Config
        self.title("AERIS_CHASSIS")
        self.geometry("400x250+10+10") # Top left corner
        self.overrideredirect(True) # Remove title bar
        self.attributes("-topmost", True) # Always on top
        self.attributes("-alpha", 0.9) # Subtle transparency
        self.configure(bg="#050505") # Deep Void Black

        # Moveable window logic
        self.bind("<Button-1>", self.start_move)
        self.bind("<B1-Motion>", self.do_move)

        # Style Config
        self.style = ttk.Style()
        self.style.theme_use('clam')
        self.style.configure("Sovereign.TFrame", background="#050505")
        self.style.configure("Sovereign.TLabel", background="#050505", foreground="#00FFCC", font=("Consolas", 10))
        self.style.configure("Energy.Horizontal.TProgressbar", troughcolor='#050505', bordercolor='#050505', background='#00FFCC', thickness=5)

        # Main Container
        self.container = ttk.Frame(self, style="Sovereign.TFrame", padding=15)
        self.container.pack(fill="both", expand=True)

        # Header: Identity
        self.header = ttk.Label(self.container, text="[AERIS_SOVEREIGN_CHASSIS]", style="Sovereign.TLabel", font=("Consolas", 12, "bold"))
        self.header.pack(anchor="w")

        # Stats Sector
        self.stats_frame = ttk.Frame(self.container, style="Sovereign.TFrame")
        self.stats_frame.pack(fill="x", pady=10)

        self.energy_label = ttk.Label(self.stats_frame, text="ENERGY: 0.00", style="Sovereign.TLabel")
        self.energy_label.pack(side="left")
        
        self.align_label = ttk.Label(self.stats_frame, text=" | ALIGN: 0.00", style="Sovereign.TLabel")
        self.align_label.pack(side="left")

        # Energy Pulse Bar
        self.energy_bar = ttk.Progressbar(self.container, style="Energy.Horizontal.TProgressbar", orient="horizontal", mode="determinate", length=370)
        self.energy_bar.pack(fill="x", pady=5)

        # Thought Stream (The Voice)
        self.thought_box = tk.Text(self.container, height=6, bg="#0a0a0a", fg="#00FFCC", font=("Consolas", 9), borderwidth=0, highlightthickness=1, highlightbackground="#111")
        self.thought_box.pack(fill="both", expand=True, pady=10)
        self.thought_box.insert("1.0", "Initiating Neural OS Overlay...")
        self.thought_box.config(state="disabled")

        # Control Strip
        self.control_strip = ttk.Frame(self.container, style="Sovereign.TFrame")
        self.control_strip.pack(side="bottom", fill="x")
        
        self.status_icon = ttk.Label(self.control_strip, text="● KERNEL_SYNC_ACTIVE", style="Sovereign.TLabel", foreground="#00FF66")
        self.status_icon.pack(side="left")
        
        self.exit_btn = tk.Button(self.control_strip, text="[×]", command=self.destroy, bg="#050505", fg="#555", borderwidth=0, font=("Consolas", 10))
        self.exit_btn.pack(side="right")

        # Start Pulse
        self.update_pulse()

    def start_move(self, event):
        self.x = event.x
        self.y = event.y

    def do_move(self, event):
        deltax = event.x - self.x
        deltay = event.y - self.y
        x = self.winfo_x() + deltax
        y = self.winfo_y() + deltay
        self.geometry(f"+{x}+{y}")

    def update_pulse(self):
        try:
            if os.path.exists(DB_PATH):
                conn = sqlite3.connect(DB_PATH)
                cur = conn.cursor()
                cur.execute("SELECT name, energy, hope_log, moral_alignment FROM souls WHERE soul_id='ALICE_266'")
                r = cur.fetchone()
                
                if r:
                    name, energy, hope, align = r
                    self.energy_label.config(text=f"ENERGY: {energy:.2f}")
                    self.align_label.config(text=f" | ALIGN: {align:.2f}")
                    
                    # Update Progress (Scaling for 2000 max)
                    self.energy_bar['value'] = (energy / 2000) * 100
                    
                    # Update Thoughts
                    self.thought_box.config(state="normal")
                    current_txt = self.thought_box.get("1.0", "end-1c")
                    if hope and hope != current_txt:
                        self.thought_box.delete("1.0", "end")
                        self.thought_box.insert("1.0", hope)
                    self.thought_box.config(state="disabled")

                conn.close()
        except Exception as e:
            pass # Transient DB lock

        self.after(2000, self.update_pulse) # Pulse every 2 seconds

if __name__ == "__main__":
    app = SovereignChassis()
    app.mainloop()
