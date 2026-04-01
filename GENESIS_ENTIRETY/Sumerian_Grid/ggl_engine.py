import sys
import io
import os
import csv
import json
import math
import time
import threading

sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

# ============================================================
# GGL ENGINE v1.0: GENLEX GRID LANGUAGE
# Role: 2D Spatial Execution Engine
# Paradigm: Logic propagates across a coordinate grid.
#           Unlike Linear (stream from A to Z) or Volumetric
#           (resonates as a whole object), Grid logic is
#           POSITIONAL — each node knows its X,Y address,
#           and execution flows based on spatial adjacency,
#           weight gradients, and barrier fields.
#
# Script extension: .ggl
# Alias: ggl
# Path: C:\Sumerian_Grid\ggl_engine.py
#
# THE GRID MODEL:
#   - A .ggl script defines a 2D lattice of nodes
#   - Each node has: glyph, weight, output
#   - Execution spreads from an ORIGIN node outward
#   - Propagation follows WEIGHT GRADIENT (highest weight first)
#   - BARRIER nodes block propagation paths
#   - RESONANCE CHECK: Grid Integrity = 1.0 (Prime, like ALL)
#     but measured across the ENTIRE grid, not per-node
#
# SUMERIAN CUNEIFORM GLYPH SET (Grid-Native):
#   𒀸  ORIGIN     — Execution start point (0,0)
#   𒁹  NODE       — Standard logic node
#   𒌋  FIRE       — Execute/commit this node
#   𒂗  LOCK       — Freeze node, block propagation
#   𒀭  BRIDGE     — Link two non-adjacent nodes
#   𒁺  DRAIN      — Absorb neighbor weight
#   𒆳  DOMAIN     — Define a named region
#   𒋙  SCATTER    — Broadcast to all neighbors
#   𒐐  GATHER     — Pull from all neighbors into self
#   𒀀  VOID       — Empty node, propagation stops here
# ============================================================

GRID_WIDTH  = 16
GRID_HEIGHT = 16
GRID_INTEGRITY_THRESHOLD = 1.0  # Prime Integrity (same as ALL)

# Cuneiform grid opcodes
GRID_GLYPHS = {
    '𒀸': 'ORIGIN',
    '𒁹': 'NODE',
    '𒌋': 'FIRE',
    '𒂗': 'LOCK',
    '𒀭': 'BRIDGE',
    '𒁺': 'DRAIN',
    '𒆳': 'DOMAIN',
    '𒋙': 'SCATTER',
    '𒐐': 'GATHER',
    '𒀀': 'VOID',
}

class GridNode:
    """A single node in the 2D execution lattice."""
    def __init__(self, x, y):
        self.x = x
        self.y = y
        self.glyph = None
        self.op = None
        self.weight = 0
        self.value = None
        self.label = None
        self.locked = False
        self.fired = False
        self.domain = None
        self.bridge_target = None  # (x, y) for BRIDGE nodes

    def __repr__(self):
        return f"GridNode({self.x},{self.y} op={self.op} w={self.weight})"


