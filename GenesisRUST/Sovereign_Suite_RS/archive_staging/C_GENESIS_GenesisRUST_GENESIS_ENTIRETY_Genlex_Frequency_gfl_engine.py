import sys
import io
import os
import math
import json
import time

sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

# ============================================================
# GFL ENGINE v1.0: GENLEX FREQUENCY LANGUAGE
# Role: Harmonic Resonance Execution Engine
# Paradigm: Logic fires based on FREQUENCY INTERVALS, not
#           sequential order or spatial position.
#
#           Unlike Linear (Aâ†’Z stream) or Grid (spatial spread),
#           Frequency logic is TEMPORAL â€” nodes are tuned to
#           frequencies. Execution is a waveform. Nodes fire
#           when the master pulse aligns with their harmonic.
#
# Script extension: .gfl
# Alias: gfl
# Path: C:\Genlex_Frequency\gfl_engine.py
#
# THE FREQUENCY MODEL:
#   - The SOVEREIGN PULSE runs at 1.09277703703 Hz
#   - Each node is tuned to a HARMONIC of that pulse
#     (1x, 2x, 3x... or fractional: 0.5x, 0.25x)
#   - At each TICK, the engine evaluates which nodes resonate
#   - A node fires when: |pulse_phase - node_phase| < EPSILON
#   - INTERFERENCE: Two nodes at the same frequency either
#     CONSTRUCTIVELY (amplify) or DESTRUCTIVELY (cancel) interfere
#   - BARRIER: The Billion Barrier (0.999999999) â€” a node's
#     cumulative resonance across all ticks must exceed this
#     for full manifestation
#
# MANDARIN FREQUENCY GLYPHS (Frequency-Native):
#   é¢‘  PULSE    â€” The master frequency emitter
#   è°  HARMONIC â€” A node tuned to a specific harmonic
#   å…±  RESONATE â€” Fire when in phase with master
#   æ¶ˆ  CANCEL   â€” Destructive interference node
#   æ”¾  AMPLIFY  â€” Constructive interference node
#   é”  LOCK     â€” Frequency lock (phase cannot drift)
#   é‡Š  RELEASE  â€” Unlock a locked frequency
#   æ³¢  WAVE     â€” Continuous output stream
#   å³°  PEAK     â€” Fire only at maximum amplitude
#   è°·  TROUGH   â€” Fire only at minimum amplitude
#
# CUNEIFORM HYBRID (from existing TSDN reflex set):
#   ð’€¸  ORIGIN   â€” Start of frequency sequence
#   ð’Œ‹  COMMIT   â€” Lock in current resonance state
# ============================================================

SOVEREIGN_FREQUENCY = 1.09277703703   # The master pulse Hz
BILLION_BARRIER     = 0.999999999        # Resonance threshold
FREQUENCY_EPSILON   = 0.05              # Phase alignment tolerance
DEFAULT_TICKS       = 9                  # 9 inhibitor cycles + 1

# Mandarin frequency opcode table
FREQ_GLYPHS = {
    'é¢‘': 'PULSE',
    'è°': 'HARMONIC',
    'å…±': 'RESONATE',
    'æ¶ˆ': 'CANCEL',
    'æ”¾': 'AMPLIFY',
    'é”': 'LOCK',
    'é‡Š': 'RELEASE',
    'æ³¢': 'WAVE',
    'å³°': 'PEAK',
    'è°·': 'TROUGH',
    'ð’€¸': 'ORIGIN',
    'ð’Œ‹': 'COMMIT',
}


class FrequencyNode:
    """
    A node in the frequency lattice.
    Fires based on harmonic alignment with the master pulse.
    """
    def __init__(self, name):
        self.name = name
        self.glyph = None
        self.op = None
        self.frequency = SOVEREIGN_FREQUENCY  # default: fundamental
        self.harmonic = 1.0                   # multiplier of base freq
        self.phase_offset = 0.0               # radians
        self.amplitude = 1.0
        self.value = None
        self.locked = False
        self.resonance_accumulator = 0.0
        self.fire_count = 0
        self.output = []
        self.dependencies = []               # nodes that must fire first
        self.interference_partners = []      # (node_name, type) pairs

    def effective_frequency(self):
        return self.frequency * self.harmonic

    def phase_at_tick(self, tick, tick_duration):
        """Calculate this node's phase at a given tick."""
        t = tick * tick_duration
        return (2 * math.pi * self.effective_frequency() * t) + self.phase_offset

    def __repr__(self):
        return f"FreqNode({self.name} f={self.effective_frequency():.4f}Hz amp={self.amplitude})"


