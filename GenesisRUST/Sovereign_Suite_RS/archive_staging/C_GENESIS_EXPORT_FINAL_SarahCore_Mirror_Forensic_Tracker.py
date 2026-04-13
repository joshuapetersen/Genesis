import os
import json
import platform
import getpass
from datetime import datetime

class ForensicTracker:
    """
    5W+H FORENSIC TRACKING SYSTEM
    
    Captures complete context for every system event:
    - WHO: User, process, trigger source
    - WHAT: Action performed, change type
    - WHERE: File path, module, function
    - WHEN: Timestamp, duration, frequency
    - WHY: Reasoning, justification, intent
    - HOW: Method, mechanism, approach
    """
    
    def __init__(self, core_dir=None):
        self.core_dir = core_dir or os.path.dirname(os.path.abspath(__file__))
        self.forensic_log = os.path.join(self.core_dir, "forensic_audit.jsonl")
        self.context_cache = self._build_context()
    
    def _build_context(self):
        """
        Builds system context for WHO and WHERE.
        """
        return {
            "system_user": getpass.getuser(),
            "hostname": platform.node(),
            "os": platform.system(),
            "os_version": platform.version(),
            "python_version": platform.python_version(),
            "workspace": self.core_dir
        }
    
    def track_event(self, event_data):
        """
        Tracks an event with complete 5W+H context.
        
        event_data must include:
        - who: {user, trigger, process, actor}
        - what: {action, change_type, operation}
        - where: {file, module, function, line_range}
        - when: {timestamp, duration, session_id}
        - why: {reason, justification, goal}
        - how: {method, mechanism, approach, tools_used}
        """
        
        # Validate required fields
        required = ['who', 'what', 'where', 'when', 'why', 'how']
        for field in required:
            if field not in event_data:
                event_data[field] = {"status": "NOT_PROVIDED"}
        
        # Enrich with system context
        enriched_event = {
            "event_id": self._generate_event_id(),
            "system_context": self.context_cache,
            **event_data,
            "forensic_timestamp": datetime.now().isoformat()
        }
        
        # Write to forensic log
        self._write_forensic_entry(enriched_event)
        
        return enriched_event
    
    def track_code_change(self, filename, change_type, old_content, new_content, 
                         trigger, reason, method, actor="AUTONOMY"):
        """
        Specialized tracker for code changes with full 5W+H.
        """
        
        # Calculate change metrics
        lines_changed = self._calculate_line_changes(old_content, new_content)
        
        event = {
            "who": {
                "actor": actor,  # AUTONOMY, MANUAL, EVOLUTION, SYNC
                "trigger": trigger,  # What initiated this
                "user": self.context_cache["system_user"],
                "process": "Sarah_Evolution_Engine"
            },
            "what": {
                "action": "CODE_CHANGE",
                "change_type": change_type,  # UPDATE, CREATE, DELETE, EVOLVE, FIX, OPTIMIZE
                "scope": "SOURCE_CODE",
                "lines_added": lines_changed["added"],
                "lines_removed": lines_changed["removed"],
                "net_change": lines_changed["net"]
            },
            "where": {
                "file": filename,
                "filepath": os.path.join(self.core_dir, filename),
                "module": filename.replace('.py', ''),
                "workspace": self.core_dir,
                "repository": os.path.dirname(self.core_dir)
            },
            "when": {
                "timestamp": datetime.now().isoformat(),
                "unix_time": datetime.now().timestamp(),
                "session_id": self._get_session_id(),
                "duration_seconds": None  # To be filled if tracked
            },
            "why": {
                "stated_reason": reason,
                "intent": self._infer_intent(change_type, reason),
                "goal": self._infer_goal(change_type),
                "justification": reason,
                "impact": "Improves code quality" if "OPTIMIZE" in change_type else "Fixes issue" if "FIX" in change_type else "Adds functionality"
            },
            "how": {
                "method": method,  # "LLM_GENERATION", "MANUAL_EDIT", "AUTOMATED_FIX"
                "mechanism": "Self_Optimizer" if "EVOLVE" in change_type else "Direct_Edit",
                "approach": "Incremental" if lines_changed["net"] < 50 else "Major_Refactor",
                "tools_used": ["Gemini_Genesis_Core", "Self_Optimizer", "Advanced_Change_Tracking"],
                "technique": "Prompt-based evolution" if "EVOLVE" in change_type else "Direct modification"
            }
        }
        
        return self.track_event(event)
    
    def track_system_event(self, event_type, action, reason, details=None):
        """
        Tracks general system events (boot, sync, config change, etc.)
        """
        event = {
            "who": {
                "actor": "SYSTEM",
                "trigger": "AUTOMATED",
                "user": self.context_cache["system_user"],
                "process": "Sarah_Core"
            },
            "what": {
                "action": action,
                "event_type": event_type,
                "scope": "SYSTEM",
                "details": details or {}
            },
            "where": {
                "location": "Sarah_Core",
                "workspace": self.core_dir,
                "component": details.get("component", "UNKNOWN") if details else "UNKNOWN"
            },
            "when": {
                "timestamp": datetime.now().isoformat(),
                "unix_time": datetime.now().timestamp(),
                "session_id": self._get_session_id()
            },
            "why": {
                "stated_reason": reason,
                "intent": "SYSTEM_OPERATION",
                "goal": "MAINTAIN_OPERATIONAL_STATE"
            },
            "how": {
                "method": "AUTOMATED_PROCESS",
                "mechanism": "System_Core",
                "approach": "ROUTINE"
            }
        }
        
        return self.track_event(event)
    
    def query_forensics(self, who=None, what=None, where=None, when_range=None, why=None, how=None):
        """
        Queries forensic log with any combination of 5W+H filters.
        """
        if not os.path.exists(self.forensic_log):
            return []
        
        results = []
        
        try:
            with open(self.forensic_log, 'r') as f:
                for line in f.readlines():
                    entry = json.loads(line)
                    
                    # Apply filters
                    if who and not self._match_field(entry.get('who', {}), who):
                        continue
                    if what and not self._match_field(entry.get('what', {}), what):
                        continue
                    if where and not self._match_field(entry.get('where', {}), where):
                        continue
                    if why and not self._match_field(entry.get('why', {}), why):
                        continue
                    if how and not self._match_field(entry.get('how', {}), how):
                        continue
                    
                    results.append(entry)
        except Exception as e:
            print(f"[Forensics] Query error: {e}")
        
        return results
    
    def generate_forensic_report(self, hours=24):
        """
        Generates a comprehensive forensic report.
        """
        from datetime import timedelta
        cutoff = datetime.now() - timedelta(hours=hours)
        
        events = self.query_forensics()
        recent_events = [e for e in events if datetime.fromisoformat(e['forensic_timestamp']) > cutoff]
        
        # Aggregate by WHO
        by_actor = {}
        for event in recent_events:
            actor = event['who'].get('actor', 'UNKNOWN')
            by_actor[actor] = by_actor.get(actor, 0) + 1
        
        # Aggregate by WHAT
        by_action = {}
        for event in recent_events:
            action = event['what'].get('action', 'UNKNOWN')
            by_action[action] = by_action.get(action, 0) + 1
        
        # Aggregate by WHERE
        by_location = {}
        for event in recent_events:
            location = event['where'].get('file', event['where'].get('location', 'UNKNOWN'))
            by_location[location] = by_location.get(location, 0) + 1
        
        report = {
            "report_timestamp": datetime.now().isoformat(),
            "time_range_hours": hours,
            "total_events": len(recent_events),
            "by_actor": by_actor,
            "by_action": by_action,
            "by_location": by_location,
            "most_active_actor": max(by_actor.items(), key=lambda x: x[1])[0] if by_actor else None,
            "most_common_action": max(by_action.items(), key=lambda x: x[1])[0] if by_action else None,
            "most_affected_location": max(by_location.items(), key=lambda x: x[1])[0] if by_location else None
        }
        
        return report
    
    def _generate_event_id(self):
        """Generates unique event ID."""
        import hashlib
        timestamp = str(datetime.now().timestamp())
        return hashlib.sha256(timestamp.encode()).hexdigest()[:16]
    
    def _get_session_id(self):
        """Returns current session ID (process start time)."""
        return str(datetime.now().date())
    
    def _calculate_line_changes(self, old_content, new_content):
        """Calculates line-level changes."""
        old_lines = old_content.splitlines() if old_content else []
        new_lines = new_content.splitlines() if new_content else []
        
        added = len(new_lines) - len(old_lines)
        return {
            "added": max(0, added),
            "removed": max(0, -added),
            "net": added
        }
    
    def _infer_intent(self, change_type, reason):
        """Infers intent from change type and reason."""
        reason_lower = reason.lower()
        
        if "optimize" in reason_lower or change_type == "OPTIMIZE":
            return "PERFORMANCE_IMPROVEMENT"
        elif "fix" in reason_lower or change_type == "FIX":
            return "BUG_RESOLUTION"
        elif "add" in reason_lower or change_type == "CREATE":
            return "FEATURE_ADDITION"
        elif "evolve" in reason_lower or change_type == "EVOLVE":
            return "SELF_IMPROVEMENT"
        else:
            return "MAINTENANCE"
    
    def _infer_goal(self, change_type):
        """Infers goal from change type."""
        goals = {
            "OPTIMIZE": "Reduce complexity and improve efficiency",
            "FIX": "Resolve error or bug",
            "EVOLVE": "Self-improve architecture",
            "CREATE": "Add new capability",
            "UPDATE": "Maintain and refine existing code",
            "DELETE": "Remove obsolete or redundant code"
        }
        return goals.get(change_type, "General maintenance")
    
    def _match_field(self, field_dict, query):
        """Matches query against field dictionary."""
        query_str = str(query).lower()
        for value in field_dict.values():
            if query_str in str(value).lower():
                return True
        return False
    
    def _write_forensic_entry(self, entry):
        """Writes entry to forensic log."""
        try:
            with open(self.forensic_log, 'a') as f:
                f.write(json.dumps(entry) + "\n")
        except Exception as e:
            print(f"[Forensics] Failed to write: {e}")


if __name__ == "__main__":
    tracker = ForensicTracker()
    
    print("[FORENSIC TRACKER] Initialized")
    print("\nGenerating sample forensic report...")
    
    report = tracker.generate_forensic_report(hours=24)
    
    print(f"\n[FORENSIC REPORT - LAST 24 HOURS]")
    print(f"Total Events: {report['total_events']}")
    print(f"Most Active Actor: {report['most_active_actor']}")
    print(f"Most Common Action: {report['most_common_action']}")
    print(f"Most Affected Location: {report['most_affected_location']}")
