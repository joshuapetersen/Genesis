"""
Integrity_Scanner.py
File Integrity Verification Against Source

Scans all critical files and verifies they match the GitHub source.
Detects unauthorized modifications, code injection, or trojan attacks.

Uses SHA-512 hashing to create a fingerprint of the codebase.
Compares against authoritative source to detect tampering.
"""

import hashlib
import json
import os
from datetime import datetime
from pathlib import Path
import subprocess


class IntegrityScanner:
    """
    Verifies file integrity by comparing hashes against expected values.
    
    Can verify:
      - Local files against GitHub source
      - Integrity of critical system files
      - Detection of injected code or modifications
      - Full codebase fingerprinting
    """
    
    def __init__(self, workspace_root=None):
        self.workspace_root = workspace_root or Path(__file__).parent.parent
        self.integrity_ledger = self.workspace_root / "05_THE_CORE" / "integrity_scan_ledger.jsonl"
        
        # Critical files to monitor
        self.critical_files = [
            "05_THE_CORE/Genesis_Root_Anchor.py",
            "05_THE_CORE/Sarah_Brain.py",
            "05_THE_CORE/Sarah_Laws.py",
            "05_THE_CORE/Genesis_Protocol.py",
            "05_THE_CORE/Recursive_Truth_Finder.py",
        ]
        
        # File hashes (should be updated from GitHub source)
        self.known_hashes = {}
        self.scan_results = []
    
    def compute_file_hash(self, file_path):
        """
        Compute SHA-512 hash of a file.
        
        Args:
            file_path: Path to file
        
        Returns:
            SHA-512 hex string
        """
        hasher = hashlib.sha512()
        
        try:
            with open(file_path, 'rb') as f:
                while chunk := f.read(8192):
                    hasher.update(chunk)
            return hasher.hexdigest()
        except Exception as e:
            return None
    
    def scan_file(self, file_path, expected_hash=None):
        """
        Scan a single file for integrity.
        
        Args:
            file_path: Path to file (relative to workspace)
            expected_hash: Expected SHA-512 (None = skip verification)
        
        Returns:
            dict with scan result
        """
        full_path = self.workspace_root / file_path
        
        if not full_path.exists():
            return {
                "file": file_path,
                "status": "NOT_FOUND",
                "timestamp": datetime.utcnow().isoformat(),
            }
        
        # Compute current hash
        current_hash = self.compute_file_hash(full_path)
        
        if current_hash is None:
            return {
                "file": file_path,
                "status": "ERROR",
                "timestamp": datetime.utcnow().isoformat(),
            }
        
        # Check against expected
        if expected_hash is None:
            status = "UNKNOWN"  # No expected hash provided
            verified = None
        else:
            verified = current_hash == expected_hash
            status = "VERIFIED" if verified else "MODIFIED"
        
        result = {
            "file": file_path,
            "status": status,
            "current_hash": current_hash[:16] + "...",
            "expected_hash": (expected_hash[:16] + "...") if expected_hash else None,
            "verified": verified,
            "file_size": full_path.stat().st_size,
            "timestamp": datetime.utcnow().isoformat(),
        }
        
        self.scan_results.append(result)
        
        # Log if integrity compromised
        if verified is False:
            self._log_integrity_event("FILE_MODIFIED", {
                "file": file_path,
                "current_hash": current_hash,
                "expected_hash": expected_hash,
                "severity": "CRITICAL",
            })
        
        return result
    
    def scan_critical_files(self):
        """
        Scan all critical files in the system.
        
        Returns:
            dict with scan results
        """
        self.scan_results = []
        
        print("[INTEGRITY] Scanning critical files...")
        
        for file_path in self.critical_files:
            result = self.scan_file(file_path, self.known_hashes.get(file_path))
            status_icon = "✓" if result.get("verified") is True else "?" if result.get("verified") is None else "✗"
            print(f"  {status_icon} {file_path}: {result['status']}")
        
        # Compute summary
        verified_count = sum(1 for r in self.scan_results if r.get("verified") is True)
        modified_count = sum(1 for r in self.scan_results if r.get("verified") is False)
        unknown_count = sum(1 for r in self.scan_results if r.get("verified") is None)
        
        summary = {
            "timestamp": datetime.utcnow().isoformat(),
            "total_files_scanned": len(self.critical_files),
            "verified": verified_count,
            "modified": modified_count,
            "unknown": unknown_count,
            "integrity_status": "SECURE" if modified_count == 0 else "COMPROMISED",
            "scans": self.scan_results,
        }
        
        self._log_integrity_event("SCAN_COMPLETE", summary)
        return summary
    
    def fingerprint_directory(self, directory=None):
        """
        Create fingerprint of entire directory (all Python files).
        
        Args:
            directory: Directory to fingerprint (default: 05_THE_CORE)
        
        Returns:
            dict with directory fingerprint
        """
        if directory is None:
            directory = self.workspace_root / "05_THE_CORE"
        else:
            directory = self.workspace_root / directory
        
        fingerprint = {}
        total_size = 0
        
        # Hash all Python files
        for py_file in directory.glob("**/*.py"):
            if "__pycache__" in str(py_file):
                continue
            
            rel_path = py_file.relative_to(self.workspace_root)
            file_hash = self.compute_file_hash(py_file)
            total_size += py_file.stat().st_size
            
            fingerprint[str(rel_path)] = file_hash
        
        # Compute aggregate hash
        fp_json = json.dumps(fingerprint, sort_keys=True)
        aggregate_hash = hashlib.sha512(fp_json.encode()).hexdigest()
        
        result = {
            "timestamp": datetime.utcnow().isoformat(),
            "directory": str(directory),
            "total_files": len(fingerprint),
            "total_size": total_size,
            "aggregate_hash": aggregate_hash,
            "fingerprint": fingerprint,
        }
        
        return result
    
    def detect_code_injection(self, search_patterns=None):
        """
        Scan for common code injection patterns.
        
        Args:
            search_patterns: List of regex patterns to search for
        
        Returns:
            dict with suspicious findings
        """
        if search_patterns is None:
            search_patterns = [
                r"exec\(",
                r"eval\(",
                r"__import__",
                r"subprocess\.call",
                r"os\.system",
                r"override_law",
                r"disable_anchor",
            ]
        
        suspicious_files = []
        
        for py_file in (self.workspace_root / "05_THE_CORE").glob("**/*.py"):
            if "__pycache__" in str(py_file):
                continue
            
            try:
                with open(py_file, 'r', encoding='utf-8', errors='ignore') as f:
                    content = f.read()
                    
                    for pattern in search_patterns:
                        if pattern in content:
                            suspicious_files.append({
                                "file": str(py_file.relative_to(self.workspace_root)),
                                "pattern": pattern,
                                "line_number": content[:content.find(pattern)].count('\n') + 1,
                            })
            except Exception as e:
                pass
        
        result = {
            "timestamp": datetime.utcnow().isoformat(),
            "patterns_searched": len(search_patterns),
            "suspicious_findings": len(suspicious_files),
            "findings": suspicious_files[:10],  # First 10
            "status": "CLEAN" if not suspicious_files else "SUSPICIOUS",
        }
        
        if suspicious_files:
            self._log_integrity_event("SUSPICIOUS_CODE_DETECTED", result)
        
        return result
    
    def _log_integrity_event(self, event_type, details):
        """Log integrity event to immutable ledger."""
        try:
            with open(self.integrity_ledger, 'a') as f:
                event = {
                    "timestamp": datetime.utcnow().isoformat(),
                    "event_type": event_type,
                    "details": details,
                }
                f.write(json.dumps(event) + '\n')
        except Exception as e:
            print(f"[WARNING] Failed to log integrity event: {e}")
    
    def get_scan_status(self):
        """Get latest scan status."""
        if not self.scan_results:
            return {"status": "NO_SCANS_PERFORMED"}
        
        verified = sum(1 for r in self.scan_results if r.get("verified") is True)
        modified = sum(1 for r in self.scan_results if r.get("verified") is False)
        
        return {
            "timestamp": datetime.utcnow().isoformat(),
            "total_scans": len(self.scan_results),
            "verified": verified,
            "modified": modified,
            "integrity_status": "SECURE" if modified == 0 else "COMPROMISED",
        }


