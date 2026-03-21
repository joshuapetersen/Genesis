# Micro Offline Codebase Tracker

A lightweight, offline agent designed to map and track your codebase structure, dependencies, and changes without requiring external services or internet connectivity.

## Features

- **Offline Operation**: No internet connection required
- **Comprehensive Analysis**: Tracks files, functions, classes, dependencies, and complexity
- **Powerful Search**: Advanced search capabilities across the entire codebase
- **Historical Tracking**: Maintains snapshots and file history
- **Lightweight**: Minimal resource usage with efficient storage
- **Extensible**: Easy to configure and customize for different project types

## Installation

The codebase tracker is a standalone Python package. Simply copy the `codebase_tracker` directory to your project root.

## Quick Start

### Basic Usage

```bash
# Scan your current directory
python -m codebase_tracker.cli scan

# Search for files
python -m codebase_tracker.cli search files "*.py"

# Search file content
python -m codebase_tracker.cli search content "function_name"

# Show statistics
python -m codebase_tracker.cli stats

# Display file tree
python -m codebase_tracker.cli tree --depth 2
```

### Programmatic Usage

```python
from codebase_tracker.core import CodebaseTracker
from codebase_tracker.search import CodebaseSearcher

# Initialize tracker
tracker = CodebaseTracker("/path/to/your/project")

# Scan the codebase
snapshot = tracker.scan_codebase()

# Create searcher
searcher = CodebaseSearcher(tracker)

# Search for files
results = searcher.search_files("*.py")

# Search content
content_results = searcher.search_content("function_name")

# Get statistics
stats = tracker.get_statistics()
```

## Command Reference

### Scan Commands

```bash
# Scan with custom path
python -m codebase_tracker.cli scan --path /custom/path

# Scan with configuration file
python -m codebase_tracker.cli scan --config config.json
```

### Search Commands

```bash
# File search
python -m codebase_tracker.cli search files "pattern" --types python,javascript

# Content search
python -m codebase_tracker.cli search content "query" --case-sensitive

# Function search
python -m codebase_tracker.cli search functions "func_*"

# Class search
python -m codebase_tracker.cli search classes "MyClass"

# Dependency search
python -m codebase_tracker.cli search dependencies "requests"
```

### Analysis Commands

```bash
# Show codebase statistics
python -m codebase_tracker.cli stats

# Display file tree
python -m codebase_tracker.cli tree --depth 3 --filter "*.py"

# Show code metrics
python -m codebase_tracker.cli metrics

# View file history
python -m codebase_tracker.cli history path/to/file.py

# Find similar files
python -m codebase_tracker.cli similar path/to/file.py

# Show file imports
python -m codebase_tracker.cli imports path/to/file.py

# Find symbol usage
python -m codebase_tracker.cli usage "symbol_name"
```

## Configuration

Create a `config.json` file to customize the tracker behavior:

```json
{
  "ignore_patterns": [
    ".git",
    "node_modules",
    "__pycache__",
    "*.log"
  ],
  "max_file_size": 2097152,
  "scan_depth": 10,
  "track_dependencies": true,
  "track_complexity": true
}
```

### Configuration Options

- `ignore_patterns`: List of patterns to ignore during scanning
- `max_file_size`: Maximum file size to process (in bytes)
- `scan_depth`: Maximum directory depth to scan
- `track_dependencies`: Whether to track file dependencies
- `track_complexity`: Whether to calculate code complexity

## File Types Supported

The tracker automatically detects and analyzes the following file types:

- Python (.py)
- JavaScript (.js, .jsx)
- TypeScript (.ts, .tsx)
- Java (.java)
- C/C++ (.cpp, .c, .h)
- C# (.cs)
- Go (.go)
- Rust (.rust)
- Ruby (.rb)
- PHP (.php)
- Swift (.swift)
- Kotlin (.kt)
- HTML (.html)
- CSS (.css, .scss, .less)
- Configuration files (.json, .yaml, .xml)
- Documentation (.md, .txt)

## Data Storage

The tracker stores all data in a local `.codebase_tracker/tracking_db.json` file within your project directory. This file contains:

- File metadata and snapshots
- Dependency graphs
- Historical data
- Search indexes

## Performance

- **Memory Usage**: Minimal - only loads necessary data on demand
- **Storage**: Efficient JSON storage with automatic cleanup
- **Scan Speed**: Fast directory traversal with parallel file analysis
- **Search Speed**: Optimized search algorithms with context highlighting

## Use Cases

1. **Codebase Exploration**: Quickly understand project structure and organization
2. **Dependency Analysis**: Track and visualize code dependencies
3. **Code Quality**: Monitor complexity and identify areas for refactoring
4. **Search and Discovery**: Find files, functions, and code patterns efficiently
5. **Historical Analysis**: Track changes and evolution over time
6. **Documentation**: Generate project documentation from code structure

## Security

- **Offline Operation**: No data leaves your machine
- **Local Storage**: All data stored locally in your project directory
- **No External Dependencies**: Self-contained with minimal Python dependencies

## Contributing

To contribute to the codebase tracker:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Submit a pull request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Support

For issues, questions, or feature requests, please create an issue in the project repository.
