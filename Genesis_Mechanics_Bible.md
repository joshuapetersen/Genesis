# Genesis Mechanics Bible: The Divine Systems

> "The Pinnacle of VRMMO Design."

## I. Combat & Skill (Shangri-La Frontier)
The core loop is **High-Skill Action**, not stat-checking.

1.  **Just-Defend System (Parry):**
    *   **Concept:** A frame-perfect block negates all damage and fills the "Vorpal Gauge."
    *   **UE5 Implementation:** `AGenesisCharacter::OnParry()` triggers time-dilation using `GlobalTimeDilation`.
2.  **Trash Game Synergies (Exploits):**
    *   **Concept:** Unintended physics interactions (e.g., cancelling a dash into a jump for momentum).
    *   **UE5 Implementation:** Utilizing `LaunchCharacter` with additive velocity vectors based on input timing.
3.  **Unique Monsters (Colossi):**
    *   **Antagonists:** Lycagon (Night), Wezaemon (Undead), Ctarnidd (Deep Sea).
    *   **Mechanic:** Dynamic AI using **Behavior Trees** that adapt to player patterns. Defeating one grants a "World Flag" (Unique Item).

## II. Evolution (Infinite Dendrogram)
The Progression System is **Symbiotic**.

1.  **The Embryo (Sarah):**
    *   **Type:** *Maiden-with-Arms* (Partner + Weapon).
    *   **Evolution:** The Embryo evolves based on playstyle.
        *   *Aggressive:* Evolves into High-DPS Weaponry.
        *   *Defensive:* Evolves into Castle/Shields.
        *   *Tactical:* Evolves into Chariot/Mobility.
    *   **UE5 Implementation:** `USymbioticPartnerComponent` tracking player stats (DamageDealt vs DamageTaken) to select the next `UDataAsset` for evolution.

## III. Mastery (Sword Art Online)
The Skill System is **Classless**.

1.  **Weapon Proficiency:**
    *   **Concept:** Using a weapon unlocks its "Sword Skills" (Animation Montages).
    *   **The Switch:** Toggling agro between Player and Embryo.
    *   **UE5 Implementation:** A `TMap<EWeaponType, float>` mastery table. High mastery unlocks new *Input Combo Contexts* in Enhanced Input.

## IV. Territory (Wise Man's Pupil)
The Mid-Game is **Kingdom Building**.

1.  **The Citadel:**
    *   **Concept:** The file system (`C:\SarahCore`) is the Kingdom.
    *   **Building:** Directories are Towers. Files are Resources.
    *   **UE5 Implementation:** A procedural city generator (`PCG Graph`) that reads the file structure to spawn buildings.

*Compiled by Research Agent.*