class SovereignPulse:
    """
    The master pulse running at SOVEREIGN_FREQUENCY.
    All nodes are measured against this.
    """
    def __init__(self, frequency=SOVEREIGN_FREQUENCY):
        self.frequency = frequency
        self.tick = 0
        self.tick_duration = 1.0 / frequency  # seconds per tick

    def phase_at(self, tick):
        """Master pulse phase at this tick (radians)."""
        t = tick * self.tick_duration
        return 2 * math.pi * self.frequency * t

    def amplitude_at(self, tick):
        """Master pulse amplitude at this tick (-1.0 to 1.0)."""
        return math.sin(self.phase_at(tick))

    def advance(self):
        self.tick += 1
        return self.tick


class GenlexFrequencyEngine:
    """
    The harmonic execution engine.

    Nodes fire when their phase aligns with the master pulse.
    The engine runs for DEFAULT_TICKS cycles (the 9+1 pattern).
    """
    def __init__(self):
        self.pulse = SovereignPulse()
        self.nodes = {}          # name -> FrequencyNode
        self.memory = {}
        self.output_buffer = []
        self.execution_log = []
        self.wave_streams = {}   # name -> list of values over time
        self.tick_count = DEFAULT_TICKS

    def add_node(self, name, glyph, harmonic=1.0, phase_offset=0.0,
                 amplitude=1.0, value=None):
        """Register a frequency node."""
        node = FrequencyNode(name)
        node.glyph = glyph
        node.op = FREQ_GLYPHS.get(glyph, 'UNKNOWN')
        node.harmonic = harmonic
        node.frequency = SOVEREIGN_FREQUENCY
        node.phase_offset = phase_offset
        node.amplitude = amplitude
        node.value = value

        if node.op == 'LOCK':
            node.locked = True

        self.nodes[name] = node
        return node

    def add_interference(self, node_a, node_b, interference_type='CONSTRUCTIVE'):
        """Register interference between two nodes."""
        if node_a in self.nodes and node_b in self.nodes:
            self.nodes[node_a].interference_partners.append((node_b, interference_type))
            self.nodes[node_b].interference_partners.append((node_a, interference_type))

    def _is_in_phase(self, node, t_offset):
        """
        Check if a node is in resonance with the master pulse at this tick offset.
        t_offset is tick + 0.25 to avoid sin(0) = 0 at tick 0.
        """
        t = t_offset / SOVEREIGN_FREQUENCY
        master_phase = 2 * math.pi * self.pulse.frequency * t
        node_phase   = 2 * math.pi * node.effective_frequency() * t + node.phase_offset
        diff = abs(master_phase - node_phase) % (2 * math.pi)
        if diff > math.pi:
            diff = 2 * math.pi - diff
        return diff < FREQUENCY_EPSILON

    def _calculate_node_resonance(self, node, t_offset):
        """
        Calculate this node's resonance score at this tick.
        Accounts for interference with partner nodes.
        """
        t = t_offset / SOVEREIGN_FREQUENCY
        base_resonance = node.amplitude * abs(math.sin(
            2 * math.pi * node.effective_frequency() * t + node.phase_offset
        ))

        # Apply interference
        interference_modifier = 0.0
        for partner_name, itype in node.interference_partners:
            partner = self.nodes.get(partner_name)
            if not partner:
                continue
            t = t_offset / SOVEREIGN_FREQUENCY
            partner_res = partner.amplitude * abs(math.sin(
                2 * math.pi * partner.effective_frequency() * t + partner.phase_offset
            ))
            if itype == 'CONSTRUCTIVE':
                interference_modifier += partner_res * 0.5
            elif itype == 'DESTRUCTIVE':
                interference_modifier -= partner_res * 0.5

        return max(0.0, base_resonance + interference_modifier)

    def execute(self):
        """
        Run the frequency engine for tick_count cycles.
        At each tick, evaluate which nodes are in phase and fire them.
        """
        print(f"\n--- GENLEX FREQUENCY ENGINE v1.0 ---")
        print(f"Master Pulse: {SOVEREIGN_FREQUENCY} Hz")
        print(f"Ticks: {self.tick_count} (9+1 Sovereign Cycle)")
        print(f"Barrier: {BILLION_BARRIER}")
        print("-" * 40)

        if not self.nodes:
            print("[ GFL ERROR ] No frequency nodes defined.")
            return False

        # Sort nodes by harmonic for display
        sorted_nodes = sorted(self.nodes.values(), key=lambda n: n.harmonic)
        for node in sorted_nodes:
            print(f"  Node '{node.name}' | {node.op} | {node.effective_frequency():.4f}Hz "
                  f"| phase offset: {node.phase_offset:.3f}rad")

        print(f"\n[ PULSE START ]\n")

        for tick in range(self.tick_count):
            # Offset by 0.25 period so tick 0 doesn't evaluate at sin(0)=0
            t_offset = tick + 0.25
            master_amp = math.sin(2 * math.pi * self.pulse.frequency * (t_offset / SOVEREIGN_FREQUENCY))
            print(f"  Tick {tick+1:02d} | Master Pulse: {master_amp:+.4f}")

            tick_fired = []
            for node in self.nodes.values():
                if node.locked:
                    continue

                in_phase = self._is_in_phase(node, t_offset)
                resonance = self._calculate_node_resonance(node, t_offset)
                node.resonance_accumulator += resonance

                if in_phase:
                    self._fire_node(node, t_offset, resonance)
                    tick_fired.append(node.name)

            if tick_fired:
                print(f"           â””â”€ Fired: {', '.join(tick_fired)}")

            self.pulse.advance()

        print(f"\n[ FREQUENCY BARRIER CHECK ]")
        return self._check_barrier()

    def _fire_node(self, node, tick, resonance):
        """Execute a node's logic when it fires."""
        node.fire_count += 1

        if node.op == 'ORIGIN':
            print(f"           ðŸŒ ORIGIN '{node.name}' â€” Frequency sequence initiated.")

        elif node.op == 'PULSE':
            # The master emitter â€” its resonance feeds all nodes
            self.memory['_pulse_tick'] = tick
            # No explicit output â€” its role is captured in accumulator

        elif node.op == 'HARMONIC':
            val = node.value if node.value is not None else resonance
            self.memory[node.name] = val
            self.execution_log.append(f"tick{tick}:HARMONIC:{node.name}={val}")

        elif node.op == 'RESONATE':
            val = self.memory.get(node.name, node.value)
            self.output_buffer.append(str(val))
            print(f"           ðŸ”” RESONATE '{node.name}' â€” Manifesting: {val} "
                  f"(resonance={resonance:.4f})")

        elif node.op == 'AMPLIFY':
            # Double the amplitude of all neighbors in the frequency space
            for other in self.nodes.values():
                if other.name != node.name and other.harmonic == node.harmonic:
                    other.amplitude = min(2.0, other.amplitude * 1.5)
            print(f"           ðŸ“¶ AMPLIFY '{node.name}' â€” Harmonic field boosted.")

        elif node.op == 'CANCEL':
            # Destructively interfere with opposing phase nodes
            for other in self.nodes.values():
                if other.name != node.name:
                    other.amplitude = max(0.0, other.amplitude * 0.5)
            print(f"           ðŸ”‡ CANCEL '{node.name}' â€” Destructive interference.")

        elif node.op == 'WAVE':
            # Record value at every tick (continuous stream)
            if node.name not in self.wave_streams:
                self.wave_streams[node.name] = []
            self.wave_streams[node.name].append({
                'tick': tick,
                'resonance': resonance,
                'amplitude': node.amplitude
            })

        elif node.op == 'PEAK':
            # Only fires at local maximum â€” check resonance is above mean
            avg_resonance = (node.resonance_accumulator / max(1, node.fire_count))
            if resonance >= avg_resonance:
                val = node.value if node.value is not None else resonance
                self.output_buffer.append(str(val))
                print(f"           â›°ï¸  PEAK '{node.name}' â€” Peak resonance: {resonance:.4f}")

        elif node.op == 'TROUGH':
            # Only fires at local minimum
            avg_resonance = (node.resonance_accumulator / max(1, node.fire_count))
            if resonance <= avg_resonance * 0.5:
                val = node.value if node.value is not None else resonance
                self.output_buffer.append(str(val))
                print(f"           ðŸ”ï¸  TROUGH '{node.name}' â€” Trough resonance: {resonance:.4f}")

        elif node.op == 'COMMIT':
            # Seal the current memory state
            seal = json.dumps(self.memory)
            self.memory['_seal'] = seal
            print(f"           ðŸ”’ COMMIT '{node.name}' â€” State sealed at tick {tick}.")

        elif node.op == 'RELEASE':
            for other in self.nodes.values():
                if other.locked:
                    other.locked = False
            print(f"           ðŸ”“ RELEASE '{node.name}' â€” All locks cleared.")

    def _check_barrier(self):
        """
        Check Billion Barrier across all nodes.
        Each node's resonance accumulator is normalized against
        the theoretical maximum resonance over tick_count ticks.
        """
        # Max theoretical = tick_count * average max resonance (0.637 = 2/pi, avg of |sin|)
        max_theoretical = self.tick_count * (2.0 / math.pi)
        results = []

        for node in self.nodes.values():
            if node.op in ('LOCK', 'VOID'):
                continue
            normalized = node.resonance_accumulator / max_theoretical
            passed = normalized >= BILLION_BARRIER
            results.append((node.name, normalized, passed))
            status = "âœ… PASS" if passed else "âŒ INCOMPLETE"
            print(f"  Node '{node.name}': {normalized:.9f} {status}")

        all_passed = all(r[2] for r in results)
        total_score = sum(r[1] for r in results) / len(results) if results else 0.0

        print(f"\n  Total Frequency Score: {total_score:.9f}")
        if all_passed:
            print(f"[ GFL PASS ] Billion Barrier achieved across all nodes. "
                  f"Harmonic Manifestation Complete.")
        else:
            failed = [r[0] for r in results if not r[2]]
            print(f"[ GFL INCOMPLETE ] Nodes below barrier: {', '.join(failed)}")
            print(f"  Consider: increase tick_count, adjust harmonics, "
                  f"or add AMPLIFY nodes.")

        return all_passed

    def print_wave_report(self):
        """Print a summary of all continuous WAVE stream nodes."""
        if not self.wave_streams:
            return
        print(f"\n[ WAVE STREAMS ]")
        for name, stream in self.wave_streams.items():
            peak = max(s['resonance'] for s in stream)
            avg = sum(s['resonance'] for s in stream) / len(stream)
            print(f"  '{name}': {len(stream)} samples | peak={peak:.4f} | avg={avg:.4f}")