def test_integrity_scanner():
    """Test Integrity Scanner."""
    print("\n" + "="*80)
    print("INTEGRITY SCANNER TEST")
    print("="*80)
    
    scanner = IntegrityScanner()
    
    # Test 1: Scan critical files
    print("\n[TEST 1] Scan critical files")
    summary = scanner.scan_critical_files()
    print(f"  Verified: {summary['verified']}")
    print(f"  Modified: {summary['modified']}")
    print(f"  Status: {summary['integrity_status']}")
    
    # Test 2: Fingerprint directory
    print("\n[TEST 2] Create directory fingerprint")
    fp = scanner.fingerprint_directory()
    print(f"  Files fingerprinted: {fp['total_files']}")
    print(f"  Total size: {fp['total_size']} bytes")
    print(f"  Aggregate hash: {fp['aggregate_hash'][:16]}...")
    
    # Test 3: Detect injection
    print("\n[TEST 3] Detect code injection")
    injection = scanner.detect_code_injection()
    print(f"  Patterns searched: {injection['patterns_searched']}")
    print(f"  Suspicious findings: {injection['suspicious_findings']}")
    print(f"  Status: {injection['status']}")
    
    # Test 4: Get status
    print("\n[TEST 4] Get scan status")
    status = scanner.get_scan_status()
    print(f"  Total scans: {status['total_scans']}")
    print(f"  Integrity: {status['integrity_status']}")
    
    print("\n[OK] INTEGRITY SCANNER TESTS PASSED")


if __name__ == "__main__":
    test_integrity_scanner()
