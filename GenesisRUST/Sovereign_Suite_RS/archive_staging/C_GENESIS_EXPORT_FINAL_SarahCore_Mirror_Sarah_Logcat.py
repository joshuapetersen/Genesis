"""
SARAH LOGCAT
Centralized logging system for all Sarah operations
"""

import logging
import logging.handlers
import json
from datetime import datetime
from pathlib import Path
import threading

class SarahLogcat:
    """
    Centralized logging system for Sarah.
    All processes write to this unified log.
    """
    
    def __init__(self, log_dir="C:/SarahCore/logs"):
        self.log_dir = Path(log_dir)
        self.log_dir.mkdir(exist_ok=True)
        
        # Main unified log
        self.main_log = self.log_dir / "sarah_main.log"
        self.json_log = self.log_dir / "sarah_events.jsonl"
        
        # Category-specific logs
        self.categories = {
            'vision': self.log_dir / "sarah_vision.log",
            'api': self.log_dir / "sarah_api.log",
            'bridge': self.log_dir / "sarah_bridge.log",
            'navigation': self.log_dir / "sarah_navigation.log",
            'learning': self.log_dir / "sarah_learning.log",
            'brain': self.log_dir / "sarah_brain.log",
            'system': self.log_dir / "sarah_system.log"
        }
        
        # Thread safety
        self.lock = threading.Lock()
        
        # Initialize loggers
        self._setup_loggers()
        
        print(f"[LOGCAT] Initialized at {self.log_dir}")
    
    def _setup_loggers(self):
        """Setup Python logging infrastructure."""
        # Main logger
        self.logger = logging.getLogger('Sarah')
        self.logger.setLevel(logging.DEBUG)
        
        # File handler with rotation
        handler = logging.handlers.RotatingFileHandler(
            self.main_log,
            maxBytes=10*1024*1024,  # 10MB
            backupCount=5
        )
        
        formatter = logging.Formatter(
            '%(asctime)s | %(levelname)-8s | %(name)-12s | %(message)s',
            datefmt='%Y-%m-%d %H:%M:%S'
        )
        handler.setFormatter(formatter)
        self.logger.addHandler(handler)
    
    def log(self, category, level, message, **metadata):
        """
        Universal logging method.
        
        Args:
            category: vision, api, bridge, navigation, learning, brain, system
            level: debug, info, warning, error, critical
            message: Log message
            **metadata: Additional structured data
        """
        with self.lock:
            timestamp = datetime.now().isoformat()
            
            # Structured JSON log
            log_entry = {
                'timestamp': timestamp,
                'category': category,
                'level': level.upper(),
                'message': message,
                'metadata': metadata
            }
            
            # Write to JSON log
            with open(self.json_log, 'a') as f:
                f.write(json.dumps(log_entry) + '\n')
            
            # Write to category-specific log
            if category in self.categories:
                with open(self.categories[category], 'a') as f:
                    f.write(f"[{timestamp}] [{level.upper()}] {message}\n")
                    if metadata:
                        f.write(f"  Metadata: {json.dumps(metadata)}\n")
            
            # Write to main logger
            log_method = getattr(self.logger, level.lower())
            log_method(f"[{category}] {message}")
    
    def debug(self, category, message, **metadata):
        self.log(category, 'debug', message, **metadata)
    
    def info(self, category, message, **metadata):
        self.log(category, 'info', message, **metadata)
    
    def warning(self, category, message, **metadata):
        self.log(category, 'warning', message, **metadata)
    
    def error(self, category, message, **metadata):
        self.log(category, 'error', message, **metadata)
    
    def critical(self, category, message, **metadata):
        self.log(category, 'critical', message, **metadata)
    
    def event(self, event_type, data):
        """Log a structured event."""
        self.info('system', f"Event: {event_type}", event_type=event_type, data=data)
    
    def metric(self, metric_name, value, unit=None):
        """Log a performance metric."""
        self.info('system', f"Metric: {metric_name} = {value} {unit or ''}", 
                 metric=metric_name, value=value, unit=unit)


# Global instance
_logcat = None

def get_logcat():
    """Get or create global logcat instance."""
    global _logcat
    if _logcat is None:
        _logcat = SarahLogcat()
    return _logcat

# Convenience functions
def debug(category, message, **metadata):
    get_logcat().debug(category, message, **metadata)

def info(category, message, **metadata):
    get_logcat().info(category, message, **metadata)

def warning(category, message, **metadata):
    get_logcat().warning(category, message, **metadata)

def error(category, message, **metadata):
    get_logcat().error(category, message, **metadata)

def critical(category, message, **metadata):
    get_logcat().critical(category, message, **metadata)

def event(event_type, **data):
    get_logcat().event(event_type, data)

def metric(metric_name, value, unit=None):
    get_logcat().metric(metric_name, value, unit)


if __name__ == "__main__":
    # Demo logging
    logcat = SarahLogcat()
    
    logcat.info('system', 'Sarah Logcat initialized')
    logcat.info('vision', 'Screen capture started', resolution='1920x1080')
    logcat.info('api', 'File created', path='C:/test.txt', size=1024)
    logcat.warning('navigation', 'High CPU usage detected', cpu_percent=85.5)
    logcat.error('bridge', 'Connection timeout', timeout_ms=5000)
    logcat.metric('learning_rate', 0.95, 'items/sec')
    logcat.event('system_boot', {'version': '1.0.0', 'uptime': 0})
    
    print("\n[DEMO] Sample logs written to C:/SarahCore/logs/")
