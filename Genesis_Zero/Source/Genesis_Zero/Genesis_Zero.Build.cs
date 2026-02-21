// Copyright Genesis OS

using UnrealBuildTool;

public class Genesis_Zero : ModuleRules
{
	public Genesis_Zero(ReadOnlyTargetRules Target) : base(Target)
	{
		PCHUsage = PCHUsageMode.UseExplicitOrSharedPCHs;

		PublicDependencyModuleNames.AddRange(new string[] { "Core", "CoreUObject", "Engine", "InputCore", "EnhancedInput", "Sockets", "Networking", "Json", "JsonUtilities", "ProceduralMeshComponent" });

		PrivateDependencyModuleNames.AddRange(new string[] {  });
	}
}
