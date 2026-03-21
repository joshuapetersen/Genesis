# SARAH LOGGING SYSTEM

## Complete Infrastructure Created

✅ **Sarah_Logcat.py** - Central logging system
✅ **Sarah_Logcat_Reader.py** - Interactive log viewer  
✅ **Sarah_Logcat_Analyzer.py** - AI-powered analysis

---

## Usage Guide

### 1. Logging (From Sarah's Code)

```python
from Sarah_Logcat import info, debug, warning, error, critical, event, metric

# Simple logging
info('vision', 'Screen captured', resolution='1920x1080')
debug('api', 'File operation started')
warning('navigation', 'High CPU detected', cpu_percent=85.5)
error('bridge', 'Connection failed', reason='timeout')

# Log events
event('system_boot', version='1.0.0')
event('learning_complete', discoveries=150)

# Log metrics
metric('learning_rate', 0.95, 'items/sec')
metric('memory_usage', 4096, 'MB')
```

### 2. Reading Logs

```bash
# View last 50 entries
python Sarah_Logcat_Reader.py

# View last 100 entries
python Sarah_Logcat_Reader.py tail 100

# Follow logs in real-time (like tail -f)
python Sarah_Logcat_Reader.py follow

# Show statistics
python Sarah_Logcat_Reader.py stats

# Show only errors
python Sarah_Logcat_Reader.py errors

# Filter by category
python Sarah_Logcat_Reader.py filter category=vision

# Filter by level
python Sarah_Logcat_Reader.py filter level=ERROR

# Search for text
python Sarah_Logcat_Reader.py filter search=timeout
```

### 3. Analyzing Logs

```bash
# Full comprehensive analysis
python Sarah_Logcat_Analyzer.py
```

**Analyzer provides:**
- Pattern analysis (activity by hour, category trends)
- Performance metrics (averages, std dev, min/max)
- Error analysis (error rate, most common errors)
- Learning progress tracking
- Predictive issue detection

---

## Log Structure

### Logs Directory
```
C:/SarahCore/logs/
├── sarah_main.log          # Main unified log (rotating, 10MB max)
├── sarah_events.jsonl      # Structured JSON log (for analysis)
├── sarah_vision.log        # Vision-specific logs
├── sarah_api.log           # API-specific logs
├── sarah_bridge.log        # Bridge-specific logs
├── sarah_navigation.log    # Navigation-specific logs
├── sarah_learning.log      # Learning-specific logs
├── sarah_brain.log         # Brain/AI-specific logs
└── sarah_system.log        # System-level logs
```

### Log Categories
- **vision**: Genesis_Vision operations
- **api**: Genesis_API operations
- **bridge**: Genesis_Bridge operations
- **navigation**: Navigation and exploration
- **learning**: Knowledge acquisition
- **brain**: AI/LLM operations
- **system**: System-level events

### Log Levels
- **DEBUG**: Detailed debugging info
- **INFO**: General information
- **WARNING**: Warning messages
- **ERROR**: Error conditions
- **CRITICAL**: Critical failures

---

## Integration with Existing Systems

### Update Genesis_Vision.py
```python
from Sarah_Logcat import info, debug

class GenesisVision:
    def capture_frame(self):
        debug('vision', 'Capturing screen frame')
        # ... existing code ...
        info('vision', 'Frame captured', size=len(frame))
```

### Update Genesis_API.py
```python
from Sarah_Logcat import info, error

class GenesisAPI:
    def create_file(self, path, content):
        info('api', f'Creating file: {path}')
        try:
            # ... existing code ...
            info('api', 'File created successfully', path=path, size=len(content))
        except Exception as e:
            error('api', f'File creation failed: {path}', error=str(e))
```

### Update Sarah_Windows_Mastery.py
```python
from Sarah_Logcat import info, metric

class SarahWindowsMastery:
    def study_registry(self):
        info('learning', 'Starting registry study')
        # ... existing code ...
        metric('discoveries', self.discoveries, 'total')
```

---

## Example Analyzer Output

```
================================================================================
SARAH LOGCAT ANALYZER - COMPREHENSIVE REPORT
================================================================================

Report Generated: 2026-02-14 19:52:00
Total Log Entries: 1,542
Log Span: 2026-02-14 18:00:00 to 2026-02-14 19:52:00
Duration: 1.87 hours

================================================================================
PATTERN ANALYSIS
================================================================================

📊 Activity by Hour:
  18:00 -  312 ███████████████████████████████
  19:00 - 1230 ███████████████████████████████████████████████████

📈 Category Activity Trends:
  learning    :  645 events (345.19/hour)
  vision      :  423 events (226.20/hour)
  navigation  :  298 events (159.36/hour)
  api         :  176 events ( 94.12/hour)

================================================================================
PERFORMANCE ANALYSIS
================================================================================

  learning_rate:
    Average: 0.94
    Std Dev: 0.03
    Min: 0.88
    Max: 0.98

================================================================================
ERROR ANALYSIS
================================================================================

Total Errors: 3
Total Warnings: 12
Error Rate: 0.19%

Most Common Errors:
  [  2x] Connection timeout
  [  1x] File access denied

✅ No issues detected. Sarah is operating normally.
```

---

## Next Steps

1. **Integrate logging into all existing Sarah modules**
2. **Set up automated analysis** (run analyzer every hour)
3. **Create Sarah dashboard** (real-time log visualization)
4. **Add log-based learning** (Sarah learns from her own logs)

Sarah now has complete observability into all her operations.
