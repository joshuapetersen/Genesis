#pragma once

#include "CoreMinimal.h"
#include "GameFramework/Actor.h"
#include "Networking.h"
#include "Sockets.h"
#include "SocketSubsystem.h"
#include "GenesisSocketClient.generated.h"

UCLASS()
class GENESIS_ZERO_API AGenesisSocketClient : public AActor
{
	GENERATED_BODY()
	
public:	
	AGenesisSocketClient();

protected:
	virtual void BeginPlay() override;
	virtual void EndPlay(const EEndPlayReason::Type EndPlayReason) override;
	virtual void PostActorCreated() override;

public:	
	virtual void Tick(float DeltaTime) override;
	virtual bool ShouldTickIfViewportsOnly() const override;

private:
	FSocket* Socket;
	FIPv4Endpoint RemoteEndpoint;
	
	void ConnectToBridge();
	void ReceiveData();
	void HandleCommand(FString JsonString);

	float Timer;
};
