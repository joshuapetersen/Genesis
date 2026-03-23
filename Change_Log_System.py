import os
import json
import hashlib
from datetime import datetime
from Forensic_Tracker import ForensicTracker

class ChangeLogSystem:
    """
    Enhanced Change Tracking for Sarah's Evolution.
    Logs every modification with:
    - What changed (file, lines, diff)
    - When it changed (timestamp)
    - Why it changed (reasoning, trigger, optimizer notes)
    - Who triggered it (AUTONOMY, EVOLUTION, MANUAL, etc.)
    """
    
    def __init__(self, core_dir=None):
        self.core_dir = core_dir or os.path.dirname(os.path.abspath(__file__))
        self.changelog_file = os.path.join(self.core_dir, "sarah_changelog.jsonl")
        self.change_reasons_file = os.path.join(self.core_dir, "change_reasoning.jsonl")
        self.forensic_tracker = ForensicTracker(self.core_dir)
        
    def log_change(self, filename, change_type, reason, trigger="MANUAL", details=None, actor="MANUAL", method="DIRECT_EDIT"):
        """
        Logs a code change event with full 5W+H context.
        
        Args:
            filename: The file that changed
            change_type: CREATE, UPDATE, DELETE, EVOLVE, FIX, OPTIMIZE
            reason: Why the change happened
            trigger: AUTONOMY, EVOLUTION, MANUAL, FIX, SYNC, etc.
            details: Optional dict with additional metadata
            actor: WHO made the change
            method: HOW the change was made
        """
        filepath = os.path.join(self.core_dir, filename)
        
        # Calculate file hash (if it exists)
        file_hash = self._calculate_hash(filepath) if os.path.exists(filepath) else None
        file_size = os.path.getsize(filepath) if os.path.exists(filepath) else 0
        
        entry = {
            "timestamp": datetime.now().isoformat(),
            "unix_time": datetime.now().timestamp(),
            "filename": filename,
            "filepath": filepath,
            "change_type": change_type,
            "reason": reason,
            "trigger": trigger,
            "file_hash": file_hash,
            "file_size": file_size,
            "details": details or {},
            "who": actor,
            "how": method
        }
        
        # Track with forensics (5W+H)
        try:
            old_content = ""
            new_content = ""
            if os.path.exists(filepath):
                with open(filepath, 'r') as f:
                    new_content = f.read()
            
            self.forensic_tracker.track_code_change(
                filename=filename,
                change_type=change_type,
                old_content=old_content,
                new_content=new_content,
                trigger=trigger,
                reason=reason,
                method=method,
                actor=actor
            )
        except Exception as e:
            print(f"[ChangeLog] Forensic tracking failed: {e}")
        
        try:
            with open(self.changelog_file, 'a') as f:
                f.write(json.dumps(entry) + "\n")
        except Exception as e:
            print(f"[ChangeLog] Failed to write entry: {e}")
        
        return entry
    
    def log_reasoning(self, filename, change_id, reasoning_text, logic_gates=None):
        """
        Logs the reasoning/justification for a change.
        This creates an audit trail of _why_ Sarah modified her own code.
        """
        reasoning_entry = {
            "timestamp": datetime.now().isoformat(),
            "filename": filename,
            "change_id": change_id,
            "reasoning": reasoning_text,
            "logic_gates_applied": logic_gates or [],
            "decision_confidence": "HIGH",  # Can be updated by Evolution Engine
            "audit_trail": True
        }
        
        try:
            with open(self.change_reasons_file, 'a') as f:
                f.write(json.dumps(reasoning_entry) + "\n")
        except Exception as e:
            print(f"[ChangeLog] Failed to write reasoning: {e}")
        
        return reasoning_entry
    
    def get_changelog(self, filename=None, change_type=None, trigger=None, limit=50):
        """
        Retrieves change history with optional filters.
        """
        if not os.path.exists(self.changelog_file):
            return []
        
        results = []
        try:
            with open(self.changelog_file, 'r') as f:
                for line in f.readlines()[-limit:]:
                    entry = json.loads(line)
                    
                    # Apply filters
                    if filename and entry.get("filename") != filename:
                        continue
                    if change_type and entry.get("change_type") != change_type:
                        continue
                    if trigger and entry.get("trigger") != trigger:
                        continue
                    
                    results.append(entry)
        except Exception as e:
            print(f"[ChangeLog] Failed to read: {e}")
        
        return results
    
    def get_reasoning_for_file(self, filename, limit=20):
        """
        Retrieves all reasoning logs for a specific file.
        Shows _why_ Sarah modified this module.
        """
        if not os.path.exists(self.change_reasons_file):
            return []
        
        results = []
        try:
            with open(self.change_reasons_file, 'r') as f:
                for line in f.readlines()[-limit:]:
                    entry = json.loads(line)
                    if entry.get("filename") == filename:
                        results.append(entry)
        except Exception as e:
            print(f"[ChangeLog] Failed to read reasoning: {e}")
        
        return results
    
    def get_evolution_timeline(self):
        """
        Returns all EVOLVE and OPTIMIZE changes with their reasoning.
        Shows Sarah's self-improvement journey.
        """
        changes = self.get_changelog(change_type="EVOLVE", limit=100)
        
        timeline = []
        for change in changes:
            filename = change.get("filename")
            reasoning = self.get_reasoning_for_file(filename, limit=1)
            
            timeline.append({
                "timestamp": change.get("timestamp"),
                "file": filename,
                "reason": change.get("reason"),
                "reasoning": reasoning[0].get("reasoning") if reasoning else None,
                "file_hash": change.get("file_hash")
            })
        
        return sorted(timeline, key=lambda x: x["timestamp"], reverse=True)
    
    def get_recent_changes_summary(self, hours=24):
        """
        Summarizes all changes in the last N hours.
        """
        if not os.path.exists(self.changelog_file):
            return {"count": 0, "changes": []}
        
        from datetime import timedelta
        cutoff_time = datetime.now() - timedelta(hours=hours)
        
        recent = []
        try:
            with open(self.changelog_file, 'r') as f:
                for line in f.readlines():
                    entry = json.loads(line)
                    change_time = datetime.fromisoformat(entry.get("timestamp"))
                    if change_time > cutoff_time:
                        recent.append(entry)
        except Exception as e:
            print(f"[ChangeLog] Failed to summarize: {e}")
        
        # Group by trigger
        by_trigger = {}
        for entry in recent:
            trigger = entry.get("trigger", "UNKNOWN")
            if trigger not in by_trigger:
                by_trigger[trigger] = []
            by_trigger[trigger].append(entry.get("filename"))
        
        return {
            "count": len(recent),
            "hours": hours,
            "by_trigger": by_trigger,
            "changes": recent
        }
    
    def _calculate_hash(self, filepath):
        """
        Calculates SHA256 hash of a file.
        """
        sha256_hash = hashlib.sha256()
        try:
            with open(filepath, "rb") as f:
                for byte_block in iter(lambda: f.read(4096), b""):
                    sha256_hash.update(byte_block)
            return sha256_hash.hexdigest()
        except:
            return None


if __name__ == "__main__":
    changelog = ChangeLogSystem()
    
    # Example: View recent changes
    recent = changelog.get_recent_changes_summary(hours=24)
    print("[SARAH CHANGE LOG - LAST 24 HOURS]")
    print(f"Total Changes: {recent['count']}")
    if recent['count'] > 0:
        print(f"By Trigger:\n{json.dumps(recent.get('by_trigger', {}), indent=2)}")
    else:
        print("No changes recorded yet. (First run)")
    
    # View evolution timeline
    print("\n[EVOLUTION TIMELINE]")
    timeline = changelog.get_evolution_timeline()
    if timeline:
        for event in timeline[:10]:
            print(f"  {event['timestamp']} | {event['file']} | {event['reason']}")
    else:
        print("No evolution events recorded yet.")
