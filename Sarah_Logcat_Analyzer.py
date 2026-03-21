"""
SARAH LOGCAT ANALYZER
AI-powered log analysis and insights
"""

import json
from pathlib import Path
from datetime import datetime, timedelta
from collections import defaultdict, Counter
import statistics

class SarahLogcatAnalyzer:
    """
    Analyzes Sarah's logs to extract insights and patterns.
    """
    
    def __init__(self, log_dir="C:/SarahCore/logs"):
        self.log_dir = Path(log_dir)
        self.json_log = self.log_dir / "sarah_events.jsonl"
        self.logs = []
        
        self._load_logs()
    
    def _load_logs(self):
        """Load all logs into memory for analysis."""
        if not self.json_log.exists():
            print(f"[ERROR] Log file not found: {self.json_log}")
            return
        
        with open(self.json_log, 'r') as f:
            for line in f:
                try:
                    self.logs.append(json.loads(line))
                except:
                    pass
        
        print(f"[ANALYZER] Loaded {len(self.logs)} log entries")
    
    def analyze_patterns(self):
        """Detect patterns in Sarah's behavior."""
        print("=" * 80)
        print("PATTERN ANALYSIS")
        print("=" * 80 + "\n")
        
        # Time-based patterns
        hour_distribution = defaultdict(int)
        category_timeline = defaultdict(list)
        
        for entry in self.logs:
            timestamp = datetime.fromisoformat(entry['timestamp'])
            hour = timestamp.hour
            category = entry['category']
            
            hour_distribution[hour] += 1
            category_timeline[category].append(timestamp)
        
        # Most active hours
        print("📊 Activity by Hour:")
        for hour in range(24):
            count = hour_distribution.get(hour, 0)
            bar = '█' * (count // 10)
            print(f"  {hour:02d}:00 - {count:4d} {bar}")
        
        # Category trends
        print("\n📈 Category Activity Trends:")
        for category, timestamps in category_timeline.items():
            if len(timestamps) > 1:
                duration = (max(timestamps) - min(timestamps)).total_seconds()
                rate = len(timestamps) / max(duration / 3600, 1)  # per hour
                print(f"  {category:12s}: {len(timestamps):5d} events ({rate:.2f}/hour)")
    
    def analyze_performance(self):
        """Analyze performance metrics."""
        print("\n" + "=" * 80)
        print("PERFORMANCE ANALYSIS")
        print("=" * 80 + "\n")
        
        # Extract metrics
        metrics = defaultdict(list)
        
        for entry in self.logs:
            if entry.get('metadata', {}).get('metric'):
                metric_name = entry['metadata']['metric']
                value = entry['metadata']['value']
                metrics[metric_name].append(float(value))
        
        # Analyze each metric
        for metric_name, values in metrics.items():
            if values:
                avg = statistics.mean(values)
                if len(values) > 1:
                    stddev = statistics.stdev(values)
                else:
                    stddev = 0
                
                print(f"  {metric_name}:")
                print(f"    Average: {avg:.2f}")
                print(f"    Std Dev: {stddev:.2f}")
                print(f"    Min: {min(values):.2f}")
                print(f"    Max: {max(values):.2f}")
                print()
    
    def analyze_errors(self):
        """Analyze error patterns."""
        print("\n" + "=" * 80)
        print("ERROR ANALYSIS")
        print("=" * 80 + "\n")
        
        errors = [e for e in self.logs if e['level'] in ['ERROR', 'CRITICAL']]
        warnings = [e for e in self.logs if e['level'] == 'WARNING']
        
        print(f"Total Errors: {len(errors)}")
        print(f"Total Warnings: {len(warnings)}")
        print(f"Error Rate: {len(errors) / max(len(self.logs), 1) * 100:.2f}%\n")
        
        # Most common errors
        error_messages = Counter([e['message'] for e in errors])
        
        if error_messages:
            print("Most Common Errors:")
            for message, count in error_messages.most_common(10):
                print(f"  [{count:3d}x] {message[:60]}")
        
        # Errors by category
        errors_by_category = Counter([e['category'] for e in errors])
        
        if errors_by_category:
            print("\nErrors by Category:")
            for category, count in errors_by_category.most_common():
                print(f"  {category:12s}: {count:3d}")
    
    def analyze_learning_progress(self):
        """Analyze Sarah's learning progress."""
        print("\n" + "=" * 80)
        print("LEARNING PROGRESS ANALYSIS")
        print("=" * 80 + "\n")
        
        # Find learning-related events
        learning_events = [e for e in self.logs if e['category'] == 'learning']
        
        print(f"Total Learning Events: {len(learning_events)}")
        
        # Extract discoveries
        discoveries = sum(1 for e in learning_events if 'discover' in e['message'].lower())
        
        print(f"Discoveries Made: {discoveries}")
        
        # Learning timeline
        if learning_events:
            first = datetime.fromisoformat(learning_events[0]['timestamp'])
            last = datetime.fromisoformat(learning_events[-1]['timestamp'])
            duration = (last - first).total_seconds()
            
            print(f"Learning Duration: {duration / 3600:.2f} hours")
            print(f"Learning Rate: {len(learning_events) / max(duration / 3600, 1):.2f} events/hour")
    
    def generate_report(self):
        """Generate comprehensive analysis report."""
        print("\n" + "=" * 80)
        print("SARAH LOGCAT ANALYZER - COMPREHENSIVE REPORT")
        print("=" * 80 + "\n")
        
        print(f"Report Generated: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}")
        print(f"Total Log Entries: {len(self.logs)}")
        
        if self.logs:
            first_entry = datetime.fromisoformat(self.logs[0]['timestamp'])
            last_entry = datetime.fromisoformat(self.logs[-1]['timestamp'])
            print(f"Log Span: {first_entry} to {last_entry}")
            print(f"Duration: {(last_entry - first_entry).total_seconds() / 3600:.2f} hours\n")
        
        self.analyze_patterns()
        self.analyze_performance()
        self.analyze_errors()
        self.analyze_learning_progress()
        
        print("\n" + "=" * 80)
        print("END OF REPORT")
        print("=" * 80)
    
    def predict_issues(self):
        """Predict potential issues based on log patterns."""
        print("\n" + "=" * 80)
        print("PREDICTIVE ANALYSIS")
        print("=" * 80 + "\n")
        
        issues = []
        
        # Check error rate trend
        if len(self.logs) > 100:
            recent_logs = self.logs[-100:]
            recent_errors = sum(1 for e in recent_logs if e['level'] in ['ERROR', 'CRITICAL'])
            error_rate = recent_errors / 100
            
            if error_rate > 0.10:
                issues.append(f"⚠️  High error rate in recent activity: {error_rate*100:.1f}%")
        
        # Check for repeated errors
        recent_messages = [e['message'] for e in self.logs[-50:] if e['level'] == 'ERROR']
        repeated = Counter(recent_messages)
        
        for message, count in repeated.items():
            if count > 5:
                issues.append(f"⚠️  Repeated error ({count}x): {message[:50]}")
        
        # Print findings
        if issues:
            print("Potential Issues Detected:\n")
            for issue in issues:
                print(f"  {issue}")
        else:
            print("✅ No issues detected. Sarah is operating normally.")
        
        print()


if __name__ == "__main__":
    analyzer = SarahLogcatAnalyzer()
    analyzer.generate_report()
    analyzer.predict_issues()
