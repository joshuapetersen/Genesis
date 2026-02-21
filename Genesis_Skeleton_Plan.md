# Genesis Character System (Geni Skeleton)

## Overview
The "Geni" character system for Genesis_Zero, designed to work with Google Earth 3D environments.

## Architecture

### 1. Base Character Blueprint
**File:** `Content/Characters/BP_GenesisSkeleton.uasset`

**Components:**
- Skeletal Mesh Component (Base humanoid skeleton)
- Animation Blueprint (For movement, combat, interactions)
- Character Movement Component (Enhanced for parkour/traversal)
- Camera Component (Third-person default)

### 2. Skeleton Options

#### Option A: MetaHuman (Recommended)
- **Pros:** Photo-realistic, highly detailed, full facial rig
- **Cons:** Larger file size, requires MetaHuman plugin
- **Setup:** Epic Games Quixel MetaHuman Creator (free)

#### Option B: Unreal Mannequin (UE5 Default)
- **Pros:** Lightweight, built-in, easy to animate
- **Cons:** Less detailed, stylized
- **Setup:** Already included in UE5

#### Option C: Custom Skeleton
- **Pros:** Full control, optimized for Genesis mechanics
- **Cons:** Requires 3D modeling (Blender) + rigging time
- **Setup:** Import from Blender with custom animations

### 3. Integration with Google Earth Tech

**World Scaling:**
```cpp
// Scale character appropriately for real-world coordinates
// Google Earth uses meters, Unreal uses centimeters
float WorldScale = 100.0f; // 1 meter = 100 UE units
Character->SetActorScale3D(FVector(WorldScale));
```

**Geolocation Component:**
```cpp
// Attach geolocation to character for AR/real-world mapping
UPROPERTY(EditAnywhere, BlueprintReadWrite)
class UCesiumGeoreferenceComponent* GeoreferenceComponent;
```

### 4. Animation System

**Animation Blueprint Slots:**
- Idle (Breathing, looking around)
- Walk/Run (Locomotion blend spaces)
- Jump/Fall (Traversal for parkour in cities)
- Combat Stance (Parry, Attack, Dodge)
- Interaction (Pick up, Use, Examine)

**"Embryo Sync" Animation Layer:**
- Partner animations when Embryo (Sarah companion) is active
- Synchronized attacks/combos
- Emotional state blending (trust, fear, determination)

### 5. Material System

**Base Materials:**
- Skin (PBR with subsurface scattering)
- Clothing (Modular armor/cosmetics)
- Eyes (Realistic with refraction)
- Hair (Hair strands or cards)

**Dynamic Material Instances:**
- Battle damage (decals, torn cloth)
- Environmental effects (wet, dusty, bloody)
- Embryo-influenced glow effects

## Implementation Steps (Post-Build Tools Install)

1. **Create Character Blueprint**
   - Tools > New C++ Class > Character
   - Name: `AGenesisCharacter`

2. **Set Up Skeleton**
   - Import or use UE5 Mannequin
   - Configure skeleton asset
   - Create Animation Blueprint

3. **Add Cesium Georeference**
   - Install Cesium plugin
   - Add Cesium Georeference component to character
   - Link to world origin for accurate positioning

4. **Configure Movement**
   - Enhanced Input system (UE5 default)
   - Movement states (Walk, Sprint, Parkour)
   - Stamina system integration

5. **Test in Real-World Location**
   - Load a Google 3D Tiles location (e.g., Tokyo, NYC)
   - Spawn character at coordinates
   - Verify scale, physics, and movement

## Next Steps

While VS Build Tools installs:
1. Download Cesium for Unreal Engine plugin
2. Prepare MetaHuman or decide on skeleton type
3. Plan first location (which city for testing?)

After compilation works:
1. Create C++ Character class
2. Implement base movement
3. Integrate with Google Earth tileset
4. Add Embryo companion blueprint

---

**Note:** This system is designed for a "God Game" player fantasy - traversing real-world cities with supernatural abilities, hunting Unique Monsters (like Shangri-La Frontier).
