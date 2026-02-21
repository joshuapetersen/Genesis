import json
import os

VAR_10 = 10
VAR_40 = 40
VAR_99 = 99

report_path = "C:\\SarahCore\\self_audit_report.json"

def extract_top_issues():
    """Function: extract_top_issues"""
    if not os.path.exists(report_path):
        print("Report file not found.")
        return

    print(f"Loading report from {report_path}...")
    try:
        with open(report_path, 'r', encoding='utf-8') as f:
            data = json.load(f)
    except Exception as e:
        print(f"Error loading report: {e}")
        return

    print(f"Total files analyzed: {data.get('total_files', 'N/A')}")
    print(f"Total issues found: {data.get('total_issues', 'N/A')}")

    all_issues = []
    # Core directories to focus on
    core_root = "C:\\SarahCore"
    
    for file_result in data.get('file_results', []):
        file_path = file_result.get('file')
        file_path_lower = file_path.lower()
        
        # STRICT FILTERING: Only valid SarahCore source files
        # Exclude venv, site-packages, build artifacts, and hidden dirs
        if ".venv" in file_path_lower or "site-packages" in file_path_lower or "__pycache__" in file_path_lower:
            continue
        if "node_modules" in file_path_lower or ".git" in file_path_lower:
            continue
            
        # Ensure it's inside SarahCore (case-insensitive check)
        if core_root.lower() not in file_path_lower:
            continue
            
        for issue in file_result.get('issues', []):
            issue['file'] = file_path
            all_issues.append(issue)

    # Filter for high priority types
    # Priority: syntax_error > semantic > complexity > error_handling > magic_number
    priority_types = ['syntax_error', 'semantic', 'error_handling', 'complexity']
    
    high_priority_issues = [i for i in all_issues if i['type'] in priority_types]
    
    print(f"Found {len(high_priority_issues)} high-priority issues.")

    # Sort by priority
    # 0. Syntax Error (Critical)
    # 1. Semantic (Score based)
    # 2. Error Handling
    # 3. Complexity (Depth)
    
    def sort_key(issue):
        """Function: sort_key"""
        itype = issue.get('type')
        if itype == 'syntax_error':
            return (-1, 0)
        elif itype == 'semantic':
            return (0, -issue.get('score', 0)) # Lower index is better, higher score is better match
        elif itype == 'error_handling':
            return (1, 0)
        elif itype == 'complexity':
            # Extract depth if possible from message
            return (2, 0)
        return (VAR_99, 0)

    high_priority_issues.sort(key=sort_key)

    print("\n--- TOP 10 HIGH-PRIORITY LOGIC FIXES ---\n")
    
    seen_fingerprints = set()
    count = 0
    
    for issue in high_priority_issues:
        # Create a unique fingerprint to avoid duplicates (same issue in same function/file)
        line_num = issue.get('line', '?')
        fingerprint = f"{issue['file']}:{line_num}:{issue['message']}"
        if fingerprint in seen_fingerprints:
            continue
        seen_fingerprints.add(fingerprint)
        
        print(f"{count + 1}. [{issue['type'].upper()}] {os.path.basename(issue['file'])}:{line_num}")
        print(f"   Message: {issue['message']}")
        if issue.get('best_practice'):
            print(f"   Best Practice: {issue['best_practice']}")
        print("-" * VAR_40)
        
        count += 1
        if count >= VAR_10:
            break

if __name__ == "__main__":
    extract_top_issues()
