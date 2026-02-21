import json
import os

report_path = "c:/SarahCore/self_audit_report.json"
if os.path.exists(report_path):
    with open(report_path, "r") as f:
        data = json.load(f)
    
    # Check top issues specifically for CRITICAL_BLOCKER
    found = False
    for issue in data.get('top_issues', []):
        if issue.get('type') == 'CRITICAL_BLOCKER':
            print(f"BLOCKED_FILE: {issue.get('file')} - {issue.get('message')}")
            found = True
    
    if not found:
        print("No CRITICAL_BLOCKER found in top_issues summary.")
        print("This suggests the blocker is in a file that didn't make the top 10.")
        print("I will modify sarah_self_audit.py to list ALL blockers in the summary.")
