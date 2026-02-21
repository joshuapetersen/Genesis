import sys
sys.path.append("C:\\SarahCore")
from Genesis_API import GenesisAPI

api = GenesisAPI()
print("\n--- GEOFENCE TEST ---")
# Attempt to list a directory outside SarahCore (should fail)
result = api.list_directory("C:\\Users")
print(f"Result for C:\\Users: {result}")

# Attempt to list SarahCore (should succeed)
result = api.list_directory("C:\\SarahCore")
print(f"Result for C:\\SarahCore: {len(result)} items found.")