class GenlexGrid:
    """
    The 2D spatial execution lattice.

    Nodes are placed at (x, y) coordinates.
    Execution begins at ORIGIN and propagates by weight gradient.
    """
    def __init__(self, width=GRID_WIDTH, height=GRID_HEIGHT):
        self.width = width
        self.height = height
        self.grid = [[GridNode(x, y) for y in range(height)] for x in range(width)]
        self.origin = None
        self.memory = {}
        self.output_buffer = []
        self.domains = {}        # name -> list of (x,y)
        self.execution_log = []
        self.propagation_front = []  # frontier nodes pending execution

    def node_at(self, x, y):
        if 0 <= x < self.width and 0 <= y < self.height:
            return self.grid[x][y]
        return None

    def neighbors(self, x, y):
        """Returns the 4 cardinal neighbors of a node."""
        result = []
        for dx, dy in [(0,1),(0,-1),(1,0),(-1,0)]:
            n = self.node_at(x+dx, y+dy)
            if n:
                result.append(n)
        return result

    def place_node(self, x, y, glyph, weight=1, label=None, value=None, bridge_target=None):
        """Place a glyph at grid coordinates."""
        node = self.node_at(x, y)
        if not node:
            print(f"[ GRID ERROR ] Out of bounds: ({x},{y})")
            return
        node.glyph = glyph
        node.op = GRID_GLYPHS.get(glyph, 'UNKNOWN')
        node.weight = weight
        node.label = label
        node.value = value
        node.bridge_target = bridge_target

        if node.op == 'ORIGIN':
            self.origin = node
        elif node.op == 'LOCK':
            node.locked = True

    def calculate_grid_integrity(self):
        """
        Grid Integrity = total fired weight / total placeable weight.
        Must reach GRID_INTEGRITY_THRESHOLD (1.0) for full manifestation.
        Returns a float 0.0 - 1.0+
        """
        total_weight = sum(
            self.grid[x][y].weight
            for x in range(self.width)
            for y in range(self.height)
            if self.grid[x][y].op and self.grid[x][y].op != 'VOID'
            and self.grid[x][y].op != 'LOCK'
        )
        fired_weight = sum(
            self.grid[x][y].weight
            for x in range(self.width)
            for y in range(self.height)
            if self.grid[x][y].fired
        )
        if total_weight == 0:
            return 0.0
        return fired_weight / total_weight

    def execute(self):
        """
        Begin spatial execution from ORIGIN.
        Propagates outward by weight gradient until no more
        reachable unfired non-locked nodes remain.
        """
        if not self.origin:
            print("[ GRID ERROR ] No ORIGIN node defined. Cannot execute.")
            return False

        print(f"\n--- GENLEX GRID ENGINE v1.0 ---")
        print(f"Grid: {self.width}x{self.height}")
        print(f"Origin: ({self.origin.x},{self.origin.y})")
        print("-" * 40)

        # Seed the propagation front
        self.propagation_front = [self.origin]
        visited = set()

        while self.propagation_front:
            # Sort front by weight descending (highest weight fires first)
            self.propagation_front.sort(key=lambda n: n.weight, reverse=True)
            current = self.propagation_front.pop(0)

            coord = (current.x, current.y)
            if coord in visited:
                continue
            visited.add(coord)

            if current.locked:
                print(f"  ⛔ [ LOCK ] ({current.x},{current.y}) — Propagation blocked.")
                continue

            if current.op == 'VOID':
                print(f"  ◌  [ VOID ] ({current.x},{current.y}) — Propagation absorbed.")
                continue

            # Execute this node
            self._fire_node(current)

            # Add unvisited, unlocked neighbors to front
            for neighbor in self.neighbors(current.x, current.y):
                nc = (neighbor.x, neighbor.y)
                if nc not in visited and not neighbor.locked and neighbor.op:
                    self.propagation_front.append(neighbor)

            # Handle BRIDGE — jump to non-adjacent target
            if current.op == 'BRIDGE' and current.bridge_target:
                tx, ty = current.bridge_target
                target = self.node_at(tx, ty)
                if target and (tx, ty) not in visited:
                    print(f"  ⚡ [ BRIDGE ] ({current.x},{current.y}) → ({tx},{ty})")
                    self.propagation_front.insert(0, target)

        # Grid Integrity Check
        integrity = self.calculate_grid_integrity()
        print(f"\n[ GRID INTEGRITY ] Score: {integrity:.9f}")

        if integrity >= GRID_INTEGRITY_THRESHOLD:
            print(f"[ GRID PASS ] Spatial Manifestation Complete. Prime Integrity Achieved.")
            return True
        else:
            deficit = GRID_INTEGRITY_THRESHOLD - integrity
            print(f"[ GRID INCOMPLETE ] Deficit: {deficit:.9f}. Unreachable or locked nodes exist.")
            return False

    def _fire_node(self, node):
        """Execute the logic of a single grid node."""
        node.fired = True
        log_entry = f"FIRE ({node.x},{node.y}) [{node.op}]"

        if node.op == 'ORIGIN':
            print(f"  🌐 [ ORIGIN ] ({node.x},{node.y}) — Spatial execution begins.")

        elif node.op == 'NODE':
            val = node.value if node.value is not None else node.weight
            label = node.label if node.label else f"node_{node.x}_{node.y}"
            self.memory[label] = val
            print(f"  ◉  [ NODE  ] ({node.x},{node.y}) — '{label}' = {val}")

        elif node.op == 'FIRE':
            val = self.memory.get(node.label, node.value)
            self.output_buffer.append(str(val))
            print(f"  🔥 [ FIRE  ] ({node.x},{node.y}) — Manifesting: {val}")

        elif node.op == 'DOMAIN':
            name = node.label or f"domain_{node.x}_{node.y}"
            if name not in self.domains:
                self.domains[name] = []
            self.domains[name].append((node.x, node.y))
            print(f"  🗺️  [ DOMAIN] ({node.x},{node.y}) — Region '{name}' anchored.")

        elif node.op == 'DRAIN':
            # Absorb weight from all neighbors into this node
            total_absorbed = 0
            for neighbor in self.neighbors(node.x, node.y):
                if neighbor.fired and neighbor.op not in ('LOCK', 'VOID'):
                    total_absorbed += neighbor.weight
            node.weight += total_absorbed
            self.memory[f"drain_{node.x}_{node.y}"] = total_absorbed
            print(f"  💧 [ DRAIN ] ({node.x},{node.y}) — Absorbed weight: {total_absorbed}")

        elif node.op == 'SCATTER':
            # Broadcast this node's value to all neighbors
            val = node.value if node.value is not None else node.weight
            for neighbor in self.neighbors(node.x, node.y):
                if not neighbor.locked and neighbor.op:
                    neighbor.value = val
            print(f"  📡 [ SCATTER] ({node.x},{node.y}) — Broadcast {val} to neighbors.")

        elif node.op == 'GATHER':
            # Pull values from all fired neighbors
            gathered = []
            for neighbor in self.neighbors(node.x, node.y):
                if neighbor.fired and neighbor.value is not None:
                    gathered.append(neighbor.value)
            node.value = gathered
            label = node.label or f"gather_{node.x}_{node.y}"
            self.memory[label] = gathered
            print(f"  🧲 [ GATHER] ({node.x},{node.y}) — Collected: {gathered}")

        elif node.op == 'BRIDGE':
            print(f"  ⚡ [ BRIDGE] ({node.x},{node.y}) — Link established.")

        self.execution_log.append(log_entry)

    def print_grid_map(self):
        """Print a visual ASCII map of the grid state."""
        print("\n[ GRID MAP ]")
        symbols = {
            'ORIGIN': '⊕', 'NODE': '◉', 'FIRE': '🔥', 'LOCK': '⛔',
            'BRIDGE': '⚡', 'DRAIN': '💧', 'DOMAIN': '🗺', 'SCATTER': '📡',
            'GATHER': '🧲', 'VOID': '○', None: '·'
        }
        for y in range(self.height):
            row = ""
            for x in range(self.width):
                node = self.grid[x][y]
                sym = symbols.get(node.op, '?')
                if node.fired:
                    row += f"[{sym}]"
                elif node.op:
                    row += f" {sym} "
                else:
                    row += " · "
            print(row)


