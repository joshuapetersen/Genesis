"""
Coding Encyclopedia Indexer
Downloads and indexes comprehensive programming knowledge for Sarah's R&D capabilities.
"""
import os
import json
import hashlib
from Sovereign_Constants import HEX_RADIX, VAR_16, VAR_2000, VAR_3, VAR_10
import lancedb
from typing import List, Dict
from Sovereign_Constants import SOVEREIGN_ANCHOR, ACE_64_BIT_MASK
from sovereign_data_source import SOVEREIGN_KNOWLEDGE_ENTRIES

class CodingEncyclopediaIndexer:
    """
    Indexes comprehensive coding knowledge using ACE Token 64-bit fingerprints.
    Covers algorithms, data structures, design patterns, and documentation.
    """
    
    def __init__(self, db_path: str = "C:\GenesisOS_Core\\vault\\coding_encyclopedia"):
        self.db_path = db_path
        os.makedirs(self.db_path, exist_ok=True)
        
        self.db = lancedb.connect(self.db_path)
        self.table_name = "coding_knowledge"
        self.SOVEREIGN_ANCHOR = SOVEREIGN_ANCHOR
        
        print(f"[Coding Encyclopedia] Initialized at {self.db_path}")
    
    def generate_ace_fingerprint(self, term: str) -> str:
        """
        Generates a 64-bit ACE Token fingerprint for a coding term.
        """
        combined = f"{term.lower()}{self.SOVEREIGN_ANCHOR}"
        hash_obj = hashlib.sha256(combined.encode())
        ace_fp = int(hash_obj.hexdigest(), HEX_RADIX) & ACE_64_BIT_MASK
        return hex(ace_fp)
    
    def load_python_stdlib(self) -> List[Dict]:
        """
        Loads Python standard library documentation.
        """
        print("[Coding Encyclopedia] Loading Python standard library...")
        
        # Python standard library modules with descriptions
        stdlib_modules = {
            # Built-in Functions
            "abs": {"desc": "Return the absolute value of a number", "category": "builtin", "complexity": "O(1)"},
            "all": {"desc": "Return True if all elements are true", "category": "builtin", "complexity": "O(n)"},
            "any": {"desc": "Return True if any element is true", "category": "builtin", "complexity": "O(n)"},
            "enumerate": {"desc": "Return an enumerate object yielding index-value pairs", "category": "builtin", "complexity": "O(1)"},
            "filter": {"desc": "Filter elements based on a function", "category": "builtin", "complexity": "O(n)"},
            "map": {"desc": "Apply function to every item of iterable", "category": "builtin", "complexity": "O(n)"},
            "sorted": {"desc": "Return a sorted list from an iterable", "category": "builtin", "complexity": "O(n log n)"},
            "zip": {"desc": "Aggregate elements from multiple iterables", "category": "builtin", "complexity": "O(n)"},
            
            # Data Structures
            "list": {"desc": "Mutable sequence, typically used to store collections", "category": "data_structure", "complexity": "append O(1), insert O(n)"},
            "dict": {"desc": "Hash table mapping keys to values", "category": "data_structure", "complexity": "get/set O(1)"},
            "set": {"desc": "Unordered collection of unique elements", "category": "data_structure", "complexity": "add/remove O(1)"},
            "tuple": {"desc": "Immutable sequence", "category": "data_structure", "complexity": "access O(1)"},
            
            # Standard Library Modules
            "os": {"desc": "Operating system interfaces", "category": "stdlib", "use_cases": ["file operations", "process management"]},
            "sys": {"desc": "System-specific parameters and functions", "category": "stdlib", "use_cases": ["command line args", "exit codes"]},
            "json": {"desc": "JSON encoder and decoder", "category": "stdlib", "use_cases": ["API responses", "config files"]},
            "datetime": {"desc": "Date and time manipulation", "category": "stdlib", "use_cases": ["timestamps", "scheduling"]},
            "re": {"desc": "Regular expression operations", "category": "stdlib", "use_cases": ["pattern matching", "text parsing"]},
            "pathlib": {"desc": "Object-oriented filesystem paths", "category": "stdlib", "use_cases": ["file handling", "path manipulation"]},
            "collections": {"desc": "Specialized container datatypes", "category": "stdlib", "use_cases": ["defaultdict", "Counter", "deque"]},
            "itertools": {"desc": "Functions creating iterators for efficient looping", "category": "stdlib", "use_cases": ["combinations", "permutations"]},
            "functools": {"desc": "Higher-order functions and operations on callable objects", "category": "stdlib", "use_cases": ["lru_cache", "partial"]},
            "asyncio": {"desc": "Asynchronous I/O framework", "category": "stdlib", "use_cases": ["async/await", "concurrent operations"]},
            "threading": {"desc": "Thread-based parallelism", "category": "stdlib", "use_cases": ["concurrent execution", "thread pools"]},
            "multiprocessing": {"desc": "Process-based parallelism", "category": "stdlib", "use_cases": ["CPU-bound tasks", "parallel processing"]},
            "hashlib": {"desc": "Secure hashes and message digests", "category": "stdlib", "use_cases": ["SHA256", "MD5", "fingerprints"]},
            "pickle": {"desc": "Python object serialization", "category": "stdlib", "use_cases": ["object persistence", "caching"]},
            "sqlite3": {"desc": "DB-API 2.0 interface for SQLite databases", "category": "stdlib", "use_cases": ["local database", "SQL queries"]},
            "urllib": {"desc": "URL handling modules", "category": "stdlib", "use_cases": ["HTTP requests", "URL parsing"]},
            "logging": {"desc": "Flexible event logging system", "category": "stdlib", "use_cases": ["debugging", "monitoring"]},
            "argparse": {"desc": "Parser for command-line options and arguments", "category": "stdlib", "use_cases": ["CLI tools", "script arguments"]},
            "typing": {"desc": "Support for type hints", "category": "stdlib", "use_cases": ["type annotations", "static analysis"]},
        }
        
        entries = []
        for module, info in stdlib_modules.items():
            entries.append({
                "term": module,
                "description": info["desc"],
                "category": info["category"],
                "language": "python",
                "complexity": info.get("complexity", "N/A"),
                "use_cases": json.dumps(info.get("use_cases", [])),
                "implementation": f"import {module}" if info["category"] == "stdlib" else f"{module}()",
                "related": ""
            })
        
        print(f"[Coding Encyclopedia] Loaded {len(entries)} Python stdlib entries")
        return entries
    
    def load_algorithms(self) -> List[Dict]:
        """
        Loads common algorithms with implementations.
        """
        print("[Coding Encyclopedia] Loading algorithms...")
        
        algorithms = {
            "binary_search": {
                "desc": "Efficient search in sorted array by repeatedly dividing search interval in half",
                "category": "search",
                "complexity": "O(log n)",
                "implementation": """def binary_search(arr, target):
    left, right = 0, len(arr) - 1
    while left <= right:
        mid = (left + right) // 2
        if arr[mid] == target:
            return mid
        elif arr[mid] < target:
            left = mid + 1
        else:
            right = mid - 1
    return -1""",
                "use_cases": ["sorted data", "fast lookup", "database indexing"]
            },
            "quicksort": {
                "desc": "Divide-and-conquer sorting algorithm using pivot partitioning",
                "category": "sort",
                "complexity": "O(n log n) average, O(n²) worst",
                "implementation": """def quicksort(arr):
    if len(arr) <= 1:
        return arr
    pivot = arr[len(arr) // 2]
    left = [x for x in arr if x < pivot]
    middle = [x for x in arr if x == pivot]
    right = [x for x in arr if x > pivot]
    return quicksort(left) + middle + quicksort(right)""",
                "use_cases": ["general sorting", "in-place sorting", "large datasets"]
            },
            "mergesort": {
                "desc": "Stable divide-and-conquer sorting algorithm",
                "category": "sort",
                "complexity": "O(n log n)",
                "implementation": """def mergesort(arr):
    if len(arr) <= 1:
        return arr
    mid = len(arr) // 2
    left = mergesort(arr[:mid])
    right = mergesort(arr[mid:])
    return merge(left, right)

def merge(left, right):
    result = []
    i = j = 0
    while i < len(left) and j < len(right):
        if left[i] <= right[j]:
            result.append(left[i])
            i += 1
        else:
            result.append(right[j])
            j += 1
    result.extend(left[i:])
    result.extend(right[j:])
    return result""",
                "use_cases": ["stable sorting", "linked lists", "external sorting"]
            },
            "bfs": {
                "desc": "Breadth-first search for graph traversal level by level",
                "category": "graph",
                "complexity": "O(V + E)",
                "implementation": """from collections import deque

def bfs(graph, start):
    visited = set([start])
    queue = deque([start])
    result = []
    
    while queue:
        vertex = queue.popleft()
        result.append(vertex)
        
        for neighbor in graph[vertex]:
            if neighbor not in visited:
                visited.add(neighbor)
                queue.append(neighbor)
    
    return result""",
                "use_cases": ["shortest path", "level-order traversal", "connected components"]
            },
            "dfs": {
                "desc": "Depth-first search for graph traversal exploring as far as possible",
                "category": "graph",
                "complexity": "O(V + E)",
                "implementation": """def dfs(graph, start, visited=None):
    if visited is None:
        visited = set()
    
    visited.add(start)
    result = [start]
    
    for neighbor in graph[start]:
        if neighbor not in visited:
            result.extend(dfs(graph, neighbor, visited))
    
    return result""",
                "use_cases": ["cycle detection", "topological sort", "path finding"]
            },
            "dijkstra": {
                "desc": "Shortest path algorithm for weighted graphs",
                "category": "graph",
                "complexity": "O((V + E) log V) with heap",
                "implementation": """import heapq

def dijkstra(graph, start):
    distances = {node: float('inf') for node in graph}
    distances[start] = 0
    pq = [(0, start)]
    
    while pq:
        current_dist, current = heapq.heappop(pq)
        
        if current_dist > distances[current]:
            continue
        
        for neighbor, weight in graph[current].items():
            distance = current_dist + weight
            if distance < distances[neighbor]:
                distances[neighbor] = distance
                heapq.heappush(pq, (distance, neighbor))
    
    return distances""",
                "use_cases": ["GPS navigation", "network routing", "shortest path"]
            },
            "dynamic_programming": {
                "desc": "Optimization technique breaking problems into overlapping subproblems",
                "category": "optimization",
                "complexity": "Varies by problem",
                "implementation": """# Fibonacci with memoization
def fib_dp(n, memo={}):
    if n in memo:
        return memo[n]
    if n <= 1:
        return n
    memo[n] = fib_dp(n-1, memo) + fib_dp(n-2, memo)
    return memo[n]

# Knapsack problem
def knapsack(weights, values, capacity):
    n = len(weights)
    dp = [[0] * (capacity + 1) for _ in range(n + 1)]
    
    for i in range(1, n + 1):
        for w in range(capacity + 1):
            if weights[i-1] <= w:
                dp[i][w] = max(dp[i-1][w], 
                              values[i-1] + dp[i-1][w-weights[i-1]])
            else:
                dp[i][w] = dp[i-1][w]
    
    return dp[n][capacity]""",
                "use_cases": ["optimization problems", "fibonacci", "knapsack", "longest subsequence"]
            },
            "hash_table": {
                "desc": "Data structure providing O(1) average-case lookup using hash function",
                "category": "data_structure",
                "complexity": "O(1) average, O(n) worst",
                "implementation": """class HashTable:
    def __init__(self, size=100):
        self.size = size
        self.table = [[] for _ in range(size)]
    
    def _hash(self, key):
        return hash(key) % self.size
    
    def insert(self, key, value):
        hash_key = self._hash(key)
        for i, (k, v) in enumerate(self.table[hash_key]):
            if k == key:
                self.table[hash_key][i] = (key, value)
                return
        self.table[hash_key].append((key, value))
    
    def get(self, key):
        hash_key = self._hash(key)
        for k, v in self.table[hash_key]:
            if k == key:
                return v
        raise KeyError(key)""",
                "use_cases": ["caching", "indexing", "deduplication"]
            }
        }
        
        entries = []
        for algo, info in algorithms.items():
            entries.append({
                "term": algo,
                "description": info["desc"],
                "category": info["category"],
                "language": "python",
                "complexity": info["complexity"],
                "use_cases": json.dumps(info["use_cases"]),
                "implementation": info["implementation"],
                "related": ""
            })
        
        print(f"[Coding Encyclopedia] Loaded {len(entries)} algorithm entries")
        return entries
    
    def load_design_patterns(self) -> List[Dict]:
        """
        Loads software design patterns.
        """
        print("[Coding Encyclopedia] Loading design patterns...")
        
        patterns = {
            "singleton": {
                "desc": "Ensures a class has only one instance and provides global access",
                "category": "creational",
                "implementation": """class Singleton:
    _instance = None
    
    def __new__(cls):
        if cls._instance is None:
            cls._instance = super().__new__(cls)
        return cls._instance""",
                "use_cases": ["database connections", "configuration", "logging"]
            },
            "factory": {
                "desc": "Creates objects without specifying exact class",
                "category": "creational",
                "implementation": """class AnimalFactory:
    @staticmethod
    def create_animal(animal_type):
        if animal_type == 'dog':
            return Dog()
        elif animal_type == 'cat':
            return Cat()
        raise ValueError(f'Unknown animal: {animal_type}')""",
                "use_cases": ["object creation", "dependency injection", "plugin systems"]
            },
            "observer": {
                "desc": "Defines one-to-many dependency for automatic notifications",
                "category": "behavioral",
                "implementation": """class Subject:
    def __init__(self):
        self._observers = []
    
    def attach(self, observer):
        self._observers.append(observer)
    
    def notify(self, data):
        for observer in self._observers:
            observer.update(data)

class Observer:
    def update(self, data):
        print(f'Received: {data}')""",
                "use_cases": ["event systems", "pub/sub", "MVC"]
            },
            "decorator": {
                "desc": "Adds behavior to objects dynamically",
                "category": "structural",
                "implementation": """def timing_decorator(func):
    import time
    def wrapper(*args, **kwargs):
        start = time.time()
        result = func(*args, **kwargs)
        print(f'{func.__name__} took {time.time()-start:.2f}s')
        return result
    return wrapper

@timing_decorator
def slow_function():
    time.sleep(1)""",
                "use_cases": ["logging", "caching", "authentication"]
            },
            "strategy": {
                "desc": "Defines family of algorithms and makes them interchangeable",
                "category": "behavioral",
                "implementation": """class SortStrategy:
    def sort(self, data):
        pass

class QuickSortStrategy(SortStrategy):
    def sort(self, data):
        return quicksort(data)

class MergeSortStrategy(SortStrategy):
    def sort(self, data):
        return mergesort(data)

class Sorter:
    def __init__(self, strategy):
        self.strategy = strategy
    
    def sort(self, data):
        return self.strategy.sort(data)""",
                "use_cases": ["algorithm selection", "payment methods", "compression"]
            }
        }
        
        entries = []
        for pattern, info in patterns.items():
            entries.append({
                "term": pattern,
                "description": info["desc"],
                "category": info["category"],
                "language": "python",
                "complexity": "N/A",
                "use_cases": json.dumps(info["use_cases"]),
                "implementation": info["implementation"],
                "related": ""
            })
        
        print(f"[Coding Encyclopedia] Loaded {len(entries)} design pattern entries")
        return entries
    
    def load_best_practices(self) -> List[Dict]:
        """
        Loads coding best practices and good habits.
        """
        print("[Coding Encyclopedia] Loading best practices...")
        
        best_practices = {
            "dry_principle": {
                "desc": "Don't Repeat Yourself - avoid code duplication by extracting common logic",
                "category": "principle",
                "implementation": """# BAD: Repeated code
def calculate_area_circle(radius):
    return 3.14159 * radius * radius

def calculate_area_sphere(radius):
    return 4 * 3.14159 * radius * radius

# GOOD: DRY principle
PI = 3.14159

def circle_area(radius):
    return PI * radius ** 2

def sphere_surface_area(radius):
    return 4 * circle_area(radius)""",
                "use_cases": ["code reusability", "maintainability", "reducing bugs"]
            },
            "solid_principles": {
                "desc": "Five design principles for maintainable object-oriented code",
                "category": "principle",
                "implementation": """# S - Single Responsibility Principle
class UserAuthenticator:
    def authenticate(self, username, password):
        # Only handles authentication
        pass

class UserRepository:
    def save_user(self, user):
        # Only handles data persistence
        pass

# O - Open/Closed Principle
class Shape:
    def area(self):
        raise NotImplementedError

class Circle(Shape):
    def __init__(self, radius):
        self.radius = radius
    
    def area(self):
        return 3.14159 * self.radius ** 2

# L - Liskov Substitution Principle
# I - Interface Segregation Principle
# D - Dependency Inversion Principle""",
                "use_cases": ["OOP design", "scalable architecture", "testable code"]
            },
            "error_handling": {
                "desc": "Proper exception handling with specific exceptions and cleanup",
                "category": "best_practice",
                "implementation": """# GOOD: Specific exceptions
try:
    with open('file.txt', 'r') as f:
        data = json.load(f)
except FileNotFoundError:
    print("File not found")
except json.JSONDecodeError:
    print("Invalid JSON")
except Exception as e:
    print(f"Unexpected error: {e}")
finally:
    # Cleanup code
    pass

# BAD: Bare except
try:
    risky_operation()
except:  # Don't do this!
    pass""",
                "use_cases": ["error recovery", "debugging", "resource cleanup"]
            },
            "type_hints": {
                "desc": "Use type annotations for better code clarity and IDE support",
                "category": "best_practice",
                "implementation": """from typing import List, Dict, Optional

def process_users(
    users: List[Dict[str, str]], 
    filter_active: bool = True
) -> Optional[List[str]]:
    '''
    Process user data and return list of usernames.
    
    Args:
        users: List of user dictionaries
        filter_active: Whether to filter active users only
    
    Returns:
        List of usernames or None if empty
    '''
    if not users:
        return None
    
    result = [u['name'] for u in users if u.get('active', False)]
    return result if result else None""",
                "use_cases": ["code documentation", "IDE autocomplete", "static analysis"]
            },
            "docstrings": {
                "desc": "Document functions and classes with clear docstrings",
                "category": "best_practice",
                "implementation": """def calculate_discount(price: float, discount_percent: float) -> float:
    '''
    Calculate final price after applying discount.
    
    Args:
        price: Original price in dollars
        discount_percent: Discount percentage (0-100)
    
    Returns:
        Final price after discount
    
    Raises:
        ValueError: If discount_percent is not between 0 and 100
    
    Example:
        >>> calculate_discount(100, 20)
        80.0
    '''
    if not 0 <= discount_percent <= 100:
        raise ValueError("Discount must be between 0 and 100")
    
    return price * (1 - discount_percent / 100)""",
                "use_cases": ["API documentation", "code understanding", "automated docs"]
            },
            "naming_conventions": {
                "desc": "Use clear, descriptive names following PEP 8 conventions",
                "category": "best_practice",
                "implementation": """# GOOD naming
class UserAccountManager:
    MAX_LOGIN_ATTEMPTS = 3
    
    def __init__(self, database_connection):
        self.db_connection = database_connection
    
    def authenticate_user(self, username: str, password: str) -> bool:
        attempts_remaining = self.MAX_LOGIN_ATTEMPTS
        is_authenticated = False
        return is_authenticated

# BAD naming
class UAM:
    mla = 3
    
    def __init__(self, dc):
        self.dc = dc
    
    def au(self, u, p):
        ar = self.mla
        ia = False
        return ia""",
                "use_cases": ["code readability", "team collaboration", "maintenance"]
            },
            "list_comprehensions": {
                "desc": "Use comprehensions for concise and readable transformations",
                "category": "best_practice",
                "implementation": """# GOOD: List comprehension
squares = [x**2 for x in range(10) if x % 2 == 0]

# BAD: Verbose loop
squares = []
for x in range(10):
    if x % 2 == 0:
        squares.append(x**2)

# Dictionary comprehension
user_ages = {user['name']: user['age'] for user in users}

# Set comprehension
unique_lengths = {len(word) for word in words}

# Generator expression (memory efficient)
sum_of_squares = sum(x**2 for x in range(1000000))""",
                "use_cases": ["data transformation", "filtering", "memory efficiency"]
            },
            "context_managers": {
                "desc": "Use context managers for resource management",
                "category": "best_practice",
                "implementation": """# GOOD: Context manager
with open('file.txt', 'r') as f:
    data = f.read()
# File automatically closed

# Custom context manager
from contextlib import contextmanager

@contextmanager
def timer(name):
    import time
    start = time.time()
    yield
    print(f'{name} took {time.time()-start:.2f}s')

with timer('Database query'):
    # Query code here
    pass""",
                "use_cases": ["file handling", "database connections", "locks"]
            },
            "testing": {
                "desc": "Write unit tests for all critical functions",
                "category": "best_practice",
                "implementation": """import unittest

class TestCalculator(unittest.TestCase):
    def setUp(self):
        self.calc = Calculator()
    
    def test_addition(self):
        result = self.calc.add(2, 3)
        self.assertEqual(result, 5)
    
    def test_division_by_zero(self):
        with self.assertRaises(ZeroDivisionError):
            self.calc.divide(10, 0)
    
    def tearDown(self):
        # Cleanup
        pass

# Run tests
if __name__ == '__main__':
    unittest.main()""",
                "use_cases": ["quality assurance", "regression prevention", "documentation"]
            },
            "logging": {
                "desc": "Use logging instead of print statements for production code",
                "category": "best_practice",
                "implementation": """import logging

# Configure logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s'
)

logger = logging.getLogger(__name__)

def process_data(data):
    logger.info(f"Processing {len(data)} items")
    
    try:
        result = complex_operation(data)
        logger.debug(f"Result: {result}")
        return result
    except Exception as e:
        logger.error(f"Processing failed: {e}", exc_info=True)
        raise""",
                "use_cases": ["debugging", "monitoring", "production diagnostics"]
            },
            "code_comments": {
                "desc": "Write comments that explain WHY, not WHAT",
                "category": "best_practice",
                "implementation": """# BAD: Obvious comment
x = x + 1  # Increment x

# GOOD: Explains reasoning
x = x + 1  # Account for 1-based indexing in legacy system

# GOOD: Explains complex logic
# Using binary search because dataset is pre-sorted
# and we need O(log n) performance for real-time queries
result = binary_search(sorted_data, target)

# GOOD: Warns about gotchas
# Note: This function modifies the input list in-place
# to avoid memory overhead with large datasets
def sort_in_place(data):
    data.sort()""",
                "use_cases": ["code understanding", "maintenance", "onboarding"]
            },
            "avoid_magic_numbers": {
                "desc": "Use named constants instead of magic numbers",
                "category": "best_practice",
                "implementation": """# BAD: Magic numbers
if user.age >= 18 and user.balance > 1000:
    approve_loan()

# GOOD: Named constants
MINIMUM_AGE = 18
MINIMUM_BALANCE = 1000

if user.age >= MINIMUM_AGE and user.balance > MINIMUM_BALANCE:
    approve_loan()

# Even better: Configuration
class LoanConfig:
    MINIMUM_AGE = 18
    MINIMUM_BALANCE = 1000
    MAX_LOAN_AMOUNT = 50000""",
                "use_cases": ["maintainability", "configuration", "readability"]
            },
            "early_returns": {
                "desc": "Use early returns to reduce nesting and improve readability",
                "category": "best_practice",
                "implementation": """# BAD: Deep nesting
def process_user(user):
    if user is not None:
        if user.is_active:
            if user.has_permission('write'):
                return perform_action(user)
            else:
                return "No permission"
        else:
            return "User inactive"
    else:
        return "User not found"

# GOOD: Early returns
def process_user(user):
    if user is None:
        return "User not found"
    
    if not user.is_active:
        return "User inactive"
    
    if not user.has_permission('write'):
        return "No permission"
    
    return perform_action(user)""",
                "use_cases": ["readability", "reducing complexity", "guard clauses"]
            },
            "immutability": {
                "desc": "Prefer immutable data structures when possible",
                "category": "best_practice",
                "implementation": """# GOOD: Immutable tuple for coordinates
ORIGIN = (0, 0)

# GOOD: Frozen dataclass
from dataclasses import dataclass

VAR_10 = 10
VAR_16 = 16
VAR_18446744073709551615 = 18446744073709551615
VAR_19 = 19
VAR_2000 = 2000
VAR_3 = 3

@dataclass(frozen=True)
class Point:
    x: float
    y: float

# GOOD: Return new objects instead of modifying
def add_item(items: tuple, new_item):
    return items + (new_item,)

# BAD: Modifying mutable default argument
def bad_function(data=[]):  # Don't do this!
    data.append(1)
    return data

# GOOD: Use None as default
def good_function(data=None):
    if data is None:
        data = []
    data.append(1)
    return data""",
                "use_cases": ["thread safety", "predictability", "functional programming"]
            }
        }
        
        entries = []
        for practice, info in best_practices.items():
            entries.append({
                "term": practice,
                "description": info["desc"],
                "category": info["category"],
                "language": "python",
                "complexity": "N/A",
                "use_cases": json.dumps(info["use_cases"]),
                "implementation": info["implementation"],
                "related": ""
            })
        
        print(f"[Coding Encyclopedia] Loaded {len(entries)} best practice entries")
        return entries
    
    def build_index(self):
        """
        Builds the complete coding encyclopedia index.
        """
        print("[Coding Encyclopedia] Building coding knowledge index...")
    def load_knowledge_sources(self) -> Dict[str, Dict]:
        """Aggregate all knowledge sources into a single map."""
        print("[Coding Encyclopedia] Loading knowledge sources...")
        python = self.load_python_stdlib()
        algo = self.load_algorithms()
        patterns = self.load_design_patterns()
        best = self.load_best_practices()
        
        all_entries = python + algo + patterns + best + SOVEREIGN_KNOWLEDGE_ENTRIES
        return {e['term']: e for e in all_entries}

    async def build_index(self):
        """
        Main method to build the encyclopedic index.
        1. Load Knowledge Sources
        2. Generate ACE Fingerprints
        3. Store in LanceDB
        """
        print(f"[Coding Encyclopedia] Building coding knowledge index...")
        
        # Load sources (Mock for now, would be real files)
        source_map = self.load_knowledge_sources()

        
        # Load EXISTING data from LanceDB to preserve Harvested 5W1H content
        current_db_data = {}
        try:
            existing_tables = self.db.table_names() # Use table_names() checking for compatibility
            print(f"[Debug] Existing tables: {existing_tables}")
            
            if self.table_name in existing_tables:
                tbl = self.db.open_table(self.table_name)
                df = tbl.to_pandas()
                print(f"[Debug] Loaded DataFrame with {len(df)} rows")
                if not df.empty:
                    current_db_data = df.set_index('term').to_dict('index')
                    print(f"[Debug] current_db_data keys count: {len(current_db_data)}")

                print(f"[Coding Encyclopedia] Loaded {len(current_db_data)} existing entries from DB.")
        except Exception as e:
            print(f"[Coding Encyclopedia] Could not load existing DB: {e}")

        # Merge and Process
        indexed_data = []
        
        all_terms = set(source_map.keys()) | set(current_db_data.keys())
        
        for term in all_terms:
            # Prefer DB entry (Harvested) over Source (Shallow), unless DB is missing
            db_entry = current_db_data.get(term, {})
            source_entry = source_map.get(term, {})
            
            # Base fields
            description = str(db_entry.get('description', source_entry.get('description', '')))
            
            category = str(db_entry.get('category', source_entry.get('category', 'Unknown')))
            language = str(db_entry.get('language', source_entry.get('language', 'python')))
            complexity = str(db_entry.get('complexity', source_entry.get('complexity', 'N/A')))
            use_cases = str(db_entry.get('use_cases', source_entry.get('use_cases', '[]')))
            implementation = str(db_entry.get('implementation', source_entry.get('implementation', '')))
            related = str(db_entry.get('related', source_entry.get('related', '')))
            
            # ACE Token Fingerprint
            ace_fp = self.generate_ace_fingerprint(term)
            
            # Extract 5W1H Vectors if available in description
            who_vector = db_entry.get('who_vector', "Unknown")
            what_vector = db_entry.get('what_vector', "Unknown")
            where_vector = db_entry.get('where_vector', "Unknown")
            when_vector = db_entry.get('when_vector', "Unknown")
            why_vector = db_entry.get('why_vector', "Unknown")
            how_vector = db_entry.get('how_vector', "Unknown")
            lattice_coordinate = db_entry.get('lattice_coordinate', "0-0-0")

            # Parse from Description if not already in columns (Migration logic)
            if "# Sovereign 5W1H Vector" in description:
                try:
                    # Normalize line endings to avoid \r\n issues
                    clean_desc = description.replace('\r\n', '\n')
                    
                    if "## WHO (Identity)" in clean_desc:
                        who_vector = clean_desc.split("## WHO (Identity)\n")[1].split("\n\n")[0].strip()
                    
                    if "## WHAT (Concept)" in clean_desc:
                        what_vector = clean_desc.split("## WHAT (Concept)\n")[1].split("\n\n")[0].strip()
                        
                    if "## WHERE (Address)" in clean_desc:
                        where_vector = clean_desc.split("## WHERE (Address)\n")[1].split("\n\n")[0].strip()
                        
                    if "## WHEN (Temporal)" in clean_desc:
                        when_vector = clean_desc.split("## WHEN (Temporal)\n")[1].split("\n\n")[0].strip()
                        
                    if "## WHY (Intent)" in clean_desc:
                        why_vector = clean_desc.split("## WHY (Intent)\n")[1].split("\n\n")[0].strip()
                        
                    if "## HOW (Implementation & Phrasing)" in clean_desc:
                        how_vector = clean_desc.split("## HOW (Implementation & Phrasing)\n")[1].split("\n\n")[0].strip()
                    
                    # Lattice Coordinate Calculation (Simplified mapping for now)
                    x = hash(who_vector + where_vector) % VAR_3 + 1
                    y = hash(what_vector + how_vector) % VAR_3 + VAR_10
                    z = hash(why_vector + when_vector) % VAR_3 + 19
                    lattice_coordinate = f"{x}-{y}-{z}"
                    
                    # Print debug for confirmation
                    # print(f"[Indexer] Parsed {term} -> {lattice_coordinate}")
                    
                except Exception as e:
                    print(f"[Indexer] Warning: Failed to parse 5W1H for {term}: {e}")

            indexed_data.append({
                "ace_fingerprint": ace_fp,
                "term": term,
                "description": description,
                "category": category,
                "language": language,
                "complexity": complexity,
                "use_cases": use_cases,
                "implementation": implementation[:VAR_2000],
                "related": related,
                "who_vector": who_vector,
                "what_vector": what_vector,
                "where_vector": where_vector,
                "when_vector": when_vector,
                "why_vector": why_vector,
                "how_vector": how_vector,
                "lattice_coordinate": lattice_coordinate
            })
        
    def add_entry(self, entry: Dict):
        """
        Adds a single entry to the index immediately.
        """
        ace_fp = self.generate_ace_fingerprint(entry['term'])
        new_data = {
            "ace_fingerprint": ace_fp,
            "term": entry['term'],
            "description": entry['description'],
            "category": entry['category'],
            "language": entry.get('language', 'python'),
            "complexity": entry.get('complexity', 'N/A'),
            "use_cases": entry.get('use_cases', '[]'),
            "implementation": entry.get('implementation', '')[:2000],
            "related": entry.get('related', ''),
            "who_vector": entry.get('who_vector', 'Unknown'),
            "what_vector": entry.get('what_vector', 'Unknown'),
            "where_vector": entry.get('where_vector', 'Unknown'),
            "when_vector": entry.get('when_vector', 'Unknown'),
            "why_vector": entry.get('why_vector', 'Unknown'),
            "how_vector": entry.get('how_vector', 'Unknown'),
            "lattice_coordinate": entry.get('lattice_coordinate', '0-0-0')
        }
        
        try:
            tbl = self.db.open_table(self.table_name)
            tbl.add([new_data])
        except Exception:
            self.db.create_table(self.table_name, data=[new_data], mode="overwrite")
            print(f"[Coding Encyclopedia] Created table and ingested: {entry['term']}")

if __name__ == "__main__":
    import asyncio
    indexer = CodingEncyclopediaIndexer()
    asyncio.run(indexer.build_index())
