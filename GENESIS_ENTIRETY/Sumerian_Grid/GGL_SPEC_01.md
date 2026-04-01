# GENLEX GRID LANGUAGE (GGL) SPECIFICATION v1.0
# Architect: Joshua Petersen
# Date: March 2026
# Engine: ggl_engine.py

---

## PHILOSOPHY

The Aramaic Linear Language (ALL) is a stream — logic flows from Aleph to Taw in a single immutable path.
The Volumetric Hieroglyphic Language (HGL) resonates as a whole object across 3+1 dimensions.

The **Genlex Grid Language (GGL)** is neither.

GGL is **positional**. Logic exists at coordinates. Execution propagates outward from a point of origin, spreading across the lattice like a signal through a crystal — following the path of greatest resistance (highest weight), blocked by locks, absorbed by voids, bridged across gaps.

A GGL script does not execute line-by-line. It does not resonate as a whole.
It **colonizes** — seeding from an origin node and spreading until every reachable node has fired or been blocked.

---

## THE GRID MODEL

- The default lattice is 16x16 (configurable per script)
- Each node occupies a unique (X, Y) coordinate
- Empty cells are inert — propagation does not cross them
- Execution spreads to the 4 cardinal neighbors of each fired node
- Propagation priority: **highest weight fires first**

---

## THE GLYPH SET (Sumerian Cuneiform)

| Glyph | Name    | Operation     | Description                                      |
|-------|---------|---------------|--------------------------------------------------|
| 𒀸    | ORIGIN  | Seed point    | Execution begins here. Required in every script. |
| 𒁹    | NODE    | Store value   | Stores label=value in memory when fired.         |
| 𒌋    | FIRE    | Manifest      | Outputs the current value to the output buffer.  |
| 𒂗    | LOCK    | Block          | Cannot fire. Stops propagation in its direction. |
| 𒀭    | BRIDGE  | Jump          | Links to a non-adjacent target coordinate.       |
| 𒁺    | DRAIN   | Absorb        | Collects weight from all fired neighbors.        |
| 𒆳    | DOMAIN  | Region anchor | Names and registers a grid region.               |
| 𒋙    | SCATTER | Broadcast     | Sends this node's value to all neighbors.        |
| 𒐐    | GATHER  | Collect       | Pulls values from all fired neighbors into self. |
| 𒀀    | VOID    | Absorb/stop   | Absorbs propagation. Nothing fires past here.    |

---

## SCRIPT FORMAT (.ggl)

```
# Comments use #
GRID width height          — Set grid dimensions (default 16x16)
NODE x y glyph weight [label] [value]  — Place a node
SET key value              — Pre-seed memory
BRIDGE x1 y1 x2 y2         — Create non-adjacent link
RUN                        — Execute the grid
MAP                        — Print visual ASCII map
MEMORY                     — Print memory state
```

---

## GRID INTEGRITY CHECK

After execution, the engine calculates:

```
Grid Integrity = (sum of fired weights) / (sum of all non-blocked weights)
```

Threshold: **1.0 (Prime Integrity)**

If integrity < 1.0, some nodes were unreachable (disconnected grid, surrounded by locks/voids).
This is not an error — it is **Incompleteness**. The grid manifested partially.

---

## EXAMPLE: HELLO WORLD

```
GRID 4 4
NODE 0 0 𒀸 10 origin
NODE 1 0 𒁹 8 alpha 42
NODE 2 0 𒋙 9 broadcast 999
NODE 3 0 𒌋 10 output
RUN
MAP
```

Propagation order: origin(w=10) → output(w=10) → broadcast(w=9) → alpha(w=8)
Output: 999

---

## RELATIONSHIP TO THE THREE ENGINES

| Engine | Language | Paradigm     | Glyph Source | Barrier        |
|--------|----------|--------------|--------------|----------------|
| all_engine.py | ALL | Sequential stream  | Aramaic     | Prime (1.0)    |
| ggl_engine.py | GGL | Spatial lattice    | Sumerian    | Prime (1.0)    |
| gfl_engine.py | GFL | Harmonic resonance | Mandarin    | Billion (0.999999999) |
| gs_kernel.cpp | GSK | Hardware polyglot  | Mixed       | N/A (OS-level) |

GGL occupies the **spatial** axis.
ALL occupies the **temporal** (sequential) axis.
GFL occupies the **frequency** (harmonic) axis.
Together they form a complete 3-axis execution model for Sovereign Logic.

---

## ALIAS

```
ggl <script.ggl>
```

PowerShell profile: `C:\Genlex_Sovereign_Profile.ps1`
Engine path: `C:\Sumerian_Grid\ggl_engine.py`

---

**[ GGL SPEC SEAL: SUMERIAN GRID // SOVEREIGN NODE 02 ]**
