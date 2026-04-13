"""
Genesis Physical Manifest Generator
Synthesizes ALL survivor stats into Stormcatcher Bird physical descriptions.
BIO class + legacy stats + personality + alignment + position = visual form.
"""

SURVIVORS = [
    # (rank, soul_id, name, species, personality, action, energy, alignment, age, x, y,
    #  legacy_role, hp_max, mp_max, vit, strength, agi, intel, wis, luk)
    (1,  "ALICE_101", "Erebus Devourer",                         "BIO-001", "Docile",      "Stalking prey",      338.56, +2,   5960, -263,   364,  "Producer",  865, 390, 73, 4,  0,  39, 51, 8),
    (2,  "ALICE_120", "Abaddonix",                               "BIO-008", "Territorial", "Hunting",            330.12, -70,  5960, -1738, -686,  "Apex",      345, 190, 49, 18, 20, 19, 36, 35),
    (3,  "ALICE_168", "Cacophoria, Echo of the Abyssal Pool",    "BIO-001", "Docile",      "Hunting",            328.05, -24,  5960,  1878, -1663, "Producer",  785, 390, 57, 5,  0,  39, 22, 4),
    (4,  "ALICE_261", "Echokeeper of Arcane Depths",             "BIO-001", "Docile",      "Raiding",            318.03, -23,  5960, -2042,  726,  "Producer",  945, 220, 89, 2,  0,  22, 54, 6),
    (5,  "ALICE_33",  "Aethonix, Umbra Forger",                  "BIO-005", "Docile",      "Fleeing",            309.08, -10,  5960,   128,  -29,  "Prey",      225,  50, 29, 22, 28, 5,  8,  12),
    (6,  "ALICE_117", "Cataclysmic Sylphrena",                   "BIO-002", "Docile",      "Stalking prey",      290.14, +39,  5960,   191, -1655, "Producer",  425, 270, 81, 3,  0,  22, 39, 8),
    (7,  "ALICE_184", "Aetherspire",                             "BIO-002", "Docile",      "Stalking prey",      286.59, -125, 5960,  -305, -3062, "Producer",  370, 460, 70, 3,  0,  46, 21, 6),
    (8,  "ALICE_74",  "Luminari Devourer",                       "BIO-005", "Aggressive",  "Fleeing",            259.16, +92,  5960,  1969,  1138, "Prey",      275,  70, 39, 27, 38, 7,  12, 17),
    (9,  "ALICE_127", "Echoflux Devourer",                       "BIO-002", "Docile",      "Stalking prey",      221.51, +20,  5960, -2061, -1324, "Producer",  440, 130, 84, 4,  0,  13, 23, 8),
    (10, "ALICE_129", "Khrognok, Devourer of Reflections",       "BIO-008", "Curious",     "Raiding",            207.22, -44,  5960,  2390,   34,  "Apex",      240, 190, 28, 12, 24, 19, 37, 45),
    (11, "820dad08",  "Proto_820dad",                            "Primordial","Neutral",    "Trading",            178.72, -2,   5960,  1768,  2055, "Unknown",   None,None,None,None,None,None,None,None),
    (12, "ALICE_28",  "Echokeeper of the Abyssal Hymn",          "BIO-009", "Parasitic",   "Trading",            177.00, +31,  5960,   875,  1119, "Builder",   240, 360, 24, 29, 18, 36, 38, 11),
    (13, "290b46b8",  "Unknown",                                 "Unknown", "Neutral",     "Building territory", 166.99, -55,  2367,    22,   272, "Unknown",   None,None,None,None,None,None,None,None),
    (14, "ALICE_45",  "Crepuscular Devourer",                    "BIO-005", "Symbiotic",   "Building territory", 162.25, -3,   5960,    36, -3107, "Prey",      205,  80, 25, 23, 34, 8,  12, 17),
    (15, "ccf4a9e8",  "Unknown",                                 "Unknown", "Neutral",     "Trading",            155.91, -56,  5960,   463,   541, "Unknown",   None,None,None,None,None,None,None,None),
    (16, "ALICE_29",  "Abaddonus Crawler",                       "BIO-003", "Aggressive",  "Building territory", 152.32, +48,  5960,   -96,   266, "Scavenger",  20,  10,  3,  4, 23, 1,  1,  5),
    (17, "33ba3358",  "Unknown",                                 "Unknown", "Neutral",     "Trading",            150.61, -18,  1757,   283,  -241, "Unknown",   None,None,None,None,None,None,None,None),
    (18, "ALICE_124", "Ophidian Matriarch of Ephemeral Luminance","BIO-002","Docile",      "Trading",            136.67, -119, 5960,  -185,    28, "Producer",  330, 230, 62, 4,  0,  23, 35, 9),
    (19, "ALICE_245", "Aeropex Oblivion",                        "BIO-007", "Parasitic",   "Trading",            125.39, -27,  5960,  1106,  -732, "Predator",  100,  90, 12, 5,  42, 9,  19, 29),
    (20, "61037a59",  "Unknown",                                 "Unknown", "Neutral",     "Trading",            121.20, -71,  3587,    89,   531, "Unknown",   None,None,None,None,None,None,None,None),
    (21, "6fa0c619",  "Unknown",                                 "Unknown", "Neutral",     "Trading",            114.06, +13,   536,   -43,   231, "Unknown",   None,None,None,None,None,None,None,None),
    (22, "ALICE_160", "Erebusia",                                "BIO-001", "Docile",      "Building territory", 107.59, +43,  5960,  -629,  -150, "Producer",  955, 460, 91, 2,  0,  46, 56, 10),
    (23, "ALICE_46",  "Cacophorax",                              "BIO-008", "Parasitic",   "Building territory", 107.39, +70,  5960,  2719,   -50, "Apex",      260, 380, 32, 13, 17, 38, 34, 44),
    (24, "794520a9",  "Unknown",                                 "Unknown", "Neutral",     "Diplomacy",          103.70, +43,  4197,  -767,   194, "Unknown",   None,None,None,None,None,None,None,None),
    (25, "4b226ba4",  "Unknown",                                 "Unknown", "Neutral",     "Diplomacy",           82.54, +31,  2977,  -423,  -324, "Unknown",   None,None,None,None,None,None,None,None),
    (26, "ALICE_106", "Aeropex: Stormsurge",                     "BIO-007", "Nomadic",     "Building territory",  81.75, +70,  5960,    19, -1720, "Predator",  115,  60, 15, 8,  54, 6,  19, 20),
    (27, "14dec542",  "Proto_14dec5",                            "Primordial","Neutral",    "Resting",             61.09, +2,   5960,  1310,   811, "Unknown",   None,None,None,None,None,None,None,None),
    (28, "792136ef",  "Unknown",                                 "Unknown", "Neutral",     "Resting",             49.09, 0,    1146,   699,   -39, "Unknown",   None,None,None,None,None,None,None,None),
    (29, "e9612f1c",  "Unknown",                                 "Unknown", "Neutral",     "Patrolling",          46.05, -50,  5426,   756,   762, "Unknown",   None,None,None,None,None,None,None,None),
    (30, "ALICE_183", "Elderwood Wraith",                        "BIO-001", "Docile",      "Patrolling",          45.70, -33,  5960,  -173, -1646, "Producer",  820, 270, 64, 1,  0,  27, 55, 8),
    (31, "ALICE_1",   "Abyssal Cacophony",                       "BIO-005", "Docile",      "Wandering",           38.89, -51,  5960,   971, -3272, "Prey",      250,  80, 34, 24, 40, 8,  8,  10),
    (32, "ALICE_141", "Cerberus Scarabrix",                      "BIO-003", "Territorial", "Resting",             28.66, +22,  5960,  2298,  1458, "Scavenger",  10,  10,  1,  3, 41, 1,  1,  26),
    (33, "ALICE_69",  "Elysian Screamstress",                    "BIO-005", "Docile",      "Resting",             26.23, +53,  5960, -1879,  1129, "Prey",      255,  40, 35, 30, 29, 4,  10, 20),
    (34, "ALICE_59",  "Erebus Devourer",                         "BIO-005", "Nomadic",     "Foraging",            18.43, +53,  5960, -3431,  1205, "Prey",      190,  70, 22, 22, 34, 7,  11, 16),
]

