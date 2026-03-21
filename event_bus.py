import asyncio
from typing import Callable, Dict, Any, List
from dataclasses import dataclass
import json
from pathlib import Path
import time

@dataclass
class Event:
    """Event data structure."""
    type: str
    data: Dict[str, Any]
    timestamp: float
    source: str

class EventBus:
    """Centralized event bus for decoupled communication."""
    
    def __init__(self):
        self.subscribers: Dict[str, List[Callable]] = {}
        self.event_log: List[Event] = []
        self.lock = asyncio.Lock()
    
    async def publish(self, event: Event) -> None:
        """Publish an event to all subscribers."""
        async with self.lock:
            self.event_log.append(event)
            await self._notify_subscribers(event)
    
    async def subscribe(self, event_type: str, callback: Callable) -> None:
        """Subscribe to specific event types."""
        if event_type not in self.subscribers:
            self.subscribers[event_type] = []
        self.subscribers[event_type].append(callback)
    
    async def _notify_subscribers(self, event: Event) -> None:
        """Notify all subscribers of an event."""
        if event.type in self.subscribers:
            for callback in self.subscribers[event.type]:
                try:
                    await callback(event)
                except Exception as e:
                    print(f"Error in event handler: {e}")
    
    async def get_event_log(self, since: float = 0) -> List[Event]:
        """Get event log since a specific timestamp."""
        async with self.lock:
            return [e for e in self.event_log if e.timestamp >= since]