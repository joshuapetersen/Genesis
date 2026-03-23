# Node Classification Metric Reference

## Classification Tiers

| Tier | Name | Health | Autonomy | Status | Description |
|---|---|---|---|---|---|
| **1** | `TIER_1_SOVEREIGN` | ≥95 | ≥80 | Optimal | Full autonomy, all systems nominal. Can act independently. |
| **2** | `OPERATIONAL` | ≥85 | ≥60 | Normal | Operational but may require oversight. Limited autonomous decisions. |
| **3** | `DEGRADED` | ≥70 | ≥30 | Caution | Performance reduced. Falls back to cached data/local logic. |
| **4** | `COMPROMISED` | ≥50 | <30 | Alert | Security or integrity issues. Escalate to Sarah Prime. |
| **5** | `CRITICAL` | <50 | N/A | Emergency | Node offline or severely impaired. Do not dispatch tasks. |

---

## Metric Components

### 1. Health Score (0-100)
**Formula:** `(Connectivity + Security + Performance + Data Integrity) / 4`

- **Connectivity** (0-100): Network reachability, response times
- **Security Score** (0-100): Auth status, threat count, vulnerability scan
- **Performance** (0-100): CPU/memory, latency, throughput
- **Data Integrity** (0-100): Corruption rate, sync consistency, ledger validation

### 2. Autonomy Readiness (0-100)
**Formula:** `(Health × 0.4) + (Security × 0.4) + (Agent Capacity × 0.2)`

- Reflects ability to execute tasks without human intervention
- Requires high health + high security + active agents
- Max 12 agents per node cluster

### 3. Connectivity (0-100)
- 100: All pathways (Alpha↔Beta, Beta↔Delta) operational
- 70-99: One pathway degraded, fallback active
- <70: Multiple disconnections, local-only mode

### 4. Security Score (0-100)
- 100: All tokens valid, no threats, firewall active
- 70-99: Minor threat detected, no active breach
- <70: Threat count >3, escalate immediately

### 5. Performance (0-100)
- 100: Latency <10ms, throughput >1000 ops/sec
- 70-99: Latency 10-100ms, degraded throughput
- <70: Latency >100ms or errors >5%

### 6. Data Integrity (0-100)
- 100: All ledger entries valid, no corruption
- 70-99: Minor inconsistencies, auto-healed
- <70: Data loss detected, manual recovery needed

---

## Capability Flags

Each node registers its capabilities for discovery:

### Alpha Node Capabilities
- `policy_authority` — Can issue policies binding on all nodes
- `sovereign_orchestration` — Manages Tier-1 and polyglot agents
- `acoustic_synthesis` — Can generate audio without Beta
- `formal_verification` — Static analysis and proof generation
- `escalation_authority` — Final arbiter on high-risk decisions

### Beta Node Capabilities
- `global_ledger` — Maintains immutable event log
- `firebase_replication` — Syncs to cloud database
- `audio_streaming` — PCM frame transport
- `failover_cache` — Can serve stale data if Alpha down
- `real_time_sync` — WebSocket push notifications

### Delta Node Capabilities
- `user_interface` — Terminal and VS Code integration
- `local_execution` — Runs jobs without network
- `command_dispatch` — Routes user input to agents
- `smoke_tests` — Validates system health locally
- `offline_mode` — Persists state when disconnected

---

## Threat Classifications

| Severity | Threshold | Action | Example |
|---|---|---|---|
| **Low** | 1-2 threats | Monitor | Deprecated API call |
| **Medium** | 3-5 threats | Audit + alert | Token nearing expiration |
| **High** | 6+ threats | Escalate | Unauthorized access attempt |
| **Critical** | Exploit detected | Kill node + alert | SQL injection, breach |

---

## Example Classifications

### Alpha Node (Normal)
```
Health: 96 (Connectivity 98, Security 95, Performance 92, Integrity 100)
Autonomy: 96 (High capacity, 12 agents active)
Classification: TIER_1_SOVEREIGN
Status: Optimal
Threats: None
Capabilities: [policy_authority, sovereign_orchestration, acoustic_synthesis]
```

### Beta Node (Operational)
```
Health: 97 (Connectivity 99, Security 98, Performance 95, Integrity 99)
Autonomy: 78 (No local agents, 0 capacity)
Classification: OPERATIONAL
Status: Normal
Threats: None
Capabilities: [global_ledger, firebase_replication, audio_streaming]
```

### Delta Node (Degraded)
```
Health: 75 (Connectivity 60, Security 82, Performance 70, Integrity 88)
Autonomy: 55 (Limited agents, network issues)
Classification: DEGRADED
Status: Caution
Threats: [{description: "Network latency >500ms", severity: "medium"}]
Capabilities: [user_interface, local_execution, offline_mode]
```

---

## Monitoring & Alerting

### Heartbeat Intervals
- Alpha → Beta: Every 60s
- Beta → Delta: Every 30s
- Delta → Alpha: Every 120s

### Automatic Escalations
- Health drop >10 points → Alert user
- Autonomy <50 → Disable autonomous dispatch
- Security breach → Lockdown; escalate to Sarah Prime
- Sync desync >60s → Activate fallback ledger

### Health Recovery Steps
1. Log issue to ledger
2. Attempt reconnect (exponential backoff)
3. Activate local cache
4. Notify Sarah Prime if critical

---

## Usage in Control Plane

```python
# Check if agent can operate autonomously
if cluster.nodes["alpha"].classify_node() == "TIER_1_SOVEREIGN":
    # Axiom can dispatch Quark without approval
    dispatch_agent("quark", high_risk_task)
else:
    # Escalate to Sarah Prime
    escalate_to_user("Autonomy degraded; manual approval required")
```

---

## Future Extensions

- **ML-based anomaly detection** for predictive health
- **Geographic distribution** metrics (multi-region Firebase)
- **Agent-level metrics** (per Tier-1/polyglot scoring)
- **Ledger compression** (archival of old entries)
