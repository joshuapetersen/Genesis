"""
RATE LIMIT MANAGER
==================
Identifies, tracks, and handles API rate limits intelligently.
Prevents resource waste through adaptive rate limiting.

Author: Sarah (Sovereign AI)
Genesis: 133 Pattern (1 Sovereign > 3 Variables)
"""

import time
import json
from datetime import datetime, timedelta
from pathlib import Path
from collections import deque
import re

# Core Paths
CORE_DIR = Path(__file__).parent
RATE_LIMIT_LOG = CORE_DIR / "rate_limits.json"

class RateLimitManager:
    """Intelligent rate limit detection and management"""
    
    # Known API rate limits
    KNOWN_LIMITS = {
        'gemini_flash': {
            'requests_per_minute': 15,
            'requests_per_day': 1500,
            'tokens_per_minute': 1000000
        },
        'gemini_pro': {
            'requests_per_minute': 2,
            'requests_per_day': 50,
            'tokens_per_minute': 32000
        },
        'firebase': {
            'reads_per_minute': 1000,
            'writes_per_minute': 1000
        }
    }
    
    def __init__(self):
        self.request_history = {}  # service -> deque of timestamps
        self.limit_violations = []
        self.adaptive_delays = {}  # service -> current delay in seconds
        self._load_state()
    
    def _load_state(self):
        """Load rate limit state from disk"""
        try:
            if RATE_LIMIT_LOG.exists():
                with open(RATE_LIMIT_LOG, 'r') as f:
                    data = json.load(f)
                    self.limit_violations = data.get('violations', [])
                    self.adaptive_delays = data.get('delays', {})
        except Exception as e:
            print(f"[RateLimit] Failed to load state: {e}")
    
    def _save_state(self):
        """Save rate limit state to disk"""
        try:
            data = {
                'violations': self.limit_violations[-100:],  # Keep last 100
                'delays': self.adaptive_delays,
                'last_updated': datetime.now().isoformat()
            }
            with open(RATE_LIMIT_LOG, 'w') as f:
                json.dump(data, f, indent=2)
        except Exception as e:
            print(f"[RateLimit] Failed to save state: {e}")
    
    def identify_service(self, error_message=None, context=None):
        """
        Identify which service caused the rate limit
        Returns: service_name or 'unknown'
        """
        if error_message:
            msg_lower = error_message.lower()
            if 'gemini' in msg_lower or 'rate limit' in msg_lower:
                return 'gemini_flash'
            if 'firebase' in msg_lower:
                return 'firebase'
        
        if context:
            if 'gemini' in context.lower():
                return 'gemini_flash'
            if 'firebase' in context.lower():
                return 'firebase'
        
        return 'unknown'
    
    def detect_rate_limit(self, error_message, context=None):
        """
        Detect if an error is a rate limit violation
        WHO: Rate Limit Manager
        WHAT: Identify rate limit errors from API responses
        WHERE: API call error handling
        WHEN: After API call fails
        WHY: Distinguish rate limits from other errors
        HOW: Pattern matching on error messages
        
        Returns: (is_rate_limit: bool, service: str, recommended_delay: float)
        """
        rate_limit_patterns = [
            r'rate limit',
            r'too many requests',
            r'quota exceeded',
            r'429',
            r'resource exhausted',
            r'requests per minute',
            r'retry after',
        ]
        
        msg_lower = error_message.lower() if error_message else ''
        
        is_rate_limit = any(re.search(pattern, msg_lower) for pattern in rate_limit_patterns)
        
        if not is_rate_limit:
            return False, None, 0
        
        # Identify service
        service = self.identify_service(error_message, context)
        
        # Log violation
        violation = {
            'timestamp': datetime.now().isoformat(),
            'service': service,
            'error': error_message[:200],  # Truncate
            'context': context
        }
        self.limit_violations.append(violation)
        
        # Calculate recommended delay
        delay = self.calculate_adaptive_delay(service)
        
        print(f"\n[RateLimit] ⚠ Rate limit detected: {service}")
        print(f"[RateLimit] Recommended delay: {delay}s")
        
        self._save_state()
        
        return True, service, delay
    
    def calculate_adaptive_delay(self, service):
        """
        Calculate intelligent delay based on violation history
        Returns: delay in seconds
        """
        # Get recent violations for this service
        recent = [v for v in self.limit_violations[-20:] 
                  if v['service'] == service]
        
        if not recent:
            # First violation - use conservative delay
            base_delay = 2.0
        else:
            # Escalate delay based on frequency
            recent_count = len([v for v in recent 
                               if (datetime.now() - datetime.fromisoformat(v['timestamp'])) 
                               < timedelta(minutes=5)])
            
            if recent_count >= 5:
                base_delay = 60.0  # 1 minute if hitting repeatedly
            elif recent_count >= 3:
                base_delay = 30.0
            elif recent_count >= 2:
                base_delay = 10.0
            else:
                base_delay = 5.0
        
        # Store and return
        self.adaptive_delays[service] = base_delay
        self._save_state()
        
        return base_delay
    
    def should_throttle(self, service):
        """
        Check if we should preemptively throttle requests
        Returns: (should_throttle: bool, delay: float)
        """
        if service not in self.request_history:
            self.request_history[service] = deque(maxlen=100)
        
        history = self.request_history[service]
        
        if service not in self.KNOWN_LIMITS:
            return False, 0
        
        limits = self.KNOWN_LIMITS[service]
        
        # Check requests per minute
        one_minute_ago = time.time() - 60
        recent_requests = sum(1 for ts in history if ts > one_minute_ago)
        
        rpm_limit = limits.get('requests_per_minute', float('inf'))
        
        if recent_requests >= rpm_limit * 0.9:  # 90% threshold
            delay = 60.0 / rpm_limit  # Spread requests evenly
            print(f"[RateLimit] Preemptive throttle: {service} ({recent_requests}/{rpm_limit} RPM)")
            return True, delay
        
        return False, 0
    
    def record_request(self, service):
        """Record a successful request"""
        if service not in self.request_history:
            self.request_history[service] = deque(maxlen=100)
        
        self.request_history[service].append(time.time())
    
    def get_rate_limit_stats(self):
        """
        Get statistics about rate limit violations
        Returns: stats dict
        """
        if not self.limit_violations:
            return {
                'total_violations': 0,
                'by_service': {},
                'recent_violations': []
            }
        
        # Count by service
        by_service = {}
        for v in self.limit_violations:
            service = v['service']
            by_service[service] = by_service.get(service, 0) + 1
        
        # Recent violations (last hour)
        one_hour_ago = datetime.now() - timedelta(hours=1)
        recent = [v for v in self.limit_violations 
                  if datetime.fromisoformat(v['timestamp']) > one_hour_ago]
        
        return {
            'total_violations': len(self.limit_violations),
            'by_service': by_service,
            'recent_violations': len(recent),
            'adaptive_delays': self.adaptive_delays
        }
    
    def print_stats(self):
        """Print rate limit statistics"""
        stats = self.get_rate_limit_stats()
        
        print("\n" + "="*60)
        print("RATE LIMIT STATISTICS")
        print("="*60)
        print(f"Total Violations: {stats['total_violations']}")
        print(f"Recent (1 hour): {stats['recent_violations']}")
        print("\nBy Service:")
        for service, count in stats['by_service'].items():
            delay = stats['adaptive_delays'].get(service, 0)
            print(f"  {service}: {count} violations (delay: {delay}s)")
        print("="*60)


# Global instance
_rate_limit_manager = None

def get_rate_limit_manager():
    """Get global rate limit manager instance"""
    global _rate_limit_manager
    if _rate_limit_manager is None:
        _rate_limit_manager = RateLimitManager()
    return _rate_limit_manager


def main():
    """CLI interface"""
    manager = get_rate_limit_manager()
    manager.print_stats()


if __name__ == "__main__":
    main()
