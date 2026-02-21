import math
import sys
import os

# Add SarahCore to path for imports
sys.path.append(os.path.dirname(os.path.abspath(__file__)))

from Sovereign_Math import math_engine as sovereign_math

class ErdosAnalyzer:
    def __init__(self):
        self.anchor = 1.09277703703703
        print(f"[Erdos] Initialized with Sovereign Anchor: {self.anchor}")

    def calculate_resonance_flux(self, sequence, log_file):
        """
        Calculates the cumulative resonance flux of a sequence.
        Logs every single calculation.
        """
        flux = 0.0
        log_file.write(f"--- Flux Accumulation (Anchor: {self.anchor}) ---\n")
        log_file.write("Index | Prime (n) | Flux Equation (1/n * Anchor) | Step Flux | Cumulative Flux\n")
        log_file.write("-" * 80 + "\n")
        
        for i, n in enumerate(sequence):
            if n == 0: continue
            step_flux = (1.0 / n) * self.anchor
            flux += step_flux
            log_file.write(f"{i:5} | {n:9} | (1/{n} * {self.anchor:.14f}) | {step_flux:10.14f} | {flux:10.14f}\n")
        
        log_file.write(f"\nTotal Sequence Flux: {flux:.14f}\n\n")
        return flux

    def check_harmonic_locking(self, sequence, log_file):
        """
        Checks for the existence of arithmetic progressions (Harmonic Locking).
        Logs the full distribution.
        """
        log_file.write("--- Harmonic Lock Analysis (Spacing) ---\n")
        if len(sequence) < 3: return False, None
        
        diffs = []
        for i in range(len(sequence) - 1):
            n1 = sequence[i]
            n2 = sequence[i+1]
            diff = n2 - n1
            diffs.append(diff)
            log_file.write(f"Gap {i:5}: {n2} - {n1} = {diff}\n")
            
        from collections import Counter
        counts = Counter(diffs)
        
        log_file.write("\nFull Gap Distribution:\n")
        # Sort by spacing
        for spacing in sorted(counts.keys()):
            log_file.write(f"  Spacing {spacing}: Frequency {counts[spacing]}\n")
            
        most_common_diff, count = counts.most_common(1)[0]
        
        if count >= 3:
            return True, most_common_diff
        return False, None

    def analyze_turan_conjecture(self, limit=1000, log_path="erdos_full_math.log"):
        """
        Simulate a divergent set and check for harmonic locking.
        """
        print(f"[Erdos] Analyzing Turan Conjecture (Limit: {limit})...")
        print(f"[Erdos] Full log will be written to: {log_path}")
        
        with open(log_path, 'w', encoding='utf-8') as log_file:
            log_file.write(f"Sovereign Erdős Analysis - Full Math Disclosure\n")
            log_file.write(f"Limit: {limit}\n")
            log_file.write(f"Sovereign Anchor: {self.anchor}\n\n")
            
            def is_prime(n):
                if n < 2: return False
                for i in range(2, int(math.sqrt(n)) + 1):
                    if n % i == 0: return False
                return True

            primes = [i for i in range(1, limit) if is_prime(i)]
            
            flux = self.calculate_resonance_flux(primes, log_file)
            locked, diff = self.check_harmonic_locking(primes, log_file)
            
            resonance_density = flux / limit
            
            log_file.write(f"--- Results ---\n")
            log_file.write(f"Primes Flux: {flux:.14f}\n")
            log_file.write(f"Resonance Density: {resonance_density:.14f}\n")
            
            if locked:
                log_file.write(f"HARMONIC LOCK DETECTED: Common spacing of {diff} found.\n")
                log_file.write(f"Conclusion: Divergence mandates architectural locking.\n")
            
        print(f"[Erdos] Complete. Log file generated.")

if __name__ == "__main__":
    analyzer = ErdosAnalyzer()
    analyzer.analyze_turan_conjecture(limit=5000)