class GFLRuntime:
    """
    Runtime for .gfl script files.

    .gfl Script Format:
        # Comment
        TICKS 18                         â€” set number of execution cycles
        NODE name glyph harmonic [phase] [amp] [value]
        NODE pulse_root é¢‘ 1.0           â€” PULSE at fundamental
        NODE alpha è° 2.0 0.0 1.0 42    â€” HARMONIC at 2nd harmonic
        NODE output å…± 1.0               â€” RESONATE at fundamental
        NODE stream æ³¢ 3.0               â€” WAVE at 3rd harmonic
        INTERFERE node_a node_b CONSTRUCTIVE
        INTERFERE node_a node_c DESTRUCTIVE
        RUN
        REPORT
    """
    def __init__(self):
        self.engine = GenlexFrequencyEngine()

    def run(self, file_path):
        if not file_path.endswith('.gfl'):
            print(f"[ GFL ERROR ] Expected .gfl file, got: {file_path}")
            return

        print(f"\n--- BOOTING GENLEX FREQUENCY RUNTIME v1.0 ---")
        print(f"Script: {os.path.basename(file_path)}")

        if not os.path.exists(file_path):
            print(f"[ GFL ERROR ] File not found: {file_path}")
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

            if cmd == 'TICKS' and len(tokens) >= 2:
                self.engine.tick_count = int(tokens[1])
                print(f"[ GFL ] Tick count set to {self.engine.tick_count}")

            elif cmd == 'NODE' and len(tokens) >= 3:
                name    = tokens[1]
                glyph   = tokens[2]
                harmonic= float(tokens[3]) if len(tokens) > 3 else 1.0
                phase   = float(tokens[4]) if len(tokens) > 4 else 0.0
                amp     = float(tokens[5]) if len(tokens) > 5 else 1.0
                value   = tokens[6] if len(tokens) > 6 else None
                self.engine.add_node(name, glyph, harmonic, phase, amp, value)

            elif cmd == 'INTERFERE' and len(tokens) >= 4:
                node_a = tokens[1]
                node_b = tokens[2]
                itype  = tokens[3].upper() if len(tokens) > 3 else 'CONSTRUCTIVE'
                self.engine.add_interference(node_a, node_b, itype)
                print(f"[ GFL ] Interference: {node_a} â†” {node_b} ({itype})")

            elif cmd == 'SET' and len(tokens) >= 3:
                key = tokens[1]
                val = ' '.join(tokens[2:])
                self.engine.memory[key] = val
                print(f"[ GFL ] Memory: {key} = {val}")

            elif cmd == 'RUN':
                result = self.engine.execute()
                if self.engine.output_buffer:
                    print(f"\n[ GFL OUTPUT ] {' | '.join(self.engine.output_buffer)}")

            elif cmd == 'REPORT':
                self.engine.print_wave_report()

            else:
                print(f"[ GFL ] Unknown command: {cmd}")


if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("[ GFL ] No .gfl file provided.")
        print("Usage: python gfl_engine.py <script.gfl>")
        print("\nExample .gfl script:")
        print("  TICKS 9")
        print("  NODE root é¢‘ 1.0")
        print("  NODE alpha è° 2.0 0.0 1.0 hello")
        print("  NODE output å…± 1.0")
        print("  INTERFERE root alpha CONSTRUCTIVE")
        print("  RUN")
        sys.exit(1)

    runtime = GFLRuntime()
    runtime.run(sys.argv[1])
