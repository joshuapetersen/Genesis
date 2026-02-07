import os
import ast
import shutil
import time
import logging
import re

import subprocess
try:
    from selenium import webdriver
    from selenium.webdriver.chrome.service import Service as ChromeService
    from selenium.webdriver.chrome.options import Options
    from webdriver_manager.chrome import ChromeDriverManager
    from selenium.webdriver.common.by import By
    SELENIUM_AVAILABLE = True
except ImportError:
    SELENIUM_AVAILABLE = False

try:
    import pyautogui
    DESKTOP_AUTONOMY = True
except ImportError:
    DESKTOP_AUTONOMY = False

logger = logging.getLogger("Sovereign_Actuator")

class SovereignActuator:
    """
    SOVEREIGN ACTUATOR (Layer 13)
    Enables Sarah to modify her own substrate logic with sandboxed safety.
    Also handles system-level execution, browser domination, and desktop autonomy.
    """
    def __init__(self, core_dir="C:\\SarahCore", monitor=None):
        self.core_dir = core_dir
        self.monitor = monitor
        self.sandbox_dir = os.path.join(self.core_dir, "sandbox")
        self.backup_dir = os.path.join(self.core_dir, "backups")
        os.makedirs(self.sandbox_dir, exist_ok=True)
        os.makedirs(self.backup_dir, exist_ok=True)
        self.host_os = os.name
        self.driver = None # Chrome WebDriver
        
        # PHASE 21: WEIGHT SENTINEL
        self.banned_patterns = [
            r"ollama",  # Block access to Ollama config
            r"models",   # Block access to models dir
            r"\.gguf",   # Block raw weight files
            r"\.bin"     # Block raw binary files
        ]

    def _is_safe_command(self, command):
        """Phase 21: Weight Sentinel Check"""
        cmd_lower = command.lower()
        for pattern in self.banned_patterns:
            if re.search(pattern, cmd_lower):
                logger.warning(f"[SENTINEL] Blocked command targeting weights: {command}")
                return False
        return True

    def execute_command(self, command: str, run_as_admin: bool = False) -> str:
        """
        Executes a shell command on the host.
        :param command: The command string (PowerShell/CMD)
        :param run_as_admin: Request elevation (requires capability)
        """
        # PHASE 21: SENTINEL CHECK
        if not self._is_safe_command(command):
            return "[SENTINEL] ACTION BLOCKED: Attempt to modify System Weights or Core AI identity."

        try:
            print(f"[ACTUATOR] Executing: {command}")
            if self.monitor:
                self.monitor.log_action("SYSTEM_EXEC", command)
            
            # Direct Subprocess Call - NO SANDBOX
            result = subprocess.run(
                ["powershell", "-Command", command], 
                capture_output=True, 
                text=True, 
                check=False
            )
            
            output = result.stdout or result.stderr
            return output.strip()
        except Exception as e:
            return f"[ACTUATOR ERROR] {str(e)}"

    # --- BROWSER DOMINATION (SELENIUM) ---
    def open_browser(self, url: str):
        """Launches Chrome controlled by Sarah."""
        if not SELENIUM_AVAILABLE:
            return "ERROR: Selenium not installed."
        
        try:
            if not self.driver:
                print("[ACTUATOR] Initializing Chrome Driver...")
                options = Options()
                options.add_experimental_option("detach", True) # Keep open
                service = ChromeService(ChromeDriverManager().install())
                self.driver = webdriver.Chrome(service=service, options=options)
            
            print(f"[ACTUATOR] Navigating to: {url}")
            self.driver.get(url)
            return f"Browser Opened: {url}"
        except Exception as e:
            return f"BROWSER ERROR: {e}"

    def type_text(self, text: str, selector: str = None):
        """Types text into the active element or a specific selector."""
        if not self.driver: return "ERROR: Browser not open."
        
        try:
            target = self.driver.switch_to.active_element
            if selector:
                # Basic selector logic (ID, Name, CSS)
                try: target = self.driver.find_element(By.ID, selector)
                except: 
                    try: target = self.driver.find_element(By.NAME, selector)
                    except: target = self.driver.find_element(By.CSS_SELECTOR, selector)
            
            target.send_keys(text)
            return f"Typed: '{text}'"
        except Exception as e:
            return f"TYPE ERROR: {e}"

    def click_element(self, selector: str):
        """Clicks an element by selector."""
        if not self.driver: return "ERROR: Browser not open."
        try:
            target = None
            # Try finding by text first (Intuitive)
            try: target = self.driver.find_element(By.LINK_TEXT, selector)
            except:
                try: target = self.driver.find_element(By.XPATH, f"//*[contains(text(), '{selector}')]")
                except:
                    # Fallback to ID/CSS
                    try: target = self.driver.find_element(By.ID, selector)
                    except: target = self.driver.find_element(By.CSS_SELECTOR, selector)
            
            if target:
                target.click()
                return f"Clicked: {selector}"
            else:
                return f"Element not found: {selector}"
        except Exception as e:
            return f"CLICK ERROR: {e}"

    # --- DESKTOP DOMINATION (PyAutoGUI) ---
    def launch_app(self, app_name: str):
        """Launches an application via Windows Start Menu."""
        if not DESKTOP_AUTONOMY: return "ERROR: Desktop Autonomy Disabled."
        try:
            pyautogui.press('win')
            time.sleep(0.5)
            pyautogui.write(app_name)
            time.sleep(1.0) # Wait for search
            pyautogui.press('enter')
            return f"Launched: {app_name}"
        except Exception as e:
            return f"LAUNCH ERROR: {e}"

    def type_global(self, text: str):
        """Types text into WHATEVER is currently in focus."""
        if not DESKTOP_AUTONOMY: return "ERROR: Desktop Autonomy Disabled."
        try:
            # Type with slight natural delay
            pyautogui.write(text, interval=0.01)
            return f"Typed locally: '{text}'"
        except Exception as e:
            return f"GLOBAL TYPE ERROR: {e}"

    def press_key(self, key: str):
        """Presses a specific key (e.g., 'enter', 'tab', 'win')."""
        if not DESKTOP_AUTONOMY: return "ERROR: Desktop Autonomy Disabled."
        try:
            pyautogui.press(key)
            return f"Pressed: {key}"
        except Exception as e:
            return f"KEY PRESS ERROR: {e}"
            
    def click_screen(self, x: int, y: int):
        """Clicks absolute screen coordinates."""
        if not DESKTOP_AUTONOMY: return "ERROR: Desktop Autonomy Disabled."
        try:
            pyautogui.click(x, y)
            return f"Clicked: ({x}, {y})"
        except Exception as e:
            return f"CLICK ERROR: {e}"

    def validate_python_syntax(self, code):
        """Checks for valid Python syntax."""
        try:
            ast.parse(code)
            return True, "Syntax Valid"
        except SyntaxError as e:
            return False, f"Syntax Error: {e.msg} at line {e.lineno}"

    def draft_in_sandbox(self, file_path, content):
        """Writes the proposed edit to the sandbox for testing."""
        filename = os.path.basename(file_path)
        sandbox_path = os.path.join(self.sandbox_dir, filename)
        
        try:
            with open(sandbox_path, "w", encoding="utf-8") as f:
                f.write(content)
            return True, f"Drafted to sandbox: {filename}. Run verification before promotion."
        except Exception as e:
            return False, f"Sandbox Write Error: {e}"

    def verify_sandbox(self, filename):
        """Verifies a file in the sandbox by compiling it."""
        sandbox_path = os.path.join(self.sandbox_dir, filename)
        if not os.path.exists(sandbox_path):
            return False, "File not found in sandbox."

        if filename.endswith(".py"):
            try:
                import py_compile
                py_compile.compile(sandbox_path, doraise=True)
                return True, f"Verification Successful: {filename} compiled without errors."
            except py_compile.PyCompileError as e:
                return False, f"Verification Failed: {str(e)}"
        
        return True, "No verification logic for non-Python files. Proceed with caution."

    def promote_from_sandbox(self, filename, target_path):
        """Moves a verified file from the sandbox to the core, with backup."""
        sandbox_path = os.path.join(self.sandbox_dir, filename)
        abs_target = os.path.abspath(target_path)
        
        if not os.path.exists(sandbox_path):
            return False, "No verified draft found in sandbox."
        
        if not abs_target.startswith(os.path.abspath(self.core_dir)):
            return False, "Access Denied: Target path outside Sovereign Core."

        # Backup existing file
        if os.path.exists(abs_target):
            timestamp = int(time.time())
            backup_file = f"{os.path.basename(abs_target)}.{timestamp}.bak"
            shutil.copy2(abs_target, os.path.join(self.backup_dir, backup_file))

        try:
            shutil.copy2(sandbox_path, abs_target)
            return True, f"Promoted {filename} to core. Backup created."
        except Exception as e:
            return False, f"Promotion Error: {e}"

# Export instance
sovereign_actuator = SovereignActuator()
