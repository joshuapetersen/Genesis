// Copyright Genesis OS

using UnrealBuildTool;
using System.Collections.Generic;

public class Genesis_ZeroTarget : TargetRules
{
	public Genesis_ZeroTarget(TargetInfo Target) : base(Target)
	{
		Type = TargetType.Game;
		DefaultBuildSettings = BuildSettingsVersion.V6;
		IncludeOrderVersion = EngineIncludeOrderVersion.Unreal5_7;
		ExtraModuleNames.Add("Genesis_Zero");
	}
}