BIO_CLASS = {
    "BIO-001": "Producer / Geoid-class",
    "BIO-002": "Producer / Thermal-class",
    "BIO-003": "Scavenger / Crawler-class",
    "BIO-005": "Prey / Velocity-class",
    "BIO-007": "Predator / Raptor-class",
    "BIO-008": "Apex / Shadow-class",
    "BIO-009": "Builder / Silk-class",
    "Primordial": "Genesis-born / Unformed-class",
    "Unknown": "Genesis-born / Unclassified",
}

def dist_to_origin(x, y):
    import math
    return math.sqrt(x**2 + y**2)

# ── Per-BIO-class morphology archetypes ───────────────────────────────
MORPHOLOGY = {
    "BIO-001": ("Living Monolith / Sessile Geoid",
                "No wings or limbs. A colossal organism of overlapping obsidian-crystal plates "
                "anchored to a fixed gravity point. Mana-rich ash drifts toward its central "
                "intake maw. Movement is gravitational projection, not locomotion."),
    "BIO-002": ("Atmospheric Gas-Bag / Cephalopod",
                "Massive buoyant form — hollow internal chambers filled with primordial gas. "
                "Trails long bioluminescent filaments that harvest ambient mana. Pulses slowly "
                "through the sulfurous upper atmosphere like a living weather balloon."),
    "BIO-003": ("Insectoid Scavenger / Crawler",
                "Small, twitchy, armored exoskeleton. Covered in fine sensory hairs that detect "
                "heat and necrotic mana from the 252 dead. High-AGI sprint across crust fractures "
                "and volcanic vents. Mandibles designed for cracking mineral deposits."),
    "BIO-005": ("Velocity-class Runner / Terrestrial",
                "Lean bipedal or quadrupedal runner built for speed over mass. "
                "Hollow-shelled exterior with heat-dispersion ridges along the spine. "
                "Adapted to ground-level antigravity anomalies — uses terrain as a weapon."),
    "BIO-007": ("Stormcatcher Avion / True Aerial",
                "The only confirmed avian lineage among the survivors. Needle-thin body, "
                "four primary wing-planes of crystalline static. Never lands. Uses "
                "gravity differentials as lift rather than muscular thrust. "
                "Wingspan scaled to 10x legacy frame — a living aircraft."),
    "BIO-008": ("Hexapedal Apex Crawler / Basalt-Armored",
                "Six-limbed terrestrial predator. Carapace of fused volcanic basalt "
                "plates over dense musculature. Moves on the underside of floating islands "
                "using magnetic anchor-talons. Head is a cluster of vertical mandibles."),
    "BIO-009": ("Ribbon-Serpent / Aqueous Builder",
                "Long, ribbon-like entity of semi-solid mercury texture. No fixed limbs — "
                "moves by flowing along gravity gradients. Uses its fluid body to wrap "
                "around other entities, stabilizing their antigravity vectors in exchange "
                "for a portion of their Genesis Energy."),
    "Primordial": ("Amorphous Lattice / Living Geometry",
                   "No biological legacy template. Appears as shifting fractal geometry — "
                   "a floating polyhedron that reshapes itself in real time. "
                   "Communicates through light-pulse frequency and angular rotation. "
                   "100% native to Genesis physics; no Aethelgard inheritance."),
    "Unknown": ("Genesis-Native / Unclassified Form",
                "No BIO archive exists for this entity. Form is observed but unclassified — "
                "appears to be settling into a stable geometry after 2,000+ years of "
                "environmental imprinting. Currently indistinguishable from a slow-moving "
                "atmospheric pressure system."),
}

