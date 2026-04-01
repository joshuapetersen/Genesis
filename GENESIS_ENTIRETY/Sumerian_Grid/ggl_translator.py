import os
import sys
import io
import json

# Force UTF-8 for glyph rendering
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
sys.stderr = io.TextIOWrapper(sys.stderr.buffer, encoding='utf-8')

# GENLEX GRID LOGIC (GGL)
# Architecture: 60x60 Matrix (Sexagesimal)
# Logic: Spatial Indexing vs Linear Parsing

class CGLTranslator:
    def __init__(self):
        self.grid_size = 60
        # Mapping base cuneiform wedges to axis modifiers
        self.lexicon = {
            "𒀸": {"axis": "X", "value": 1},   # DIŠ-H (Horizontal)
            "𒁹": {"axis": "Y", "value": 1},   # DIŠ-V (Vertical)
            "𒌋": {"axis": "Z", "value": 10},  # U (Winkelhaken/Depth)
            "𒀭": {"axis": "CMD", "value": 0, "op": "INIT_GRID"}, # AN (Header)
            "𒂗": {"axis": "CMD", "value": 1, "op": "EXEC_LOOP"} # EN (Control)
        }

    def translate(self, sequence: str):
        print(f"--- INITIATING GGL VOLUMETRIC TRANSLATION ---")
        print(f"Input: {sequence}")
        
        matrix_coords = []
        current_x = 0
        current_y = 0
        current_z = 0
        
        for char in sequence:
            if char in self.lexicon:
                item = self.lexicon[char]
                if item["axis"] == "X":
                    current_x = (current_x + item["value"]) % self.grid_size
                elif item["axis"] == "Y":
                    current_y = (current_y + item["value"]) % self.grid_size
                elif item["axis"] == "Z":
                    current_z = (current_z + item["value"]) % self.grid_size
                elif item["axis"] == "CMD":
                    print(f"  > [OP] {item['op']} trigger at ({current_x}, {current_y}, {current_z})")
            
            # Record state at each step as a coordinate point
            matrix_coords.append((current_x, current_y, current_z))

        print(f"\n[ RESULT ] Volumetric Mapping Complete.")
        print(f"Total Grid Points: {len(matrix_coords)}")
        print(f"Final Coordinate Anchor: ({current_x}, {current_y}, {current_z})")
        
        return matrix_coords

    def verify_resonance(self, coords):
        # Volumetric Resonance: Check if total volume aligns with sexagesimal harmonics
        resonance = sum(x + y + z for x, y, z in coords) % 60
        print(f"Volumetric Resonance Index: {resonance}")
        return resonance == 0

if __name__ == "__main__":
    translator = CGLTranslator()
    # Test: Enuma Elish Fragment (Fragmentary logic)
    # AN + 3 DIŠ + 2 U + EN
    test_seq = "𒀭𒀸𒀸𒀸𒌋𒌋𒂗"
    coords = translator.translate(test_seq)
    translator.verify_resonance(coords)
