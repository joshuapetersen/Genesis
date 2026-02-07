import os
import json
from pathlib import Path

class SarahAgentFactory:
    def __init__(self, workspace_root="c:/SarahCore"):
        self.workspace_root = Path(workspace_root)
        self.openclaw_dir = self.workspace_root / "openclaw"
        self.skills_dir = self.openclaw_dir / "skills"
        self.manifest_path = self.openclaw_dir / "opencode.json"
        
        # Ensure directories exist
        self.skills_dir.mkdir(parents=True, exist_ok=True)
        
        if not self.manifest_path.exists():
            with open(self.manifest_path, "w") as f:
                json.dump({"agents": []}, f)

    def _load_manifest(self):
        try:
            with open(self.manifest_path, "r") as f:
                return json.load(f)
        except (FileNotFoundError, json.JSONDecodeError):
            return {"agents": []}

    def _save_manifest(self, data):
        with open(self.manifest_path, "w") as f:
            json.dump(data, f, indent=4)

    def create_agent(self, agent_id, directive=None):
        """
        Registers an agent in the opencode.json manifest.
        Skills are assumed to be pre-created in the skills folder.
        """
        manifest = self._load_manifest()
        
        if agent_id.lower() not in [a.lower() for a in manifest.get("agents", [])]:
            manifest.setdefault("agents", []).append(agent_id.lower())
            self._save_manifest(manifest)
            print(f"[Factory] Agent '{agent_id}' manifested successfully.")
        else:
            print(f"[Factory] Agent '{agent_id}' already exists in manifest.")
        return True

    def destroy_agent(self, agent_id):
        """
        Decommissions an agent.
        """
        manifest = self._load_manifest()
        agents = manifest.get("agents", [])
        if agent_id.lower() in [a.lower() for a in agents]:
            manifest["agents"] = [a for a in agents if a.lower() != agent_id.lower()]
            self._save_manifest(manifest)
            print(f"[Factory] Agent '{agent_id}' decommissioned.")
            return True
        return False

if __name__ == "__main__":
    # Quick Test
    factory = SarahAgentFactory()
    factory.create_agent("BridgeBot")
