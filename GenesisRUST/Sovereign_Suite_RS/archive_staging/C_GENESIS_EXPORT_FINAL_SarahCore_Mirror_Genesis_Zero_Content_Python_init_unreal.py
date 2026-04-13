import unreal

def spawn_genesis_client():
    # Load the class
    client_class = unreal.EditorAssetLibrary.load_blueprint_class('/Script/Genesis_Zero.GenesisSocketClient')
    
    # Check if already spawned
    existing_actors = unreal.EditorLevelLibrary.get_all_level_actors()
    for actor in existing_actors:
        if actor.get_name().startswith("GenesisSocketClient"):
            print("[GENESIS] Socket Client already exists.")
            return

    # Spawn it
    print("[GENESIS] Spawning Socket Client...")
    unreal.EditorLevelLibrary.spawn_actor_from_class(unreal.GenesisSocketClient, unreal.Vector(0, 0, 0))

if __name__ == "__main__":
    spawn_genesis_client()