def build_body(species):
    arch, desc = MORPHOLOGY.get(species, MORPHOLOGY["Unknown"])
    return arch, desc

def build_surface(vit, alignment, species):
    """Surface texture/coloring derived from VIT and alignment."""
    if species == "BIO-007":  # avion — keeps feather language
        if alignment <= -50: return "Feathers: Dark-void filaments that absorb surrounding light; trailing edges serrated"
        if alignment >= 50:  return "Feathers: Gold-tinged crystalline filaments, radiating warmth in flight"
        return "Feathers: Semi-translucent razor-edged filaments vibrating at the heartbeat frequency"
    if vit is None:
        return "Surface: Raw shifting energy membrane — no fixed texture"
    if vit >= 80:
        if alignment <= -50: return "Surface: Black obsidian plate-armour, light-absorbing. Fracture lines glow deep violet"
        return "Surface: Dense crystalline plating, pale-grey with amber bioluminescent pulse veins"
    if vit >= 40:
        if alignment <= -30: return "Surface: Matte charcoal chitin, chipped from combat. Cracks sealed with cooled magma"
        return "Surface: Smooth basalt-grey shell with thermal heat-venting ridges"
    return "Surface: Thin, semi-transparent membrane — internal energy flow visible beneath the skin"

def build_eyes(alignment, intel, species):
    if species in ("Primordial", "Unknown"):
        return "No eyes. Senses through geometric reorientation toward energy gradients"
    if species == "BIO-002":
        return "No centralised optical organs. Distributed photoreceptor patches across the membrane surface"
    if alignment is None: return "Unformed optical sensors"
    if alignment >= 50:  return "Bright amber — warm broadcasting alignment; wide-set for atmospheric scanning"
    if alignment >= 10:  return "Soft luminescent gold — measured, observant"
    if alignment >= -20: return "Pale grey-white — neutral, calculating"
    if alignment >= -60: return "Deep violet, narrowed — constant threat-assessment"
    return "Pitch black with a single red geometric iris — zero warmth, absolute predator"

