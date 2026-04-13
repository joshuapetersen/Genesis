import traceback
import sys
from all_engine import GenlexLinearRuntime

try:
    GENLEX_MAPPING = r"C:\Genlex_Linear\genlex_mapping.csv"
    runtime = GenlexLinearRuntime(GENLEX_MAPPING)
    runtime.run(sys.argv[1])
except Exception:
    traceback.print_exc()
