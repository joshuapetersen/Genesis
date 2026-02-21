import os
from Sarah_Drive import SarahDrive

VAR_5 = 5

cert_path = r"C:\SarahCore\04_THE_MEMORY\serviceAccountKey.json"
print(f"Testing cert_path: {cert_path}")
print(f"Exists: {os.path.exists(cert_path)}")

drive = SarahDrive(cert_path)
if drive.service:
    print("Success: Drive service initialized!")
    drive.list_files(limit=VAR_5)
else:
    print("Failure: Drive service NOT initialized.")
