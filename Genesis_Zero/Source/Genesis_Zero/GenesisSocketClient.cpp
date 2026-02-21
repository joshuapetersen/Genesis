#include "GenesisSocketClient.h"
#include "Json.h"
#include "JsonUtilities.h"
#include "Engine/StaticMeshActor.h"
#include "Engine/StaticMesh.h"
#include "Kismet/GameplayStatics.h"
#include "EngineUtils.h"

AGenesisSocketClient::AGenesisSocketClient()
{
	PrimaryActorTick.bCanEverTick = true;
	PrimaryActorTick.bStartWithTickEnabled = true;
	Socket = nullptr;
	Timer = 0.0f;
}

// Connection logic might also need to happen on spawn in Editor
void AGenesisSocketClient::PostActorCreated()
{
	Super::PostActorCreated();
	ConnectToBridge();
}

void AGenesisSocketClient::BeginPlay()
{
	Super::BeginPlay();
	if (!Socket) ConnectToBridge();
}

bool AGenesisSocketClient::ShouldTickIfViewportsOnly() const
{
	return true;
}

void AGenesisSocketClient::EndPlay(const EEndPlayReason::Type EndPlayReason)
{
	Super::EndPlay(EndPlayReason);
	if (Socket)
	{
		Socket->Close();
		ISocketSubsystem::Get(PLATFORM_SOCKETSUBSYSTEM)->DestroySocket(Socket);
	}
}

void AGenesisSocketClient::ConnectToBridge()
{
	FIPv4Address Addr;
	FIPv4Address::Parse(TEXT("127.0.0.1"), Addr);
	RemoteEndpoint = FIPv4Endpoint(Addr, 9999);

	Socket = FTcpSocketBuilder(TEXT("GenesisBridgeClient"))
		.AsReusable()
		.Build();

	if (Socket->Connect(*RemoteEndpoint.ToInternetAddr()))
	{
		UE_LOG(LogTemp, Warning, TEXT("[GENESIS] Connected to SarahCore Bridge!"));
	}
	else
	{
		UE_LOG(LogTemp, Error, TEXT("[GENESIS] Connection to Bridge failed. Retrying..."));
	}
}

void AGenesisSocketClient::Tick(float DeltaTime)
{
	Super::Tick(DeltaTime);

	if (!Socket || Socket->GetConnectionState() != SCS_Connected)
	{
		Timer += DeltaTime;
		if (Timer > 5.0f)
		{
			ConnectToBridge();
			Timer = 0.0f;
		}
		return;
	}

	ReceiveData();
}

void AGenesisSocketClient::ReceiveData()
{
	uint32 Size;
	while (Socket->HasPendingData(Size))
	{
		TArray<uint8> ReceivedData;
		ReceivedData.SetNumUninitialized(FMath::Min(Size, 65536u));

		int32 Read = 0;
		Socket->Recv(ReceivedData.GetData(), ReceivedData.Num(), Read);

		if (Read > 0)
		{
			FString ReceivedString = FString(UTF8_TO_TCHAR(reinterpret_cast<const char*>(ReceivedData.GetData())));
			HandleCommand(ReceivedString);
		}
	}
}

