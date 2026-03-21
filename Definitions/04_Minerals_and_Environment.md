# 04 - Minerals and Environment

## The Living VRAM Terrian
The physical world is not just a mesh in Unreal Engine. It is a mathematical grid in the CuPy GPU matrix. The environment itself has Stats, XP, and Elements.

## Geological Elements (The Earth)
Minerals spawn procedurally on the Z-axis terrain matrix. They act as "Stationary High-Density XP Nodes". When an entity (like a Sapient) mines or absorbs them, they gain massive stat boosts based on the mineral.

1. **Iron Ore (Common)**
   - **Effect:** +1 STR permanently when consumed/forged.
   - **Spawn:** Mountains / High Z-axis.

2. **Mana Crystal (Uncommon)**
   - **Effect:** +5 INT permanently. Restores full MP.
   - **Spawn:** Deep forests or Arcane Pools.

3. **Mithril (Rare)**
   - **Effect:** +10 AGI, +10 STR. 
   - **Spawn:** Deep underground / Caverns.

4. **Orichalcum (Legendary)**
   - **Effect:** +50 All Stats. Gives the [Absolute Carapace] gene dormant sequence.
   - **Spawn:** Only drops from Ancient Earth UBMs.

## Environmental Stress Zones (The 3 Pillars of Mutation)
The map has invisible overlapping zones that track data. Spending time in these zones induces "Environmental Stress" in an entity's genome, paving the way for Tri-Synthesis Mutations.

1. **The Blood Plateau (High Violence Zones)**
   - **Trigger:** If more than 50 combat interactions happen in a 1000m radius, this zone is born.
   - **Stress:** Applies the `Blood Soaked` variable.
   - **Mutation:** Entities that level up here evolve into aggressive, physical, berserker-type variants. (E.g., A wolf mutating into a **Blood-Lycan UBM**).

2. **The Arcane Pools (High Magic Zones)**
   - **Trigger:** Generated naturally by high concentrations of Mana Crystals or Ancient Oaks.
   - **Stress:** Applies the `Mana Saturation` variable.
   - **Mutation:** Entities here evolve into magical variants. (E.g., A bug mutating into an **Arcane Beetle UBM**).

3. **The Scorched Earth (High Heat/Fire Zones)**
   - **Trigger:** Areas hit by [Meteor Swarm] or high-level fire spells, or natural volcanoes.
   - **Stress:** Applies the `Heat Resistance` variable.
   - **Mutation:** Entities adapt to the heat, becoming immune to fire and often learning fire spells innately. (E.g., A bird mutating into a **Phoenix UBM**).

4. **The Void Rifts (Data Corruption Zones)**
   - **Trigger:** Areas where the Hypervisor struggles with physics clipping, or near the absolute edges of the bounds (`> 19000.0`).
   - **Stress:** Applies the `Spatial Anomaly` variable.
   - **Mutation:** This is the most dangerous zone. Entities here mutate into completely unpredictable **Glitch/Void UBMs** that defy normal physics (e.g., teleporting instead of walking).
