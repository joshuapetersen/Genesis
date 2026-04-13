import json
import uuid
from datetime import datetime
from typing import Dict, List, Optional, Callable
from enum import Enum


class TaskStatus(Enum):
    PENDING = "pending"
    RUNNING = "running"
    COMPLETED = "completed"
    FAILED = "failed"
    ESCALATED = "escalated"


class Task:
    """Atomic task unit for agent execution."""

    def __init__(self, agent_id: str, operation: str, priority: int = 5, risk_level: str = "low"):
        self.task_id = str(uuid.uuid4())
        self.agent_id = agent_id
        self.operation = operation
        self.priority = priority  # 1-10, 10 highest
        self.risk_level = risk_level
        self.status = TaskStatus.PENDING
        self.created_at = datetime.utcnow().isoformat()
        self.started_at = None
        self.completed_at = None
        self.result = None
        self.error = None

    def to_dict(self) -> Dict:
        return {
            "task_id": self.task_id,
            "agent_id": self.agent_id,
            "operation": self.operation,
            "priority": self.priority,
            "risk_level": self.risk_level,
            "status": self.status.value,
            "created_at": self.created_at,
            "started_at": self.started_at,
            "completed_at": self.completed_at,
            "result": self.result,
            "error": self.error,
        }


class AutonomousAgent:
    """Autonomous agent with task dispatch and execution."""

    def __init__(self, agent_id: str, pillar: str, supervisor: str, languages: List[str]):
        self.agent_id = agent_id
        self.pillar = pillar
        self.supervisor = supervisor
        self.languages = languages
        self.task_queue: List[Task] = []
        self.completed_tasks: List[Task] = []
        self.performance_score = 0.5  # 0-1, starts neutral
        self.tasks_completed = 0
        self.tasks_failed = 0
        self.autonomy_level = 0.3  # 0-1, start low
        self.last_action = None
        self.is_active = False

    def enqueue_task(self, task: Task) -> str:
        """Add task to queue."""
        self.task_queue.append(task)
        self.task_queue.sort(key=lambda t: t.priority, reverse=True)
        return task.task_id

    def execute_next_task(self) -> Optional[Task]:
        """Pop and execute highest-priority task."""
        if not self.task_queue:
            return None

        task = self.task_queue.pop(0)
        task.status = TaskStatus.RUNNING
        task.started_at = datetime.utcnow().isoformat()

        # Simulate execution based on operation type
        success = self._execute_operation(task)

        task.completed_at = datetime.utcnow().isoformat()
        if success:
            task.status = TaskStatus.COMPLETED
            task.result = f"[{self.agent_id}] {task.operation} completed"
            self.tasks_completed += 1
            self.performance_score = min(1.0, self.performance_score + 0.05)
        else:
            task.status = TaskStatus.FAILED
            task.error = f"[{self.agent_id}] {task.operation} failed"
            self.tasks_failed += 1
            self.performance_score = max(0.0, self.performance_score - 0.1)

        # Update autonomy level based on performance
        self._update_autonomy()

        self.completed_tasks.append(task)
        self.last_action = task.task_id
        return task

    def _execute_operation(self, task: Task) -> bool:
        """Simulate operation execution."""
        # Stub implementation; in production, dispatch to real handlers
        operations_available = {
            "analyze": True,
            "draft": True,
            "patch": True,
            "design": True,
            "plan": True,
            "deploy": task.risk_level != "high",  # High-risk fails without approval
            "read": True,
            "optimize": True,
        }
        return operations_available.get(task.operation, False)

    def _update_autonomy(self) -> None:
        """Increase autonomy based on performance and task completion."""
        if self.tasks_completed > 5:
            self.autonomy_level = min(1.0, self.performance_score * 0.8 + 0.2)

    def get_status(self) -> Dict:
        """Return agent status."""
        return {
            "agent_id": self.agent_id,
            "pillar": self.pillar,
            "supervisor": self.supervisor,
            "performance_score": round(self.performance_score, 2),
            "autonomy_level": round(self.autonomy_level, 2),
            "tasks_completed": self.tasks_completed,
            "tasks_failed": self.tasks_failed,
            "queue_size": len(self.task_queue),
            "is_active": self.is_active,
            "last_action": self.last_action,
        }


