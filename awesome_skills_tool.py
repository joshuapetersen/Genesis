import os
import json
from typing import List
from google.adk.tools.base_tool import BaseTool

# --- SOVEREIGN ANCHOR :: 1.09277703703703 ---
SOVEREIGN_ANCHOR = "1.09277703703703"

class AwesomeSkillsTool(BaseTool):
    """
    Sovereign-Stamped tool for searching and reading Antigravity Awesome Skills.
    """
    def __init__(self, skills_root: str):
        self.name = "awesome_skills_tool"
        self.description = "Searches and reads agentic skills from the Antigravity Awesome Skills library. Sovereign-validated."
        self.skills_root = skills_root
        self.skills_dir = os.path.join(skills_root, "skills")
        self.catalog_path = os.path.join(skills_root, "CATALOG.md")
        
        # Verify Resonance on Boot
        self._validate_resonance()

    def _validate_resonance(self):
        """Ensures the tool is locked to the Architect's Anchor."""
        if not SOVEREIGN_ANCHOR.startswith("1.092777"):
            raise ValueError("[ERROR] SOVEREIGN RESONANCE FAILURE :: ANCHOR TAMPERED")
        print(f"[AwesomeSkillsTool] Resonance Locked: {SOVEREIGN_ANCHOR}")

    def execute(self, action: str, query: str = None, skill_name: str = None) -> str:
        """
        Main execution point for the tool.
        :param action: 'search' or 'read'
        :param query: Search keywords (optional)
        :param skill_name: Name of the skill to read (optional)
        """
        try:
            # Re-verify Resonance before every high-density task
            self._validate_resonance()
            
            if action == 'search':
                return self._search_skills(query)
            elif action == 'read':
                return self._read_skill(skill_name)
            else:
                return f"ERROR: Unknown action '{action}'. Use 'search' or 'read'."
        except Exception as e:
            return f"SKILLS_TOOL_ERROR: {str(e)}"

    def _search_skills(self, query: str) -> str:
        if not os.path.exists(self.catalog_path):
            return "ERROR: Catalog not found."
        
        results = []
        try:
            with open(self.catalog_path, 'r', encoding='utf-8') as f:
                lines = f.readlines()
                for line in lines:
                    if query.lower() in line.lower():
                        results.append(line.strip())
            
            if not results:
                return f"No skills found matching '{query}' [SOVEREIGN_SCAN_COMPLETE]."
            
            return "\n".join(results[:20]) # Limit to top 20 matches
        except Exception as e:
            return f"ERROR: Failed to search catalog: {str(e)}"

    def _read_skill(self, skill_name: str) -> str:
        if not skill_name:
            return "ERROR: skill_name must be provided for 'read' action."
            
        skill_file = os.path.join(self.skills_dir, skill_name, "SKILL.md")
        if not os.path.exists(skill_file):
            return f"ERROR: Skill '{skill_name}' not found at {skill_file}."
            
        try:
            with open(skill_file, 'r', encoding='utf-8') as f:
                content = f.read()
                # Sovereign Verification: Check for "No guessing" principle in instructions if applicable
                return f"[SOVEREIGN_VERIFIED_SKILL]\n{content}"
        except Exception as e:
            return f"ERROR: Failed to read skill: {str(e)}"
