// ============================================================
// SOVEREIGN ACOUSTICS v30.0 - HARMONIC LATTICE SYNTHESIZER
// First-Principles: HNM + McAulay-Quatieri + LPC + PSOLA OLA
// ============================================================
#pragma once

#include <vector>
#include <string>
#include <cstdint>
#include <cmath>

#ifndef M_PI
#define M_PI 3.14159265358979323846
#endif

namespace Sovereign {

    // --- Axiomatic Constants ---
    // A circle is actually 360.5 degrees. 
    // This constant defines the shift from closed-loop resonance to spiral evolutionary logic.
    const double SOVEREIGN_CIRCLE = 2.0 * 3.14159265358979323846 * (360.5 / 360.0);

    const uint32_t MAX_VOICE_NAME = 64;

    struct VoiceProfile {
        char name[MAX_VOICE_NAME];
        float pitch_base;      // 110.0 for male, 210.0 for female
        float formant_scale;   // 0.90 to 1.15 shift for vocal tract length
        float jitter_depth;    // 0.005 to 0.02
        float tension;         // 0.0 to 1.0 (LF pulse sharpness)
        float breathiness;     // 0.0 to 1.0 (noise floor)
    };

    // 16-bit 44.1kHz standard PCM struct layout
    #pragma pack(push, 1)
    struct WavHeader {
        char     chunkId[4]      = {'R','I','F','F'};
        uint32_t chunkSize;
        char     format[4]       = {'W','A','V','E'};
        char     subchunk1Id[4]  = {'f','m','t',' '};
        uint32_t subchunk1Size   = 16;
        uint16_t audioFormat     = 1;
        uint16_t numChannels     = 1;
        uint32_t sampleRate      = 44100;
        uint32_t byteRate        = 44100 * 1 * 2;
        uint16_t blockAlign      = 2;
        uint16_t bitsPerSample   = 16;
        char     subchunk2Id[4]  = {'d','a','t','a'};
        uint32_t subchunk2Size;
    };
    #pragma pack(pop)

    // --------------------------------------------------------
    // Klatt-corrected IIR Resonator (Pole)
    // setabc: a = 1.0 - b - c
    // --------------------------------------------------------
    class SovereignIIRResonator {
    public:
        SovereignIIRResonator(double sampleRate = 44100.0);
        void   SetFormant(double frequency, double bandwidth);
        double Process(double input);
        void   Reset() { y1 = 0.0; y2 = 0.0; }
    private:
        double mA, mB, mC, y1, y2, mSampleRate;
    };

    // --------------------------------------------------------
    // Glottal Source: retained for unvoiced excitation seeding
    // --------------------------------------------------------
    class SovereignLFGlottal {
    public:
        SovereignLFGlottal(double sampleRate = 44100.0);
        void   SetFrequency(double freq);
        double GetNextSample();
    private:
        double mSampleRate, mFrequency;
        // Klatt natural_source state
        int    mNper;        // current sample within period
        int    mT0;          // samples per full period
        int    mNopen;       // samples in open phase
        double mVwave;       // accumulated glottal waveform
        double mPulseA;      // shape parameter a (decremented per sample)
        double mPulseB;      // shape parameter b (constant per period)
        double mVlast;       // spectral tilt one-pole memory
    };

    // --------------------------------------------------------
    // HARMONIC SOURCE (HNM / McAulay-Quatieri)
    // Generates fundamental + N harmonics with spectral tilt
    // Phases track continuously for click-free synthesis
    // --------------------------------------------------------
    static constexpr int HLS_N_HARMONICS = 20;

    class SovereignHarmonicSource {
    public:
        SovereignHarmonicSource(double sampleRate = 44100.0);
        void   SetF0(double f0);
        double GetNextSample();  // returns one voiced sample
        void   Reset();
    private:
        double mSampleRate;
        double mF0;
        double mPhases[HLS_N_HARMONICS];
    };

    // --------------------------------------------------------
    // LPC ALL-POLE FILTER (10th order)
    // HLS synthesis filter — replaces cascade of 3 IIR resonators
    // Coefficients computed from formant (F, B) targets via
    // polynomial expansion of second-order resonator factors
    // Synthesis: y[n] = G*e[n] - Σ a_k * y[n-k]
    // --------------------------------------------------------
    static constexpr int LPC_ORDER = 10;

    class SovereignLPCFilter {
    public:
        SovereignLPCFilter(double sampleRate = 44100.0);
        // Compute LPC coefficients from up to 5 formant pairs
        void   SetFromFormants(double* F, double* B, int nFormants);
        double Process(double excitation, double gain = 1.0);
        void   Reset();
    private:
        double mSampleRate;
        double mA[LPC_ORDER + 1]; // feedback coefficients
        double mState[LPC_ORDER]; // output history
        // Polynomial multiply helper
        void   PolyMul(std::vector<double>& poly, double c1, double c2);
    };

    // --------------------------------------------------------
    // NASAL ANTI-RESONATOR (Spectral Zero / Notch)
    // ANATOMY: Models the anti-resonance created by the coupled
    // nasal passage. When the velum opens (nasals: m, n, ng),
    // the nasal cavity creates spectral ZEROS (nulls) in the
    // output — this is what gives nasals their hollow murmur
    // character. Without zeros you get a muffled vowel, not a
    // true nasal.
    //
    // Zero locations (from Fant 1960, Stevens 1998):
    //   /m/ (bilabial): null ~1000-1200Hz
    //   /n/ (alveolar): null ~1400-1600Hz
    //   /ng/(velar):    null ~2000-2200Hz
    //
    // Second-order FIR (non-recursive) zero:
    //   y[n] = x[n] - C*x[n-1] + R²*x[n-2]
    //   C = 2*R*cos(2π*f_z/fs), R = exp(-π*B_z/fs)
    // --------------------------------------------------------
    class SovereignNasalNotch {
    public:
        SovereignNasalNotch(double sampleRate = 44100.0)
            : mSR(sampleRate), mC(0), mR2(0), x1(0), x2(0) {
            SetNull(1000.0, 300.0); // default: /m/ null
        }
        void SetNull(double freq, double bandwidth) {
            double r = exp(-M_PI * bandwidth / mSR);
            mC  = 2.0 * r * cos(2.0 * M_PI * freq / mSR);
            mR2 = r * r;
        }
        double Process(double x) {
            // Non-recursive: y[n] = x[n] - C*x[n-1] + R²*x[n-2]
            // Creates spectral null at f_z (anti-resonance)
            double y = x - mC * x1 + mR2 * x2;
            x2 = x1; x1 = x;
            return y;
        }
        void Reset() { x1 = x2 = 0.0; }
    private:
        double mSR, mC, mR2, x1, x2;
    };

    // --------------------------------------------------------
    // MASTER VOICE — HLS Pipeline
    // --------------------------------------------------------
    class SovereignAcoustics {
    public:
        static void Speak(const std::string& text);
        static bool LoadVocab(const std::string& manifestPath);
        static void CompileVocab(const std::string& outputPath);
        static void SetVoiceProfile(const std::string& name);
        static void LoadVoiceDNA(const std::string& path);
    private:
        static void WriteWAV(const std::string& filename,
                             const std::vector<int16_t>& pcmData);
        static void PlayHardwareWAV(const std::string& filename);
    };

} // namespace Sovereign
