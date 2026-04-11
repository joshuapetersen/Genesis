// ============================================================
// SOVEREIGN VOCABULARY COMPILER SPECIFICATION
// First-Principles Binary Atom Manifest
// ============================================================
#pragma once

#include <cstdint>
#include <vector>
#include <string>

namespace Sovereign {

    // A single frame of acoustic resonance (Calculated from 57D Geometry + Klatt v2)
    #pragma pack(push, 1)
    struct ResonanceFrame {
        float f1, f2, f3;    // Formant Frequencies (Hz)
        float bw1, bw2, bw3; // Bandwidths (Hz)
        float intensity;     // Volume/Amplitude
        float voicing;       // Mixed excitation ratio: 1.0 = pure glottal, 0.0 = pure noise
        float f0;            // Fundamental frequency (Hz) — Klatt v2 pitch trajectory
        float a_noise;       // Aspiration noise amplitude [0..1] — Klatt v2 mixed excitation
    };

    // A Diphone Atom: The mathematical transition from one sound to another.
    // Length: Constant 25 frames (approx 150ms at 166Hz update rate)
    struct DiphoneAtom {
        char key[8];         // Unique identifier (e.g., "s_o")
        uint32_t id;         // Numeric hash for fast lookup
        ResonanceFrame frames[32]; // Baked trajectory buffer
    };

    // The Master Vocabulary Manifest Header
    struct VocabHeader {
        uint32_t magic = 0x534F5658; // "SOVX" (Sovereign Vocab Extended v2, Klatt)
        uint32_t version = 2;
        uint32_t atomCount;
        uint64_t dataOffset; // Offset to start of DiphoneAtom array
    };
    #pragma pack(pop)

} // namespace Sovereign
