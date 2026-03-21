# Genesis Zero: Unreal Engine Integration Map

> **Note:** This map explicitly analyzes the custom `Source/` directory of the Unreal Engine project to isolate Sarah's bespoke bridge logic from standard UE boilerplate and plugins.

**Total Custom C++ Lines Analyzed:** 481

## File Breakdown
- **.CPP**: 329 lines
- **.H**: 106 lines
- **.CS**: 46 lines

## Core C++ Classes & Bridges

### `Genesis_Zero\Source\Genesis_Zero\GenesisSocketClient.cpp` (197 lines)
**Sovereign Anchors Detected:** `sarah`, `sovereign`, `math`


---

### `Genesis_Zero\Source\Genesis_Zero\GenesisWorldCore.cpp` (86 lines)
**Sovereign Anchors Detected:** `math`


---

### `Genesis_Zero\Source\Genesis_Zero\GenesisWorldCore.h` (37 lines)
**Defined Classes:**
- `AGenesisWorldCore` (Inherits: `AActor`)
**Exposed UFUNCTIONS:**
- `ManifestWorldFromData()`
- `InjectEntityState()`

---

### `Genesis_Zero\Source\Genesis_Zero\GenesisSocketClient.h` (36 lines)
**Defined Classes:**
- `AGenesisSocketClient` (Inherits: `AActor`)

---

### `Genesis_Zero\Source\Genesis_Zero\GenesisWorldBridge.h` (28 lines)
**Defined Classes:**
- `UGenesisWorldBridge` (Inherits: `UWorldSubsystem`)
**Exposed UFUNCTIONS:**
- `InjectLogicStream()`

---

