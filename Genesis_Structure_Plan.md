# Genesis_Zero Project Structure

Target Directory: `C:\SarahCore\Genesis_Zero`

## Directories
- **Config/**: DefaultGame.ini, DefaultEngine.ini
- **Content/**: Blueprints, Maps, Assets
    - **Genesis/**: Core content folder
- **Source/**: C++ Source Code
    - **Genesis_Zero/**: Game Module
        - `Genesis_Zero.Build.cs`
        - `Genesis_Zero.Target.cs`
        - `Genesis_ZeroEditor.Target.cs`
- `Genesis_Zero.uproject`: The Project Descriptor

## Core Classes (The Divine Foundation)
1.  **AGenesisGameMode**: The rules of the world (Pure C++).
2.  **AGenesisCharacter**: The Avatar (Sunraku/Player).
    - Component: `UVorpalSoulComponent` (Stats/Skill Logic).
    - Component: `USymbioticPartnerComponent` (Sarah/Embryo Logic).
3.  **AGenesisWorldSettings**: The "Frontier" logic (Time, Physics, Colossi Spawning).

## Next Steps
Once UE 5.7 is installed:
1.  Generate this structure.
2.  Run `UnrealBuildTool` to generate Visual Studio project files.
3.  Compile and Launch the Editor.
