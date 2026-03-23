# Sarah Sovereign Communication Protocol
## Inter-Node Message Specification (Alpha/Beta/Delta)

---

## 1. Protocol Layers

### Transport Layer
- **Alpha ↔ Beta**: Firebase Realtime Database REST API + WebSocket streams
- **Beta ↔ Delta**: REST/WebSocket (local polling or event subscription)
- **Delta ↔ Alpha**: Authenticated HTTP/WebSocket via Delta → Beta → Alpha chain
- **Fallback**: Local filesystem sync (`04_THE_MEMORY/`) when network unavailable

### Authentication Layer
- **Per-Agent Tokens**: Environment variables (e.g., `AXIOM_TOKEN`, `NAVIGATOR_TOKEN`)
- **Bearer Token Format**: `Authorization: Bearer <token>`
- **Token Validation**: Control plane gate via `agent_control_plane.py`
- **Escalation**: High-risk ops require dual-signature (agent + Sarah Prime)

### Message Format (JSON)
```json
{
  "message_id": "uuid",
  "timestamp": "2025-12-26T00:00:00Z",
  "source_node": "delta|alpha|beta",
  "source_agent": "sentinel|navigator|executor|[tier1]|[polyglot]",
  "target_node": "alpha|beta|delta",
  "target_agent": "sarah-prime|[any_agent]",
  "message_type": "command|query|response|event|escalation",
  "operation": "read|write|analyze|dispatch|execute",
  "risk_level": "low|medium|high|critical",
  "payload": {
    "content": "...",
    "context": {...},
    "args": {...}
  },
  "auth": {
    "token": "...",
    "signature": "hmac-sha256(payload)",
    "ttl_seconds": 3600
  },
  "metadata": {
    "version": "1.0",
    "encoding": "utf-8",
    "compression": "none|gzip"
  }
}
```

---

## 2. Node Communication Channels

### Alpha Node (Logic Paramount)
- **Inbound**: Queries from Beta/Delta, approved directives
- **Outbound**: Policy updates, manifests, escalation decisions
- **Storage**: Canonical files (`05_THE_CORE/`), GitHub
- **Rate Limits**: 10 req/sec per agent

### Beta Node (Persistence Engine)
- **Inbound**: Manifest syncs from Alpha, state updates from Delta
- **Outbound**: Cached manifests to Delta, event logs
- **Storage**: Firebase Realtime DB (`/beta_node/`)
- **Rate Limits**: 50 req/sec (per project)

### Delta Node (Command Anchor)
- **Inbound**: User commands, queries to Alpha/Beta
- **Outbound**: Execution results, logs
- **Storage**: Local filesystem (`04_THE_MEMORY/`)
- **Rate Limits**: Unlimited (local context)

---

## 3. Message Types & Flows

### Command Flow (User → Alpha)
```
Delta (user input)
  → [Validate via local control plane]
  → Beta (route)
  → [Rate limit check]
  → Alpha (execute)
  → [Log to ledger]
  → Beta (persist)
  → Delta (return result)
```

### Query Flow (Agent → Alpha)
```
[Any Agent] (question)
  → [Sign request]
  → Beta (forward if remote)
  → Alpha (retrieve)
  → [Cache in Beta]
  → [Return to requestor]
```

### Escalation Flow (High-Risk Op)
```
[Agent] (risky operation)
  → [Mark as escalation]
  → Sarah Prime (manual approval)
  → [If approved] → Execute
  → [If denied] → Log and abort
```

### Event Broadcast (State Change)
```
Alpha (publishes policy update)
  → Beta (stores in `/beta_node/events/`)
  → Delta (polls and caches locally)
  → [All agents notified]
```

---

## 4. Error Handling & Retry Logic

### Status Codes
- `200`: Success
- `400`: Bad request (auth fail, malformed message)
- `403`: Forbidden (operation denied by policy)
- `429`: Rate limited
- `500`: Server error (retry with exponential backoff)
- `503`: Service unavailable (fallback to local cache)

### Retry Strategy
- **Exponential Backoff**: 1s, 2s, 4s, 8s, 16s (max 5 retries)
- **Circuit Breaker**: After 3 failures, fallback to local for 60s
- **Fallback Order**: Alpha → Beta → Delta (local filesystem)

### Timeout Thresholds
- **Command execution**: 30s
- **Query response**: 5s
- **Escalation approval**: 300s (manual)
- **Ledger sync**: 10s

