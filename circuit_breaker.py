import asyncio
from typing import Callable, Any
from dataclasses import dataclass
import time

@dataclass
class CircuitState:
    """Circuit breaker state."""
    status: str  # 'CLOSED', 'OPEN', 'HALF_OPEN'
    failure_count: int
    last_failure_time: float
    retry_after: float

class CircuitBreaker:
    """Circuit breaker pattern for fault tolerance."""
    
    def __init__(self, failure_threshold: int = 5, recovery_timeout: float = 60.0):
        self.failure_threshold = failure_threshold
        self.recovery_timeout = recovery_timeout
        self.state: CircuitState = CircuitState(
            status='CLOSED',
            failure_count=0,
            last_failure_time=0,
            retry_after=0
        )
    
    async def call(self, func: Callable, *args, **kwargs) -> Any:
        """Call a function with circuit breaker protection."""
        if self.state.status == 'OPEN':
            if time.time() > self.state.retry_after:
                self.state.status = 'HALF_OPEN'
            else:
                raise Exception("Circuit breaker is OPEN")
        
        try:
            result = await func(*args, **kwargs)
            if self.state.status == 'HALF_OPEN':
                self._reset()
            return result
        except Exception as e:
            self._record_failure()
            raise
    
    def _record_failure(self) -> None:
        """Record a failure and update state."""
        self.state.failure_count += 1
        self.state.last_failure_time = time.time()
        
        if self.state.failure_count >= self.failure_threshold:
            self.state.status = 'OPEN'
            self.state.retry_after = time.time() + self.recovery_timeout
    
    def _reset(self) -> None:
        """Reset the circuit breaker."""
        self.state = CircuitState(
            status='CLOSED',
            failure_count=0,
            last_failure_time=0,
            retry_after=0
        )