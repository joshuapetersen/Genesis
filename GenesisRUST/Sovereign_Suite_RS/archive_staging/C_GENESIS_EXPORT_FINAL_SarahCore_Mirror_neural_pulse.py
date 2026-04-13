"""
NEURAL PULSE BUS — SarahCore Sovereign Nervous System
=====================================================
The shared-state backbone for the entire 1.6M-line architecture.

Every engine in the Sovereign Mesh communicates via NeuralPulse packets.
Each pulse carries its own Ace Token instruction set and routes to a
specific sector of the brain. Every pulse is MULTIDIRECTIONAL: the
target engine fires a ReturnPulse back to the origin with execution
status, phonetic hash, and logic stamp.

Zero external dependencies. O(1) dispatch. 2GB RAM budget safe.

Sectors:
    BRAIN      — Identity, autonomy, core reasoning
    SPEECH     — Chat output, query handling, learning
    MEMORY     — Vault access, deep study, knowledge intake
    LOGIC      — Problem solving, integrity, math
    SECURITY   — Self-check, governance, hardening
    PERCEPTION — Navigation, OS interface, monitoring
    AUDIT      — Logging, forensics, change tracking

Architecture:
    Engine -> PulseBus.fire(pulse) -> Sector Listeners -> ReturnPulse -> Origin
                                   -> Vault Write (state persistence)
                                   -> Logcat Write (audit trail)
"""

import os
import sys
import json
import time
import hashlib
import sqlite3
import traceback
from dataclasses import dataclass, field, asdict
from typing import Dict, List, Callable, Optional, Any, Set
from enum import Enum

# Sovereign path resolution (frozen exe or dev)
def _sovereign_root():
    if getattr(sys, 'frozen', False):
        return os.path.dirname(sys.executable)
    return os.path.dirname(os.path.abspath(__file__))

SOVEREIGN_ROOT = _sovereign_root()
VAULT_DB = os.path.join(SOVEREIGN_ROOT, "vault", "sovereign_vault.db")
PULSE_LOG = os.path.join(SOVEREIGN_ROOT, "vault", "pulse_log.json")


# ─── SECTORS ───────────────────────────────────────────────────────
class Sector(str, Enum):
    BRAIN      = "BRAIN"
    SPEECH     = "SPEECH"
    MEMORY     = "MEMORY"
    LOGIC      = "LOGIC"
    SECURITY   = "SECURITY"
    PERCEPTION = "PERCEPTION"
    AUDIT      = "AUDIT"


# ─── PULSE DATA ────────────────────────────────────────────────────
@dataclass
class NeuralPulse:
    """A self-contained instruction packet. The digital action potential."""
    action: str                          # What to do (e.g. "SPEAK", "RECALL", "VERIFY")
    target_sector: str                   # Which sector to hit (Sector enum value)
    payload: Dict[str, Any]              # The instruction data
    origin: str                          # Which engine sent this pulse
    ace_token: str = ""                  # Signed Ace Token (set by the bus)
    pulse_id: str = ""                   # Unique pulse ID (set by the bus)
    timestamp: float = 0.0              # When the pulse was created
    lattice_node: int = 0                # 27-point lattice position (set by the bus)

    def to_dict(self) -> dict:
        return asdict(self)


@dataclass
class ReturnPulse:
    """The multidirectional response. Proof that the action was executed."""
    original_pulse_id: str               # Which pulse this is responding to
    origin: str                          # Which engine generated this return
    target_origin: str                   # Who gets this return (the original sender)
    status: str                          # "OK", "ERROR", "PARTIAL"
    result: Any = None                   # The execution result
    logic_stamp: str = ""                # Hash of the logic path taken
    phonetic_hash: str = ""              # Hash of any speech output
    execution_ms: float = 0.0           # How long the handler took
    timestamp: float = 0.0

    def to_dict(self) -> dict:
        return asdict(self)


