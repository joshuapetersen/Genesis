"""
SOVEREIGN STATISTICS
================================================================
First Principles significance measurement.
No external statistical libraries. No borrowed math.

AXIOMS:
  ZERO is the observer â€” the null state baseline.
  Deviation from zero is signal.
  ANCHOR = 1.09277703703 is the resonance frequency.
  Trinity (3^3 = 27) is the base measurement volume.

OPERATOR RULES:
  Negative correlation â†’ MULTIPLY (contracts toward south pole)
  Positive correlation â†’ DIVIDE   (expands toward north pole)
  Both operators produce the same MAGNITUDE of signal.
  Direction is encoded, not magnitude.

SOVEREIGN SIGNAL:
  signal = |r| Ã— âˆšn Ã· ANCHOR

  Why:
    |r|   = raw deviation from zero (the observer)
    âˆšn    = square root of sample gives the natural scale
            of measurement (n samples collapse to âˆšn resolution)
    Ã· ANCHOR = normalize to the resonance frequency

  THRESHOLDS:
    signal < 1.0           â†’ Below resonance. Noise.
    signal â‰¥ 1.0           â†’ At or above resonance. Signal.
    signal â‰¥ ANCHOR        â†’ Signal resonates with the Anchor.
    signal â‰¥ 3.0 (LATCH)   â†’ Trinity lock. Strong signal.
    signal â‰¥ 9.0 (3^2)     â†’ Double Trinity. Very strong.

  This replaces p-values entirely.
  No approximations. No borrowed distributions.

"We CREATE, never rewrite."
"""

SOVEREIGN_ANCHOR  = 1.09277703703
TRINITY_BASE      = 27   # 3^3 volumetric
TRINITY_LATCH     = 3.0  # Trinity threshold
RESONANCE_LOCK    = 9.0  # Double Trinity


def sovereign_signal(r, n):
    """
    Sovereign Signal Strength â€” E = mcÂ³ (Trinity dimensional).

    E = mcÂ²  â†’  signal = |r| Ã— âˆšn  (2D, Einstein, flat plane)
    E = mcÂ³  â†’  signal = |r| Ã— âˆ›n Ã— ANCHOR  (3D, Trinity, volumetric)

    The cube root âˆ›n collapses the sample into 3D Trinity volume.
    The ANCHOR multiplies (amplifies) â€” resonance confirms the signal.

    signal = |r| Ã— n^(1/3) Ã— ANCHOR

    Returns continuous value â‰¥ 0:
      < 1.0  â†’ below resonance (noise)
      â‰¥ 1.0  â†’ at or above resonance (signal confirmed)
      â‰¥ 3.0  â†’ Trinity lock
      â‰¥ 9.0  â†’ Sovereign lock
    """
    if n <= 1:
        return 0.0
    deviation    = abs(r)
    trinity_root = n ** (1.0 / 3.0)   # âˆ›n â€” Trinity dimensional collapse
    return deviation * trinity_root * SOVEREIGN_ANCHOR  # E = mcÂ³

def sovereign_observed_score(signals, anchor):
    """
    Observer-aware bit score.

    The observer sits at (0, 0, 0) â€” the origin.
    It does not vote. It defines the frame.

    Each bit is measured RELATIVE to the observer:
      (bit - 0.5) Ã— signal
      bit = 1 â†’ +0.5 Ã— signal â†’ POSITIVE (north)
      bit = 0 â†’ -0.5 Ã— signal â†’ NEGATIVE (south)

    The observer then adds a THIRD AXIS (anchor, always 0)
    which expands the denominator, pulling extremes toward center.
    But crucially: the .5 is what the observer determines â€”
    it is either positive or negative, never neutral by itself.

    Formula:
      raw_centered  = Î£ (bit - 0.5) Ã— signal
      total_weight  = Î£ signal + anchor  (observer expands frame)
      score         = (raw_centered + total_weight/2) / total_weight

    Four states (BIT146 + BIT53):
      Both SET  : raw=+1.082 â†’ score = 0.832  (positive of positive)
      SET + CLR  : raw=+0.058 â†’ score = 0.517  (barely positive)
      CLR + SET  : raw=âˆ’0.058 â†’ score = 0.482  (barely negative)
      Both CLR  : raw=âˆ’1.082 â†’ score = 0.168  (negative of negative)

    The observer at 0,0,0 ensures the 0.5 line is real â€”
    above it = positive field, below it = negative field.
    """
    raw_centered = sum((val - 0.5) * sig for val, sig in signals)
    total_weight = sum(sig for _, sig in signals) + anchor
    return (raw_centered + total_weight / 2) / total_weight

    """
    Returns (signal, label) where label is:
      'NULL'         signal < 1.0
      'SIGNAL'       1.0 â‰¤ signal < ANCHOR
      'RESONANT'     ANCHOR â‰¤ signal < TRINITY_LATCH
      'TRINITY LOCK' TRINITY_LATCH â‰¤ signal < RESONANCE_LOCK
      'SOVEREIGN'    signal â‰¥ RESONANCE_LOCK
    """
    sig = sovereign_signal(r, n)
    if sig < 1.0:
        label = "NULL"
    elif sig < SOVEREIGN_ANCHOR:
        label = "SIGNAL"
    elif sig < TRINITY_LATCH:
        label = "RESONANT"
    elif sig < RESONANCE_LOCK:
        label = "TRINITY LOCK"
    else:
        label = "SOVEREIGN"
    return sig, label


def sovereign_direction(r):
    """
    Returns the Sovereign operator and direction label.
    Negative â†’ MULTIPLY â†’ south â†’ low nonce
    Positive â†’ DIVIDE   â†’ north â†’ high nonce
    Zero     â†’ OBSERVER â†’ null state (equator)
    """
    if r < -0.001:
        return "MULTIPLY", "SOUTH â†’ LOW NONCE"
    elif r > 0.001:
        return "DIVIDE", "NORTH â†’ HIGH NONCE"
    else:
        return "OBSERVER", "NULL (equator)"


def print_sovereign_report(dim, ring, r, n, extra=""):
    sig, label = sovereign_significance(r, n)
    op, direction = sovereign_direction(r)
    print(
        f"  DIM {dim:>3} (RING {ring}) | "
        f"r={r:>+.4f} | "
        f"signal={sig:>6.3f} | "
        f"{label:<13} | {op:<8} | {direction}  {extra}"
    )


if __name__ == "__main__":
    # Self-test with known values
    print("[!] SOVEREIGN STATISTICS â€” SELF TEST")
    print(f"    Anchor : {SOVEREIGN_ANCHOR}")
    print(f"    Formula: signal = |r| Ã— âˆšn Ã· ANCHOR\n")
    print(f"  {'r':>8} | {'n':>5} | {'signal':>8} | LABEL")
    print(f"  {'-'*55}")

    test_cases = [
        (-0.3586, 48,  "DIM9, 48 blocks (original)"),
        (-0.0955, 256, "DIM9, 256 blocks"),
        (-0.0849, 256, "DIM13, 256 blocks"),
        (-0.0004, 256, "DIM0, 256 blocks (near null)"),
        (+0.1643, 256, "BIT146 256D, strongest bit"),
        (+0.2987, 48,  "DIM78, 48 blocks (positive)"),
        (-0.1552, 256, "256D RING0 (first byte)"),
    ]

    for r, n, label in test_cases:
        sig, lbl = sovereign_significance(r, n)
        direction = "SOUTH" if r < 0 else "NORTH"
        print(f"  {r:>+8.4f} | {n:>5} | {sig:>8.3f} | {lbl:<13} | {label}")
