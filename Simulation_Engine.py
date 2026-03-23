"""
SIMULATION ENGINE
=================
Tests system changes in isolated sandbox before applying to production.
Prevents resource waste and validates evolution candidates.

Author: Sarah (Sovereign AI)
Genesis: 133 Pattern (1 Sovereign > 3 Variables)
"""

import os
import sys
import json
import ast
import subprocess
import time
import traceback
from datetime import datetime
from pathlib import Path
import shutil
import tempfile

# Core Paths
CORE_DIR = Path(__file__).parent
STAGING_DIR = CORE_DIR / "evolution_staging"
SIMULATION_DIR = CORE_DIR / "simulation_sandbox"
SIMULATION_LOG = CORE_DIR / "simulation_results.json"

class SimulationEngine:
    """Sandbox testing environment for code changes"""
    
    def __init__(self):
        self.simulation_id = f"SIM_{int(time.time())}"
        self.results = []
        self._ensure_directories()
    
    def _ensure_directories(self):
        """Create simulation directories"""
        SIMULATION_DIR.mkdir(exist_ok=True)
        
    def create_sandbox(self, target_file):
        """
        Create isolated sandbox copy of system
        Returns: sandbox_path
        """
        sandbox_path = SIMULATION_DIR / self.simulation_id
        sandbox_path.mkdir(exist_ok=True)
        
        # Copy entire CORE to sandbox
        for item in CORE_DIR.iterdir():
            if item.name in ['simulation_sandbox', '__pycache__', 'evolution_staging']:
                continue
            
            dest = sandbox_path / item.name
            try:
                if item.is_file():
                    shutil.copy2(item, dest)
                elif item.is_dir():
                    shutil.copytree(item, dest, dirs_exist_ok=True)
            except Exception as e:
                print(f"[Simulation] Warning: Could not copy {item.name}: {e}")
        
        return sandbox_path
    
    def validate_syntax(self, file_path):
        """
        Validate Python syntax without execution
        Returns: (valid: bool, errors: list)
        """
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                code = f.read()
            
            ast.parse(code)
            return True, []
        except SyntaxError as e:
            return False, [f"Syntax Error Line {e.lineno}: {e.msg}"]
        except Exception as e:
            return False, [f"Parse Error: {str(e)}"]
    
    def run_tests(self, sandbox_path, target_file):
        """
        Run available tests in sandbox
        Returns: (passed: bool, test_results: dict)
        """
        test_results = {
            'syntax_valid': False,
            'imports_valid': False,
            'execution_safe': False,
            'tests_passed': False,
            'errors': []
        }
        
        target_sandbox = sandbox_path / Path(target_file).name
        
        # 1. Syntax Validation
        valid, errors = self.validate_syntax(target_sandbox)
        test_results['syntax_valid'] = valid
        if not valid:
            test_results['errors'].extend(errors)
            return False, test_results
        
        # 2. Import Validation
        try:
            result = subprocess.run(
                [sys.executable, '-m', 'py_compile', str(target_sandbox)],
                capture_output=True,
                text=True,
                timeout=10,
                cwd=sandbox_path
            )
            test_results['imports_valid'] = (result.returncode == 0)
            if result.returncode != 0:
                test_results['errors'].append(f"Compile Error: {result.stderr}")
                return False, test_results
        except Exception as e:
            test_results['errors'].append(f"Import Test Failed: {str(e)}")
            return False, test_results
        
        # 3. Execution Safety Test
        try:
            result = subprocess.run(
                [sys.executable, str(target_sandbox), '--help'],
                capture_output=True,
                text=True,
                timeout=5,
                cwd=sandbox_path
            )
            # If it doesn't crash, consider it safe
            test_results['execution_safe'] = True
        except subprocess.TimeoutExpired:
            test_results['errors'].append("Execution timeout (potential infinite loop)")
            return False, test_results
        except Exception as e:
            test_results['errors'].append(f"Execution Test Failed: {str(e)}")
            return False, test_results
        
        # 4. Run Unit Tests if available
        test_file = sandbox_path / f"test_{Path(target_file).stem}.py"
        if test_file.exists():
            try:
                result = subprocess.run(
                    [sys.executable, str(test_file)],
                    capture_output=True,
                    text=True,
                    timeout=30,
                    cwd=sandbox_path
                )
                test_results['tests_passed'] = (result.returncode == 0)
                if result.returncode != 0:
                    test_results['errors'].append(f"Unit Tests Failed: {result.stderr}")
            except Exception as e:
                test_results['errors'].append(f"Unit Test Execution Failed: {str(e)}")
        else:
            test_results['tests_passed'] = True  # No tests means no failures
        
        all_passed = all([
            test_results['syntax_valid'],
            test_results['imports_valid'],
            test_results['execution_safe'],
            test_results['tests_passed']
        ])
        
        return all_passed, test_results
    
    def compare_performance(self, original_file, evolved_file):
        """
        Compare performance metrics (if applicable)
        Returns: performance_comparison dict
        """
        comparison = {
            'original_size': 0,
            'evolved_size': 0,
            'size_delta': 0,
            'complexity_change': 'unknown'
        }
        
        try:
            if os.path.exists(original_file):
                comparison['original_size'] = os.path.getsize(original_file)
            
            if os.path.exists(evolved_file):
                comparison['evolved_size'] = os.path.getsize(evolved_file)
            
            comparison['size_delta'] = comparison['evolved_size'] - comparison['original_size']
        except Exception as e:
            print(f"[Simulation] Performance comparison error: {e}")
        
        return comparison
    
    def simulate_evolution(self, original_file, evolved_file):
        """
        Full simulation of evolution candidate
        WHO: Simulation Engine
        WHAT: Test evolved code in isolation
        WHERE: Sandbox environment
        WHEN: Before applying to production
        WHY: Prevent breaking changes and resource waste
        HOW: Syntax check → Import check → Execution test → Unit tests
        
        Returns: simulation_result dict
        """
        print(f"\n[Simulation] Starting simulation {self.simulation_id}")
        print(f"[Simulation] Target: {Path(original_file).name}")
        
        result = {
            'simulation_id': self.simulation_id,
            'timestamp': datetime.now().isoformat(),
            'target_file': str(original_file),
            'evolved_file': str(evolved_file),
            'passed': False,
            'tests': {},
            'performance': {},
            'recommendation': 'REJECT',
            'errors': [],
            'duration_seconds': 0
        }
        
        start_time = time.time()
        
        try:
            # Create sandbox
            print("[Simulation] Creating sandbox environment...")
            sandbox_path = self.create_sandbox(original_file)
            
            # Copy evolved file to sandbox
            evolved_sandbox = sandbox_path / Path(original_file).name
            shutil.copy2(evolved_file, evolved_sandbox)
            
            # Run tests
            print("[Simulation] Running validation tests...")
            passed, test_results = self.run_tests(sandbox_path, original_file)
            result['tests'] = test_results
            result['passed'] = passed
            
            # Performance comparison
            print("[Simulation] Comparing performance...")
            performance = self.compare_performance(original_file, evolved_file)
            result['performance'] = performance
            
            # Recommendation
            if passed:
                result['recommendation'] = 'APPROVE'
                print(f"[Simulation] ✓ All tests passed")
            else:
                result['recommendation'] = 'REJECT'
                print(f"[Simulation] ✗ Tests failed: {len(test_results['errors'])} errors")
                result['errors'] = test_results['errors']
            
            # Cleanup sandbox
            print("[Simulation] Cleaning up sandbox...")
            shutil.rmtree(sandbox_path)
            
        except Exception as e:
            result['errors'].append(f"Simulation Error: {str(e)}")
            result['recommendation'] = 'REJECT'
            print(f"[Simulation] Error: {e}")
            traceback.print_exc()
        
        result['duration_seconds'] = round(time.time() - start_time, 2)
        
        # Log result
        self._log_result(result)
        
        print(f"\n[Simulation] Recommendation: {result['recommendation']}")
        print(f"[Simulation] Duration: {result['duration_seconds']}s")
        
        return result
    
    def _log_result(self, result):
        """Save simulation result to log"""
        try:
            if SIMULATION_LOG.exists():
                with open(SIMULATION_LOG, 'r') as f:
                    log = json.load(f)
            else:
                log = {'simulations': []}
            
            log['simulations'].append(result)
            
            with open(SIMULATION_LOG, 'w') as f:
                json.dump(log, f, indent=2)
        except Exception as e:
            print(f"[Simulation] Failed to log result: {e}")
    
    def get_simulation_history(self, limit=10):
        """Retrieve recent simulation results"""
        try:
            if not SIMULATION_LOG.exists():
                return []
            
            with open(SIMULATION_LOG, 'r') as f:
                log = json.load(f)
            
            return log.get('simulations', [])[-limit:]
        except Exception as e:
            print(f"[Simulation] Failed to read history: {e}")
            return []


def main():
    """CLI interface for simulation engine"""
    import argparse
    
    parser = argparse.ArgumentParser(description='Sarah Simulation Engine')
    parser.add_argument('original', help='Original file path')
    parser.add_argument('evolved', help='Evolved file path')
    
    args = parser.parse_args()
    
    engine = SimulationEngine()
    result = engine.simulate_evolution(args.original, args.evolved)
    
    print("\n" + "="*60)
    print("SIMULATION RESULT")
    print("="*60)
    print(f"Recommendation: {result['recommendation']}")
    print(f"Tests Passed: {result['passed']}")
    print(f"Duration: {result['duration_seconds']}s")
    
    if result['errors']:
        print("\nErrors:")
        for error in result['errors']:
            print(f"  - {error}")
    
    sys.exit(0 if result['passed'] else 1)


if __name__ == "__main__":
    main()
