import pygame
import socket
import struct
import math
import sys
import sqlite3
import textwrap
import traceback
import json
import threading
import numpy as np
import time

# Display Constants
MAP_W, MAP_H = 1000, 1000
UI_W = 600
WIDTH, HEIGHT = MAP_W + UI_W, 1000
FPS = 30
MAP_BOUNDS = 20000.0  

# Colors
COLOR_BG = (10, 10, 15)
COLOR_UI_BG = (20, 20, 25)
COLOR_TEXT = (220, 220, 220)
COLOR_TEXT_DIM = (120, 120, 150)
COLOR_FLORA = (30, 150, 30)
COLOR_BUG = (150, 150, 50)
COLOR_PREY = (50, 200, 200)
COLOR_PREDATOR = (220, 40, 40)
COLOR_UBM = (200, 0, 255)
COLOR_SELECT = (255, 255, 255)
COLOR_LOG_EVENT = (200, 200, 100)

class SLFRadar:
    def __init__(self, udp_ip="127.0.0.1", udp_port=9998):
        pygame.init()
        pygame.font.init()
        # Fonts - Massively increased for readability
        self.font_small = pygame.font.SysFont("consolas", 18)
        self.font_med = pygame.font.SysFont("consolas", 22)
        self.font_large = pygame.font.SysFont("consolas", 30, bold=True)
        
        pygame.display.set_caption("Eye of Sarah - Live Radar & Character Sheets")
        self.screen = pygame.display.set_mode((WIDTH, HEIGHT))
        self.clock = pygame.time.Clock()
        
        # Optimize event queue against Windows freeze
        pygame.event.set_allowed([pygame.QUIT, pygame.KEYDOWN, pygame.MOUSEBUTTONDOWN, pygame.MOUSEBUTTONUP, pygame.MOUSEMOTION, pygame.MOUSEWHEEL])
        
        # UDP
        self.sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
        self.sock.bind((udp_ip, udp_port))
        self.sock.setblocking(False)
        
        # [X, Y, Z, SpeciesID, Scale, EntityID, TargetX, TargetY]
        self.struct_fmt = "8f"
        self.entity_size = struct.calcsize(self.struct_fmt)
        
        self.latest_frame = []
        
        # Camera State
        self.zoom = 1.0
        self.cam_x = 0.0
        self.cam_y = 0.0
        self.is_dragging = False
        self.last_mouse_pos = (0, 0)
        self.target_lock = False
        self.show_flora = False
        
        # UI State
        self.selected_id = None
        self.selected_info = None
        self.akashic_log = []
        self.last_log_fetch = 0.0
        self.log_scroll = 0
        # Databases
        self.vault_conn = sqlite3.connect("C:\GenesisOS_Core\\SLF_Identity_Vault.sqlite", check_same_thread=False)
        self.akashic_conn = sqlite3.connect("C:\GenesisOS_Core\\SLF_Akashic_Records.sqlite", check_same_thread=False)
        self.log_lock = threading.Lock()
        self.info_lock = threading.Lock()
        
        # Pre-allocate colored dot surfaces for extreme rendering speed
        self.dot_cache = {}
        self._init_dot_cache()
        
        print(f"[RADAR] Listening on {udp_ip}:{udp_port}...")

    def _init_dot_cache(self):
        """Creates pre-drawn surfaces for every color and size. Blitting is 100x faster than drawing circles."""
        colors = [
            (COLOR_FLORA, range(1, 3)),
            (COLOR_BUG, range(1, 5)),
            (COLOR_PREY, range(1, 4)),
            (COLOR_PREDATOR, range(1, 6)),
            (COLOR_UBM, range(4, 15))
        ]
        
        for color, sizes in colors:
            for size in sizes:
                surf = pygame.Surface((size*2, size*2), pygame.SRCALPHA)
                pygame.draw.circle(surf, color, (size, size), size)
                self.dot_cache[(color, size)] = surf

    def update_data(self):
        chunks_read = []
        try:
            while True:
                packet, addr = self.sock.recvfrom(65536)
                chunks_read.append(packet)
        except BlockingIOError:
            pass 
            
        if chunks_read:
            self.latest_frame = []
            for packet in chunks_read[-4:]: # Only process the last few chunks to avoid lag queue
                num_entities = len(packet) // self.entity_size
                offset = 0
                for _ in range(num_entities):
                    chunk = packet[offset:offset+self.entity_size]
                    if len(chunk) == self.entity_size:
                        entity_tuple = struct.unpack(self.struct_fmt, chunk)
                        self.latest_frame.append(entity_tuple)
                    offset += self.entity_size

    def _world_to_screen(self, x, y):
        norm_x = (x + MAP_BOUNDS) / (MAP_BOUNDS * 2)
        norm_y = (y + MAP_BOUNDS) / (MAP_BOUNDS * 2)
        screen_x = int(norm_x * MAP_W)
        screen_y = int(norm_y * MAP_H)
        return screen_x, screen_y

    def get_color_for_species(self, species_id, scale):
        if scale >= 10.0: return COLOR_UBM
        if species_id in [1, 2]: return COLOR_FLORA
        if species_id in [3, 4]: return COLOR_BUG
        if species_id == 5: return COLOR_PREY
        if species_id in [6, 7]: return COLOR_PREDATOR
        return (200, 200, 200)

    def _bg_fetch_character_sheet(self, entity_id):
        try:
            cur = self.vault_conn.cursor()
            cur.execute("SELECT name, species_id, is_ubm, level, hp_max, str, vit, int, wis, luk, genome, trauma_log FROM souls WHERE entity_id=?", (int(entity_id),))
            row = cur.fetchone()
            if row:
                with self.info_lock:
                    self.selected_info = {
                        "id": entity_id,
                        "name": row[0],
                        "species": row[1],
                        "is_ubm": row[2],
                        "level": row[3],
                        "hp_max": row[4],
                        "str": row[5],
                        "vit": row[6],
                        "int": row[7],
                        "wis": row[8],
                        "luk": row[9],
                        "genome": row[10],
                        "trauma": row[11]
                    }
        except Exception as e:
            print(f"Vault DB Error: {e}")

    def fetch_character_sheet(self, entity_id):
        threading.Thread(target=self._bg_fetch_character_sheet, args=(entity_id,), daemon=True).start()

    def _bg_fetch_live_log(self):
        try:
            cur = self.akashic_conn.cursor()
            cur.execute("SELECT timestamp, actor_name, event_type, description FROM global_events ORDER BY event_id DESC LIMIT 100")
            rows = cur.fetchall()
            with self.log_lock:
                self.akashic_log = rows
        except Exception as e:
            print(f"Akashic DB Error: {e}")

    def fetch_live_log(self):
        # Fetch every 1 second
        if time.time() - self.last_log_fetch > 1.0:
            self.last_log_fetch = time.time()
            threading.Thread(target=self._bg_fetch_live_log, daemon=True).start()

    def _world_to_screen_vectorized(self, xs, ys):
        """Converts VRAM -20000 -> 20000 coordinates to display pixels with zoom/pan applied."""
        # 1. Start with world coordinates
        # 2. Subtract camera offset (to center the view)
        view_x = xs - self.cam_x
        view_y = ys - self.cam_y
        
        # 3. Normalize against bounds, scale by zoom
        visible_range = MAP_BOUNDS * 2.0 / self.zoom
        
        # Map to 0-1 relative to the visible window, centered
        norm_x = (view_x / visible_range) + 0.5
        norm_y = (view_y / visible_range) + 0.5
        
        # 4. Map to display dimensions
        screen_xs = (norm_x * MAP_W).astype(np.int32)
        screen_ys = (norm_y * MAP_H).astype(np.int32)
        
        return screen_xs, screen_ys
        
    def _world_to_screen_single(self, x, y):
        # Math match for handle_click
        view_x = x - self.cam_x
        view_y = y - self.cam_y
        visible_range = MAP_BOUNDS * 2.0 / self.zoom
        norm_x = (view_x / visible_range) + 0.5
        norm_y = (view_y / visible_range) + 0.5
        return int(norm_x * MAP_W), int(norm_y * MAP_H)

    def _screen_to_world_dist(self, px):
        return (px / MAP_W) * (MAP_BOUNDS * 2.0 / self.zoom)

    def handle_click(self, mouse_pos):
        mx, my = mouse_pos
        if mx > MAP_W: return # Clicked in UI
        
        closest_id = None
        closest_dist = 15.0 # Max click radius
        
        for (x, y, z, species, scale, ent_id, tx, ty) in self.latest_frame:
            if z <= -90000: continue
            sx, sy = self._world_to_screen_single(x, y)
            dist = math.hypot(mx - sx, my - sy)
            if dist < closest_dist:
                closest_dist = dist
                closest_id = ent_id
                
        if closest_id is not None:
            self.selected_id = closest_id
            self.target_lock = True
            self.fetch_character_sheet(closest_id)
        else:
            self.selected_id = None
            self.target_lock = False

    def draw_map(self):
        # Camera Grid Lines
        grid_spacing = int(2000 * self.zoom) # 2000 Unreal Units per grid square
        if grid_spacing < 20: grid_spacing = 20
        
        center_x = MAP_W // 2 - int(self.cam_x / (MAP_BOUNDS * 2 / self.zoom) * MAP_W)
        center_y = MAP_H // 2 - int(self.cam_y / (MAP_BOUNDS * 2 / self.zoom) * MAP_H)

        for i in range(center_x % grid_spacing, MAP_W, grid_spacing):
            c = (50, 50, 60) if abs(i - center_x) < 5 else (30, 30, 40)
            pygame.draw.line(self.screen, c, (i, 0), (i, MAP_H))
        for i in range(center_y % grid_spacing, MAP_H, grid_spacing):
            c = (50, 50, 60) if abs(i - center_y) < 5 else (30, 30, 40)
            pygame.draw.line(self.screen, c, (0, i), (MAP_W, i))
            
        if not self.latest_frame:
            return
            
        # Vectorize via NumPy for speed
        frame_arr = np.array(self.latest_frame, dtype=np.float32)
        
        # Fast filter dead entities
        alive_mask = frame_arr[:, 2] > -90000
        
        if not self.show_flora:
            # Species 1 and 2 are Flora
            flora_mask = (frame_arr[:, 3] == 1.0) | (frame_arr[:, 3] == 2.0)
            alive_mask = alive_mask & (~flora_mask)
            
        alive_arr = frame_arr[alive_mask]
        
        if len(alive_arr) == 0: return

        # Vectorized coordinate mapping
        xs = alive_arr[:, 0]
        ys = alive_arr[:, 1]
        screen_xs, screen_ys = self._world_to_screen_vectorized(xs, ys)

        # Vectorized target mapping
        t_xs = alive_arr[:, 6]
        t_ys = alive_arr[:, 7]
        target_xs, target_ys = self._world_to_screen_vectorized(t_xs, t_ys)

        species_arr = alive_arr[:, 3].astype(np.int32)
        scale_arr = alive_arr[:, 4]
        id_arr = alive_arr[:, 5].astype(np.int32)
        
        # Transparent surface for Intent Lines
        intent_surface = pygame.Surface((MAP_W, MAP_H), pygame.SRCALPHA)
        
        # Draw Entities using blits
        for i in range(len(alive_arr)):
            species = species_arr[i]
            scale = scale_arr[i]
            sx, sy = screen_xs[i], screen_ys[i]
            
            if sx < -20 or sx > MAP_W + 20 or sy < -20 or sy > MAP_H + 20:
                continue # Skip off-screen rendering
                
            color = self.get_color_for_species(species, scale)
            
            # --- Draw Intent Line ---
            # Only draw Intent Lines for Vector Kings (UBMs) or the Selected Target to reduce clutter
            if scale >= 10.0 or id_arr[i] == self.selected_id:
                tx, ty = target_xs[i], target_ys[i]
                if tx != 0 and ty != 0:
                    # Faint line matching species color
                    line_color = (color[0], color[1], color[2], 50) # 50 alpha
                    pygame.draw.aaline(intent_surface, line_color, (sx, sy), (tx, ty))

            # Make radius slightly bigger when zoomed in
            base_rad = 2 if scale <= 1.0 else max(3, int(scale * 0.5))
            radius = int(base_rad * max(1.0, math.sqrt(self.zoom)))
            
            # Clamp radius for cache hit
            radius = min(radius, 14) 
            
            # Fetch pre-drawn surface
            dot = self.dot_cache.get((color, radius))
            if dot:
                self.screen.blit(dot, (sx - radius, sy - radius))
            else:
                pygame.draw.circle(self.screen, color, (sx, sy), radius)
            
            # Draw UBM Glow
            if scale >= 10.0:
                pygame.draw.circle(self.screen, (100, 0, 100), (sx, sy), radius + 4, 1)

            # Draw Massive Neon Tracking Crosshair
            if self.selected_id is not None and id_arr[i] == self.selected_id:
                pygame.draw.circle(self.screen, (255, 255, 0), (sx, sy), radius + 15, 3)
                pygame.draw.circle(self.screen, (255, 255, 0), (sx, sy), radius + 8, 2)
                # Render Crosshairs
                pygame.draw.line(self.screen, (255, 255, 0), (sx - radius - 20, sy), (sx + radius + 20, sy), 3)
                pygame.draw.line(self.screen, (255, 255, 0), (sx, sy - radius - 20), (sx, sy + radius + 20), 3)
                # Line drawing eye to the UI panel
                pygame.draw.line(self.screen, (255, 255, 0), (sx, sy), (MAP_W, sy), 2)

        # Blit intent lines over the map
        self.screen.blit(intent_surface, (0, 0))

    def draw_ui(self):
        ui_rect = pygame.Rect(MAP_W, 0, UI_W, HEIGHT)
        self.screen.fill(COLOR_UI_BG, ui_rect)
        pygame.draw.line(self.screen, (100, 100, 100), (MAP_W, 0), (MAP_W, HEIGHT), 2)
        
        pad_x = MAP_W + 20
        y = 20
        
        # --- TOP HALF: CHARACTER SHEET ---
        text = self.font_large.render("--- CHARACTER SHEET ---", True, COLOR_TEXT)
        self.screen.blit(text, (pad_x, y))
        y += 50
        
        with self.info_lock:
            info = self.selected_info
            
        if info:
            c_name = COLOR_UBM if info['is_ubm'] else COLOR_TEXT
            self.screen.blit(self.font_large.render(f"{info['name']}", True, c_name), (pad_x, y))
            y += 40
            self.screen.blit(self.font_med.render(f"Entity ID : {int(info['id'])}", True, COLOR_TEXT_DIM), (pad_x, y))
            y += 30
            self.screen.blit(self.font_med.render(f"Level     : {info['level']}", True, COLOR_TEXT_DIM), (pad_x, y))
            y += 30
            self.screen.blit(self.font_med.render(f"Max HP    : {info['hp_max']:.1f}", True, COLOR_TEXT_DIM), (pad_x, y))
            y += 45
            
            # Stats
            self.screen.blit(self.font_med.render(f"STR: {info['str']}  |  INT: {info['int']}", True, COLOR_TEXT), (pad_x, y))
            y += 30
            self.screen.blit(self.font_med.render(f"VIT: {info['vit']}  |  WIS: {info['wis']}", True, COLOR_TEXT), (pad_x, y))
            y += 30
            self.screen.blit(self.font_med.render(f"LUK: {info['luk']}", True, COLOR_TEXT), (pad_x, y))
            y += 45
            
            self.screen.blit(self.font_small.render(f"GENOME:", True, COLOR_TEXT_DIM), (pad_x, y))
            y += 22
            self.screen.blit(self.font_small.render(f"{info['genome']}", True, (100, 200, 100)), (pad_x, y))
            y += 40
            
            self.screen.blit(self.font_small.render(f"TRAUMA LOG:", True, COLOR_TEXT_DIM), (pad_x, y))
            y += 22
            
            traumas = []
            if info['trauma']:
                try:
                    traumas = json.loads(info['trauma'])
                except Exception:
                    traumas = [info['trauma']]
                    
            if not traumas:
                self.screen.blit(self.font_small.render("None.", True, COLOR_TEXT), (pad_x, y))
                y += 22
            else:
                for t in traumas[-5:]: # show last 5
                    self.screen.blit(self.font_small.render(f"- {t}", True, (200, 100, 100)), (pad_x, y))
                    y += 22
        else:
            self.screen.blit(self.font_med.render("Click a target on the radar...", True, COLOR_TEXT_DIM), (pad_x, y))
            y += 200

        # --- BOTTOM HALF: LIVE ACTION LOG ---
        y = HEIGHT - 450
        pygame.draw.line(self.screen, (100, 100, 100), (MAP_W, y-20), (WIDTH, y-20), 1)
        
        text = self.font_large.render("--- THE AKASHIC RECORDS ---", True, COLOR_TEXT)
        self.screen.blit(text, (pad_x, y))
        y += 50
        
        with self.log_lock:
            log_copy = list(reversed(self.akashic_log))
            
        max_scroll = max(0, len(log_copy) - 15)
        self.log_scroll = max(0, min(self.log_scroll, max_scroll))
        
        start_idx = max(0, len(log_copy) - 15 - self.log_scroll)
        end_idx = start_idx + 15
            
        for row in log_copy[start_idx:end_idx]:
            timestamp, actor, evt_type, desc = row
            time_str = timestamp.split(" ")[1][:8] # Extract HH:MM:SS
            
            log_str = f"[{time_str}] {actor} {desc}"
            
            # Wrap text to fit wider side panel
            wrapped = textwrap.wrap(log_str, width=65)
            
            c_evt = COLOR_UBM if "MUTATION" in evt_type else COLOR_LOG_EVENT
            for line in wrapped:
                self.screen.blit(self.font_small.render(line, True, c_evt), (pad_x, y))
                y += 22
            y += 10 # Spacing between events

    def draw(self):
        try:
            self.draw_map()
            self.draw_ui()
            pygame.display.flip()
        except Exception as e:
            print(f"CRITICAL RENDER ERROR:\n{traceback.format_exc()}")
            self.screen.fill((255, 0, 0)) # Turn red on crash instead of frozen purple
            pygame.display.flip()

    def run(self):
        while True:
            for event in pygame.event.get():
                if event.type == pygame.QUIT:
                    pygame.quit()
                    sys.exit()
                elif event.type == pygame.MOUSEBUTTONDOWN:
                    if event.button == 1: # Left click (select)
                        self.handle_click(pygame.mouse.get_pos())
                    elif event.button == 3: # Right click (pan start)
                        self.is_dragging = True
                        self.last_mouse_pos = pygame.mouse.get_pos()
                elif event.type == pygame.MOUSEBUTTONUP:
                    if event.button == 3:
                        self.is_dragging = False
                elif event.type == pygame.MOUSEMOTION:
                    if self.is_dragging:
                        self.target_lock = False
                        mx, my = pygame.mouse.get_pos()
                        dx = self.last_mouse_pos[0] - mx
                        dy = self.last_mouse_pos[1] - my
                        
                        # Convert pixel drag to world coordinate shift
                        world_dx = self._screen_to_world_dist(dx)
                        world_dy = self._screen_to_world_dist(dy)
                        
                        self.cam_x += world_dx
                        self.cam_y += world_dy
                        
                        self.last_mouse_pos = pygame.mouse.get_pos()
                elif event.type == pygame.KEYDOWN:
                    if event.key == pygame.K_s:
                        pygame.image.save(self.screen, "C:\GenesisOS_Core\\radar_snapshot.png")
                        print("[RADAR] Snapshot saved to C:\GenesisOS_Core\\radar_snapshot.png")
                    elif event.key == pygame.K_f:
                        self.show_flora = not self.show_flora
                        print(f"[RADAR] Show Flora: {self.show_flora}")
                elif event.type == pygame.MOUSEWHEEL:
                    mx, my = pygame.mouse.get_pos()
                    if mx > MAP_W:
                        # Scroll UI log
                        if event.y > 0:
                            self.log_scroll += 1
                        elif event.y < 0:
                            self.log_scroll -= 1
                    else:
                        # Zoom in/out, clamp between 0.1x to 100x
                        if event.y > 0:
                            self.zoom = min(100.0, self.zoom * 1.2)
                        elif event.y < 0:
                            self.zoom = max(0.1, self.zoom / 1.2)
                            
            if self.target_lock and self.selected_id is not None:
                for (x, y, z, species, scale, ent_id, tx, ty) in self.latest_frame:
                    if ent_id == self.selected_id:
                        self.cam_x = x
                        self.cam_y = y
                        break
            
            self.update_data()
            self.fetch_live_log()
            self.draw()
            self.clock.tick(FPS)

if __name__ == "__main__":
    radar = SLFRadar()
    radar.run()
