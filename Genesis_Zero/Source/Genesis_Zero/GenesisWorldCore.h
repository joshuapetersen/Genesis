// Copyright Genesis OS

#pragma once

#include "CoreMinimal.h"
#include "GameFramework/Actor.h"
#include "ProceduralMeshComponent.h"
#include "Components/InstancedStaticMeshComponent.h"
#include "GenesisWorldCore.generated.h"

UCLASS()
class GENESIS_ZERO_API AGenesisWorldCore : public AActor
{
	GENERATED_BODY()
	
public:	
	AGenesisWorldCore();

protected:
	virtual void BeginPlay() override;

public:	
	// The core renderer for the terrain
	UPROPERTY(VisibleAnywhere, BlueprintReadOnly, Category = "Genesis|World")
	UProceduralMeshComponent* ProceduralMesh;

	// The renderer for thousands of entities (Mass ECS Visualizer)
	UPROPERTY(VisibleAnywhere, BlueprintReadOnly, Category = "Genesis|Entities")
	UInstancedStaticMeshComponent* EntityISM;

	// The Visual Manifestation Function
	UFUNCTION(BlueprintCallable, Category = "Genesis|Architect")
	void ManifestWorldFromData(int32 GridSize, float Spacing);

	UFUNCTION(BlueprintCallable, Category = "Genesis|Architect")
	void InjectEntityState(const TArray<FTransform>& EntityTransforms);
};
