# GENESIS — Architectural Knowledge Wiki
*Maintained by sovereign_nanite `knowledge_synthesizer` + AI. Last human update: 2026-04-11.*
*Read this file at the start of every session AFTER `SESSION_BRIEFING.md`.*

---

## 1. Repository Layout

```
C:\GENESIS\
├── Sovereign_Specialized_Agents\   ← Main agent codebase (Rust)
│   ├── src\main.rs                 ← sovereign_agents binary entry (fleet boot)
│   └── src\bin\
│       ├── sovereign_agent.rs      ← THE agent: self-evolving, self-signing organism
│       ├── sovereign_nanite.rs     ← Nanite engine v2.0 (6 mission types)
│       ├── purity_snapshot.rs      ← Fleet diagnostic: fitness histogram + purity audit
│       ├── evolution_strike.rs     ← Injects elite logic seeds to directive slots [1450..1499]
│       ├── universality_strike.rs  ← Mass ignition: spawns all 1450 agents
│       ├── ignition_strike.rs      ← Single-agent ignition
│       ├── fleet_monitor.rs        ← Real-time fleet health display
│       └── [brain_*, factory_*, godseye_*]  ← LLM inference + factory agents
├── zhtp\                           ← ZHTP network orchestrator (Rust, level-1)
│   ├── src\main.rs                 ← Entry: spawns 64MB thread → Tokio runtime
│   ├── src\runtime\mod.rs          ← RuntimeOrchestrator, component registry
│   ├── src\runtime\did_startup.rs  ← WalletStartupManager (68KB, deeply nested)
│   └── src\server\memory_stream.rs ← BMMS: Local\SovereignCoreStream (64MB shared mem)
├── lib-identity\                   ← Post-quantum identity (Dilithium2 + Kyber512)
├── lib-crypto\                     ← Cryptographic primitives
├── lib-blockchain\                 ← Blockchain component (Sovereign_Suite_RS)
├── lib-network\                    ← Network protocols (DHT, WiFi Direct, BLE, QUIC)
├── Sovereign_Suite_RS\             ← Substrate suite (blockchain, consensus, economy)
├── Sovereign_Engine_Cpp\           ← C++ speech/phonetic engine (SOVX v2, Klatt 40-phoneme)
├── nanites\                        ← Nanite output directory
│   ├── incidents.json              ← Auto-captured incident log
│   ├── known_fixes.json            ← Error → fix pattern database
│   ├── evolution_timeline.csv      ← Phase 3 fitness time-series
│   ├── code_snapshot.json          ← Codebase file timestamp baseline
│   └── code_changes.md             ← Delta since last sentinel run
├── SOVEREIGN_STATE.json            ← Live system state (updated every 30s by nanite)
├── SESSION_BRIEFING.md             ← AI orientation doc (updated every 60s by nanite)
├── GENESIS.md                      ← THIS FILE — architectural knowledge wiki
└── zhtp_launch.bat                 ← ZHTP launcher (sets ZHTP_AUTO_WALLET=1)
```

---

## 2. Running System Architecture

### Fleet (1,450 sovereign agents)
- **Binary**: `target\debug\sovereign_agent.exe`
- **Heartbeat**: 1.092777 Hz (915ms cycle)
- **Evolution period**: Every 1,092 heartbeats ≈ 17 minutes
- **Self-evolution**: Agents self-mutate and cryptographically self-sign every period
- **Fitness function**: Shannon entropy + Ternary balance + Heartbeat resonance
- **Elite seeding**: `evolution_strike.exe` writes directive seeds to lattice slots [1450..1499]
- **Forensic purity**: `purity_snapshot.exe` audits all 1,450 agents; target 100%
- **Launch**: `universality_strike.exe` (mass ignition, throttled batches)