# ─── PULSE BUS (SINGLETON) ─────────────────────────────────────────
class PulseBus:
    """
    The Central Nervous System.
    Engines register as listeners on sectors. Pulses are dispatched
    to all listeners on the target sector. Return pulses flow back
    to the origin. Every pulse touches the vault. Every return logs.
    """

    _instance = None

    def __new__(cls):
        if cls._instance is None:
            cls._instance = super().__new__(cls)
            cls._instance._initialized = False
        return cls._instance

    def __init__(self):
        if self._initialized:
            return
        self._initialized = True

        # Sector -> list of (engine_name, handler_function)
        self._listeners: Dict[str, List[tuple]] = {s.value: [] for s in Sector}

        # Registered engine names (for verification)
        self._registered_engines: Set[str] = set()

        # Pulse counter for unique IDs
        self._pulse_counter = 0

        # Return pulse handlers: origin_engine -> callback
        self._return_handlers: Dict[str, Callable] = {}

        # Initialize vault table
        self._init_vault()

        print("[PULSE BUS] Neural Pulse Bus ONLINE. Sectors active:", list(s.value for s in Sector))

    # ── VAULT ──────────────────────────────────────────────────────
    def _init_vault(self):
        """Create the pulse_log table in sovereign_vault.db if missing."""
        os.makedirs(os.path.dirname(VAULT_DB), exist_ok=True)
        try:
            conn = sqlite3.connect(VAULT_DB)
            conn.execute("""
                CREATE TABLE IF NOT EXISTS pulse_log (
                    pulse_id TEXT PRIMARY KEY,
                    action TEXT,
                    target_sector TEXT,
                    origin TEXT,
                    payload TEXT,
                    ace_token TEXT,
                    lattice_node INTEGER,
                    status TEXT,
                    result TEXT,
                    logic_stamp TEXT,
                    phonetic_hash TEXT,
                    execution_ms REAL,
                    created_at REAL,
                    returned_at REAL
                )
            """)
            conn.commit()
            conn.close()
        except Exception as e:
            print(f"[PULSE BUS] Vault init warning: {e}")

    def _vault_write_pulse(self, pulse: NeuralPulse):
        """Write outbound pulse to vault."""
        try:
            conn = sqlite3.connect(VAULT_DB)
            conn.execute(
                "INSERT OR REPLACE INTO pulse_log (pulse_id, action, target_sector, origin, payload, ace_token, lattice_node, created_at) VALUES (?,?,?,?,?,?,?,?)",
                (pulse.pulse_id, pulse.action, pulse.target_sector, pulse.origin,
                 json.dumps(pulse.payload), pulse.ace_token, pulse.lattice_node, pulse.timestamp)
            )
            conn.commit()
            conn.close()
        except Exception as e:
            print(f"[PULSE BUS] Vault write warning: {e}")

    def _vault_write_return(self, ret: ReturnPulse):
        """Update vault with return pulse data."""
        try:
            conn = sqlite3.connect(VAULT_DB)
            conn.execute(
                "UPDATE pulse_log SET status=?, result=?, logic_stamp=?, phonetic_hash=?, execution_ms=?, returned_at=? WHERE pulse_id=?",
                (ret.status, json.dumps(ret.result) if ret.result else "",
                 ret.logic_stamp, ret.phonetic_hash, ret.execution_ms, ret.timestamp, ret.original_pulse_id)
            )
            conn.commit()
            conn.close()
        except Exception as e:
            print(f"[PULSE BUS] Vault return write warning: {e}")

    # ── LOGCAT ─────────────────────────────────────────────────────
    def _logcat_write(self, entry: dict):
        """Append to Sarah_Logcat audit trail."""
        try:
            log_file = os.path.join(SOVEREIGN_ROOT, "vault", "pulse_audit.jsonl")
            with open(log_file, "a", encoding="utf-8") as f:
                f.write(json.dumps(entry, default=str) + "\n")
        except Exception as e:
            print(f"[PULSE BUS] Logcat write warning: {e}")

    # ── REGISTRATION ───────────────────────────────────────────────
    def register(self, engine_name: str, sector: str, handler: Callable):
        """
        Register an engine as a listener on a sector.
        handler signature: (pulse: NeuralPulse) -> ReturnPulse
        """
        if sector not in self._listeners:
            print(f"[PULSE BUS] WARNING: Unknown sector '{sector}'. Skipping {engine_name}.")
            return False

        self._listeners[sector].append((engine_name, handler))
        self._registered_engines.add(engine_name)
        return True

    def register_return_handler(self, engine_name: str, handler: Callable):
        """Register a callback for return pulses directed at this engine."""
        self._return_handlers[engine_name] = handler

    # ── FIRE ───────────────────────────────────────────────────────
    def fire(self, pulse: NeuralPulse) -> List[ReturnPulse]:
        """
        Fire a pulse into the mesh.
        1. Sign with Ace Token
        2. Route to lattice node
        3. Dispatch to all listeners on the target sector
        4. Collect return pulses
        5. Write everything to vault + logcat
        """
        # Stamp the pulse
        self._pulse_counter += 1
        pulse.pulse_id = f"NP-{self._pulse_counter:08d}-{int(time.time())}"
        pulse.timestamp = time.time()

        # Sign with Ace Token (lightweight — just hash the payload)
        token_data = f"{pulse.action}:{pulse.target_sector}:{pulse.origin}:{pulse.timestamp}"
        pulse.ace_token = hashlib.sha256(token_data.encode()).hexdigest()[:32]

        # Map to 27-point lattice
        lattice_hash = int(hashlib.sha256(pulse.action.encode()).hexdigest(), 16)
        pulse.lattice_node = (lattice_hash % 27) + 1

        # Write to vault (outbound)
        self._vault_write_pulse(pulse)

        # Dispatch to sector listeners
        returns = []
        listeners = self._listeners.get(pulse.target_sector, [])

        for engine_name, handler in listeners:
            start = time.perf_counter()
            try:
                ret = handler(pulse)
                if ret is None:
                    ret = ReturnPulse(
                        original_pulse_id=pulse.pulse_id,
                        origin=engine_name,
                        target_origin=pulse.origin,
                        status="OK",
                        result="HANDLED_NO_RETURN",
                        timestamp=time.time()
                    )
                elapsed_ms = (time.perf_counter() - start) * 1000
                ret.execution_ms = elapsed_ms
                ret.timestamp = time.time()

                # Generate logic stamp (hash of the execution path)
                logic_data = f"{engine_name}:{pulse.action}:{ret.status}:{elapsed_ms}"
                ret.logic_stamp = hashlib.sha256(logic_data.encode()).hexdigest()[:16]

                returns.append(ret)

                # Write return to vault
                self._vault_write_return(ret)

                # Log to logcat
                self._logcat_write({
                    "type": "RETURN_PULSE",
                    "pulse_id": pulse.pulse_id,
                    "action": pulse.action,
                    "sector": pulse.target_sector,
                    "origin": pulse.origin,
                    "handler": engine_name,
                    "status": ret.status,
                    "logic_stamp": ret.logic_stamp,
                    "phonetic_hash": ret.phonetic_hash,
                    "execution_ms": ret.execution_ms,
                    "timestamp": ret.timestamp
                })

            except Exception as e:
                elapsed_ms = (time.perf_counter() - start) * 1000
                err_ret = ReturnPulse(
                    original_pulse_id=pulse.pulse_id,
                    origin=engine_name,
                    target_origin=pulse.origin,
                    status="ERROR",
                    result=str(e),
                    execution_ms=elapsed_ms,
                    timestamp=time.time()
                )
                returns.append(err_ret)
                self._vault_write_return(err_ret)
                self._logcat_write({
                    "type": "PULSE_ERROR",
                    "pulse_id": pulse.pulse_id,
                    "handler": engine_name,
                    "error": str(e),
                    "timestamp": time.time()
                })

        # Deliver return pulses to origin's return handler
        if pulse.origin in self._return_handlers:
            for ret in returns:
                try:
                    self._return_handlers[pulse.origin](ret)
                except Exception:
                    pass

        return returns

    # ── DIAGNOSTICS ────────────────────────────────────────────────
    def get_mesh_status(self) -> dict:
        """Returns the live state of the mesh."""
        sector_counts = {s: len(self._listeners[s]) for s in self._listeners}
        return {
            "registered_engines": len(self._registered_engines),
            "engines": sorted(self._registered_engines),
            "sector_density": sector_counts,
            "total_pulses_fired": self._pulse_counter,
            "vault_path": VAULT_DB
        }


