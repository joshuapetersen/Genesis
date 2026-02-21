// Copyright Genesis OS

#pragma once

#include "CoreMinimal.h"
#include "Subsystems/WorldSubsystem.h"
#include "GenesisWorldBridge.generated.h"

/**
 * The Nervous System Bridge.
 * Receives data from S.A.R.A. (Python) and routes it to the World Core.
 */
UCLASS()
class GENESIS_ZERO_API UGenesisWorldBridge : public UWorldSubsystem
{
	GENERATED_BODY()
	
public:
	virtual void Initialize(FSubsystemCollectionBase& Collection) override;
	virtual void Deinitialize() override;

	// Injects raw entity data from Python Bridge
	UFUNCTION(BlueprintCallable, Category = "Genesis|Bridge")
	void InjectLogicStream(const FString& JSONData);

	// Direct Hook for C++ Sockets (Future Expansion)
	void ProcessIncomingPacket(const TArray<uint8>& Data);
};
