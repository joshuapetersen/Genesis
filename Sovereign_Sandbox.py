import os
import subprocess
import time
import logging
from pathlib import Path

# --- THE SOVEREIGN SANDBOX ---
SANDBOX_DIR = Path("c:\\SarahCore\\07_THE_SANDBOX")
SANDBOX_DIR.mkdir(parents=True, exist_ok=True)
EXECUTION_TIMEOUT = 15 # Seconds

logging.basicConfig(level=logging.INFO, format='%(asctime)s - [SANDBOX] - %(levelname)s - %(message)s')
logger = logging.getLogger("Sandbox_Evaluator")

class SovereignSandbox:
    """
    Phase IV isolated execution environment for the Swarm's Coder Agent.
    Allows AERIS to write, execute, and test experimental code before merging.
    """
    def __init__(self):
        self.sandbox_path = SANDBOX_DIR
        
    def write_experiment(self, filename: str, code: str) -> Path:
        """Writes experimental code into the sandbox."""
        # Sanitize filename to prevent directory traversal
        safe_name = os.path.basename(filename)
        file_path = self.sandbox_path / safe_name
        
        with open(file_path, "w", encoding="utf-8") as f:
            f.write(code)
            
        logger.info(f"Experiment written: {safe_name}")
        return file_path
        
    def execute_and_evaluate(self, file_path: Path) -> dict:
        """Executes the sandboxed code and capturing results."""
        if not file_path.exists():
            return {"status": "ERROR", "message": "File not found"}
            
        if file_path.suffix != '.py':
             return {"status": "ERROR", "message": "Sandbox currently only evaluates Python (.py) scripts."}

        logger.info(f"Evaluating {file_path.name}...")
        start_time = time.time()
        
        try:
            # Execute in an isolated subprocess
            result = subprocess.run(
                ["python", str(file_path)],
                capture_output=True,
                text=True,
                timeout=EXECUTION_TIMEOUT
            )
            
            execution_time = time.time() - start_time
            
            report = {
                "status": "SUCCESS" if result.returncode == 0 else "FAIL",
                "return_code": result.returncode,
                "stdout": result.stdout[:2000],  # Truncate to prevent log bloat
                "stderr": result.stderr[:2000],
                "execution_time": round(execution_time, 2),
                "billion_barrier_pass": result.returncode == 0 # Simplistic pass for now
            }
            
            logger.info(f"Evaluation complete. Status: {report['status']} (Code: {report['return_code']})")
            return report
            
        except subprocess.TimeoutExpired:
            logger.warning(f"Execution Timeout! The script exceeded {EXECUTION_TIMEOUT}s.")
            return {
                "status": "TIMEOUT",
                "message": f"Script forced closed after {EXECUTION_TIMEOUT} seconds.",
                "billion_barrier_pass": False
            }
        except Exception as e:
            logger.error(f"Sandbox Fault: {e}")
            return {
                "status": "ERROR",
                "message": str(e),
                "billion_barrier_pass": False
            }

    def wipe_sandbox(self):
        """Purges the sandbox for the next experiment."""
        for file in self.sandbox_path.glob("*"):
            if file.is_file():
                try:
                    file.unlink()
                except Exception as e:
                    logger.warning(f"Could not delete {file.name}: {e}")
        logger.info("Sandbox purged.")

if __name__ == "__main__":
    box = SovereignSandbox()
    
    # Test Experiment
    test_code = """print('Hello from the Sovereign Sandbox!')
x = 10 * 144
print(f'Validation value: {x}')"""
    test_file = box.write_experiment("test_run.py", test_code)
    results = box.execute_and_evaluate(test_file)
    print("--- SANDBOX REPORT ---")
    import pprint
    pprint.pprint(results)
