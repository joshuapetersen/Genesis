"""
PULSE SYSTEM
============
Batches data transmission into hourly pulses to prevent rate limit violations.
Accumulates events and transmits efficiently in controlled bursts.

Author: Sarah (Sovereign AI)
Genesis: 133 Pattern (1 Sovereign > 3 Variables)
"""

import json
import time
from datetime import datetime, timedelta
from pathlib import Path
from collections import defaultdict
import threading

# Core Paths
CORE_DIR = Path(__file__).parent
MEMORY_DIR = CORE_DIR.parent / "04_THE_MEMORY"
PULSE_QUEUE = CORE_DIR / "pulse_queue.json"
PULSE_LOG = CORE_DIR / "pulse_history.json"

class PulseSystem:
    """Batched data transmission system"""
    
    def __init__(self, pulse_interval_seconds=3600):  # Default: 1 hour
        self.pulse_interval = pulse_interval_seconds
        self.queue = defaultdict(list)  # destination -> list of events
        self.last_pulse = {}  # destination -> timestamp
        self.pulse_history = []
        self._load_queue()
        self._load_history()
        self.lock = threading.Lock()
    
    def _load_queue(self):
        """Load pending events from disk"""
        try:
            if PULSE_QUEUE.exists():
                with open(PULSE_QUEUE, 'r') as f:
                    data = json.load(f)
                    self.queue = defaultdict(list, data.get('queue', {}))
                    self.last_pulse = data.get('last_pulse', {})
        except Exception as e:
            print(f"[Pulse] Failed to load queue: {e}")
    
    def _save_queue(self):
        """Save pending events to disk"""
        try:
            data = {
                'queue': dict(self.queue),
                'last_pulse': self.last_pulse,
                'updated': datetime.now().isoformat()
            }
            with open(PULSE_QUEUE, 'w') as f:
                json.dump(data, f, indent=2)
        except Exception as e:
            print(f"[Pulse] Failed to save queue: {e}")
    
    def _load_history(self):
        """Load pulse history from disk"""
        try:
            if PULSE_LOG.exists():
                with open(PULSE_LOG, 'r') as f:
                    data = json.load(f)
                    self.pulse_history = data.get('pulses', [])
        except Exception as e:
            print(f"[Pulse] Failed to load history: {e}")
    
    def _save_history(self):
        """Save pulse history to disk"""
        try:
            data = {
                'pulses': self.pulse_history[-1000:],  # Keep last 1000
                'updated': datetime.now().isoformat()
            }
            with open(PULSE_LOG, 'w') as f:
                json.dump(data, f, indent=2)
        except Exception as e:
            print(f"[Pulse] Failed to save history: {e}")
    
    def queue_event(self, destination, event_type, event_data):
        """
        Add event to pulse queue
        WHO: Pulse System
        WHAT: Queue event for batched transmission
        WHERE: Pulse queue in memory/disk
        WHEN: When event occurs
        WHY: Batch events to avoid rate limits
        HOW: Accumulate in queue, transmit on schedule
        
        Args:
            destination: Where to send (e.g., 'firebase', 'gemini', 'local')
            event_type: Type of event (e.g., 'log', 'metric', 'evolution')
            event_data: Event payload (dict)
        """
        with self.lock:
            event = {
                'timestamp': datetime.now().isoformat(),
                'type': event_type,
                'data': event_data
            }
            
            self.queue[destination].append(event)
            self._save_queue()
            
            # Check if pulse is due
            self._check_pulse_due(destination)
    
    def _check_pulse_due(self, destination):
        """Check if pulse should be sent for destination"""
        if destination not in self.last_pulse:
            self.last_pulse[destination] = datetime.now().isoformat()
            return
        
        last = datetime.fromisoformat(self.last_pulse[destination])
        now = datetime.now()
        
        if (now - last).total_seconds() >= self.pulse_interval:
            self._send_pulse(destination)
    
    def _send_pulse(self, destination):
        """
        Send batched pulse to destination
        Returns: (success: bool, event_count: int)
        """
        with self.lock:
            if destination not in self.queue or not self.queue[destination]:
                return True, 0
            
            events = self.queue[destination]
            event_count = len(events)
            
            print(f"\n[Pulse] Sending pulse to {destination}: {event_count} events")
            
            try:
                # Route to appropriate handler
                if destination == 'firebase':
                    success = self._send_to_firebase(events)
                elif destination == 'local':
                    success = self._send_to_local(events)
                else:
                    print(f"[Pulse] Unknown destination: {destination}")
                    success = False
                
                if success:
                    # Record pulse
                    pulse_record = {
                        'timestamp': datetime.now().isoformat(),
                        'destination': destination,
                        'event_count': event_count,
                        'success': True
                    }
                    self.pulse_history.append(pulse_record)
                    self._save_history()
                    
                    # Clear queue
                    self.queue[destination] = []
                    self.last_pulse[destination] = datetime.now().isoformat()
                    self._save_queue()
                    
                    print(f"[Pulse] ✓ Pulse sent successfully")
                    return True, event_count
                else:
                    print(f"[Pulse] ✗ Pulse failed, will retry")
                    return False, event_count
                    
            except Exception as e:
                print(f"[Pulse] Error sending pulse: {e}")
                return False, event_count
    
    def _send_to_firebase(self, events):
        """Send events to Firebase (batched)"""
        try:
            from Neural_Memory_Core import NeuralMemorySystem
            
            nms = NeuralMemorySystem()
            
            # Batch write to Firebase
            for event in events:
                # Store in appropriate collection based on type
                collection = f"pulses/{event['type']}"
                nms.store_memory(collection, event['data'])
            
            return True
        except Exception as e:
            print(f"[Pulse] Firebase error: {e}")
            return False
    
    def _send_to_local(self, events):
        """Send events to local log files (batched)"""
        try:
            # Group by event type
            by_type = defaultdict(list)
            for event in events:
                by_type[event['type']].append(event)
            
            # Write each type to its log
            for event_type, type_events in by_type.items():
                log_file = CORE_DIR / f"{event_type}_pulse_log.json"
                
                # Load existing
                if log_file.exists():
                    with open(log_file, 'r') as f:
                        existing = json.load(f)
                else:
                    existing = {'events': []}
                
                # Append
                existing['events'].extend(type_events)
                
                # Save
                with open(log_file, 'w') as f:
                    json.dump(existing, f, indent=2)
            
            return True
        except Exception as e:
            print(f"[Pulse] Local write error: {e}")
            return False
    
    def force_pulse(self, destination=None):
        """
        Force immediate pulse transmission
        Args:
            destination: Specific destination or None for all
        """
        with self.lock:
            if destination:
                self._send_pulse(destination)
            else:
                for dest in list(self.queue.keys()):
                    self._send_pulse(dest)
    
    def get_queue_status(self):
        """Get current queue statistics"""
        with self.lock:
            status = {
                'destinations': {},
                'total_queued': 0,
                'next_pulse': {}
            }
            
            for dest, events in self.queue.items():
                status['destinations'][dest] = len(events)
                status['total_queued'] += len(events)
                
                if dest in self.last_pulse:
                    last = datetime.fromisoformat(self.last_pulse[dest])
                    next_pulse = last + timedelta(seconds=self.pulse_interval)
                    remaining = (next_pulse - datetime.now()).total_seconds()
                    status['next_pulse'][dest] = max(0, int(remaining))
            
            return status
    
    def print_status(self):
        """Print queue status"""
        status = self.get_queue_status()
        
        print("\n" + "="*60)
        print("PULSE SYSTEM STATUS")
        print("="*60)
        print(f"Total Queued Events: {status['total_queued']}")
        print(f"Pulse Interval: {self.pulse_interval}s ({self.pulse_interval/3600:.1f}h)")
        print("\nQueues:")
        for dest, count in status['destinations'].items():
            next_in = status['next_pulse'].get(dest, 0)
            print(f"  {dest}: {count} events (next pulse in {next_in}s)")
        print("="*60)


# Global instance
_pulse_system = None

def get_pulse_system():
    """Get global pulse system instance"""
    global _pulse_system
    if _pulse_system is None:
        _pulse_system = PulseSystem()
    return _pulse_system


def main():
    """CLI interface"""
    import argparse
    
    parser = argparse.ArgumentParser(description='Sarah Pulse System')
    parser.add_argument('--status', action='store_true', help='Show queue status')
    parser.add_argument('--force', action='store_true', help='Force pulse transmission')
    parser.add_argument('--destination', help='Specific destination for force pulse')
    
    args = parser.parse_args()
    
    pulse = get_pulse_system()
    
    if args.status:
        pulse.print_status()
    
    if args.force:
        pulse.force_pulse(args.destination)
        print("\n[Pulse] Force pulse completed")
    
    if not (args.status or args.force):
        pulse.print_status()


if __name__ == "__main__":
    main()
