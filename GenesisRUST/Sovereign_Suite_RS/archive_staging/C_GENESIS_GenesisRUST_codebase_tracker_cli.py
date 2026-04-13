"""
Command-line interface for the micro offline codebase tracker.

This module provides a user-friendly CLI for interacting with the codebase tracker.
"""

import argparse
import json
import sys
import os
from pathlib import Path
from typing import Optional
import time
from datetime import datetime

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

from core import CodebaseTracker
from search import CodebaseSearcher


class CodebaseTrackerCLI:
    """Command-line interface for the codebase tracker."""
    
    def __init__(self):
        self.tracker = None
        self.searcher = None
    
    def run(self):
        """Run the CLI application."""
        parser = argparse.ArgumentParser(
            description="Micro Offline Codebase Tracker",
            formatter_class=argparse.RawDescriptionHelpFormatter,
            epilog="""
Examples:
  python -m codebase_tracker.cli scan --path /path/to/project
  python -m codebase_tracker.cli search --query "function_name" --type python
  python -m codebase_tracker.cli stats
  python -m codebase_tracker.cli tree --depth 2
            """
        )
        
        subparsers = parser.add_subparsers(dest='command', help='Available commands')
        
        # Scan command
        scan_parser = subparsers.add_parser('scan', help='Scan the codebase')
        scan_parser.add_argument('--path', type=str, help='Path to codebase (default: current directory)')
        scan_parser.add_argument('--config', type=str, help='Path to configuration file')
        
        # Search commands
        search_parser = subparsers.add_parser('search', help='Search the codebase')
        search_parser.add_argument('--path', type=str, default='.', help='Path to codebase (default: current directory)')
        search_subparsers = search_parser.add_subparsers(dest='search_type', help='Search type')
        
        # File search
        file_search_parser = search_subparsers.add_parser('files', help='Search for files')
        file_search_parser.add_argument('pattern', type=str, help='Search pattern')
        file_search_parser.add_argument('--types', type=str, help='Comma-separated file types')
        file_search_parser.add_argument('--path-filter', type=str, help='Path filter pattern')
        
        # Content search
        content_search_parser = search_subparsers.add_parser('content', help='Search file content')
        content_search_parser.add_argument('query', type=str, help='Search query')
        content_search_parser.add_argument('--types', type=str, help='Comma-separated file types')
        content_search_parser.add_argument('--case-sensitive', action='store_true', help='Case sensitive search')
        
        # Function search
        func_search_parser = search_subparsers.add_parser('functions', help='Search for functions')
        func_search_parser.add_argument('pattern', type=str, help='Function name pattern')
        func_search_parser.add_argument('--types', type=str, help='Comma-separated file types')
        
        # Class search
        class_search_parser = search_subparsers.add_parser('classes', help='Search for classes')
        class_search_parser.add_argument('pattern', type=str, help='Class name pattern')
        class_search_parser.add_argument('--types', type=str, help='Comma-separated file types')
        
        # Dependency search
        dep_search_parser = search_subparsers.add_parser('dependencies', help='Search dependencies')
        dep_search_parser.add_argument('dependency', type=str, help='Dependency name')
        dep_search_parser.add_argument('--types', type=str, help='Comma-separated file types')
        
        # Stats command
        stats_parser = subparsers.add_parser('stats', help='Show codebase statistics')
        
        # Tree command
        tree_parser = subparsers.add_parser('tree', help='Show file tree')
        tree_parser.add_argument('--depth', type=int, default=3, help='Maximum tree depth')
        tree_parser.add_argument('--filter', type=str, help='Path filter pattern')
        
        # Metrics command
        metrics_parser = subparsers.add_parser('metrics', help='Show code metrics')
        
        # History command
        history_parser = subparsers.add_parser('history', help='Show file history')
        history_parser.add_argument('file', type=str, help='File path')
        
        # Similar command
        similar_parser = subparsers.add_parser('similar', help='Find similar files')
        similar_parser.add_argument('file', type=str, help='File path')
        similar_parser.add_argument('--threshold', type=float, default=0.8, help='Similarity threshold')
        
        # Imports command
        imports_parser = subparsers.add_parser('imports', help='Show file imports')
        imports_parser.add_argument('file', type=str, help='File path')
        
        # Usage command
        usage_parser = subparsers.add_parser('usage', help='Find symbol usage')
        usage_parser.add_argument('symbol', type=str, help='Symbol to search for')
        usage_parser.add_argument('--types', type=str, help='Comma-separated file types')
        
        args = parser.parse_args()
        
        if not args.command:
            parser.print_help()
            return
        
        try:
            self._handle_command(args)
        except KeyboardInterrupt:
            print("\nOperation cancelled by user.")
            sys.exit(1)
        except Exception as e:
            print(f"Error: {e}")
            sys.exit(1)
    
    def _handle_command(self, args):
        """Handle the parsed command."""
        if args.command == 'scan':
            self._handle_scan(args)
        elif args.command == 'search':
            self._handle_search(args)
        elif args.command == 'stats':
            self._handle_stats()
        elif args.command == 'tree':
            self._handle_tree(args)
        elif args.command == 'metrics':
            self._handle_metrics()
        elif args.command == 'history':
            self._handle_history(args)
        elif args.command == 'similar':
            self._handle_similar(args)
        elif args.command == 'imports':
            self._handle_imports(args)
        elif args.command == 'usage':
            self._handle_usage(args)
    
    def _ensure_tracker(self, args):
        """Ensure tracker is initialized."""
        if self.tracker is None:
            path = getattr(args, 'path', None) or '.'
            config_path = getattr(args, 'config', None)
            self.tracker = CodebaseTracker(path, config_path)
            self.searcher = CodebaseSearcher(self.tracker)
    
    def _handle_scan(self, args):
        """Handle scan command."""
        self._ensure_tracker(args)
        print(f"Scanning codebase at {self.tracker.root_path}...")
        
        start_time = time.time()
        snapshot = self.tracker.scan_codebase()
        end_time = time.time()
        
        print(f"Scan completed in {end_time - start_time:.2f} seconds")
        print(f"Found {snapshot.total_files} files, {snapshot.total_lines} lines of code")
        
        # Show file type distribution
        print("\nFile types:")
        for file_type, count in snapshot.file_types.items():
            print(f"  {file_type}: {count}")
    
    def _handle_search(self, args):
        """Handle search commands."""
        self._ensure_tracker(args)
        
        if args.search_type == 'files':
            self._handle_file_search(args)
        elif args.search_type == 'content':
            self._handle_content_search(args)
        elif args.search_type == 'functions':
            self._handle_function_search(args)
        elif args.search_type == 'classes':
            self._handle_class_search(args)
        elif args.search_type == 'dependencies':
            self._handle_dependency_search(args)
    
    def _handle_file_search(self, args):
        """Handle file search."""
        if self.tracker is None:
            print("No tracker initialized. Please run a scan first.")
            return
        file_types = args.types.split(',') if args.types else None
        results = self.searcher.search_files(args.pattern, file_types, args.path_filter)
        
        if not results:
            print("No files found matching the pattern.")
            return
        
        print(f"Found {len(results)} files:")
        for result in results:
            metadata = result['metadata']
            print(f"  {result['path']} ({metadata['file_type']}, {metadata['size']} bytes)")
    
    def _handle_content_search(self, args):
        """Handle content search."""
        if self.tracker is None:
            print("No tracker initialized. Please run a scan first.")
            return
        file_types = args.types.split(',') if args.types else None
        results = self.searcher.search_content(args.query, file_types, args.case_sensitive)
        
        if not results:
            print("No content found matching the query.")
            return
        
        print(f"Found {len(results)} files with matches:")
        for result in results:
            metadata = result['metadata']
            print(f"\n{result['path']} ({result['matches']} matches)")
            print(f"  File type: {metadata['file_type']}")
            print(f"  Lines: {metadata['lines_of_code']}")
            
            # Show first few highlights
            for i, highlight in enumerate(result['highlights'][:3]):
                print(f"\n  Match {i+1}:")
                print(f"  {highlight}")
    
    def _handle_function_search(self, args):
        """Handle function search."""
        if self.tracker is None:
            print("No tracker initialized. Please run a scan first.")
            return
        file_types = args.types.split(',') if args.types else None
        results = self.searcher.search_functions(args.pattern, file_types)
        
        if not results:
            print("No functions found matching the pattern.")
            return
        
        print(f"Found {len(results)} files with matching functions:")
        for result in results:
            print(f"\n{result['path']}:")
            for func in result['functions']:
                print(f"  {func}")
    
    def _handle_class_search(self, args):
        """Handle class search."""
        if self.tracker is None:
            print("No tracker initialized. Please run a scan first.")
            return
        file_types = args.types.split(',') if args.types else None
        results = self.searcher.search_classes(args.pattern, file_types)
        
        if not results:
            print("No classes found matching the pattern.")
            return
        
        print(f"Found {len(results)} files with matching classes:")
        for result in results:
            print(f"\n{result['path']}:")
            for cls in result['classes']:
                print(f"  {cls}")
    
    def _handle_dependency_search(self, args):
        """Handle dependency search."""
        if self.tracker is None:
            print("No tracker initialized. Please run a scan first.")
            return
        file_types = args.types.split(',') if args.types else None
        results = self.searcher.search_dependencies(args.dependency, file_types)
        
        if not results:
            print(f"No files found that depend on '{args.dependency}'.")
            return
        
        print(f"Found {len(results)} files that depend on '{args.dependency}':")
        for result in results:
            metadata = result['metadata']
            print(f"  {result['path']} ({metadata['file_type']})")
    
    def _handle_stats(self):
        """Handle stats command."""
        if self.tracker is None:
            print("No tracker initialized. Please run a scan first.")
            return
        stats = self.tracker.get_statistics()
        
        print("Codebase Statistics:")
        print(f"  Total files: {stats['total_files']}")
        print(f"  Total lines: {stats['total_lines']}")
        print(f"  Last scan: {stats['last_scan']}")
        print(f"  Snapshots: {stats['snapshots_count']}")
        
        print("\nFile types:")
        for file_type, count in stats['file_types'].items():
            print(f"  {file_type}: {count}")
    
    def _handle_tree(self, args):
        """Handle tree command."""
        if self.tracker is None:
            print("No tracker initialized. Please run a scan first.")
            return
        tree = self.searcher.get_file_tree(args.depth)
        print(tree)
    
    def _handle_metrics(self):
        """Handle metrics command."""
        if self.tracker is None:
            print("No tracker initialized. Please run a scan first.")
            return
        metrics = self.searcher.get_code_metrics()
        
        print("Code Metrics:")
        print(f"  Total files: {metrics['total_files']}")
        print(f"  Total lines: {metrics['total_lines']}")
        print(f"  Average complexity: {metrics['complexity']['avg_complexity']:.2f}")
        print(f"  Maximum complexity: {metrics['complexity']['max_complexity']:.2f}")
        
        print("\nComplex files (top 5):")
        for file_info in metrics['complexity']['complex_files'][:5]:
            print(f"  {file_info['path']}: complexity {file_info['complexity']:.2f}, {file_info['lines']} lines")
        
        print(f"\nTotal dependencies: {metrics['dependencies']['total_dependencies']}")
        print("\nMost dependent files (top 5):")
        for file_info in metrics['dependencies']['most_dependent_files'][:5]:
            print(f"  {file_info['path']}: {file_info['dependency_count']} dependencies")
    
    def _handle_history(self, args):
        """Handle history command."""
        self._ensure_tracker(argparse.Namespace(path='.'))
        history = self.tracker.get_file_history(args.file)
        
        if not history:
            print(f"No history found for file: {args.file}")
            return
        
        print(f"History for {args.file}:")
        for entry in history:
            timestamp = datetime.fromtimestamp(entry['timestamp'])
            metadata = entry['metadata']
            print(f"  {timestamp}: {metadata['size']} bytes, {metadata['lines_of_code']} lines")
    
    def _handle_similar(self, args):
        """Handle similar command."""
        self._ensure_tracker(argparse.Namespace(path='.'))
        similar_files = self.tracker.find_similar_files(args.file, args.threshold)
        
        if not similar_files:
            print(f"No similar files found for: {args.file}")
            return
        
        print(f"Files similar to {args.file}:")
        for file_path in similar_files:
            print(f"  {file_path}")
    
    def _handle_imports(self, args):
        """Handle imports command."""
        self._ensure_tracker(argparse.Namespace(path='.'))
        imports = self.searcher.find_imports(args.file)
        
        if not imports:
            print(f"No imports found in file: {args.file}")
            return
        
        print(f"Imports in {args.file}:")
        for imp in imports:
            print(f"  {imp}")
    
    def _handle_usage(self, args):
        """Handle usage command."""
        self._ensure_tracker(argparse.Namespace(path='.'))
        file_types = args.types.split(',') if args.types else None
        results = self.searcher.find_usage(args.symbol, file_types)
        
        if not results:
            print(f"No usage found for symbol: {args.symbol}")
            return
        
        print(f"Usage of '{args.symbol}' found in {len(results)} locations:")
        for result in results:
            print(f"\n{result['path']} ({result['type']})")
            if result['type'] == 'usage' and 'matches' in result:
                print(f"  Matches: {result['matches']}")


def main():
    """Main entry point."""
    cli = CodebaseTrackerCLI()
    cli.run()


if __name__ == '__main__':
    main()