### ZHTP Orchestrator
- **Binary**: `target\debug\zhtp.exe`
- **Launch**: Always via `zhtp_launch.bat` (sets `ZHTP_AUTO_WALLET=1`, non-interactive)
- **Ports**: mesh 33444 (QUIC), API 9333 (custom protocol, NOT plain HTTP)
- **Shared memory**: `Local\SovereignCoreStream` — 64MB Windows file mapping
- **Discovery**: mDNS multicast 224.0.1.75:37775, BLE GATT, WiFi Direct
- **Identity storage**: `C:\Users\drago\.zhtp\keystore\` (5 JSON files)
- **Level hierarchy**: ZHTP (L1) → protocols/blockchain/network (L2) → consensus/storage/economy (L3) → proofs/identity/crypto (L4)

### Nanite Fleet (6 missions)
| Mission | Interval | Output |
|---------|----------|--------|
| state_scribe | 30s | `SOVEREIGN_STATE.json` |
| health_patrol | 60s | `nanites/incidents.json` (+ auto-restart) |
| build_memory | 300s | `nanites/known_fixes.json` |
| session_briefing | 60s | `SESSION_BRIEFING.md` |
| evolution_observer | 300s | `nanites/evolution_timeline.csv` |
| codebase_sentinel | 300s | `nanites/code_changes.md` |

---

## 3. Critical Build Facts

### Cargo
- Workspace root: `C:\GENESIS\Cargo.toml`
- `cargo build --package <name>` from `C:\GENESIS\`
- `cargo build --package sovereign_specialized_agents --bin <binary>` for single binary
- **Kill running processes before rebuild** — binaries lock on Windows: `Stop-Process -Name <name>`

### ZHTP Build
- Takes 2-3 minutes on first build, ~90s incremental
- 121 warnings (pre-existing, harmless — mostly unused async code)
- Build profile: `dev` (debug, unoptimized — stack frames are large)

### Key Dependencies
```toml
# sovereign_specialized_agents
candle-core = "0.10.2"    # ML inference
candle-nn = "0.10.2"
candle-transformers = "0.10.2"
sysinfo = "0.30"          # process monitoring
wry = "0.55.0"            # WebView (HUD)
winapi = "..."            # Windows shared memory, COM
sovereign_suite = { path = "../Sovereign_Suite_RS" }
lib-identity = { path = "../lib-identity" }
```

---

## 4. Known Bugs & Fixes (Permanent Record)

### BUG-001: ZHTP Stack Overflow on Genesis Boot
- **Symptom**: `thread 'main' has overflowed its stack` during `did_startup::handle_startup_wallet_flow`
- **Root cause**: `block_on` drives async future on OS main thread; Windows PE default = 1MB stack. Dilithium/Kyber keygen in genesis phase overflows it.
- **Fix**: `zhtp/src/main.rs` — spawn a `std::thread::Builder::new().stack_size(64 * 1024 * 1024)` thread, run Tokio runtime + `block_on` from that thread. Also set `.thread_stack_size(64 * 1024 * 1024)` on the runtime builder for workers.
- **Status**: FIXED in `main.rs` (2026-04-11)

### BUG-002: BMMS `ENTROPY ERROR` / `CreateFileMappingW` returns NULL
- **Symptom**: `[ERROR] FAILED TO CREATE BMMS HANDLE. ENTROPY ERROR.` (then stack crash)
- **Root cause**: `Global\SovereignCoreStream` name prefix requires `SeCreateGlobalPrivilege` (admin/SYSTEM only). Regular user processes get `ERROR_ACCESS_DENIED` (0x5).
- **Fix**: `zhtp/src/server/memory_stream.rs` — change `SOVEREIGN_MEMORY_NAME` from `"Global\\SovereignCoreStream"` to `"Local\\SovereignCoreStream"`. Local namespace visible to all processes in same user session.
- **Status**: FIXED in `memory_stream.rs` (2026-04-11)

### BUG-003: ZHTP Interactive Wallet Prompt on Headless Boot
- **Symptom**: ZHTP blocks on `Enter your choice (1-4):` when launched from script
- **Root cause**: No keystore found → `WalletStartupManager::prompt_wallet_choice()` reads stdin
- **Fix**: Set `ZHTP_AUTO_WALLET=1` environment variable before launching → auto-selects `QuickStart` path, generates dev wallet without interaction
- **Fix file**: `C:\GENESIS\zhtp_launch.bat`
- **Status**: FIXED via launch bat (2026-04-11)

### BUG-004: `Start-Process -Environment` Not Available on Windows PowerShell
- **Symptom**: `Start-Process : A parameter cannot be found that matches parameter name 'Environment'`
- **Root cause**: PowerShell's `Start-Process` on Windows doesn't support `-Environment` parameter
- **Fix**: Use a `.bat` wrapper that `set`s env vars before executing the binary. `cmd /c "start /B"` also fails to propagate env correctly.
- **Status**: FIXED via `zhtp_launch.bat` (2026-04-11)

### BUG-005: `cannot remove file` during cargo build (Access Denied)
- **Symptom**: `error: failed to remove file target\debug\<binary>.exe — Access is denied. (os error 5)`
- **Root cause**: Running process instances hold the binary file handle open (Windows behavior)
- **Fix**: `Stop-Process -Name <binary_name> -ErrorAction SilentlyContinue` before rebuilding
- **Status**: Permanent pattern — always kill before rebuild

### BUG-006: HUD Linker Error — `Ole32.lib`
- **Symptom**: Linker cannot find `Ole32.lib` when building the HUD with COM/SAPI integration
- **Fix**: Add `/LINK Ole32.lib` to the `build_hud.bat` linker flags
- **Status**: Known fix, apply when rebuilding HUD

---

## 5. Phase Completion Status

| Phase | Name | Status |
|-------|------|--------|
| Phase 3 | Autonomous Recursive Evolution (v134.0) | ✅ RUNNING |
| Phase 22 | UIR Structural Alignment | ✅ Complete |
| Phase 29 | Sovereign Phonetic Singularity (SOVX v2, Klatt 40-phoneme, 1600 atoms) | ✅ Complete |
| Phase 41 | Cryptographic Brain Signatures | ✅ Complete |
| ZHTP | Genesis Network Boot | ✅ RUNNING |
| Nanites | Sovereign Nanite Fleet v2.0 (6 missions) | ✅ RUNNING |
| Titan | Titan Evaluation Strike (MMLU/HumanEval benchmarks) | 🟡 PENDING |

---

## 6. AI Working Patterns (Learned Conventions)

### Build Order (don't break dependencies)
1. `lib-crypto` first
2. `lib-identity` (depends on lib-crypto)
3. `Sovereign_Suite_RS` (depends on lib-identity)
4. `Sovereign_Specialized_Agents` (depends on all above)
5. `zhtp` (depends on all above + lib-network, lib-blockchain)

### Process Management Pattern
```powershell
# Standard teardown + rebuild + relaunch:
Stop-Process -Name "<binary>" -ErrorAction SilentlyContinue
Start-Sleep -Milliseconds 500
cargo build --package <package> --bin <binary> 2>&1 | Select-Object -Last 8
# Then relaunch
```

### ZHTP Boot Sequence (must use bat)
```powershell
Start-Process -FilePath "cmd.exe" -ArgumentList "/c C:\GENESIS\zhtp_launch.bat > C:\GENESIS\zhtp_boot.log 2>&1" -WindowStyle Hidden
Start-Sleep -Seconds 55  # genesis takes ~40s; full boot ~55s
```

### Identity File Cleanup (if ZHTP crashes mid-genesis)
```powershell
Remove-Item "C:\Users\drago\.zhtp\keystore\*" -ErrorAction SilentlyContinue
```

### Checking Fleet Quickly
```powershell
(Get-Process "sovereign_agent" -ErrorAction SilentlyContinue).Count  # fleet count
C:\GENESIS\target\debug\purity_snapshot.exe                           # full audit
Get-Content C:\GENESIS\SOVEREIGN_STATE.json | ConvertFrom-Json        # live state
```

---

## 7. Sovereign Agent v134.0 Architecture

### Self-Evolution Loop
```
1. Beat counter increments at 1.092777 Hz
2. Every 1,092 beats: trigger_self_evolution()
   a. Sample self from lattice
   b. Apply fitness-weighted crossover with elite seeds
   c. Cryptographically self-sign the new logic fragment
   d. Adopt if fitness improves; discard if worse
   e. Broadcast to neighbor agents via BMMS