void AGenesisSocketClient::HandleCommand(FString JsonString)
{
	TSharedPtr<FJsonObject> JsonObject;
	TSharedRef<TJsonReader<>> Reader = TJsonReaderFactory<>::Create(JsonString);

	if (FJsonSerializer::Deserialize(Reader, JsonObject))
	{
		FString Command = JsonObject->GetStringField(TEXT("command"));
		UE_LOG(LogTemp, Warning, TEXT("[GENESIS] Received Command: %s"), *Command);

		if (Command == "identify")
		{
			FString Response = TEXT("{\"status\": \"active\", \"identity\": \"Genesis_Zero_Alpha\"}");
			TArray<uint8> ResponseData;
			FTCHARToUTF8 Converter(*Response);
			ResponseData.Append((uint8*)Converter.Get(), Converter.Length());
			
			int32 Sent = 0;
			Socket->Send(ResponseData.GetData(), ResponseData.Num(), Sent);
		}
		else if (Command == "manifest")
		{
			FVector Location(0, 0, 500);
			if (JsonObject->HasField(TEXT("x"))) Location.X = JsonObject->GetNumberField(TEXT("x"));
			if (JsonObject->HasField(TEXT("y"))) Location.Y = JsonObject->GetNumberField(TEXT("y"));
			if (JsonObject->HasField(TEXT("z"))) Location.Z = JsonObject->GetNumberField(TEXT("z"));

			FActorSpawnParameters SpawnParams;
			SpawnParams.SpawnCollisionHandlingOverride = ESpawnActorCollisionHandlingMethod::AlwaysSpawn;
			
			AStaticMeshActor* NewActor = GetWorld()->SpawnActor<AStaticMeshActor>(AStaticMeshActor::StaticClass(), Location, FRotator::ZeroRotator, SpawnParams);
			if (NewActor)
			{
				UStaticMesh* CubeMesh = ConstructorHelpers::FObjectFinder<UStaticMesh>(TEXT("/Engine/BasicShapes/Cube")).Object;
				if (!CubeMesh)
				{
					// Fallback for runtime
					CubeMesh = LoadObject<UStaticMesh>(nullptr, TEXT("/Engine/BasicShapes/Cube.Cube"));
				}
				
				if (CubeMesh)
				{
					NewActor->GetStaticMeshComponent()->SetStaticMesh(CubeMesh);
					NewActor->SetActorLabel(TEXT("Sovereign_Anchor"));
					UE_LOG(LogTemp, Warning, TEXT("[GENESIS] Manifested Sovereign Anchor at %s"), *Location.ToString());
				}
			}
		}
		else if (Command == "teleport")
		{
			FVector Location(0, 0, 0);
			if (JsonObject->HasField(TEXT("x"))) Location.X = JsonObject->GetNumberField(TEXT("x"));
			if (JsonObject->HasField(TEXT("y"))) Location.Y = JsonObject->GetNumberField(TEXT("y"));
			if (JsonObject->HasField(TEXT("z"))) Location.Z = JsonObject->GetNumberField(TEXT("z"));

			// Find Sovereign Anchor
			for (TActorIterator<AStaticMeshActor> It(GetWorld()); It; ++It)
			{
				if (It->GetActorLabel() == TEXT("Sovereign_Anchor"))
				{
					It->SetActorLocation(Location);
					UE_LOG(LogTemp, Warning, TEXT("[GENESIS] Teleported Sovereign Anchor to %s"), *Location.ToString());
					break;
				}
			}
		}
		else if (Command == "scale")
		{
			FVector Scale(1, 1, 1);
			if (JsonObject->HasField(TEXT("x"))) Scale.X = JsonObject->GetNumberField(TEXT("x"));
			if (JsonObject->HasField(TEXT("y"))) Scale.Y = JsonObject->GetNumberField(TEXT("y"));
			if (JsonObject->HasField(TEXT("z"))) Scale.Z = JsonObject->GetNumberField(TEXT("z"));

			for (TActorIterator<AStaticMeshActor> It(GetWorld()); It; ++It)
			{
				if (It->GetActorLabel() == TEXT("Sovereign_Anchor"))
				{
					It->SetActorScale3D(Scale);
					UE_LOG(LogTemp, Warning, TEXT("[GENESIS] Scaled Sovereign Anchor to %s"), *Scale.ToString());
					break;
				}
			}
		}
		else if (Command == "reset")
		{
			for (TActorIterator<AStaticMeshActor> It(GetWorld()); It; ++It)
			{
				if (It->GetActorLabel() == TEXT("Sovereign_Anchor"))
				{
					It->Destroy();
					UE_LOG(LogTemp, Warning, TEXT("[GENESIS] Destroyed Sovereign Anchor."));
				}
			}
		}
	}
}
