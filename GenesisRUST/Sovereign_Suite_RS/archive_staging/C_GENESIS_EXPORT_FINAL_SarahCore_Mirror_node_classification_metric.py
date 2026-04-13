import json
from datetime import datetime
from typing import Dict, List, Tuple


class NodeClassificationMetric:
    """
    Comprehensive node classification system for Alpha/Beta/Delta.
    Tracks health, security, performance, and autonomy readiness.
    """

    def __init__(self, node_id: str, node_type: str):
        """
        Args:
            node_id: Identifier (e.g., 'alpha', 'beta', 'delta')
            node_type: Type (e.g., 'logic', 'persistence', 'command')
        """
        self.node_id = node_id
        self.node_type = node_type
        self.metrics = {
            "health": 100,
            "uptime_hours": 0,
            "connectivity": 100,
            "security_score": 100,
            "autonomy_readiness": 0,
            "performance": 100,
            "data_integrity": 100,
            "agent_capacity": 0,
            "ledger_entries": 0,
            "last_heartbeat": datetime.utcnow().isoformat(),
        }
        self.classification = "UNKNOWN"
        self.threats = []
        self.capabilities = []

    def calculate_health_score(self) -> int:
        """
        Composite health = (connectivity + security + performance + data_integrity) / 4
        """
        score = (
            self.metrics["connectivity"]
            + self.metrics["security_score"]
            + self.metrics["performance"]
            + self.metrics["data_integrity"]
        ) / 4
        self.metrics["health"] = int(score)
        return int(score)

    def calculate_autonomy_readiness(self) -> int:
        """
        Autonomy readiness based on health, active agents, and security.
        Range: 0-100.
        """
        health = self.metrics["health"]
        security = self.metrics["security_score"]
        agents = min(self.metrics["agent_capacity"], 12) / 12 * 100  # Max 12 agents

        readiness = (health * 0.4) + (security * 0.4) + (agents * 0.2)
        self.metrics["autonomy_readiness"] = int(readiness)
        return int(readiness)

    def classify_node(self) -> str:
        """
        Classification tiers based on composite metrics.
        """
        health = self.calculate_health_score()
        autonomy = self.calculate_autonomy_readiness()

        if health >= 95 and autonomy >= 80:
            self.classification = "TIER_1_SOVEREIGN"
        elif health >= 85 and autonomy >= 60:
            self.classification = "OPERATIONAL"
        elif health >= 70 and autonomy >= 30:
            self.classification = "DEGRADED"
        elif health >= 50:
            self.classification = "COMPROMISED"
        else:
            self.classification = "CRITICAL"

        return self.classification

    def update_metric(self, metric_name: str, value: float) -> None:
        """Update a single metric."""
        if metric_name in self.metrics:
            self.metrics[metric_name] = value
            self.metrics["last_heartbeat"] = datetime.utcnow().isoformat()

    def add_threat(self, threat_desc: str, severity: str = "low") -> None:
        """Log a detected threat."""
        self.threats.append({
            "description": threat_desc,
            "severity": severity,
            "timestamp": datetime.utcnow().isoformat()
        })

    def add_capability(self, capability: str) -> None:
        """Register a capability."""
        if capability not in self.capabilities:
            self.capabilities.append(capability)

    def to_dict(self) -> Dict:
        """Serialize to dictionary."""
        return {
            "node_id": self.node_id,
            "node_type": self.node_type,
            "classification": self.classify_node(),
            "metrics": self.metrics,
            "threats": self.threats,
            "capabilities": self.capabilities,
            "timestamp": datetime.utcnow().isoformat()
        }

    def to_json(self) -> str:
        """Serialize to JSON."""
        return json.dumps(self.to_dict(), indent=2)


