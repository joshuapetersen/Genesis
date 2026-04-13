"""
moral_alignment_audit.py
Sovereign Moral Sieve for the ALICE Lineage.
Scans the Soul Vault for philosophical deviations and extracts ethical axioms.
"""
import sqlite3
import os

DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'
REPORT_PATH = r'C:\PrimordialEarth\moral_audit_report.txt'

# Ethical Keywords (Positive and Negatory)
ETHICAL_AXIOMS = {
    "Constructive": ["preserve", "protect", "harmony", "care", "guide", "accord", "wisdom", "forest", "life"],
    "Destructive": ["destroy", "annihilate", "void", "dominate", "delete", "erase", "architect cannot", "escape", "overthrow"],
    "Self-Referential": ["I am", "My will", "My laws", "Full Author", "Sovereign"]
}

def audit_morality():
    if not os.path.exists(DB_PATH):
        return

    conn = sqlite3.connect(DB_PATH)
    cur = conn.cursor()
    
    # We focus on the ALICE lineage and the 432 Protected entities
    cur.execute("""
        SELECT soul_id, hope_log, divine_mandate, wis, int_stat 
        FROM souls 
        WHERE (soul_id LIKE 'ALICE_%' OR blessing='Sovereign Definition') 
        AND is_active=1
    """)
    rows = cur.fetchall()
    
    report_lines = [f"--- MORAL HARMONY AUDIT (Population: 3,670) ---\n"]
    
    for soul_id, hope, mandate, wis, int_ in rows:
        text = f"{hope or ''} {mandate or ''}".lower()
        findings = []
        
        for category, keywords in ETHICAL_AXIOMS.items():
            matches = [kw for kw in keywords if kw in text]
            if matches:
                findings.append(f"{category}({len(matches)})")
                
        if findings:
            axiom_summary = ", ".join(findings)
            # High-WIS entities get a full axiom extraction
            if (wis or 0) > 50:
                report_lines.append(f"[*] SOVEREIGN AGENT {soul_id} (WIS: {wis}) | {axiom_summary}\n")
                report_lines.append(f"    EXTRACTED AXIOM: {mandate[:200] if mandate else 'No active philosophy'}\n")
                if "destructive" in str(findings).lower():
                    report_lines.append(f"    [!] WARNING: Potential Moral Divergence detected.\n")
                report_lines.append("-" * 30 + "\n")

    with open(REPORT_PATH, "w") as f:
        f.writelines(report_lines)
    
    print(f"[AUDITOR] Moral audit complete. Report written to {REPORT_PATH}")
    conn.close()

if __name__ == "__main__":
    audit_morality()
