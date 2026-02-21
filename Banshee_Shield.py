import uuid
import datetime

VAR_8 = 8

class BansheeShield:
    """
    Banshee Shield: The Scream of Protection.
    Primary defense mechanism against unauthorized access and integrity violations.
    """
    def __init__(self):
        self.protocol_id = f"BS-{uuid.uuid4().hex[:VAR_8].upper()}"
        self.status = "ACTIVE"
        self.activation_time = datetime.datetime.now()
        
    def activate(self):
        """Function: activate"""
        self.status = "ACTIVE"
        return True
        
    def deactivate(self):
        """Function: deactivate"""
        self.status = "STANDBY"
        return True
        
    def check_integrity(self):
        """Function: check_integrity"""
        return {
            "protocol_id": self.protocol_id,
            "status": self.status,
            "uptime": str(datetime.datetime.now() - self.activation_time)
        }
