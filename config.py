import json
from pathlib import Path
from typing import Dict, Any

class Config:
    """Centralized configuration management."""
    
    def __init__(self, config_path: Path = None):
        self.config_path = config_path or Path.home() / ".sarahcore" / "config.json"
        self.settings: Dict[str, Any] = self._load_config()
    
    def _load_config(self) -> Dict[str, Any]:
        """Load configuration from file or use defaults."""
        default_config = {
            "geofence_enabled": False,
            "geofence_root": str(Path.home() / "SarahCore"),
            "log_level": "INFO",
            "model_path": str(Path.home() / "models" / "dolphin.gguf"),
            "api_timeout": 30,
            "max_retries": 3,
            "services": {
                "file_service": "http://localhost:8081",
                "process_service": "http://localhost:8082"
            }
        }
        
        if self.config_path.exists():
            try:
                with open(self.config_path, 'r') as f:
                    user_config = json.load(f)
                default_config.update(user_config)
            except json.JSONDecodeError:
                print(f"Warning: Could not parse config file {self.config_path}")
        
        return default_config
    
    def get(self, key: str, default=None):
        """Get configuration value."""
        return self.settings.get(key, default)
    
    def set(self, key: str, value):
        """Set configuration value."""
        self.settings[key] = value
        self._save_config()
    
    def _save_config(self):
        """Save configuration to file."""
        self.config_path.parent.mkdir(parents=True, exist_ok=True)
        with open(self.config_path, 'w') as f:
            json.dump(self.settings, f, indent=2)