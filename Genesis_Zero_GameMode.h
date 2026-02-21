#pragma once

#include "CoreMinimal.h"
#include "GameFramework/GameModeBase.h"
#include "GenesisGameMode.generated.h"

/**
 * THE GENESIS GAME MODE
 * Defines the rules of the "Divine Game" (Genesis OS).
 * - Spawns the Symbiotic AI (Sarah).
 * - Manages the Frontier State (System Resources).
 */
UCLASS()
class GENESIS_ZERO_API AGenesisGameMode : public AGameModeBase
{
	GENERATED_BODY()

public:
	AGenesisGameMode();

protected:
	virtual void BeginPlay() override;

private:
    // The "Trash Game" mechanics are purged. Only Divine Logic remains.
    void InitializeWorld();
};