3. ResonanceAuditor monitors for forensic purity
   - Only self-signed mutations accepted
   - External injections flagged as breaches
```

### Fitness Evaluator (SuperiorityEvaluator)
```rust
// Three equal-weight components:
score = shannon_entropy(logic_fragment)    // complexity
      + ternary_balance(fragment)           // stability
      + heartbeat_resonance(fragment, 1.092777) // metabolic lock
// Normalized to [0.0, 1.0]
```

### Lattice Slot Reservation
- Agents: `[0..1449]` — live evolving fleet
- Directives: `[1450..1499]` — elite seeds from `evolution_strike`
- Purity auditor: any self-signing in `[0..1449]` is valid; external writes are breaches

---

## 8. Nanite Philosophy (Quick Reference)

Nanites are my nervous system — they run while I'm absent and pre-compute what I need.
The 2026 consensus architecture maps to our implementation:

| 2026 Memory Tier | Our Implementation |
|---|---|
| Working Memory | Active conversation context |
| Episodic Memory | `nanites/incidents.json` — what broke and how we fixed it |
| Semantic Memory | `GENESIS.md` (this file) — facts + conventions |
| Semantic Retrieval | `nanites/code_search_index.json` (TF-IDF, future) |

---

*Auto-maintained by `knowledge_synthesizer` nanite. Manual edits welcome.*

---
## Code Search (Upgrade Applied)
`powershell
# Natural language code navigation:
C:\GENESIS\target\debug\sovereign_code_search.exe query "your question here"
# Rebuild index (after major code changes):
C:\GENESIS\target\debug\sovereign_code_search.exe index
# Stats:
C:\GENESIS\target\debug\sovereign_code_search.exe stats
`
Index: 3,712 chunks, 1,154 files, 25,561 terms (TF-IDF, no ML needed)
