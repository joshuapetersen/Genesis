"""
Search and retrieval engine for the codebase tracker.

This module provides powerful search capabilities to find files, functions,
classes, and other code elements within the tracked codebase.
"""

import re
import os
from typing import List, Dict, Any, Optional, Tuple
from pathlib import Path
import fnmatch
from dataclasses import asdict


class CodebaseSearcher:
    """Advanced search engine for codebase analysis."""
    
    def __init__(self, tracker):
        """
        Initialize the searcher with a tracker instance.
        
        Args:
            tracker: CodebaseTracker instance
        """
        self.tracker = tracker
    
    def search_files(self, pattern: str, file_types: Optional[List[str]] = None, 
                    path_filter: Optional[str] = None) -> List[Dict[str, Any]]:
        """
        Search for files matching a pattern.
        
        Args:
            pattern: Glob pattern or regex pattern
            file_types: List of file types to search (e.g., ['python', 'javascript'])
            path_filter: Path pattern to filter results
            
        Returns:
            List of matching files with metadata
        """
        results = []
        current_state = self.tracker.tracking_data["current_state"]
        
        for file_path, metadata in current_state.get("file_metadata", {}).items():
            # Apply file type filter
            if file_types and metadata["file_type"] not in file_types:
                continue
            
            # Apply path filter
            if path_filter and not fnmatch.fnmatch(file_path, path_filter):
                continue
            
            # Check if file matches pattern
            if fnmatch.fnmatch(file_path, pattern) or re.search(pattern, file_path):
                results.append({
                    "path": file_path,
                    "metadata": metadata
                })
        
        return sorted(results, key=lambda x: x["metadata"]["last_modified"], reverse=True)
    
    def search_content(self, query: str, file_types: Optional[List[str]] = None,
                      case_sensitive: bool = False) -> List[Dict[str, Any]]:
        """
        Search for content within files.
        
        Args:
            query: Search query (supports regex)
            file_types: List of file types to search
            case_sensitive: Whether search should be case sensitive
            
        Returns:
            List of matching files with content highlights
        """
        results = []
        current_state = self.tracker.tracking_data["current_state"]
        
        flags = 0 if case_sensitive else re.IGNORECASE
        pattern = re.compile(query, flags)
        
        for file_path, metadata in current_state.get("file_metadata", {}).items():
            # Apply file type filter
            if file_types and metadata["file_type"] not in file_types:
                continue
            
            # Read file content for search
            try:
                full_path = self.tracker.root_path / file_path
                with open(full_path, 'r', encoding='utf-8', errors='ignore') as f:
                    content = f.read()
                
                # Find matches
                matches = list(pattern.finditer(content))
                if matches:
                    highlights = self._extract_context(content, matches, 3)
                    
                    results.append({
                        "path": file_path,
                        "metadata": metadata,
                        "matches": len(matches),
                        "highlights": highlights
                    })
                    
            except Exception as e:
                continue
        
        return sorted(results, key=lambda x: x["matches"], reverse=True)
    
    def search_functions(self, pattern: str, file_types: Optional[List[str]] = None) -> List[Dict[str, Any]]:
        """
        Search for functions matching a pattern.
        
        Args:
            pattern: Function name pattern
            file_types: List of file types to search
            
        Returns:
            List of matching functions with file context
        """
        results = []
        current_state = self.tracker.tracking_data["current_state"]
        
        for file_path, metadata in current_state.get("file_metadata", {}).items():
            # Apply file type filter
            if file_types and metadata["file_type"] not in file_types:
                continue
            
            # Search functions
            matching_functions = [func for func in metadata.get("functions", []) 
                                if fnmatch.fnmatch(func, pattern) or re.search(pattern, func)]
            
            if matching_functions:
                results.append({
                    "path": file_path,
                    "metadata": metadata,
                    "functions": matching_functions
                })
        
        return results
    
    def search_classes(self, pattern: str, file_types: Optional[List[str]] = None) -> List[Dict[str, Any]]:
        """
        Search for classes matching a pattern.
        
        Args:
            pattern: Class name pattern
            file_types: List of file types to search
            
        Returns:
            List of matching classes with file context
        """
        results = []
        current_state = self.tracker.tracking_data["current_state"]
        
        for file_path, metadata in current_state.get("file_metadata", {}).items():
            # Apply file type filter
            if file_types and metadata["file_type"] not in file_types:
                continue
            
            # Search classes
            matching_classes = [cls for cls in metadata.get("classes", []) 
                              if fnmatch.fnmatch(cls, pattern) or re.search(pattern, cls)]
            
            if matching_classes:
                results.append({
                    "path": file_path,
                    "metadata": metadata,
                    "classes": matching_classes
                })
        
        return results
    
    def search_dependencies(self, dependency: str, file_types: Optional[List[str]] = None) -> List[Dict[str, Any]]:
        """
        Search for files that depend on a specific dependency.
        
        Args:
            dependency: Dependency name to search for
            file_types: List of file types to search
            
        Returns:
            List of files that import the dependency
        """
        results = []
        current_state = self.tracker.tracking_data["current_state"]
        
        for file_path, metadata in current_state.get("file_metadata", {}).items():
            # Apply file type filter
            if file_types and metadata["file_type"] not in file_types:
                continue
            
            # Check dependencies
            if dependency in metadata.get("dependencies", []):
                results.append({
                    "path": file_path,
                    "metadata": metadata,
                    "dependency": dependency
                })
        
        return results
    
    def find_imports(self, file_path: str) -> List[Dict[str, Any]]:
        """
        Find all imports in a specific file.
        
        Args:
            file_path: Path to the file
            
        Returns:
            List of import statements
        """
        if file_path not in self.tracker.tracking_data["current_state"]["file_metadata"]:
            return []
        
        metadata = self.tracker.tracking_data["current_state"]["file_metadata"][file_path]
        return metadata.get("imports", [])
    
    def find_usage(self, symbol: str, file_types: Optional[List[str]] = None) -> List[Dict[str, Any]]:
        """
        Find all usages of a symbol across the codebase.
        
        Args:
            symbol: Symbol to search for (function, class, variable name)
            file_types: List of file types to search
            
        Returns:
            List of files where the symbol is used
        """
        results = []
        current_state = self.tracker.tracking_data["current_state"]
        
        # First check direct matches in metadata
        for file_path, metadata in current_state.get("file_metadata", {}).items():
            # Apply file type filter
            if file_types and metadata["file_type"] not in file_types:
                continue
            
            # Check if symbol is a function or class
            if symbol in metadata.get("functions", []) or symbol in metadata.get("classes", []):
                results.append({
                    "path": file_path,
                    "metadata": metadata,
                    "type": "definition",
                    "symbol": symbol
                })
        
        # Then search content for usage
        content_results = self.search_content(
            rf'\b{re.escape(symbol)}\b', 
            file_types=file_types,
            case_sensitive=False
        )
        
        for result in content_results:
            if result["path"] not in [r["path"] for r in results]:
                results.append({
                    "path": result["path"],
                    "metadata": result["metadata"],
                    "type": "usage",
                    "symbol": symbol,
                    "matches": result["matches"]
                })
        
        return results
    
    def get_file_structure(self, path_filter: Optional[str] = None) -> Dict[str, Any]:
        """
        Get the hierarchical structure of the codebase.
        
        Args:
            path_filter: Optional path pattern to filter the structure
            
        Returns:
            Hierarchical structure of files and directories
        """
        structure = {}
        current_state = self.tracker.tracking_data["current_state"]
        
        for file_path in current_state.get("file_metadata", {}).keys():
            # Apply path filter
            if path_filter and not fnmatch.fnmatch(file_path, path_filter):
                continue
            
            parts = Path(file_path).parts
            current = structure
            
            for part in parts:
                if part not in current:
                    current[part] = {}
                current = current[part]
        
        return structure
    
    def get_file_tree(self, max_depth: int = 3) -> str:
        """
        Generate a text-based tree representation of the codebase.
        
        Args:
            max_depth: Maximum depth to display
            
        Returns:
            String representation of the file tree
        """
        structure = self.get_file_structure()
        return self._format_tree(structure, max_depth=max_depth)
    
    def _extract_context(self, content: str, matches: List[re.Match], context_lines: int = 3) -> List[str]:
        """
        Extract context around matches.
        
        Args:
            content: File content
            matches: List of regex matches
            context_lines: Number of lines to show before and after each match
            
        Returns:
            List of context snippets
        """
        lines = content.split('\n')
        highlights = []
        
        for match in matches:
            line_num = content[:match.start()].count('\n') + 1
            start_line = max(0, line_num - context_lines - 1)
            end_line = min(len(lines), line_num + context_lines)
            
            context = []
            for i in range(start_line, end_line):
                line_content = lines[i]
                if i == line_num - 1:
                    # Highlight the matching line
                    start_pos = match.start() - content.rfind('\n', 0, match.start()) - 1
                    end_pos = match.end() - content.rfind('\n', 0, match.end()) - 1
                    highlighted_line = (line_content[:start_pos] + 
                                      ">>> " + line_content[start_pos:end_pos] + " <<<" + 
                                      line_content[end_pos:])
                    context.append(f"{i+1:4d}: {highlighted_line}")
                else:
                    context.append(f"{i+1:4d}: {line_content}")
            
            highlights.append('\n'.join(context))
        
        return highlights
    
    def _format_tree(self, structure: Dict[str, Any], indent: int = 0, max_depth: int = 3) -> str:
        """
        Format the file structure as a tree.
        
        Args:
            structure: File structure dictionary
            indent: Current indentation level
            max_depth: Maximum depth to display
            
        Returns:
            Formatted tree string
        """
        if indent >= max_depth:
            return ""
        
        result = []
        items = sorted(structure.keys())
        
        for i, item in enumerate(items):
            is_last = i == len(items) - 1
            prefix = "└── " if is_last else "├── "
            result.append("  " * indent + prefix + item)
            
            if structure[item]:
                subtree = self._format_tree(structure[item], indent + 1, max_depth)
                if subtree:
                    result.append(subtree)
        
        return '\n'.join(result)
    
    def get_code_metrics(self) -> Dict[str, Any]:
        """
        Get comprehensive code metrics and statistics.
        
        Returns:
            Dictionary containing various code metrics
        """
        current_state = self.tracker.tracking_data["current_state"]
        metrics = {
            "total_files": current_state.get("total_files", 0),
            "total_lines": current_state.get("total_lines", 0),
            "file_types": current_state.get("file_types", {}),
            "complexity": {
                "avg_complexity": 0,
                "max_complexity": 0,
                "complex_files": []
            },
            "dependencies": {
                "total_dependencies": 0,
                "most_dependent_files": [],
                "dependency_types": {}
            }
        }
        
        # Calculate complexity metrics
        total_complexity = 0
        max_complexity = 0
        complex_files = []
        
        for file_path, metadata in current_state.get("file_metadata", {}).items():
            complexity = metadata.get("complexity_score", 0)
            total_complexity += complexity
            
            if complexity > max_complexity:
                max_complexity = complexity
            
            if complexity > 10:  # Threshold for complex files
                complex_files.append({
                    "path": file_path,
                    "complexity": complexity,
                    "lines": metadata.get("lines_of_code", 0)
                })
        
        if metrics["total_files"] > 0:
            metrics["complexity"]["avg_complexity"] = total_complexity / metrics["total_files"]
        
        metrics["complexity"]["max_complexity"] = max_complexity
        metrics["complexity"]["complex_files"] = sorted(complex_files, key=lambda x: x["complexity"], reverse=True)[:10]
        
        # Calculate dependency metrics
        dependencies = current_state.get("dependencies", {})
        total_deps = sum(len(deps) for deps in dependencies.values())
        metrics["dependencies"]["total_dependencies"] = total_deps
        
        # Count dependency types
        dep_types = {}
        for file_deps in dependencies.values():
            for dep in file_deps:
                dep_type = dep.split('/')[0] if '/' in dep else dep
                dep_types[dep_type] = dep_types.get(dep_type, 0) + 1
        
        metrics["dependencies"]["dependency_types"] = dep_types
        
        # Find most dependent files
        most_dependent = []
        for file_path, deps in dependencies.items():
            if len(deps) > 5:  # Threshold for highly dependent files
                most_dependent.append({
                    "path": file_path,
                    "dependency_count": len(deps)
                })
        
        metrics["dependencies"]["most_dependent_files"] = sorted(most_dependent, key=lambda x: x["dependency_count"], reverse=True)[:10]
        
        return metrics