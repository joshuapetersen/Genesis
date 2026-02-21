// Copyright Genesis OS

#include "GenesisWorldBridge.h"
#include "Kismet/GameplayStatics.h"
#include "GenesisWorldCore.h"

void UGenesisWorldBridge::Initialize(FSubsystemCollectionBase& Collection)
{
	Super::Initialize(Collection);
	UE_LOG(LogTemp, Warning, TEXT("[GENESIS] World Bridge Initialized. Listening for S.A.R.A."));
}

void UGenesisWorldBridge::Deinitialize()
{
	UE_LOG(LogTemp, Warning, TEXT("[GENESIS] World Bridge Severed."));
	Super::Deinitialize();
}

void UGenesisWorldBridge::InjectLogicStream(const FString& JSONData)
{
	// 1. Parse JSON (Placeholder)
	// 2. Find AGenesisWorldCore Actor
	// 3. Call ManifestWorldFromData

	AGenesisWorldCore* Core = Cast<AGenesisWorldCore>(UGameplayStatics::GetActorOfClass(GetWorld(), AGenesisWorldCore::StaticClass()));
	if (Core)
	{
		// Example: Logic Density -> Grid Size
		Core->ManifestWorldFromData(100, 150.0f);
	}
	else
	{
		UE_LOG(LogTemp, Error, TEXT("[GENESIS] No World Core found in scene!"));
	}
}

void UGenesisWorldBridge::ProcessIncomingPacket(const TArray<uint8>& Data)
{
	// Decode binary packet from C++ Socket
}