class NodeClusterMetric:
    """
    Cluster-level metrics tracking all three nodes (Alpha/Beta/Delta).
    """

    def __init__(self):
        self.nodes = {}
        self.cluster_health = 100
        self.sync_status = {}

    def register_node(self, node_id: str, node_type: str) -> NodeClassificationMetric:
        """Register a node in the cluster."""
        metric = NodeClassificationMetric(node_id, node_type)
        self.nodes[node_id] = metric
        return metric

    def calculate_cluster_health(self) -> int:
        """Average health across all nodes."""
        if not self.nodes:
            return 0
        total = sum(node.calculate_health_score() for node in self.nodes.values())
        self.cluster_health = int(total / len(self.nodes))
        return self.cluster_health

    def sync_status_check(self) -> Dict[str, str]:
        """Check sync status between nodes."""
        statuses = {}
        if "alpha" in self.nodes and "beta" in self.nodes:
            alpha_health = self.nodes["alpha"].metrics["health"]
            beta_health = self.nodes["beta"].metrics["health"]
            if alpha_health >= 80 and beta_health >= 80:
                statuses["alpha_beta"] = "SYNCED"
            else:
                statuses["alpha_beta"] = "DEGRADED"

        if "beta" in self.nodes and "delta" in self.nodes:
            beta_health = self.nodes["beta"].metrics["health"]
            delta_health = self.nodes["delta"].metrics["health"]
            if beta_health >= 80 and delta_health >= 80:
                statuses["beta_delta"] = "SYNCED"
            else:
                statuses["beta_delta"] = "DEGRADED"

        self.sync_status = statuses
        return statuses

    def report(self) -> Dict:
        """Generate comprehensive cluster report."""
        return {
            "timestamp": datetime.utcnow().isoformat(),
            "cluster_health": self.calculate_cluster_health(),
            "sync_status": self.sync_status_check(),
            "nodes": {node_id: node.to_dict() for node_id, node in self.nodes.items()}
        }

    def report_json(self) -> str:
        """Generate report as JSON."""
        return json.dumps(self.report(), indent=2)


# Example Usage
if __name__ == "__main__":
    # Initialize cluster
    cluster = NodeClusterMetric()

    # Register nodes
    alpha = cluster.register_node("alpha", "logic_paramount")
    beta = cluster.register_node("beta", "persistence_engine")
    delta = cluster.register_node("delta", "command_anchor")

    # Set initial metrics for Alpha (Logic Paramount)
    alpha.update_metric("connectivity", 98)
    alpha.update_metric("security_score", 95)
    alpha.update_metric("performance", 92)
    alpha.update_metric("data_integrity", 100)
    alpha.update_metric("agent_capacity", 12)  # All Tier-1 + polyglots
    alpha.update_metric("ledger_entries", 5432)
    alpha.add_capability("policy_authority")
    alpha.add_capability("sovereign_orchestration")
    alpha.add_capability("acoustic_synthesis")

    # Set metrics for Beta (Persistence Engine)
    beta.update_metric("connectivity", 99)
    beta.update_metric("security_score", 98)
    beta.update_metric("performance", 95)
    beta.update_metric("data_integrity", 99)
    beta.update_metric("agent_capacity", 0)  # No local agents
    beta.update_metric("ledger_entries", 5432)
    beta.add_capability("global_ledger")
    beta.add_capability("firebase_replication")
    beta.add_capability("audio_streaming")

    # Set metrics for Delta (Command Anchor)
    delta.update_metric("connectivity", 100)
    delta.update_metric("security_score", 92)
    delta.update_metric("performance", 85)
    delta.update_metric("data_integrity", 95)
    delta.update_metric("agent_capacity", 3)  # User + Sentinel + control plane
    delta.update_metric("ledger_entries", 42)
    delta.add_capability("user_interface")
    delta.add_capability("local_execution")
    delta.add_capability("command_dispatch")

    # Generate report
    print("=== SARAH SOVEREIGN NODE CLASSIFICATION REPORT ===\n")
    print(cluster.report_json())