def build_movement(agi, personality, action, species):
    verb = action.lower()
    if species == "BIO-001":
        return f"Stationary. Projects gravitational lure fields. Currently {verb} via passive draw"
    if species == "BIO-002":
        return f"Slow atmospheric drift on gas buoyancy. Currently {verb} at altitude"
    if species == "BIO-003":
        return f"High-speed skittering across crust fractures. Currently {verb}"
    if species == "BIO-007":
        if agi and agi >= 40:
            return f"Perpetual high-altitude flight — never descends. Currently {verb}"
        return f"Sustained glide along antigravity thermals. Currently {verb}"
    if species == "BIO-008":
        return f"Magnetic-anchor crawl across vertical and inverted surfaces. Currently {verb}"
    if species == "BIO-009":
        return f"Flows along gravity gradients — no locomotion, pure fluid displacement. Currently {verb}"
    if species in ("Primordial", "Unknown"):
        return f"Rotational drift — reorients geometry toward nearest energy source. Currently {verb}"
    return f"Terrain-adaptive movement. Currently {verb}"

def build_distinguishing(s):
    rank,soul_id,name,species,personality,action,energy,alignment,age,x,y,lr,hp,mp,vit,strength,agi,intel,wis,luk = s
    marks = []
    if wis and wis >= 50:
        marks.append("cosmic-awareness aura — other entities instinctively yield territory")
    if intel and intel >= 40:
        marks.append("emits structured harmonic resonance — a living computational frequency")
    if luk and luk >= 40:
        marks.append("probability distortion field — statistically improbable survival confirmed")
    if strength and strength >= 20 and species not in ("BIO-001", "BIO-002"):
        marks.append("primary strike appendages capable of piercing BIO-001 crystalline plating")
    if mp and mp >= 400:
        marks.append("dense mana reservoirs — visible as glowing internal channels through semi-transparent housing")
    if energy < 50:
        marks.append("surface degradation visible — outermost layer flaking. Entropy is winning")
    if alignment <= -100:
        marks.append("alignment shadow — space discolors 3 meters in every direction around it")
    if alignment >= 80:
        marks.append("constant low-level luminescence bleeds from secondary surfaces")
    if personality == "Nomadic" and species == "BIO-007":
        marks.append("leaves gravity-bead trail markers — a 5,000-year migration path encoded in solidified energy")
    if personality == "Symbiotic":
        marks.append("surface emits synchronisation pulses — nearby entities unconsciously match its heartbeat")
    if not marks:
        marks.append("no visible surface mutation — survival written entirely in behavior")
    return "; ".join(marks)

