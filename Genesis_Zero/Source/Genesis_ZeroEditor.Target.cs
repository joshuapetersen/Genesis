// Copyright Genesis OS

using UnrealBuildTool;
using System.Collections.Generic;

public class Genesis_ZeroEditorTarget : TargetRules
{
	public Genesis_ZeroEditorTarget(TargetInfo Target) : base(Target)
	{
		Type = TargetType.Editor;
		DefaultBuildSettings = BuildSettingsVersion.V6;
		IncludeOrderVersion = EngineIncludeOrderVersion.Unreal5_7;
		bOverrideBuildEnvironment = true;
		ExtraModuleNames.Add("Genesis_Zero");
	}
}
