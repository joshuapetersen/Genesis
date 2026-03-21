"""
World-Class Encyclopedia Builder
Automatically extracts and indexes coding knowledge from multiple authoritative sources.
"""
import json
import os
from typing import List, Dict
from coding_encyclopedia_indexer import CodingEncyclopediaIndexer

class WorldClassEncyclopediaBuilder:
    """
    Builds a world-class coding encyclopedia from multiple sources.
    """
    
    def __init__(self, db_path="c:\\SarahCore\\vault\\coding_encyclopedia"):
        self.indexer = CodingEncyclopediaIndexer(db_path)
        self.SOVEREIGN_ANCHOR = 1.09277703703703
        self.entries = []
        
    def add_python_stdlib_advanced(self) -> List[Dict]:
        """
        Add advanced Python standard library modules.
        """
        print("[Encyclopedia] Adding advanced Python stdlib...")
        
        advanced_modules = {
            # Concurrency & Parallelism
            "concurrent.futures": {
                "desc": "High-level interface for asynchronously executing callables",
                "category": "concurrency",
                "implementation": """from concurrent.futures import ThreadPoolExecutor, ProcessPoolExecutor

# Thread pool for I/O-bound tasks
with ThreadPoolExecutor(max_workers=4) as executor:
    futures = [executor.submit(fetch_url, url) for url in urls]
    results = [f.result() for f in futures]

# Process pool for CPU-bound tasks
with ProcessPoolExecutor(max_workers=4) as executor:
    results = executor.map(compute_intensive_task, data)""",
                "use_cases": ["parallel processing", "async execution", "task scheduling"]
            },
            "queue": {
                "desc": "Thread-safe FIFO queue implementation",
                "category": "concurrency",
                "implementation": """from queue import Queue, PriorityQueue
import threading

# Producer-consumer pattern
queue = Queue(maxsize=10)

def producer():
    for i in range(5):
        queue.put(i)

def consumer():
    while True:
        item = queue.get()
        process(item)
        queue.task_done()

# Priority queue
pq = PriorityQueue()
pq.put((1, 'high priority'))
pq.put((5, 'low priority'))""",
                "use_cases": ["producer-consumer", "task queues", "thread communication"]
            },
            
            # Data Persistence
            "shelve": {
                "desc": "Python object persistence using dictionary-like interface",
                "category": "persistence",
                "implementation": """import shelve

# Store Python objects
with shelve.open('mydata') as db:
    db['users'] = {'alice': 25, 'bob': 30}
    db['config'] = {'debug': True}

# Retrieve
with shelve.open('mydata') as db:
    users = db['users']
    config = db['config']""",
                "use_cases": ["object storage", "caching", "configuration"]
            },
            "dbm": {
                "desc": "Simple database interface for key-value storage",
                "category": "persistence",
                "implementation": """import dbm

# Create/open database
with dbm.open('cache', 'c') as db:
    db['key'] = b'value'
    value = db['key']""",
                "use_cases": ["simple databases", "caching", "key-value storage"]
            },
            
            # Networking
            "socket": {
                "desc": "Low-level networking interface",
                "category": "networking",
                "implementation": """import socket

# TCP server
server = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
server.bind(('localhost', 8080))
server.listen(5)
client, addr = server.accept()

# TCP client
client = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
client.connect(('localhost', 8080))
client.send(b'Hello')
data = client.recv(1024)""",
                "use_cases": ["network programming", "TCP/UDP", "low-level protocols"]
            },
            "http.server": {
                "desc": "Simple HTTP server implementation",
                "category": "networking",
                "implementation": """from http.server import HTTPServer, BaseHTTPRequestHandler

class Handler(BaseHTTPRequestHandler):
    def do_GET(self):
        self.send_response(200)
        self.send_header('Content-type', 'text/html')
        self.end_headers()
        self.wfile.write(b'Hello World')

server = HTTPServer(('localhost', 8000), Handler)
server.serve_forever()""",
                "use_cases": ["web servers", "API endpoints", "testing"]
            },
            
            # Data Formats
            "csv": {
                "desc": "CSV file reading and writing",
                "category": "data_format",
                "implementation": """import csv

# Read CSV
with open('data.csv', 'r') as f:
    reader = csv.DictReader(f)
    for row in reader:
        print(row['name'], row['age'])

# Write CSV
with open('output.csv', 'w', newline='') as f:
    writer = csv.DictWriter(f, fieldnames=['name', 'age'])
    writer.writeheader()
    writer.writerow({'name': 'Alice', 'age': 25})""",
                "use_cases": ["data import/export", "spreadsheets", "data processing"]
            },
            "xml.etree.ElementTree": {
                "desc": "XML parsing and creation",
                "category": "data_format",
                "implementation": """import xml.etree.ElementTree as ET

# Parse XML
tree = ET.parse('data.xml')
root = tree.getroot()
for child in root:
    print(child.tag, child.attrib)

# Create XML
root = ET.Element('root')
child = ET.SubElement(root, 'child', attrib={'name': 'value'})
tree = ET.ElementTree(root)
tree.write('output.xml')""",
                "use_cases": ["XML processing", "configuration files", "data exchange"]
            },
            
            # Compression
            "gzip": {
                "desc": "Gzip compression and decompression",
                "category": "compression",
                "implementation": """import gzip

# Compress
with gzip.open('file.gz', 'wb') as f:
    f.write(b'Large data to compress')

# Decompress
with gzip.open('file.gz', 'rb') as f:
    data = f.read()""",
                "use_cases": ["file compression", "data transfer", "storage optimization"]
            },
            "zipfile": {
                "desc": "ZIP archive creation and extraction",
                "category": "compression",
                "implementation": """import zipfile

# Create ZIP
with zipfile.ZipFile('archive.zip', 'w') as zf:
    zf.write('file1.txt')
    zf.write('file2.txt')

# Extract ZIP
with zipfile.ZipFile('archive.zip', 'r') as zf:
    zf.extractall('output_dir')""",
                "use_cases": ["file archiving", "package distribution", "backup"]
            },
            
            # Testing
            "unittest.mock": {
                "desc": "Mock object library for testing",
                "category": "testing",
                "implementation": """from unittest.mock import Mock, patch, MagicMock

# Mock objects
mock_db = Mock()
mock_db.query.return_value = [{'id': 1, 'name': 'Alice'}]

# Patching
with patch('module.function') as mock_func:
    mock_func.return_value = 42
    result = module.function()  # Returns 42

# MagicMock for special methods
mock = MagicMock()
mock.__len__.return_value = 10""",
                "use_cases": ["unit testing", "test isolation", "dependency mocking"]
            },
            "doctest": {
                "desc": "Test interactive Python examples in docstrings",
                "category": "testing",
                "implementation": """def add(a, b):
    '''
    Add two numbers.
    
    >>> add(2, 3)
    5
    >>> add(-1, 1)
    0
    '''
    return a + b

if __name__ == '__main__':
    import doctest
    doctest.testmod()""",
                "use_cases": ["documentation testing", "example validation", "TDD"]
            },
            
            # Cryptography
            "secrets": {
                "desc": "Generate cryptographically strong random numbers",
                "category": "security",
                "implementation": """import secrets

# Generate secure tokens
token = secrets.token_hex(16)  # 32-char hex string
token_url = secrets.token_urlsafe(16)  # URL-safe token

# Generate random numbers
secure_random = secrets.randbelow(100)  # 0-99

# Compare strings securely (timing attack resistant)
secrets.compare_digest(hash1, hash2)""",
                "use_cases": ["token generation", "password reset", "session IDs"]
            },
            "hmac": {
                "desc": "Keyed-hashing for message authentication",
                "category": "security",
                "implementation": """import hmac
import hashlib

# Create HMAC
key = b'secret_key'
message = b'important message'
signature = hmac.new(key, message, hashlib.sha256).hexdigest()

# Verify HMAC
is_valid = hmac.compare_digest(signature, expected_signature)""",
                "use_cases": ["message authentication", "API signatures", "data integrity"]
            }
        }
        
        entries = []
        for module, info in advanced_modules.items():
            entries.append({
                "term": module,
                "description": info["desc"],
                "category": info["category"],
                "language": "python",
                "complexity": "N/A",
                "use_cases": json.dumps(info["use_cases"]),
                "implementation": info["implementation"],
                "related": "",
                "source": "python_stdlib_advanced"
            })
        
        print(f"[Encyclopedia] Added {len(entries)} advanced stdlib entries")
        return entries
    
    def add_numpy_scipy_algorithms(self) -> List[Dict]:
        """
        Add NumPy and SciPy scientific computing algorithms.
        """
        print("[Encyclopedia] Adding NumPy/SciPy algorithms...")
        
        scientific_algos = {
            "numpy_vectorization": {
                "desc": "Vectorized operations for fast array computations",
                "category": "performance",
                "implementation": """import numpy as np

# Vectorized operations (100x faster than loops)
arr = np.array([1, 2, 3, 4, 5])
squared = arr ** 2  # Element-wise squaring
normalized = (arr - arr.mean()) / arr.std()

# Broadcasting
matrix = np.array([[1, 2, 3], [4, 5, 6]])
row_vector = np.array([10, 20, 30])
result = matrix + row_vector  # Broadcasts across rows

# Universal functions (ufuncs)
np.sin(arr)  # Vectorized sine
np.exp(arr)  # Vectorized exponential""",
                "use_cases": ["numerical computing", "data processing", "machine learning"]
            },
            "scipy_optimization": {
                "desc": "Optimization algorithms for finding minima/maxima",
                "category": "optimization",
                "implementation": """from scipy.optimize import minimize, curve_fit

# Function minimization
def objective(x):
    return x[0]**2 + x[1]**2

result = minimize(objective, x0=[1, 1])
print(result.x)  # Optimal solution

# Curve fitting
def model(x, a, b):
    return a * np.exp(b * x)

params, _ = curve_fit(model, xdata, ydata)""",
                "use_cases": ["parameter tuning", "curve fitting", "optimization problems"]
            },
            "scipy_linear_algebra": {
                "desc": "Advanced linear algebra operations",
                "category": "linear_algebra",
                "implementation": """from scipy import linalg

# Solve linear system Ax = b
A = np.array([[3, 1], [1, 2]])
b = np.array([9, 8])
x = linalg.solve(A, b)

# Eigenvalues and eigenvectors
eigenvalues, eigenvectors = linalg.eig(A)

# Matrix decompositions
P, L, U = linalg.lu(A)  # LU decomposition
Q, R = linalg.qr(A)     # QR decomposition""",
                "use_cases": ["linear systems", "eigenanalysis", "matrix factorization"]
            },
            "scipy_signal_processing": {
                "desc": "Signal processing and filtering",
                "category": "signal_processing",
                "implementation": """from scipy import signal

# Design filters
b, a = signal.butter(4, 0.1, 'low')  # Butterworth filter
filtered = signal.filtfilt(b, a, noisy_signal)

# FFT for frequency analysis
from scipy.fft import fft, fftfreq
spectrum = fft(signal_data)
frequencies = fftfreq(len(signal_data), d=sampling_interval)

# Convolution
convolved = signal.convolve(signal1, signal2, mode='same')""",
                "use_cases": ["audio processing", "image filtering", "frequency analysis"]
            },
            "networkx_graphs": {
                "desc": "Graph algorithms and network analysis",
                "category": "graph_algorithms",
                "implementation": """import networkx as nx

# Create graph
G = nx.Graph()
G.add_edges_from([(1, 2), (2, 3), (3, 4), (4, 1)])

# Shortest path
path = nx.shortest_path(G, source=1, target=3)

# Centrality measures
degree_cent = nx.degree_centrality(G)
betweenness = nx.betweenness_centrality(G)

# Community detection
communities = nx.community.greedy_modularity_communities(G)

# Minimum spanning tree
mst = nx.minimum_spanning_tree(G)""",
                "use_cases": ["network analysis", "social networks", "routing algorithms"]
            }
        }
        
        entries = []
        for algo, info in scientific_algos.items():
            entries.append({
                "term": algo,
                "description": info["desc"],
                "category": info["category"],
                "language": "python",
                "complexity": "N/A",
                "use_cases": json.dumps(info["use_cases"]),
                "implementation": info["implementation"],
                "related": "",
                "source": "numpy_scipy_networkx"
            })
        
        print(f"[Encyclopedia] Added {len(entries)} scientific computing entries")
        return entries
    
    def ingest_json_content(self) -> List[Dict]:
        """
        Ingest encyclopedia entries from JSON files in data/encyclopedia_content.
        """
        print("[Encyclopedia] Scanning for JSON content packages...")
        content_dir = r"C:\SarahCore\data\encyclopedia_content"
        entries = []
        
        if not os.path.exists(content_dir):
            os.makedirs(content_dir, exist_ok=True)
            print(f"[Encyclopedia] Created content directory: {content_dir}")
            return entries

        for filename in os.listdir(content_dir):
            if filename.endswith(".json"):
                file_path = os.path.join(content_dir, filename)
                try:
                    with open(file_path, 'r', encoding='utf-8') as f:
                        data = json.load(f)
                        if isinstance(data, list):
                            print(f"[Encyclopedia] Ingesting {len(data)} entries from {filename}")
                            entries.extend(data)
                        else:
                            print(f"[Encyclopedia] Skipping {filename}: Root must be a list")
                except Exception as e:
                    print(f"[Encyclopedia] Error reading {filename}: {e}")
        
        return entries

    def build_comprehensive_index(self):
        """
        Build comprehensive encyclopedia index from all sources.
        """
        print("[Encyclopedia] Building comprehensive world-class index...")
        
        # Add all entry sources
        all_entries = []
        all_entries.extend(self.add_python_stdlib_advanced())
        all_entries.extend(self.add_numpy_scipy_algorithms())
        all_entries.extend(self.ingest_json_content())  # New dynamic source
        
        # Generate ACE fingerprints
        indexed_entries = []
        for entry in all_entries:
            ace_fp = self.indexer.generate_ace_fingerprint(entry["term"])
            
            # Ensure all fields exist
            indexed_entry = {
                "term": entry.get("term", "Unknown"),
                "ace_fingerprint": ace_fp,
                "description": entry.get("description", ""),
                "category": entry.get("category", "Uncategorized"),
                "language": entry.get("language", "python"),
                "complexity": entry.get("complexity", "N/A"),
                "use_cases": entry.get("use_cases", []),
                "implementation": entry.get("implementation", ""),
                "related": entry.get("related", [])
            }
            
            # Handle list-to-string conversion for storage if needed
            if isinstance(indexed_entry["use_cases"], list):
                indexed_entry["use_cases"] = json.dumps(indexed_entry["use_cases"])
            if isinstance(indexed_entry["related"], list):
                indexed_entry["related"] = json.dumps(indexed_entry["related"])
                
            indexed_entries.append(indexed_entry)
        
        # Add to existing table
        try:
            table = self.indexer.db.open_table(self.indexer.table_name)
            # Optional: Deduplicate or Clear? For now, append/update.
            # Ideally we check existence, but simpler to just add for now.
            table.add(indexed_entries)
            total_count = table.count_rows()
            print(f"[Encyclopedia] Successfully added {len(indexed_entries)} entries")
            print(f"[Encyclopedia] Total entries in database: {total_count}")
        except Exception as e:
            print(f"[Encyclopedia] Error adding to table: {e}")
            try:
                print(f"[Encyclopedia] Creating new table...")
                table = self.indexer.db.create_table(self.indexer.table_name, indexed_entries)
                print(f"[Encyclopedia] Created table with {len(indexed_entries)} entries")
            except Exception as e2:
                 print(f"[Encyclopedia] CRITICAL DB ERROR: {e2}")
        
        return len(indexed_entries)

def main():
    """
    Build world-class encyclopedia.
    """
    builder = WorldClassEncyclopediaBuilder()
    count = builder.build_comprehensive_index()
    print(f"\n[Encyclopedia] Complete! Added {count} world-class entries")
    print(f"[Encyclopedia] Sarah now has access to advanced Python stdlib, NumPy, SciPy, and NetworkX knowledge")

if __name__ == "__main__":
    main()