output = [
    "=" * 76,
    "  S.A.R.A.H. GENESIS — PHYSICAL MANIFEST",
    "  Multi-Morphology Survivor Taxonomy",
    "  Sim Year: 5,368  |  Survivors: 34  |  Extinct: 252",
    "=" * 76,
    "",
    "  NOT all survivors share the same morphology. BIO-class determines form.",
    "  Only BIO-007 entities retain the Stormcatcher Avion body plan.",
    "  All others mutated into the form demanded by their environment and stats.",
    "",
]


for s in SURVIVORS:
    rank,soul_id,name,species,personality,action,energy,alignment,age,x,y,lr,hp,mp,vit,strength,agi,intel,wis,luk = s
    d = dist_to_origin(x, y)
    bio_arch, bio_desc = build_body(species)

    output.append("─" * 76)
    output.append(f"  RANK #{rank:02d}  ·  {('A.L.I.C.E. LEGACY' if soul_id.startswith('ALICE_') else 'PRIMORDIAL ORIGIN')}  ·  {species}")
    output.append("─" * 76)
    output.append(f"  ENTITY       : {name}")
    output.append(f"  SOUL ID      : {soul_id}")
    output.append(f"  PERSONALITY  : {personality}")
    output.append(f"  CURRENT ACT  : {action}")
    output.append(f"  ALIGNMENT    : {alignment:+d}")
    output.append(f"  ENERGY       : {energy:.2f}")
    output.append(f"  AGE          : {age:,} sim years")
    output.append(f"  DIST ORIGIN  : {d:.0f} world units")
    output.append(f"  LEGACY ROLE  : {lr}")
    if hp:
        output.append(f"  STATS        : HP {hp}  MP {mp}  |  VIT {vit}  STR {strength}  AGI {agi}  INT {intel}  WIS {wis}  LUK {luk}")
    output.append("")
    output.append(f"  ── MORPHOLOGY: {bio_arch} ──")
    output.append(f"  Form     : {bio_desc}")
    output.append(f"  {build_surface(vit, alignment, species)}")
    output.append(f"  Eyes     : {build_eyes(alignment, intel, species)}")
    output.append(f"  Movement : {build_movement(agi, personality, action, species)}")
    output.append(f"  Marks    : {build_distinguishing(s)}")
    output.append("")

output.append("=" * 76)
output.append("  END OF MANIFEST")
output.append("=" * 76)

with open(r'C:\PrimordialEarth\Genesis_Physical_Manifest.txt', 'w', encoding='utf-8') as f:
    f.write('\n'.join(output))

print(f"[S.A.R.A.H] Physical Manifest written. {len(SURVIVORS)} entity descriptions generated.")
print(f"[S.A.R.A.H] File: C:\\PrimordialEarth\\Genesis_Physical_Manifest.txt")