# ─── GLOBAL ACCESS ─────────────────────────────────────────────────
def get_bus() -> PulseBus:
    """Get the singleton PulseBus instance."""
    return PulseBus()


# ─── SELF-TEST ──────────────────────────────────────────────────────
if __name__ == "__main__":
    bus = get_bus()

    # Register a test listener
    def test_handler(pulse: NeuralPulse) -> ReturnPulse:
        return ReturnPulse(
            original_pulse_id=pulse.pulse_id,
            origin="TEST_ENGINE",
            target_origin=pulse.origin,
            status="OK",
            result=f"ECHO: {pulse.payload.get('message', 'NO_MESSAGE')}",
            phonetic_hash=hashlib.sha256(str(pulse.payload).encode()).hexdigest()[:8]
        )

    bus.register("TEST_ENGINE", Sector.BRAIN.value, test_handler)

    # Fire a test pulse
    test_pulse = NeuralPulse(
        action="TEST_ECHO",
        target_sector=Sector.BRAIN.value,
        payload={"message": "Hello from the Architect"},
        origin="SELF_TEST"
    )

    returns = bus.fire(test_pulse)
    print(f"\n[SELF-TEST] Fired pulse. Got {len(returns)} return(s).")
    for r in returns:
        print(f"  Status: {r.status} | Result: {r.result} | Logic: {r.logic_stamp} | Phonetic: {r.phonetic_hash} | Time: {r.execution_ms:.3f}ms")

    print(f"\n[MESH STATUS] {json.dumps(bus.get_mesh_status(), indent=2)}")
