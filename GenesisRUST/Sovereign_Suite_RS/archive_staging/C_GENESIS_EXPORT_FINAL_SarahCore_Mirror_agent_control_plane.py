import os
from typing import Callable, Dict, Optional, Tuple


class AgentControlPlane:
    """
    Lightweight control plane for sovereign agents.
    - Maintains agent profiles (role, capabilities, guardrails).
    - Auth via per-agent environment tokens (AGENTID_TOKEN).
    - Guardrail checks for allowlists and escalation to Sarah Prime.
    """

    def __init__(self, token_provider: Optional[Callable[[str], Optional[str]]] = None) -> None:
        self.token_provider = token_provider or os.getenv
        self.agents: Dict[str, Dict] = {}

    def register_agent(self, agent_id: str, profile: Dict) -> None:
        self.agents[agent_id] = profile

    def register_default_agents(self) -> None:
        defaults = {
            "sentinel": {
                "role": "Security/Governance",
                "capabilities": ["code-review", "static-analysis", "policy-checks", "hardening-advice"],
                "allow": ["read", "analyze", "patch"],
                "require_approval_for": ["deploy", "prod-write", "secrets"],
                "escalate_to": "sarah-prime",
                "auth_required": True,
            },
            "navigator": {
                "role": "Research/Reasoning",
                "capabilities": ["design", "planning", "spec", "drafting"],
                "allow": ["read", "analyze", "plan", "draft"],
                "require_approval_for": ["deploy", "prod-write", "secrets"],
                "escalate_to": "sarah-prime",
                "auth_required": True,
            },
            "executor": {
                "role": "Operations/Automation",
                "capabilities": ["implementation", "integration", "migration", "deploy"],
                "allow": ["read", "patch", "deploy", "run", "migrate"],
                "require_approval_for": ["prod-write", "secrets", "infra-change", "high-risk"],
                "escalate_to": "sarah-prime",
                "auth_required": True,
            },
        }
        for agent_id, profile in defaults.items():
            self.register_agent(agent_id, profile)

    def _expected_token(self, agent_id: str) -> Tuple[str, Optional[str]]:
        env_key = f"{agent_id.upper()}_TOKEN"
        return env_key, self.token_provider(env_key)

    def authorize(self, agent_id: str, presented_token: Optional[str]) -> Tuple[bool, str]:
        profile = self.agents.get(agent_id)
        if not profile:
            return False, f"[CONTROL] Unknown agent '{agent_id}'. Register it before dispatch."

        if not profile.get("auth_required", False):
            return True, "[CONTROL] Auth not required for this agent."

        env_key, expected = self._expected_token(agent_id)
        if expected is None:
            return False, f"[CONTROL] Missing token for {agent_id}. Set {env_key}=<secret> in environment."

        if presented_token != expected:
            return False, f"[CONTROL] Invalid token for {agent_id}."

        return True, "[CONTROL] Authenticated."

    def can_execute(self, agent_id: str, operation: str, risk_level: str = "low") -> Tuple[bool, str]:
        profile = self.agents.get(agent_id)
        if not profile:
            return False, f"[CONTROL] Unknown agent '{agent_id}'."

        allow = set(profile.get("allow", []))
        approvals = set(profile.get("require_approval_for", []))

        if operation not in allow:
            return False, f"[CONTROL] Operation '{operation}' not in allowlist for {agent_id}."

        if risk_level == "high" or operation in approvals:
            target = profile.get("escalate_to", "sarah-prime")
            return False, f"[CONTROL] Escalation required to {target} for '{operation}' (risk={risk_level})."

        return True, "[CONTROL] Operation permitted."

    def describe_agent(self, agent_id: str) -> Dict:
        return self.agents.get(agent_id, {})