class Tier1Sovereign:
    """Tier-1 Sovereign supervisor overseeing polyglot agents."""

    def __init__(self, sovereign_id: str, pillar: str, supervised_agents: List[str]):
        self.sovereign_id = sovereign_id
        self.pillar = pillar
        self.supervised_agents = supervised_agents
        self.agents: Dict[str, AutonomousAgent] = {}
        self.decisions_made = 0
        self.escalations = 0
        self.authority_level = 0.9  # High initial authority

    def register_agent(self, agent: AutonomousAgent) -> None:
        """Register an agent under this supervisor."""
        self.agents[agent.agent_id] = agent

    def dispatch_task(self, agent_id: str, task: Task) -> str:
        """Dispatch task to supervised agent."""
        if agent_id not in self.agents:
            return f"Error: Agent {agent_id} not found under {self.sovereign_id}"

        agent = self.agents[agent_id]
        task_id = agent.enqueue_task(task)
        self.decisions_made += 1
        return f"Task {task_id} queued for {agent_id}"

    def run_cycle(self) -> List[Task]:
        """Execute one cycle: each agent processes one task."""
        completed = []
        for agent in self.agents.values():
            agent.is_active = True
            result = agent.execute_next_task()
            if result:
                completed.append(result)
            agent.is_active = False
        return completed

    def get_oversight_report(self) -> Dict:
        """Generate supervisory report."""
        agent_statuses = [agent.get_status() for agent in self.agents.values()]
        cluster_performance = sum(a["performance_score"] for a in agent_statuses) / len(
            agent_statuses
        ) if agent_statuses else 0

        return {
            "sovereign_id": self.sovereign_id,
            "pillar": self.pillar,
            "authority_level": self.authority_level,
            "decisions_made": self.decisions_made,
            "escalations": self.escalations,
            "cluster_performance": round(cluster_performance, 2),
            "agents": agent_statuses,
        }


class SovereignCoordinator:
    """Coordinates all Tier-1 Sovereigns."""

    def __init__(self):
        self.sovereigns: Dict[str, Tier1Sovereign] = {}
        self.all_agents: Dict[str, AutonomousAgent] = {}
        self.coordination_cycles = 0
        self.total_tasks_executed = 0

    def register_sovereign(self, sovereign: Tier1Sovereign) -> None:
        """Register a Tier-1 Sovereign."""
        self.sovereigns[sovereign.sovereign_id] = sovereign

    def register_agent(self, agent: AutonomousAgent, sovereign_id: str) -> None:
        """Register agent under a sovereign."""
        self.all_agents[agent.agent_id] = agent
        if sovereign_id in self.sovereigns:
            self.sovereigns[sovereign_id].register_agent(agent)

    def run_coordination_cycle(self) -> Dict:
        """Execute one full coordination cycle across all sovereigns."""
        results = {}
        for sovereign_id, sovereign in self.sovereigns.items():
            completed = sovereign.run_cycle()
            results[sovereign_id] = [t.to_dict() for t in completed]
            self.total_tasks_executed += len(completed)

        self.coordination_cycles += 1
        return results

    def get_cluster_status(self) -> Dict:
        """Generate full cluster status."""
        sovereign_reports = {sid: sov.get_oversight_report() for sid, sov in self.sovereigns.items()}

        return {
            "timestamp": datetime.utcnow().isoformat(),
            "coordination_cycles": self.coordination_cycles,
            "total_tasks_executed": self.total_tasks_executed,
            "sovereigns": sovereign_reports,
        }


# Example: Initialize and run
if __name__ == "__main__":
    # Create Tier-1 Sovereigns
    axiom = Tier1Sovereign("axiom", "science_technology", ["quark", "forge", "scribe"])
    vigil = Tier1Sovereign("vigil", "medical_biology", ["aegis", "helix", "pulse"])
    atlas = Tier1Sovereign("atlas", "social_info_econ", ["lattice", "strata", "chorus"])

    # Create Polyglot Agents
    agents_config = [
        ("quark", "science_technology", "axiom", ["c++", "cuda"]),
        ("forge", "synthetic_hardware", "axiom", ["rust", "c"]),
        ("scribe", "science_technology", "axiom", ["lean", "coq"]),
        ("aegis", "medical_biology", "vigil", ["python", "r"]),
        ("helix", "medical_biology", "vigil", ["python", "sbml"]),
        ("pulse", "medical_biology", "vigil", ["python", "julia"]),
        ("lattice", "social_info", "atlas", ["python", "typescript"]),
        ("strata", "economics", "atlas", ["go", "python"]),
        ("chorus", "audio_music", "atlas", ["python", "typescript"]),
    ]

    coordinator = SovereignCoordinator()
    coordinator.register_sovereign(axiom)
    coordinator.register_sovereign(vigil)
    coordinator.register_sovereign(atlas)

    for agent_id, pillar, supervisor, langs in agents_config:
        agent = AutonomousAgent(agent_id, pillar, supervisor, langs)
        coordinator.register_agent(agent, supervisor)

    # Simulate some work
    print("=== AUTONOMOUS AGENT EVOLUTION TEST ===\n")

    for cycle in range(3):
        print(f"[CYCLE {cycle + 1}]")

        # Queue tasks
        for agent_id in ["quark", "forge", "aegis", "lattice"]:
            task = Task(agent_id, "analyze", priority=7 + cycle, risk_level="low")
            coordinator.all_agents[agent_id].enqueue_task(task)

        # Run coordination cycle
        results = coordinator.run_coordination_cycle()

        # Print results
        for sovereign_id, tasks in results.items():
            for task_dict in tasks:
                status_icon = "✓" if task_dict["status"] == "completed" else "✗"
                print(
                    f"  {status_icon} {task_dict['agent_id']}: {task_dict['operation']} -> {task_dict['status']}"
                )

        print()

    # Final report
    print("=== FINAL CLUSTER STATUS ===\n")
    print(json.dumps(coordinator.get_cluster_status(), indent=2))
