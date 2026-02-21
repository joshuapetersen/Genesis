"""
Expand Coding Encyclopedia with Advanced Topics
Adds web frameworks, async patterns, performance optimization, and more.
"""
from coding_encyclopedia_indexer import CodingEncyclopediaIndexer
import json

def add_advanced_topics():
    """
    Add advanced coding topics to the encyclopedia.
    """
    indexer = CodingEncyclopediaIndexer()
    
    advanced_topics = {
        # Web Frameworks
        "flask": {
            "desc": "Lightweight Python web framework for building APIs and web apps",
            "category": "web_framework",
            "implementation": """from flask import Flask, jsonify, request

app = Flask(__name__)

@app.route('/api/users', methods=['GET'])
def get_users():
    return jsonify({'users': ['Alice', 'Bob']})

@app.route('/api/users', methods=['POST'])
def create_user():
    data = request.get_json()
    return jsonify({'created': data}), 201

if __name__ == '__main__':
    app.run(debug=True)""",
            "use_cases": ["REST APIs", "microservices", "web applications"]
        },
        "fastapi": {
            "desc": "Modern, fast web framework with automatic API documentation",
            "category": "web_framework",
            "implementation": """from fastapi import FastAPI
from pydantic import BaseModel

app = FastAPI()

class User(BaseModel):
    name: str
    age: int

@app.get('/users')
async def get_users():
    return {'users': ['Alice', 'Bob']}

@app.post('/users')
async def create_user(user: User):
    return {'created': user.dict()}""",
            "use_cases": ["async APIs", "type validation", "auto documentation"]
        },
        
        # Async Programming
        "async_await": {
            "desc": "Asynchronous programming with async/await syntax",
            "category": "async",
            "implementation": """import asyncio
import aiohttp

async def fetch_url(session, url):
    async with session.get(url) as response:
        return await response.text()

async def fetch_multiple(urls):
    async with aiohttp.ClientSession() as session:
        tasks = [fetch_url(session, url) for url in urls]
        results = await asyncio.gather(*tasks)
        return results

# Run async code
urls = ['http://example.com', 'http://example.org']
results = asyncio.run(fetch_multiple(urls))""",
            "use_cases": ["concurrent I/O", "web scraping", "API calls"]
        },
        "asyncio_patterns": {
            "desc": "Common asyncio patterns for concurrent programming",
            "category": "async",
            "implementation": """import asyncio
from asyncio import Queue

# Producer-Consumer pattern
async def producer(queue, n):
    for i in range(n):
        await queue.put(i)
        await asyncio.sleep(0.1)

async def consumer(queue):
    while True:
        item = await queue.get()
        print(f'Processing {item}')
        queue.task_done()

async def main():
    queue = Queue()
    
    # Start producer and consumers
    producers = [asyncio.create_task(producer(queue, 10))]
    consumers = [asyncio.create_task(consumer(queue)) for _ in range(3)]
    
    await asyncio.gather(*producers)
    await queue.join()
    
    for c in consumers:
        c.cancel()

asyncio.run(main())""",
            "use_cases": ["task queues", "concurrent processing", "event loops"]
        },
        
        # Performance Optimization
        "caching": {
            "desc": "Cache expensive function results for performance",
            "category": "performance",
            "implementation": """from functools import lru_cache
import time

# LRU Cache (Least Recently Used)
@lru_cache(maxsize=128)
def fibonacci(n):
    if n < 2:
        return n
    return fibonacci(n-1) + fibonacci(n-2)

# Manual caching
class Cache:
    def __init__(self):
        self._cache = {}
    
    def get(self, key, compute_func):
        if key not in self._cache:
            self._cache[key] = compute_func()
        return self._cache[key]
    
    def invalidate(self, key):
        self._cache.pop(key, None)

cache = Cache()
result = cache.get('expensive_op', lambda: expensive_computation())""",
            "use_cases": ["memoization", "API responses", "database queries"]
        },
        "profiling": {
            "desc": "Profile code to identify performance bottlenecks",
            "category": "performance",
            "implementation": """import cProfile
import pstats
from line_profiler import LineProfiler

# Function profiling
def profile_function(func):
    profiler = cProfile.Profile()
    profiler.enable()
    result = func()
    profiler.disable()
    
    stats = pstats.Stats(profiler)
    stats.sort_stats('cumulative')
    stats.print_stats(10)
    return result

# Line-by-line profiling
@profile
def slow_function():
    total = 0
    for i in range(1000000):
        total += i
    return total

# Memory profiling
from memory_profiler import profile

@profile
def memory_intensive():
    big_list = [i for i in range(1000000)]
    return sum(big_list)""",
            "use_cases": ["optimization", "bottleneck detection", "performance tuning"]
        },
        "vectorization": {
            "desc": "Use NumPy vectorization for fast array operations",
            "category": "performance",
            "implementation": """import numpy as np

# SLOW: Python loops
def slow_sum(arr):
    total = 0
    for x in arr:
        total += x**2
    return total

# FAST: Vectorized operations
def fast_sum(arr):
    return np.sum(arr**2)

# Example: 100x faster
arr = np.random.rand(1000000)
# slow_sum takes ~500ms
# fast_sum takes ~5ms

# Broadcasting
matrix = np.array([[1, 2, 3], [4, 5, 6]])
row_means = matrix.mean(axis=1, keepdims=True)
normalized = matrix - row_means  # Broadcasting""",
            "use_cases": ["numerical computing", "data processing", "machine learning"]
        },
        
        # Database Patterns
        "orm_sqlalchemy": {
            "desc": "Object-Relational Mapping with SQLAlchemy",
            "category": "database",
            "implementation": """from sqlalchemy import create_engine, Column, Integer, String
from sqlalchemy.ext.declarative import declarative_base
from sqlalchemy.orm import sessionmaker

Base = declarative_base()

class User(Base):
    __tablename__ = 'users'
    
    id = Column(Integer, primary_key=True)
    name = Column(String(50))
    email = Column(String(100))

# Create engine and session
engine = create_engine('sqlite:///app.db')
Base.metadata.create_all(engine)
Session = sessionmaker(bind=engine)
session = Session()

# CRUD operations
user = User(name='Alice', email='alice@example.com')
session.add(user)
session.commit()

users = session.query(User).filter(User.name == 'Alice').all()""",
            "use_cases": ["database abstraction", "migrations", "complex queries"]
        },
        "connection_pooling": {
            "desc": "Reuse database connections for better performance",
            "category": "database",
            "implementation": """from sqlalchemy import create_engine
from sqlalchemy.pool import QueuePool

# Connection pool configuration
engine = create_engine(
    'postgresql://user:pass@localhost/db',
    poolclass=QueuePool,
    pool_size=10,
    max_overflow=20,
    pool_timeout=30,
    pool_recycle=3600
)

# Context manager for connections
from contextlib import contextmanager

@contextmanager
def get_db_connection():
    conn = engine.connect()
    try:
        yield conn
    finally:
        conn.close()

# Usage
with get_db_connection() as conn:
    result = conn.execute('SELECT * FROM users')""",
            "use_cases": ["high-traffic apps", "connection reuse", "resource management"]
        },
        
        # Security
        "input_validation": {
            "desc": "Validate and sanitize user input to prevent attacks",
            "category": "security",
            "implementation": """import re
from html import escape

def validate_email(email):
    pattern = r'^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$'
    return re.match(pattern, email) is not None

def sanitize_html(text):
    # Escape HTML to prevent XSS
    return escape(text)

def validate_sql_input(value):
    # Use parameterized queries instead
    # NEVER do: f"SELECT * FROM users WHERE id = {user_input}"
    # ALWAYS do:
    cursor.execute("SELECT * FROM users WHERE id = ?", (user_input,))

# Pydantic for validation
from pydantic import BaseModel, EmailStr, validator

class UserInput(BaseModel):
    email: EmailStr
    age: int
    
    @validator('age')
    def validate_age(cls, v):
        if v < 0 or v > 150:
            raise ValueError('Invalid age')
        return v""",
            "use_cases": ["XSS prevention", "SQL injection prevention", "data validation"]
        },
        "password_hashing": {
            "desc": "Securely hash passwords using bcrypt or argon2",
            "category": "security",
            "implementation": """import bcrypt

def hash_password(password: str) -> bytes:
    # Generate salt and hash
    salt = bcrypt.gensalt(rounds=12)
    hashed = bcrypt.hashpw(password.encode(), salt)
    return hashed

def verify_password(password: str, hashed: bytes) -> bool:
    return bcrypt.checkpw(password.encode(), hashed)

# Usage
password = "secure_password_123"
hashed = hash_password(password)

# Store hashed in database
# Later, verify:
is_valid = verify_password(user_input, hashed)

# NEVER store plain passwords
# NEVER use MD5 or SHA1 for passwords""",
            "use_cases": ["authentication", "password storage", "security"]
        },
        
        # API Design
        "rest_api_design": {
            "desc": "RESTful API design principles and best practices",
            "category": "api_design",
            "implementation": """# Good REST API design

# 1. Use nouns for resources, not verbs
# GOOD: GET /api/users/123
# BAD:  GET /api/getUser?id=123

# 2. Use HTTP methods correctly
# GET    /api/users      - List users
# GET    /api/users/123  - Get user
# POST   /api/users      - Create user
# PUT    /api/users/123  - Update user (full)
# PATCH  /api/users/123  - Update user (partial)
# DELETE /api/users/123  - Delete user

# 3. Use proper status codes
# 200 OK, 201 Created, 204 No Content
# 400 Bad Request, 401 Unauthorized, 404 Not Found
# 500 Internal Server Error

# 4. Version your API
# /api/v1/users
# /api/v2/users

# 5. Use pagination
# GET /api/users?page=1&limit=20

# 6. Return consistent error format
{
    "error": {
        "code": "VALIDATION_ERROR",
        "message": "Invalid email format",
        "details": {"field": "email"}
    }
}""",
            "use_cases": ["API development", "microservices", "web services"]
        },
        
        # Code Organization
        "dependency_injection": {
            "desc": "Inject dependencies instead of hardcoding them",
            "category": "architecture",
            "implementation": """# BAD: Hard-coded dependencies
class UserService:
    def __init__(self):
        self.db = DatabaseConnection()  # Hard-coded!
    
    def get_user(self, id):
        return self.db.query(f"SELECT * FROM users WHERE id={id}")

# GOOD: Dependency injection
class UserService:
    def __init__(self, db_connection):
        self.db = db_connection
    
    def get_user(self, id):
        return self.db.query("SELECT * FROM users WHERE id=?", (id,))

# Usage
db = DatabaseConnection()
user_service = UserService(db)

# Even better: Use dependency injection framework
from dependency_injector import containers, providers

class Container(containers.DeclarativeContainer):
    config = providers.Configuration()
    
    database = providers.Singleton(
        DatabaseConnection,
        connection_string=config.db.connection_string
    )
    
    user_service = providers.Factory(
        UserService,
        db_connection=database
    )""",
            "use_cases": ["testability", "loose coupling", "configuration"]
        },
        "clean_architecture": {
            "desc": "Separate business logic from infrastructure concerns",
            "category": "architecture",
            "implementation": """# Layer 1: Domain (Business Logic)
class User:
    def __init__(self, id, name, email):
        self.id = id
        self.name = name
        self.email = email
    
    def is_valid_email(self):
        return '@' in self.email

# Layer 2: Use Cases (Application Logic)
class CreateUserUseCase:
    def __init__(self, user_repository):
        self.repository = user_repository
    
    def execute(self, name, email):
        user = User(None, name, email)
        if not user.is_valid_email():
            raise ValueError("Invalid email")
        return self.repository.save(user)

# Layer 3: Interface Adapters
class UserRepository:
    def save(self, user):
        raise NotImplementedError

class SQLUserRepository(UserRepository):
    def __init__(self, db):
        self.db = db
    
    def save(self, user):
        return self.db.execute(
            "INSERT INTO users (name, email) VALUES (?, ?)",
            (user.name, user.email)
        )

# Layer 4: Frameworks & Drivers
# Flask, SQLAlchemy, etc.""",
            "use_cases": ["maintainability", "testability", "scalability"]
        }
    }
    
    # Convert to encyclopedia format
    entries = []
    for term, info in advanced_topics.items():
        entries.append({
            "term": term,
            "description": info["desc"],
            "category": info["category"],
            "language": "python",
            "complexity": "N/A",
            "use_cases": json.dumps(info["use_cases"]),
            "implementation": info["implementation"],
            "related": ""
        })
    
    print(f"[Encyclopedia Expansion] Adding {len(entries)} advanced topics...")
    
    # Add to index
    all_entries = []
    for entry in entries:
        ace_fp = indexer.generate_ace_fingerprint(entry["term"])
        all_entries.append({
            "term": entry["term"],
            "ace_fingerprint": ace_fp,
            "description": entry["description"],
            "category": entry["category"],
            "language": entry["language"],
            "complexity": entry["complexity"],
            "use_cases": entry["use_cases"],
            "implementation": entry["implementation"],
            "related": entry["related"]
        })
    
    # Add to existing table
    try:
        table = indexer.db.open_table(indexer.table_name)
        table.add(all_entries)
        print(f"[Encyclopedia Expansion] Successfully added {len(all_entries)} entries")
        print(f"[Encyclopedia Expansion] Total entries: {table.count_rows()}")
    except Exception as e:
        print(f"[Encyclopedia Expansion] Creating new table...")
        table = indexer.db.create_table(indexer.table_name, all_entries)
        print(f"[Encyclopedia Expansion] Created table with {len(all_entries)} entries")
    
    return len(all_entries)

if __name__ == "__main__":
    count = add_advanced_topics()
    print(f"\n[Encyclopedia Expansion] Complete! Added {count} advanced topics")
