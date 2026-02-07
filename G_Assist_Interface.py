import os
import time
import re

class GAssistInterface:
    def __init__(self):
        self.program_data = os.environ.get('ProgramData', 'C:\\ProgramData')
        self.log_path_container = os.path.join(self.program_data, r"NVIDIA Corporation\NVIDIA App\Logs\LOG.NVDisplay.Container.exe.log")
        self.log_path_session = os.path.join(self.program_data, r"NVIDIA Corporation\NVIDIA App\SessionLogs\CxNative_Session1.log")
        self.last_tell = 0
        
        # Jargon to English Mapping
        self.translation_matrix = {
            "NvAPI_GPU_ClientPwrPoliciesGetInfo": "Checking power reserves.",
            "NvAPI_GPU_GetFullName": "Verifying identity: RTX 4050 Laptop GPU.",
            "Maximum TGP": "Power limits established.",
            "MbBroadc": "Neural pathways active.",
            "JOIN Shadowplay": "Shadowplay Visual Link established.",
            "JOIN NvBackend": "Backend Logic Core connected.",
            "NvDriverDiagnostics": "Running self-diagnostics.",
            "RestoreGsyncPersistence": "Synchronizing visual frequencies.",
            "NvAPI_GPU_GetArchInfo": "Architecture scan complete."
        }

    def _read_last_lines(self, filepath, n=20):
        """Reads the last n lines of a file safely."""
        if not os.path.exists(filepath):
            return []
        try:
            with open(filepath, 'r', encoding='utf-8', errors='ignore') as f:
                # Move to end
                f.seek(0, 2)
                fsize = f.tell()
                f.seek(max(fsize - 4096, 0), 0) # Read last 4KB
                lines = f.readlines()
                return lines[-n:]
        except Exception:
            return []

    def listen(self):
        """Returns a list of 'spoken' lines from G-Assist based on recent logs."""
        messages = []
        
        # 1. Scan Container Log (Hardware/System)
        lines = self._read_last_lines(self.log_path_container)
        for line in lines:
            for key, phrase in self.translation_matrix.items():
                if key in line and "DEBUG" not in line: # Filter noise if possible, though logs are verbose
                    # Check timestamp if strictly realtime is needed, identifying "freshness"
                    # For now, we just map the concept.
                    messages.append(f"STATUS: {phrase}")
        
        # 2. Scan Session Log (Process/Link)
        lines = self._read_last_lines(self.log_path_session)
        for line in lines:
            if "JOIN" in line:
                service = line.split("JOIN")[-1].strip()
                messages.append(f"LINK: Connection to {service} active.")
        
        # De-duplicate and keep distinct recent
        unique_messages = list(set(messages))
        
        # Format as Dialogue
        dialogue = []
        if unique_messages:
            dialogue.append("[G-ASSIST :: SENTINEL]")
            for msg in unique_messages[-3:]: # Top 3 most relevant
                dialogue.append(f"  > {msg}")
                
        return "\n".join(dialogue)

if __name__ == "__main__":
    # Test
    g = GAssistInterface()
    print(g.listen())