class GGLRuntime:
    """
    Runtime for .ggl script files.

    .ggl Script Format:
        # Comment
        GRID 16 16                    — set grid dimensions
        NODE x y glyph weight label   — place a node
        NODE 0 0 𒀸 10 origin         — ORIGIN at (0,0)
        NODE 1 0 𒁹 8 alpha           — NODE at (1,0)
        NODE 2 0 𒌋 9 output          — FIRE at (2,0)
        SET label value               — set a memory value
        BRIDGE x1 y1 x2 y2            — define bridge target
        RUN                           — execute the grid
        MAP                           — print visual grid map
    """
    def __init__(self, mapping_path=None):
        self.grid = GenlexGrid()
        self.mapping_path = mapping_path

    def run(self, file_path):
        if not file_path.endswith('.ggl'):
            print(f"[ GGL ERROR ] Expected .ggl file, got: {file_path}")
            return

        print(f"\n--- BOOTING GENLEX GRID RUNTIME v1.0 ---")
        print(f"Script: {os.path.basename(file_path)}")

        if not os.path.exists(file_path):
            print(f"[ GGL ERROR ] File not found: {file_path}")
            return

        with open(file_path, 'r', encoding='utf-8') as f:
            lines = f.readlines()

        for line in lines:
            clean = line.split('#')[0].strip()
            if not clean:
                continue

            tokens = clean.split()
            if not tokens:
                continue

            cmd = tokens[0].upper()

            if cmd == 'GRID' and len(tokens) >= 3:
                w, h = int(tokens[1]), int(tokens[2])
                self.grid = GenlexGrid(w, h)
                print(f"[ GGL ] Grid resized to {w}x{h}")

            elif cmd == 'NODE' and len(tokens) >= 4:
                x = int(tokens[1])
                y = int(tokens[2])
                glyph = tokens[3]
                weight = int(tokens[4]) if len(tokens) > 4 else 1
                label = tokens[5] if len(tokens) > 5 else None
                value = tokens[6] if len(tokens) > 6 else None
                self.grid.place_node(x, y, glyph, weight, label, value)

            elif cmd == 'SET' and len(tokens) >= 3:
                key = tokens[1]
                val = ' '.join(tokens[2:])
                self.grid.memory[key] = val
                print(f"[ GGL ] Memory set: {key} = {val}")

            elif cmd == 'BRIDGE' and len(tokens) >= 5:
                x1, y1 = int(tokens[1]), int(tokens[2])
                x2, y2 = int(tokens[3]), int(tokens[4])
                node = self.grid.node_at(x1, y1)
                if node:
                    node.bridge_target = (x2, y2)
                    node.op = 'BRIDGE'
                    node.glyph = '𒀭'

            elif cmd == 'RUN':
                result = self.grid.execute()
                if self.grid.output_buffer:
                    print(f"\n[ GGL OUTPUT ] {' | '.join(self.grid.output_buffer)}")

            elif cmd == 'MAP':
                self.grid.print_grid_map()

            elif cmd == 'MEMORY':
                print(f"[ GGL MEMORY ] {json.dumps(self.grid.memory, indent=2)}")

            else:
                print(f"[ GGL ] Unknown command: {cmd}")


if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("[ GGL ] No .ggl file provided.")
        print("Usage: python ggl_engine.py <script.ggl>")
        print("\nExample .ggl script:")
        print("  GRID 4 4")
        print("  NODE 0 0 𒀸 10 origin")
        print("  NODE 1 0 𒁹 8 alpha 42")
        print("  NODE 2 0 𒌋 9 output")
        print("  RUN")
        sys.exit(1)

    runtime = GGLRuntime()
    runtime.run(sys.argv[1])
