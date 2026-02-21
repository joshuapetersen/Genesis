// Copyright Genesis OS

#include "GenesisWorldCore.h"
#include "KismetProceduralMeshLibrary.h"

AGenesisWorldCore::AGenesisWorldCore()
{
	PrimaryActorTick.bCanEverTick = true;

	ProceduralMesh = CreateDefaultSubobject<UProceduralMeshComponent>(TEXT("ProceduralMesh"));
	ProceduralMesh->bUseAsyncCooking = true;
	RootComponent = ProceduralMesh;

	EntityISM = CreateDefaultSubobject<UInstancedStaticMeshComponent>(TEXT("EntityISM"));
	EntityISM->SetupAttachment(RootComponent);
	EntityISM->SetCollisionEnabled(ECollisionEnabled::NoCollision); // Optimization for Mass Entity
}

void AGenesisWorldCore::BeginPlay()
{
	Super::BeginPlay();
	
	// Auto-Manifest a starting zone if needed
	ManifestWorldFromData(100, 100.0f);
}

void AGenesisWorldCore::ManifestWorldFromData(int32 GridSize, float Spacing)
{
	TArray<FVector> Vertices;
	TArray<int32> Triangles;
	TArray<FVector> Normals;
	TArray<FVector2D> UV0;
	TArray<FColor> Colors;
	TArray<FProcMeshTangent> Tangents;

	// Shangri-La Terrain Generation (Simple Sine Wave for now)
	for (int32 X = 0; X <= GridSize; X++)
	{
		for (int32 Y = 0; Y <= GridSize; Y++)
		{
			float Z = FMath::Sin(X * 0.1f) * FMath::Cos(Y * 0.1f) * 200.0f;
			Vertices.Add(FVector(X * Spacing, Y * Spacing, Z));
			UV0.Add(FVector2D(X, Y));
			Colors.Add(FColor::White); // Base Color
		}
	}

	for (int32 X = 0; X < GridSize; X++)
	{
		for (int32 Y = 0; Y < GridSize; Y++)
		{
			int32 BottomLeft = X * (GridSize + 1) + Y;
			int32 BottomRight = BottomLeft + 1;
			int32 TopLeft = BottomLeft + (GridSize + 1);
			int32 TopRight = TopLeft + 1;

			Triangles.Add(BottomLeft);
			Triangles.Add(TopLeft);
			Triangles.Add(TopRight);

			Triangles.Add(BottomLeft);
			Triangles.Add(TopRight);
			Triangles.Add(BottomRight);
		}
	}

	// Calculate Normals/Tangents auto? Or manual?
	// For speed, let's let Kismet library helper or just Calculate automatically if passing empty
	// Actually CreateMeshSection requires them, but CalculateTangentsForMesh can generate them.

	// Generating simplified normals (Up)
	for(const FVector& V : Vertices) Normals.Add(FVector::UpVector); 
	for(const FVector& V : Vertices) Tangents.Add(FProcMeshTangent(1,0,0));

	ProceduralMesh->CreateMeshSection(0, Vertices, Triangles, Normals, UV0, Colors, Tangents, true);
}

void AGenesisWorldCore::InjectEntityState(const TArray<FTransform>& EntityTransforms)
{
	// Batch update instances
	EntityISM->ClearInstances();
	for (const FTransform& Trans : EntityTransforms)
	{
		EntityISM->AddInstance(Trans);
	}
}
