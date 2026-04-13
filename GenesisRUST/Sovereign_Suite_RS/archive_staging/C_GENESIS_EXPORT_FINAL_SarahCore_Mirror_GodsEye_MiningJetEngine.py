"""
GODSEYE MINING JET ENGINE
=========================
Architecture mirrored from GodsEye_Accelerator (JetEngineAccelerator).
Applied to Bitcoin nonce hashing.

  INTAKE FAN   : Stream of (band_id, nonce_start, nonce_end) batches
  COMPRESSOR   : ThreadPoolExecutor â€” all CPU threads pulling batches
  COMBUSTION   : double_sha256 of each nonce in batch
  TURBINE      : HIGH-ZERO HIT â†’ immediately re-queue surrounding
                 nonces with tighter focus (feedback loop)
  EXHAUST      : yield (zeros, nonce, digest, band) on every improvement

The Turbine is the key upgrade over linear scanning.
When a batch returns 4+ zeros, the engine doesn't move on â€”
it circles back to that region and burns tighter.

"We CREATE, never rewrite."
"""

import hashlib
import struct
import os
import concurrent.futures
from collections import deque

GODSEYE_ANCHOR = 1.09277703703

class MiningJetEngine:
    def __init__(self, max_workers=None):
        self.max_workers     = max_workers or (os.cpu_count() * 2)
        self.turbine_queue   = deque()   # high-priority nonce regions
        self.best_zeros      = 0
        self.total_combusted = 0
        self.turbine_hits    = 0

    # â”€â”€ Combustion Chamber â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
    def _combust(self, header80, band_id, nonce_start, count, step=1):
        """
        Burns through `count` nonces starting at nonce_start with given step.
        Returns the best (zeros, nonce, digest) found in this batch.
        """
        best_zeros  = 0
        best_nonce  = nonce_start
        best_digest = ""
        nonce       = nonce_start
        NONCE_MAX   = 0xFFFFFFFF

        for i in range(count):
            n       = (nonce + i * step) & NONCE_MAX
            raw     = hashlib.sha256(hashlib.sha256(
                        header80 + struct.pack("<I", n)
                      ).digest()).hexdigest()
            z = 0
            for c in raw:
                if c == "0": z += 1
                else: break

            if z > best_zeros:
                best_zeros  = z
                best_nonce  = n
                best_digest = raw

        return band_id, best_zeros, best_nonce, best_digest, count

    # â”€â”€ Intake Fan â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
    def _intake(self, header80, bands, batch_size, priority_band):
        """
        Yields (header80, band_id, start, count, step) combustion jobs.
        Priority band gets submitted first.
        Turbine queue insertions also come through here.
        """
        NONCE_MAX = 0xFFFFFFFF
        # Priority band first
        ordered = ([priority_band] +
                   [b for b in range(len(bands)) if b != priority_band])

        for band_id in ordered:
            start, end = bands[band_id]
            span       = end - start + 1
            offset     = 0
            while offset < span:
                yield (header80, band_id,
                       start + offset, min(batch_size, span - offset), 1)
                offset += batch_size

    # â”€â”€ Stream Ignition â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
    def stream_ignition(self, header80, bands, batch_size=8_000,
                        priority_band=16, seed_zones=None):
        """
        Main engine. Yields (band_id, zeros, nonce, digest) as hashes improve.
        Turbine re-queues surrounding regions on high-zero finds.

        seed_zones: list of (band_id, zeros_hint) â€” known hot bands
                    to pre-queue into turbine at startup.
                    e.g. [(14, 6), (25, 7)] from live observations.
        """
        NONCE_MAX      = 0xFFFFFFFF
        self.best_zeros = 0
        self.turbine_hits = 0

        with concurrent.futures.ThreadPoolExecutor(
                max_workers=self.max_workers) as executor:

            futures = {}

            # â”€â”€ PRE-SEED known hot zones â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
            if seed_zones:
                for hot_band, hint_zeros in seed_zones:
                    if 0 <= hot_band < len(bands):
                        start, end = bands[hot_band]
                        mid        = (start + end) // 2
                        radius     = int(NONCE_MAX / (GODSEYE_ANCHOR * 1000))
                        lo = max(start, mid - radius)
                        hi = min(end,   mid + radius)
                        # Submit tight-focus burn on known hot zone
                        for sub_start in range(lo, hi, batch_size):
                            f = executor.submit(
                                self._combust, header80, hot_band,
                                sub_start, min(batch_size, hi - sub_start), 1
                            )
                            futures[f] = (hot_band, sub_start, batch_size)

            # â”€â”€ Submit all intake jobs â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
            for job in self._intake(header80, bands, batch_size, priority_band):
                h80, bid, start, count, step = job
                f = executor.submit(self._combust, h80, bid, start, count, step)
                futures[f] = (bid, start, count)

            for future in concurrent.futures.as_completed(futures):
                bid, best_z, best_n, best_d, combusted = future.result()
                self.total_combusted += combusted

                if best_z > self.best_zeros:
                    self.best_zeros = best_z
                    yield bid, best_z, best_n, best_d

                    # â”€â”€ TURBINE FEEDBACK â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
                    if best_z >= 4:
                        self.turbine_hits += 1
                        anchor_step = int(NONCE_MAX / (GODSEYE_ANCHOR * 1000))
                        for radius in [anchor_step // 4, anchor_step // 2,
                                       anchor_step, anchor_step * 2]:
                            lo = max(0, best_n - radius)
                            hi = min(NONCE_MAX, best_n + radius)
                            if hi > lo:
                                tf = executor.submit(
                                    self._combust, header80, bid, lo,
                                    min(batch_size, hi - lo), 1
                                )
                                futures[tf] = (bid, lo, hi - lo)

    # â”€â”€ Intelligence Brief â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
    def brief(self):
        return {
            "total_combusted": self.total_combusted,
            "best_zeros":      self.best_zeros,
            "turbine_hits":    self.turbine_hits,
            "workers":         self.max_workers,
        }