---

## 5. Data Serialization & Compression

### Supported Formats
- **JSON** (default): Human-readable, debug-friendly
- **MessagePack** (optional): Binary, 40% smaller
- **Protocol Buffers** (for high-volume audio): Structured, fast

### Compression
- **GZip**: For payloads > 10KB
- **Streaming**: PCM audio frames sent uncompressed (real-time constraint)

---

## 6. Security & Integrity

### Signature Verification
- HMAC-SHA256 on entire payload
- Private key stored in `serviceAccountKey.json` (Firebase SA)
- Public key available to all nodes for verification

### Token Lifecycle
- **Generation**: `push_to_all_nodes.py` or manual setup
- **Rotation**: Recommended every 30 days
- **Revocation**: Remove from env vars; Delta broadcasts invalidation

### Audit Trail
- Every message logged to `genesis_master_ledger.jsonl`
- Immutable append-only ledger
- Timestamp + source + operation + result

---

## 7. Connection State Machine

```
DISCONNECTED
  ↓ [attempt connect]
CONNECTING
  ↓ [auth success]
AUTHENTICATED
  ↓ [healthy heartbeat]
READY (operational)
  ↓ [timeout or error]
DEGRADED (fallback to cached)
  ↓ [exceed retry threshold]
DISCONNECTED (retry later)
```

---

## 8. Heartbeat & Health Check

### Heartbeat Interval
- **Alpha → Beta**: Every 60s
- **Beta → Delta**: Every 30s (on demand)
- **Delta → Alpha**: Every 120s (when idle)

### Health Check Payload
```json
{
  "node": "alpha|beta|delta",
  "status": "healthy|degraded|error",
  "uptime_seconds": 12345,
  "agents_active": 12,
  "ledger_entries": 5432,
  "database_latency_ms": 45,
  "timestamp": "2025-12-26T00:00:00Z"
}
```

---

## 9. Example: Full Message Exchange

### Scenario: Executor Agent Deploys Code

**1. Delta receives user command:**
```json
{
  "message_id": "msg-001",
  "timestamp": "2025-12-26T10:30:00Z",
  "source_node": "delta",
  "source_agent": "executor",
  "target_node": "alpha",
  "target_agent": "executor",
  "message_type": "command",
  "operation": "deploy",
  "risk_level": "high",
  "payload": {
    "content": "Deploy audio_core to production",
    "args": {"target": "prod", "version": "1.0.0"}
  },
  "auth": {"token": "EXECUTOR_TOKEN_...", "ttl_seconds": 3600},
  "metadata": {"version": "1.0", "encoding": "utf-8"}
}
```

**2. Delta validates locally:**
- ✓ Token matches `EXECUTOR_TOKEN` env var
- ✓ Operation "deploy" in allowlist
- ✓ Risk level "high" requires escalation

**3. Delta routes to Alpha (via Beta):**
- Adds escalation flag
- Sends to Beta REST endpoint
- Beta queues for Alpha

**4. Alpha receives, validates, escalates:**
- ✓ Signature verified
- ✓ Message format valid
- ✗ Risk = high → requires Sarah Prime approval
- Sends escalation prompt to user

**5. User approves via Delta terminal:**
- Delta sends approval message with `approved_by: "user"` signature
- Alpha processes deployment
- Logs all steps to ledger

**6. Response back to Delta:**
```json
{
  "message_id": "msg-001-resp",
  "source_node": "alpha",
  "target_node": "delta",
  "message_type": "response",
  "status": "success",
  "payload": {"result": "Deployed to prod. Ledger entry: LE-12345"},
  "timestamp": "2025-12-26T10:31:15Z"
}
```

**7. All nodes updated:**
- Alpha: Canonical record stored
- Beta: Synced to Firebase ledger
- Delta: Local cache updated

---

## 10. Protocol Version & Compatibility

- **Current Version**: 1.0
- **Backward Compatibility**: Messages with older version stamps logged as warnings but still processed
- **Version Negotiation**: On first connect, nodes exchange `min_version` and `max_version`

---

## Conclusion

This protocol ensures:
- **High-signal communication** (no ambiguity)
- **Full auditability** (every message logged)
- **Graceful degradation** (fallback to local when network fails)
- **Security** (auth, signatures, rate limits)
- **Consistency** (ledger-driven state)
