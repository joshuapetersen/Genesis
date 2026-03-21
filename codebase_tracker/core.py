"""
Core tracking engine for the micro offline codebase tracker.

This module provides the main functionality for mapping, analyzing, and tracking
codebase structure and changes.
"""

import os
import json
import hashlib
import time
from pathlib import Path
from typing import Dict, List, Set, Optional, Any
from dataclasses import dataclass, asdict
from datetime import datetime
import logging

# Configure logging
logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')
logger = logging.getLogger(__name__)


@dataclass
class FileMetadata:
    """Metadata for tracked files."""
    path: str
    size: int
    last_modified: float
    hash: str
    file_type: str
    dependencies: List[str]
    functions: List[str]
    classes: List[str]
    imports: List[str]
    lines_of_code: int
    complexity_score: float


@dataclass
class CodebaseSnapshot:
    """Snapshot of the entire codebase at a point in time."""
    timestamp: float
    total_files: int
    total_lines: int
    file_types: Dict[str, int]
    dependencies: Dict[str, List[str]]
    file_metadata: Dict[str, FileMetadata]


class CodebaseTracker:
    """Main tracking engine for codebase analysis and monitoring."""
    
    def __init__(self, root_path: str, config_path: Optional[str] = None):
        """
        Initialize the codebase tracker.
        
        Args:
            root_path: Root directory of the codebase to track
            config_path: Optional path to configuration file
        """
        self.root_path = Path(root_path).resolve()
        self.config_path = config_path
        self.tracking_db_path = self.root_path / ".codebase_tracker" / "tracking_db.json"
        self.config = self._load_config()
        
        # Ensure tracking directory exists
        self.tracking_db_path.parent.mkdir(parents=True, exist_ok=True)
        
        # Load existing tracking data
        self.tracking_data = self._load_tracking_data()
        
        # File type mappings
        self.file_type_mappings = {
            '.py': 'python',
            '.js': 'javascript', 
            '.ts': 'typescript',
            '.java': 'java',
            '.cpp': 'c++',
            '.c': 'c',
            '.cs': 'csharp',
            '.go': 'go',
            '.rust': 'rust',
            '.rb': 'ruby',
            '.php': 'php',
            '.swift': 'swift',
            '.kt': 'kotlin',
            '.html': 'html',
            '.css': 'css',
            '.json': 'json',
            '.yaml': 'yaml',
            '.yml': 'yaml',
            '.xml': 'xml',
            '.md': 'markdown',
            '.txt': 'text'
        }
    
    def _load_config(self) -> Dict[str, Any]:
        """Load configuration settings."""
        default_config = {
            "ignore_patterns": [
                ".git", ".svn", ".hg",
                "node_modules", "__pycache__", ".vscode", ".idea",
                "*.log", "*.tmp", "*.bak", "*.swp",
                "build", "dist", "target", "out"
            ],
            "max_file_size": 1024 * 1024,  # 1MB
            "scan_depth": 10,
            "track_dependencies": True,
            "track_complexity": True
        }
        
        if self.config_path and os.path.exists(self.config_path):
            try:
                with open(self.config_path, 'r') as f:
                    user_config = json.load(f)
                    default_config.update(user_config)
            except Exception as e:
                logger.warning(f"Failed to load config from {self.config_path}: {e}")
        
        return default_config
    
    def _load_tracking_data(self) -> Dict[str, Any]:
        """Load existing tracking data from database."""
        if self.tracking_db_path.exists():
            try:
                with open(self.tracking_db_path, 'r') as f:
                    return json.load(f)
            except Exception as e:
                logger.warning(f"Failed to load tracking data: {e}")
        
        return {
            "snapshots": [],
            "current_state": {},
            "file_history": {},
            "last_scan": 0
        }
    
    def _save_tracking_data(self):
        """Save tracking data to database."""
        try:
            with open(self.tracking_db_path, 'w') as f:
                json.dump(self.tracking_data, f, indent=2)
        except Exception as e:
            logger.error(f"Failed to save tracking data: {e}")
    
    def _should_ignore_file(self, file_path: Path) -> bool:
        """Check if file should be ignored based on patterns."""
        relative_path = file_path.relative_to(self.root_path)
        
        for pattern in self.config["ignore_patterns"]:
            if pattern in str(relative_path) or file_path.match(pattern):
                return True
        
        # Check file size
        try:
            if file_path.stat().st_size > self.config["max_file_size"]:
                return True
        except:
            return True
        
        return False
    
    def _calculate_file_hash(self, file_path: Path) -> str:
        """Calculate SHA256 hash of file content."""
        try:
            with open(file_path, 'rb') as f:
                return hashlib.sha256(f.read()).hexdigest()
        except:
            return ""
    
    def _analyze_file(self, file_path: Path) -> Optional[FileMetadata]:
        """Analyze a single file and extract metadata."""
        try:
            stat = file_path.stat()
            file_hash = self._calculate_file_hash(file_path)
            file_type = self._get_file_type(file_path)
            
            # Read file content for analysis
            try:
                with open(file_path, 'r', encoding='utf-8', errors='ignore') as f:
                    content = f.read()
            except:
                content = ""
            
            # Extract metadata
            dependencies = self._extract_dependencies(content, file_type)
            functions = self._extract_functions(content, file_type)
            classes = self._extract_classes(content, file_type)
            imports = self._extract_imports(content, file_type)
            lines_of_code = len([line for line in content.split('\n') if line.strip()])
            complexity_score = self._calculate_complexity(content, file_type)
            
            return FileMetadata(
                path=str(file_path.relative_to(self.root_path)),
                size=stat.st_size,
                last_modified=stat.st_mtime,
                hash=file_hash,
                file_type=file_type,
                dependencies=dependencies,
                functions=functions,
                classes=classes,
                imports=imports,
                lines_of_code=lines_of_code,
                complexity_score=complexity_score
            )
            
        except Exception as e:
            logger.warning(f"Failed to analyze file {file_path}: {e}")
            return None
    
    def _get_file_type(self, file_path: Path) -> str:
        """Determine file type from extension."""
        suffix = file_path.suffix.lower()
        return self.file_type_mappings.get(suffix, 'unknown')
    
    def _extract_dependencies(self, content: str, file_type: str) -> List[str]:
        """Extract dependencies from file content."""
        if not self.config["track_dependencies"]:
            return []
        
        dependencies = []
        
        if file_type == 'python':
            # Extract Python imports
            import_lines = [line for line in content.split('\n') if line.strip().startswith(('import ', 'from '))]
            for line in import_lines:
                parts = line.strip().split()
                if len(parts) >= 2:
                    dep = parts[1].split('.')[0]
                    if dep not in dependencies:
                        dependencies.append(dep)
        
        elif file_type in ['javascript', 'typescript']:
            # Extract JS/TS imports
            import_lines = [line for line in content.split('\n') if 'import' in line and 'from' in line]
            for line in import_lines:
                if 'from' in line:
                    start = line.find("'") + 1 if "'" in line else line.find('"') + 1
                    end = line.find("'", start) if "'" in line else line.find('"', start)
                    if start > 0 and end > start:
                        dep = line[start:end]
                        if dep not in dependencies:
                            dependencies.append(dep)
        
        return dependencies
    
    def _extract_functions(self, content: str, file_type: str) -> List[str]:
        """Extract function names from file content."""
        functions = []
        
        if file_type == 'python':
            # Extract Python functions
            for line in content.split('\n'):
                line = line.strip()
                if line.startswith('def ') and '(' in line:
                    func_name = line[4:line.find('(')].strip()
                    if func_name and func_name not in functions:
                        functions.append(func_name)
        
        elif file_type in ['javascript', 'typescript']:
            # Extract JS/TS functions
            for line in content.split('\n'):
                line = line.strip()
                if 'function ' in line and '(' in line:
                    start = line.find('function ') + 9
                    end = line.find('(', start)
                    if end > start:
                        func_name = line[start:end].strip()
                        if func_name and func_name not in functions:
                            functions.append(func_name)
        
        return functions
    
    def _extract_classes(self, content: str, file_type: str) -> List[str]:
        """Extract class names from file content."""
        classes = []
        
        if file_type == 'python':
            # Extract Python classes
            for line in content.split('\n'):
                line = line.strip()
                if line.startswith('class ') and '(' in line:
                    class_name = line[6:line.find('(')].strip()
                    if class_name and class_name not in classes:
                        classes.append(class_name)
        
        elif file_type in ['javascript', 'typescript']:
            # Extract JS/TS classes
            for line in content.split('\n'):
                line = line.strip()
                if line.startswith('class ') and '{' in line:
                    class_name = line[6:line.find('{')].strip()
                    if class_name and class_name not in classes:
                        classes.append(class_name)
        
        return classes
    
    def _extract_imports(self, content: str, file_type: str) -> List[str]:
        """Extract import statements from file content."""
        imports = []
        
        if file_type == 'python':
            for line in content.split('\n'):
                line = line.strip()
                if line.startswith(('import ', 'from ')):
                    imports.append(line)
        
        elif file_type in ['javascript', 'typescript']:
            for line in content.split('\n'):
                line = line.strip()
                if 'import' in line and ('from' in line or '{' in line):
                    imports.append(line)
        
        return imports
    
    def _calculate_complexity(self, content: str, file_type: str) -> float:
        """Calculate code complexity score."""
        if not self.config["track_complexity"]:
            return 0.0
        
        complexity = 1.0  # Base complexity
        
        # Count control structures
        control_keywords = ['if', 'else', 'elif', 'for', 'while', 'try', 'except', 'finally', 'with']
        for keyword in control_keywords:
            complexity += content.count(f' {keyword} ')
        
        # Count function definitions
        if file_type == 'python':
            complexity += content.count('def ')
        elif file_type in ['javascript', 'typescript']:
            complexity += content.count('function ')
        
        return complexity
    
    def scan_codebase(self) -> CodebaseSnapshot:
        """Scan the entire codebase and create a snapshot."""
        logger.info(f"Scanning codebase at {self.root_path}")
        
        file_metadata = {}
        total_lines = 0
        file_types = {}
        dependencies = {}
        
        # Walk through all files
        for root, dirs, files in os.walk(self.root_path):
            # Skip ignored directories
            dirs[:] = [d for d in dirs if not self._should_ignore_file(Path(root) / d)]
            
            for file in files:
                file_path = Path(root) / file
                
                if self._should_ignore_file(file_path):
                    continue
                
                # Analyze file
                metadata = self._analyze_file(file_path)
                if metadata:
                    file_metadata[str(file_path.relative_to(self.root_path))] = metadata
                    total_lines += metadata.lines_of_code
                    
                    # Track file types
                    if metadata.file_type not in file_types:
                        file_types[metadata.file_type] = 0
                    file_types[metadata.file_type] += 1
                    
                    # Track dependencies
                    if metadata.dependencies:
                        dependencies[str(file_path.relative_to(self.root_path))] = metadata.dependencies
        
        # Create snapshot
        snapshot = CodebaseSnapshot(
            timestamp=time.time(),
            total_files=len(file_metadata),
            total_lines=total_lines,
            file_types=file_types,
            dependencies=dependencies,
            file_metadata=file_metadata
        )
        
        # Update tracking data
        self.tracking_data["snapshots"].append(asdict(snapshot))
        self.tracking_data["current_state"] = asdict(snapshot)
        self.tracking_data["last_scan"] = time.time()
        
        # Keep only last 10 snapshots
        if len(self.tracking_data["snapshots"]) > 10:
            self.tracking_data["snapshots"] = self.tracking_data["snapshots"][-10:]
        
        self._save_tracking_data()
        
        logger.info(f"Scan complete: {len(file_metadata)} files, {total_lines} lines")
        return snapshot
    
    def get_file_history(self, file_path: str) -> List[Dict[str, Any]]:
        """Get historical data for a specific file."""
        history = []
        for snapshot in self.tracking_data["snapshots"]:
            if file_path in snapshot["file_metadata"]:
                history.append({
                    "timestamp": snapshot["timestamp"],
                    "metadata": snapshot["file_metadata"][file_path]
                })
        return history
    
    def find_similar_files(self, file_path: str, threshold: float = 0.8) -> List[str]:
        """Find files with similar content or structure."""
        if file_path not in self.tracking_data["current_state"]["file_metadata"]:
            return []
        
        target_file = self.tracking_data["current_state"]["file_metadata"][file_path]
        similar_files = []
        
        for path, metadata in self.tracking_data["current_state"]["file_metadata"].items():
            if path == file_path:
                continue
            
            # Simple similarity check based on file type and size
            if (metadata["file_type"] == target_file["file_type"] and 
                abs(metadata["size"] - target_file["size"]) < 1000):
                similar_files.append(path)
        
        return similar_files
    
    def get_dependency_graph(self) -> Dict[str, List[str]]:
        """Get the current dependency graph."""
        return self.tracking_data["current_state"].get("dependencies", {})
    
    def get_statistics(self) -> Dict[str, Any]:
        """Get current codebase statistics."""
        current = self.tracking_data["current_state"]
        return {
            "total_files": current.get("total_files", 0),
            "total_lines": current.get("total_lines", 0),
            "file_types": current.get("file_types", {}),
            "last_scan": datetime.fromtimestamp(self.tracking_data["last_scan"]).isoformat() if self.tracking_data["last_scan"] else None,
            "snapshots_count": len(self.tracking_data["snapshots"])
        }