// ============================================================
// SOVEREIGN ACOUSTICS v30.0 - HARMONIC LATTICE SYNTHESIZER
// Custom Architecture: PSOLA OLA + HNM Harmonics + LPC Filter
// Derived from: Klatt, MBROLA, HNM, WORLD, McAulay-Quatieri, LPC
// ============================================================
#include "Sovereign_Acoustics.h"
#include "Sovereign_Vocab.h"
#include "Sovereign_Lexicon.h"
#include "Sovereign_G2P.h"
#include "Sovereign_Vocab_Compiler.cpp"

#include <fstream>
#include <cmath>
#include <iostream>
#define NOMINMAX
#include <windows.h>
#include <algorithm>
#include <map>
#include <random>
#include <sstream>
#include <vector>
#include <numeric>

#pragma comment(lib, "winmm.lib")
#pragma comment(lib, "ole32.lib")

#include <sapi.h>
#pragma comment(lib, "sapi.lib")

namespace Sovereign {

    // --------------------------------------------------------
    // RNG
    // --------------------------------------------------------
    static std::mt19937 g_RNG(1092777);
    static std::uniform_real_distribution<double> g_Dist(-1.0, 1.0);

    // --------------------------------------------------------
    // VOICE PROFILE REGISTRY
    // --------------------------------------------------------
    static std::map<std::string, VoiceProfile> g_Profiles;
    static VoiceProfile g_ActiveProfile = { "Sovereign", 110.0f, 1.0f, 0.007f, 0.4f, 0.1f };

    void SovereignAcoustics::SetVoiceProfile(const std::string& name) {
        if (g_Profiles.empty()) {
            g_Profiles["Sovereign"]   = { "Sovereign",   110.0f, 1.0f,  0.007f, 0.4f, 0.1f };
            g_Profiles["NaturalMale"] = { "NaturalMale", 102.0f, 1.05f, 0.015f, 0.2f, 0.3f };
            g_Profiles["NaturalFemale"] = { "NaturalFemale", 205.0f, 1.15f, 0.018f, 0.1f, 0.5f };
        }
        auto it = g_Profiles.find(name);
        if (it != g_Profiles.end()) {
            g_ActiveProfile = it->second;
        }
    }

    void SovereignAcoustics::LoadVoiceDNA(const std::string& path) {
        // Placeholder for JSON/Binary profile loading in future strike
    }

    // --------------------------------------------------------
    // IIR RESONATOR — Klatt setabc (preserved for LPC coeff debug)
    // --------------------------------------------------------
    SovereignIIRResonator::SovereignIIRResonator(double sampleRate)
        : mSampleRate(sampleRate), mA(0), mB(0), mC(0), y1(0), y2(0) {
        SetFormant(500.0, 200.0);
    }

    void SovereignIIRResonator::SetFormant(double frequency, double bandwidth) {
        if (frequency < 20.0) frequency = 20.0;
        double r = exp(-M_PI * bandwidth / mSampleRate);
        mB = -(r * r);
        mC = r * 2.0 * cos(2.0 * M_PI * frequency / mSampleRate);
        mA = 1.0 - mC - mB;
    }

    double SovereignIIRResonator::Process(double input) {
        double output = mA * input + mC * y1 + mB * y2;
        y2 = y1; y1 = output;
        return output;
    }

    // --------------------------------------------------------
    // GLOTTAL SOURCE — Rosenberg pulse (retained for unvoiced seeding)
    // --------------------------------------------------------
    SovereignLFGlottal::SovereignLFGlottal(double sampleRate)
        : mSampleRate(sampleRate), mFrequency(110.0),
          mNper(0), mT0(0), mNopen(0),
          mVwave(0.0), mPulseA(0.0), mPulseB(0.0), mVlast(0.0) {}

    void SovereignLFGlottal::SetFrequency(double freq) {
        if (freq < 50.0)  freq = 50.0;
        if (freq > 400.0) freq = 400.0;
        mFrequency = freq;
    }

    double SovereignLFGlottal::GetNextSample() {
        // ═══════════════════════════════════════════════════════
        // KLATT natural_source() — exact algorithm from parwave.c
        // Iles/Ing-Simmons 1993 C re-implementation of Klatt 1980
        //
        // dUg/dt = vwave[n] = a0*n - b/2*n^2  (parabola)
        // b = 1920000/nopen^2  → constant peak amplitude vs F0
        // a0 = b*nopen*0.333   → linear decrement each sample
        //
        // Shape: + small peak at nopen/3 (opening)
        //        − large trough at nopen  (CLOSURE = main excitation)
        // Asymmetry: closure spike ≈ 2× opening spike → spectral tilt
        // ═══════════════════════════════════════════════════════

        // Pitch period boundary: reset shape parameters
        if (mNper >= mT0 || mT0 == 0) {
            // Profile-aware Jitter for naturalistic period variation
            double jit = 1.0 + g_Dist(g_RNG) * g_ActiveProfile.jitter_depth;
            mT0    = std::max(4, (int)(mSampleRate / mFrequency * jit));

            // Open quotient = 0.65
            mNopen = std::max(10, (int)(0.65 * mT0));

            // Klatt B0 formula: b = 1920000 / nopen^2
            mPulseB = 1920000.0 / ((double)mNopen * mNopen);
            mPulseA = mPulseB * mNopen * 0.333;  // initial a

            mNper  = 0;
            mVwave = 0.0;
        }

        double output = 0.0;
        if (mNper < mNopen) {
            mPulseA -= mPulseB;     // decrement a each sample
            mVwave  += mPulseA;     // accumulate: vwave = a*n - b/2*n^2
            output   = mVwave * 0.00006;
        } else {
            mVwave = 0.0;           // closed phase: silence + reset
        }
        mNper++;

        // Spectral tilt: one-pole LP (TLTdb≈10 → decay=0.33)
        // Attenuates high harmonics → natural vocal source slope
        double tilted = output * 0.67 + mVlast * 0.33;
        mVlast = output;
        return tilted;
    }

    // --------------------------------------------------------
    // HARMONIC SOURCE — HNM + McAulay-Quatieri
    // Generates voiced excitation as sum of tracked harmonics.
    // Each harmonic k has freq k*F0, amplitude 1/k^0.8 (glottal tilt).
    // Phases persist across samples for continuous synthesis.
    // --------------------------------------------------------
    SovereignHarmonicSource::SovereignHarmonicSource(double sampleRate)
        : mSampleRate(sampleRate), mF0(110.0) {
        for (int i = 0; i < HLS_N_HARMONICS; i++) mPhases[i] = 0.0;
    }

    void SovereignHarmonicSource::SetF0(double f0) {
        if (f0 < 50.0) f0 = 50.0;
        mF0 = f0;
    }

    void SovereignHarmonicSource::Reset() {
        for (int i = 0; i < HLS_N_HARMONICS; i++) mPhases[i] = 0.0;
    }

    double SovereignHarmonicSource::GetNextSample() {
        double out   = 0.0;
        double Nyq   = mSampleRate * 0.5;
        // Shimmer: per-period amplitude variation
        static double shimmerCoeff = 1.0;
        static long   shimmerCount = 0;
        if (++shimmerCount > (long)(mSampleRate / mF0)) {
            shimmerCount  = 0;
            shimmerCoeff  = 1.0 + g_Dist(g_RNG) * 0.015;
        }
        for (int k = 1; k <= HLS_N_HARMONICS; k++) {
            double freq = (double)k * mF0;
            if (freq >= Nyq) break;
            // Glottal spectral tilt: -12dB/oct (Klatt 1980: 1/k^2.0)
            // Previous: 1/k^1.0 caused harsh sawtooth-like buzz.
            // 1.8 gives natural warm bass-male timbre.
            double Ak = shimmerCoeff / pow((double)k, 1.8);
            // Jitter on fundamental only
            double jitter = (k == 1) ? (1.0 + g_Dist(g_RNG) * 0.007) : 1.0;
            mPhases[k - 1] += 2.0 * M_PI * freq * jitter / mSampleRate;
            if (mPhases[k - 1] >= 2.0 * M_PI) mPhases[k - 1] -= 2.0 * M_PI;
            out += Ak * sin(mPhases[k - 1]);
        }
        return out;
    }

    // --------------------------------------------------------
    // LPC ALL-POLE FILTER — 10th order
    // SetFromFormants computes coefficients via polynomial expansion.
    // Each formant (F_i, B_i) contributes a 2nd-order factor:
    //   (1 + c1_i * z^-1 + c2_i * z^-2)
    //   c1_i = -2 * r_i * cos(2π*F_i/sr)
    //   c2_i =  r_i^2,   r_i = exp(-π*B_i/sr)
    // Product of all factors = denominator polynomial A(z).
    // Synthesis: y[n] = G*e[n] - Σ_{k=1}^{order} A[k] * y[n-k]
    // --------------------------------------------------------
    SovereignLPCFilter::SovereignLPCFilter(double sampleRate)
        : mSampleRate(sampleRate) {
        for (int i = 0; i <= LPC_ORDER; i++) mA[i] = 0.0;
        mA[0] = 1.0;
        for (int i = 0; i < LPC_ORDER; i++) mState[i] = 0.0;
    }

    void SovereignLPCFilter::PolyMul(std::vector<double>& poly, double c1, double c2) {
        std::vector<double> res(poly.size() + 2, 0.0);
        for (size_t j = 0; j < poly.size(); j++) {
            res[j]   += poly[j];
            res[j+1] += poly[j] * c1;
            res[j+2] += poly[j] * c2;
        }
        poly = res;
    }

    void SovereignLPCFilter::SetFromFormants(double* F, double* B, int nFormants) {
        std::vector<double> poly = {1.0};
        int orderUsed = 0;
        for (int i = 0; i < nFormants && orderUsed < LPC_ORDER; i++) {
            double fi = F[i], bi = B[i];
            if (fi < 20.0) fi = 20.0;
            if (bi < 20.0) bi = 20.0;
            double r  = exp(-M_PI * bi / mSampleRate);
            double c1 = -2.0 * r * cos(2.0 * M_PI * fi / mSampleRate);
            double c2 = r * r;
            PolyMul(poly, c1, c2);
            orderUsed += 2;
        }
        // Copy polynomial coefficients into mA (skip leading 1)
        for (int k = 0; k <= LPC_ORDER; k++) mA[k] = 0.0;
        for (size_t k = 0; k < poly.size() && k <= (size_t)LPC_ORDER; k++) {
            mA[k] = poly[k];
        }
    }

    double SovereignLPCFilter::Process(double excitation, double gain) {
        // y[n] = G*e[n] - sum(mA[k]*y[n-k]) for k=1..order
        double y = gain * excitation;
        for (int k = 0; k < LPC_ORDER; k++) {
            y -= mA[k + 1] * mState[k];
        }
        // Clip to prevent instability runaway
        if (y > 100.0) y = 100.0;
        if (y < -100.0) y = -100.0;
        // Shift state
        for (int k = LPC_ORDER - 1; k > 0; k--) mState[k] = mState[k-1];
        mState[0] = y;
        return y;
    }

    void SovereignLPCFilter::Reset() {
        for (int i = 0; i < LPC_ORDER; i++) mState[i] = 0.0;
    }

    // --------------------------------------------------------
    // SPECTRAL NOISE SHAPER for unvoiced consonants
    // --------------------------------------------------------
    class SovereignSpectralFilter {
    public:
        SovereignSpectralFilter(double sr) : mSR(sr), y1(0), y1_bp(0), y2_bp(0) {}

        double HPF(double x, double cutoff) {
            double alpha = 1.0 - exp(-2.0 * M_PI * cutoff / mSR);
            double out = x - y1;
            y1 = y1 + alpha * out;
            return out;
        }
        double BPF(double x, double center, double bw) {
            double R = exp(-M_PI * bw / mSR);
            double C = 2.0 * R * cos(2.0 * M_PI * center / mSR);
            double out = (1.0 - R) * x + C * y1_bp - R * R * y2_bp;
            y2_bp = y1_bp; y1_bp = out;
            return out;
        }
    private:
        double mSR, y1, y1_bp, y2_bp;
    };

    // --------------------------------------------------------
    // VOCAB CACHE
    // --------------------------------------------------------
    static std::map<uint32_t, DiphoneAtom> g_VocabCache;
    static bool g_VocabLoaded = false;

    bool SovereignAcoustics::LoadVocab(const std::string& path) {
        std::ifstream file(path, std::ios::binary);
        if (!file) return false;
        VocabHeader header;
        file.read(reinterpret_cast<char*>(&header), sizeof(VocabHeader));
        // Accept both SOVV (v1) and SOVX (v2) formats for backward compat
        if (header.magic != 0x534F5656 && header.magic != 0x534F5658) return false;
        std::vector<DiphoneAtom> atoms(header.atomCount);
        file.read(reinterpret_cast<char*>(atoms.data()), header.atomCount * sizeof(DiphoneAtom));
        for (auto& a : atoms) g_VocabCache[a.id] = a;
        g_VocabLoaded = true;
        return true;
    }

    void SovereignAcoustics::CompileVocab(const std::string& outputPath) {
        VocabCompiler::Compile(outputPath);
    }

    // ────────────────────────────────────────────────────────────
    // SAPI5 TTS — Primary voice path (dramatically clearer than
    // Klatt formant synthesis). Prefers OneCore neural voices if
    // installed (Aria, Jenny, Guy); falls back to system default
    // (David, Zira, etc.). The custom formant pipeline below
    // remains as a fallback if COM/SAPI5 is unavailable.
    // ────────────────────────────────────────────────────────────
    static bool SpeakViaSAPI5(const std::string& text) {
        // Init COM safely (RPC_E_CHANGED_MODE = already up, still proceed)
        HRESULT hr = ::CoInitializeEx(NULL, COINIT_APARTMENTTHREADED);
        bool com_inited = (hr == S_OK);

        ISpVoice* pVoice = nullptr;
        hr = ::CoCreateInstance(CLSID_SpVoice, NULL, CLSCTX_ALL,
                                IID_ISpVoice, (void**)&pVoice);
        if (FAILED(hr) || !pVoice) {
            if (com_inited) ::CoUninitialize();
            return false;
        }

        // Use whatever voice Windows has configured (David, Zira, Aria, etc.)
        // Rate=-2: slightly slower than default — significantly improves clarity.
        pVoice->SetRate(-2);
        pVoice->SetVolume(100);

        std::wstring wtext(text.begin(), text.end());
        hr = pVoice->Speak(wtext.c_str(), SPF_DEFAULT, NULL);  // blocking
        pVoice->Release();
        if (com_inited) ::CoUninitialize();
        return SUCCEEDED(hr);
    }

    // ────────────────────────────────────────────────────────────
    // HLS SPEAK — Primary: SAPI5. Fallback: Harmonic Lattice DSP.
    // ────────────────────────────────────────────────────────────
    void SovereignAcoustics::Speak(const std::string& text) {
        if (text.empty()) return;

        // ── Primary: SAPI5 TTS (clear, natural, zero-latency) ──
        // Falls through to formant synthesizer only if SAPI fails.
        if (SpeakViaSAPI5(text)) return;

        // ── Fallback: Custom Klatt Formant Synthesizer ──────────

        // ── G2P: Full English text → phoneme sequence ──────────
        // Uses Sovereign_G2P.h for any English text (500+ words
        // in exception dict + context-sensitive rules for the rest).
        // ANATOMY: This is the "brain motor command" layer —
        // the neural signal that tells each articulator what
        // position to take for each phoneme.
        std::vector<char> phones = SovereignG2P::TextToPhonemes(text);

        // ── DSP OBJECTS ─────────────────────────────────────────
        // Staying on IIR cascade: LPC ±100 clip saturated, tanh gain crushed to square wave.
        // IIR (2nd-order per resonator) is self-normalizing and stable.
        // Keeping: bass-baritone formants, VirtualSleeve BW, HarmonicSource excitation.
        SovereignHarmonicSource harmSrc(44100.0);
        SovereignLFGlottal      glottal(44100.0);
        SovereignSpectralFilter sFilter(44100.0);

        SovereignIIRResonator r1(44100.0), r2(44100.0), r3(44100.0);
        SovereignIIRResonator r4(44100.0), r5(44100.0), rN(44100.0);
        r4.SetFormant(3300.0, 200.0);
        r5.SetFormant(3750.0, 300.0);
        rN.SetFormant(280.0,  200.0);

        // NASAL ANTI-RESONATOR (spectral zero)
        // ANATOMY: The nasal passage + coupled oral cavity creates
        // spectral zeros — anti-resonances that give nasals their
        // distinctive hollow murmur. Without these zeros, /m/ and
        // /n/ sound like muffled vowels instead of true nasals.
        // Fant (1960): nasal zero ~1000-2000Hz depending on place.
        SovereignNasalNotch nasalNotch(44100.0);

        std::vector<double> floatBuffer;

        double time     = 0.0;
        double totalDur = (double)phones.size() * 0.12;
        double F0       = 100.0; // Bass-baritone fundamental

        // LP at 5kHz = essentially transparent; radiation filter handles tilt
        double lpState  = 0.0;
        double lpAlpha  = 1.0 - exp(-2.0 * M_PI * 5000.0 / 44100.0);
        // Phoneme canonical formant targets — BASS MALE (Peterson-Barney 1952 bass group)
        // /a/ = 680Hz (NOT 550 which is /ɔ/ caught, NOT 730 which is avg male)
        // Bass male: Peterson-Barney lowest-quartile group, Titze 1994 adjustment.
        struct PhFormant { double f1, f2, f3, v; };
        auto GetPF = [](char p) -> PhFormant {
            switch(p) {
                // Vowels (bass male)
                case 'a': return {680, 1050, 2400, 1.0}; // AA (father)
                case 'e': return {500, 1700, 2480, 1.0}; // EH (pet)
                case 'i': return {250, 2100, 2900, 1.0}; // IY (heed)
                case 'o': return {490,  720, 2300, 1.0}; // AO (caught)
                case 'u': return {270,  700, 2300, 1.0}; // UW (boot)
                case 'n': return {250,  900, 2100, 1.0}; // N nasal
                case 'm': return {250,  900, 2200, 1.0}; // M nasal
                case 'r': return {450, 1250, 1700, 1.0}; // R
                case 'l': return {360, 1000, 2600, 1.0}; // L
                case 'v': return {380, 1050, 2200, 0.7}; // V
                case 'z': return {240, 1000, 2300, 0.7}; // Z
                case 's': return {280, 1500, 5000, 0.0}; // S sibilant
                case 'f': return {280, 1500, 3800, 0.0}; // F fricative
                case 'k': return {270, 1980, 2800, 0.0}; // K stop
                case 't': return {280, 1700, 2600, 0.0}; // T stop
                case 'g': return {240, 1950, 2750, 1.0}; // G
                case 'd': return {280, 1650, 2450, 1.0}; // D
                case 'b': return {280,  870, 2350, 1.0}; // B
                case 'p': return {280,  870, 2350, 0.0}; // P
                case 'w': return {280,  600, 2200, 1.0}; // W
                case 'y': return {260, 1950, 2850, 1.0}; // IH (bit)
                case 'h': return {400, 1150, 2350, 0.0}; // AH schwa
                // === NEW Phase 29: 18 additional phonemes ===
                case 'Q': return {250, 1700, 2400, 1.0}; // NG (sing) — nasal velar
                case 'X': return {270, 2000, 3500, 0.0}; // SH (she) — palatal fricative
                case 'C': return {320, 1800, 3000, 0.0}; // CH (church) — affricate
                case 'J': return {380, 1700, 2800, 0.7}; // JH (judge) — voiced affricate
                case '2': return {260, 1800, 2800, 0.7}; // ZH (measure) — voiced palatal fric.
                case '3': return {280, 1400, 2500, 0.6}; // DH (the) — voiced dental fric.
                case '4': return {280, 1400, 2500, 0.0}; // TH (think) — unvoiced dental fric.
                case '9': return {500, 1500, 2500, 0.0}; // HH (house) — glottal aspirate
                case 'j': return {260, 1950, 2850, 1.0}; // Y (yet) — consonant glide
                case 'R': return {490, 1350, 1690, 1.0}; // ER (bird) — rhotic vowel
                case 'Y': return {600, 1100, 2500, 1.0}; // AY (eye) — diphthong midpoint
                case 'A': return {660, 1720, 2410, 1.0}; // AE (cat)
                case 'E': return {580,  840, 2400, 1.0}; // OW (go) — diphthong midpoint
                case '5': return {650,  950, 2300, 1.0}; // AW (cow) — diphthong midpoint
                case '6': return {450, 1850, 2480, 1.0}; // EY (say) — diphthong midpoint
                case '7': return {570, 1000, 2400, 1.0}; // OY (boy) — diphthong midpoint
                case '8': return {440,  960, 2340, 1.0}; // UH (book) — near-close vowel
                case '_': return {500, 1500, 2500, 0.0}; // silence
                case ' ': return {380,  950, 2400, 0.0}; // word boundary
                default:  return {480, 1400, 2450, 0.5};
            }
        };

        for (size_t i = 0; i + 1 < phones.size(); i++) {
            char c1 = phones[i];
            char c2 = phones[i + 1];

            // Atom lookup
            std::string key;
            key += (c1 == ' ' ? '_' : c1);
            key += '_';
            key += (c2 == ' ' ? '_' : c2);
            DiphoneAtom* atom = nullptr;
            for (auto& kv : g_VocabCache) {
                if (std::string(kv.second.key) == key) { atom = &kv.second; break; }
            }
            if (!atom) continue;

            // Coarticulation targets: c1→c2 formant trajectory
            PhFormant pf1 = GetPF(c1);
            PhFormant pf2 = GetPF(c2);

            // ═══════════════════════════════════════════════════
            // SOVEREIGN PITCH SLEEVE
            // A continuously varying F0 field, phoneme by phoneme.
            // Vowels pull pitch UP, consonant onsets pull DOWN,
            // word boundaries reset with a mini-rise (accent),
            // sentence declination falls across the utterance.
            // ═══════════════════════════════════════════════════
            auto PitchPull = [](char p) -> double {
                // FIX: Wider range (was 0.70-1.20, now 0.55-1.40) for expressive prosody.
                // ANATOMY: Lung pressure + larynx tension varies dramatically.
                // Vowels = high tension + open tract = peak F0
                // Voiceless = glottis abducted = F0 drops or stops
                switch(p) {
                    case 'a': case 'e': case 'i': case 'o': case 'u':
                    case 'Y': case 'A': case 'E': case '5': case '6': case '7':
                        return 1.40; // vowels: full glottal tension, F0 peaks
                    case 'r': case 'l': case 'w': case 'y': case 'j':
                        return 1.20; // sonorants: slightly below vowel
                    case 'n': case 'm': case 'Q':
                        return 1.10; // nasals: soft pitch
                    case 'v': case 'z': case 'g': case 'd': case 'b': case '3': case '2':
                        return 0.85; // voiced consonants: lower pitch
                    case 's': case 'f': case 'k': case 't': case 'p': case 'h':
                    case 'X': case '4': case 'C': case '9':
                        return 0.55; // voiceless: glottis open, F0 falls away
                    case ' ':
                        return 0.75; // boundary: low
                    default:
                        return 1.00;
                }
            };

            bool isWordStart  = (c1 == ' ');
            // Sentence declination: baseline falls 20% over utterance
            double posRatio   = time / std::max(totalDur, 0.001);
            double declination = 1.0 - 0.20 * posRatio;

            // Word boundary: micro-rise then settle
            double wordAccent  = isWordStart ? 1.20 : 1.0;

            // Per-phoneme pitch target from the sleeve
            double sleeveTarget = F0 * PitchPull(c2) * declination * wordAccent;

            // FIX: Faster F0 convergence (0.12→0.30) for expressive prosody.
            // At 0.12 the sleeve barely moves between phonemes — near-monotone.
            // At 0.30 it converges within 4-5 frames: natural pitch gestures.
            static double sleeveF0 = 100.0;
            sleeveF0 += 0.30 * (sleeveTarget - sleeveF0);
            sleeveF0  = std::max(60.0, std::min(280.0, sleeveF0));

            harmSrc.SetF0(sleeveF0);
            glottal.SetFrequency(sleeveF0);

            // --- FRAME LOOP: continuous synthesis ---
            double lastVT = 0.0; // radiation filter delay — scoped per-atom, not static
            // FILTER STATE RESET: clear resonator states when starting from silence.
            // Bug: r1-r5 built up noise state while iVelocity=0, burst when voicing on = "extra word".
            if (c1 == ' ') { r1.Reset(); r2.Reset(); r3.Reset(); r4.Reset(); rN.Reset(); }

            for (int f = 0; f < 32; f++) {
                ResonanceFrame& fr = atom->frames[f];

                // Subtle metabolic breath rhythm (not robotic swing)
                // Axiom Shift: A circle is 360.5 degrees. 
                // Scaling the metabolic resonance constant for spiral evolutionary logic.
                double metabolic = 1.0 + 0.04 * sin(Sovereign::SOVEREIGN_CIRCLE * 1.092777 * time);
                // DURATION CONTROL BY PHONEME CLASS
                // FIX: Fixed 160ms/diphone was the primary cause of robotic cadence.
                // Speech needs rhythm: stops ~65ms, fricatives ~96ms, vowels ~160ms.
                // ANATOMY: Lung air pressure depletes faster for consonants.
                // The more constricted the vocal tract, the shorter the segment.
                auto DurScale = [](char p) -> double {
                    switch(p) {
                        // Stops: closure + burst = ~65ms total
                        case 'p': case 'b': case 't': case 'd':
                        case 'k': case 'g': case 'C': case 'J':
                            return 0.40;
                        // Fricatives: ~96ms  
                        case 's': case 'f': case 'X': case '4':
                        case '2': case '3': case '9': case 'z': case 'v':
                            return 0.55;
                        // Nasals: ~80ms
                        case 'n': case 'm': case 'Q':
                            return 0.48;
                        // Liquids/semivowels: ~88ms
                        case 'r': case 'l': case 'w': case 'j':
                            return 0.52;
                        // Word boundary silence: ~50ms
                        case ' ': case '_':
                            return 0.30;
                        default:  // Vowels: full 160ms
                            return 1.00;
                    }
                };
                // Governing duration = shorter of c1/c2 class (transition determines)
                double durScale = std::min(DurScale(c1), DurScale(c2));
                // Vowels get full duration; consonants shorter
                if (DurScale(c1) >= 1.0 && DurScale(c2) >= 1.0) durScale = 1.00;
                int nSamples = std::max(1, (int)(0.005 * 44100.0 * metabolic * durScale));


                // COARTICULATION: interpolate formants c1→c2 across frames
                // SILENCE RULE: transitioning from/to silence (' ') is AMPLITUDE-only.
                // Formants snap to the NON-silence phoneme. iVelocity provides the fade.
                //   _→a : formantAlpha=1.0 → hold at /a/ formants, amplitude fades in
                //   a→_ : formantAlpha=0.0 → hold at /a/ formants, amplitude fades out
                // Without this: _→a interpolates from F1=400→730 = muddy for full atom.
                double alpha       = (double)f / 31.0;
                double formantAlpha = alpha;
                if (c1 == ' ') formantAlpha = 1.0; // _→X: snap to target phoneme formants
                if (c2 == ' ') formantAlpha = 0.0; // X→_: hold source phoneme formants
                double iF1    = pf1.f1 + formantAlpha * (pf2.f1 - pf1.f1);
                double iF2    = pf1.f2 + formantAlpha * (pf2.f2 - pf1.f2);
                double iF3    = pf1.f3 + formantAlpha * (pf2.f3 - pf1.f3);
                double iVoice = pf1.v   + formantAlpha * (pf2.v   - pf1.v);

                // ═══════════════════════════════════════════════════════
                // DAB INDUSTRIES VIRTUAL SLEEVE v1.0
                // Original design: DAB Industries Rust prototype
                //
                // Three physical parameters per phoneme:
                //   diameter:      tube width (0.25=plosive tight, 1.85=vowel open)
                //   flow_velocity: excitation energy (2.2=burst, 0.75=vowel, 1.1=other)
                //   tension:       glottal closure hardness (0.95=stop, 0.10=vowel)
                //
                // diameter → BW factor (1.0/diameter)
                //   plosive  (0.25) → BW×4.0  — near-flat resonators (stoplike)
                //   neutral  (1.00) → BW×1.0  — Klatt canonical BW
                //   vowel    (1.85) → BW×0.54 — sharper, more resonant
                //
                // flow_velocity → excitation amplitude scale
                //   burst (2.2) is louder than vowel (0.75) — physically correct
                // ═══════════════════════════════════════════════════════
                struct Sleeve { double diameter, velocity, tension; };
                auto GetSleeve = [](char p) -> Sleeve {
                    switch(p) {
                        // PLOSIVES — ImpactBurst state
                        case 'p': case 'b': case 't': case 'd': case 'k': case 'g':
                            return {0.25, 2.2, 0.95};
                        // AFFRICATES (stop+fricative)
                        case 'C': case 'J':
                            return {0.30, 2.0, 0.90};
                        // VOWELS — ResonantFlow state
                        case 'a': case 'e': case 'i': case 'o': case 'u':
                        case 'Y': case 'A': case 'E': case '5': case '6': case '7':
                            return {1.85, 0.75, 0.10};
                        // SONORANTS — near-vowel resonance
                        case 'r': case 'l': case 'w': case 'y': case 'j':
                            return {1.40, 0.85, 0.20};
                        // NASALS — partial constriction
                        case 'n': case 'm': case 'Q':
                            return {0.80, 0.90, 0.35};
                        // VOICED FRICATIVES — turbulent partial voicing
                        case 'v': case 'z': case '2': case '3':
                            return {0.55, 1.20, 0.55};
                        // UNVOICED FRICATIVES — turbulent, not closed
                        case 's': case 'f': case 'X': case '4': case '9':
                            return {0.50, 1.30, 0.60};
                        // RHOTIC VOWEL / HH
                        case 'R': case '8': case 'h':
                            return {1.30, 0.80, 0.25};
                        // SILENCE/BOUNDARY
                        case ' ': case '_':
                            return {1.00, 0.00, 0.40};
                        default:
                            return {1.00, 1.10, 0.40};
                    }
                };

                Sleeve s1 = GetSleeve(c1);
                Sleeve s2 = GetSleeve(c2);
                // Continuous sleeve transition (coarticulation = sleeve state blending)
                double iDiameter = s1.diameter  + alpha * (s2.diameter  - s1.diameter);
                double iVelocity = s1.velocity  + alpha * (s2.velocity  - s1.velocity);
                double iTension  = s1.tension   + alpha * (s2.tension   - s1.tension);

                // BW from sleeve diameter — always smooth, no hard voiced/unvoiced switch.
                // Old code: bw = 300 for iVoice<0.35 caused sudden BW jump at frame 12
                // of silence→vowel transitions, sounding like a separate word ("3 words" bug).
                // voiced/unvoiced distinction is in the excitation (buzz vs noise), not BW.
                double iDiam = std::max(0.5, std::min(1.2, iDiameter));
                double bw1 = 60.0  / iDiam;
                double bw2 = 90.0  / iDiam;
                double bw3 = 150.0 / iDiam;

                // F2 LOCUS for stop consonants (place-of-articulation cue)
                // Alveolar /d,t/: locus ~1800Hz
                // Bilabial /b,p/: locus ~800Hz
                // Velar /g,k/:   locus ~2300Hz (velar pinch)
                auto F2Locus = [](char p) -> double {
                    if (p=='d'||p=='t') return 1800.0;
                    if (p=='b'||p=='p') return  800.0;
                    if (p=='g'||p=='k') return 2300.0;
                    return -1.0; // no locus adjustment
                };
                double locus1 = F2Locus(c1), locus2 = F2Locus(c2);
                // Override F2 start/end from locus when transitioning
                double effF2 = iF2;
                if (locus1 > 0 && alpha < 0.4)
                    effF2 = locus1 + (alpha / 0.4) * (iF2 - locus1);
                else if (locus2 > 0 && alpha > 0.6)
                    effF2 = iF2 + ((alpha - 0.6) / 0.4) * (locus2 - iF2);

                // ShiftResonance: apply profile formant scale (spectral resizing)
                double F1_scaled = iF1 * g_ActiveProfile.formant_scale;
                double F2_scaled = effF2 * g_ActiveProfile.formant_scale;
                double F3_scaled = iF3 * g_ActiveProfile.formant_scale;

                r1.SetFormant(F1_scaled, bw1);
                r2.SetFormant(F2_scaled, bw2);
                r3.SetFormant(F3_scaled, bw3);

                // STOP CONSONANT MODEL — direction-aware
                // Onset stop (c1 is stop): atom goes FROM closure/burst TO vowel
                //   → closure at START, burst in middle, formant transition at end
                // Offset stop (c2 is stop): atom goes FROM vowel TO closure onset
                //   → formant transition at start, closure building at END (no burst here)
                bool isOnsetStop  = (c1=='d'||c1=='t'||c1=='b'||c1=='p'||c1=='k'||c1=='g');
                bool isOffsetStop = (c2=='d'||c2=='t'||c2=='b'||c2=='p'||c2=='k'||c2=='g');
                bool isVcdStop    = (c1=='d'||c1=='b'||c1=='g'||c2=='d'||c2=='b'||c2=='g');

                // Continuous sample generation — no windowing
                int atomTotalSamples = 32 * nSamples;
                int atomSampleStart  = f * nSamples;

                for (int s = 0; s < nSamples; s++) {
                    double voicing = iVoice; // from coarticulation table
                    double noise   = g_Dist(g_RNG);

                    double pos = (double)(atomSampleStart + s) / atomTotalSamples;
                    double stopAmp = 1.0;

                    if (isOnsetStop) {
                        // Closure: TRUE SILENCE — narrowed to 20% (from 32%)
                        // _→d atom already contributes ~70ms silence; this adds 19ms more
                        if (pos < 0.20) {
                            stopAmp = 0.0;
                        } else if (pos < 0.36) {
                            // BURST: r4+r5 ONLY — bypass r1/r2/r3 (low formant resonators)
                            // BUG FIXED: r1 tuned to 300-454Hz converts 2kHz HP noise to
                            // low-freq tonal ring (sounds like /z/). r4(3.3kHz)+r5(3.75kHz)
                            // amplify the burst in the correct 3-4kHz stop burst region.
                            double burstCut = 2000.0;
                            if (c1=='b'||c1=='p') burstCut =  700.0;
                            else if (c1=='g'||c1=='k') burstCut = 1500.0;
                            double burstRaw  = sFilter.HPF(g_Dist(g_RNG), burstCut) * 10.0;
                            double bFiltered = r5.Process(r4.Process(burstRaw)); // r4+r5 ONLY
                            lpState += lpAlpha * (bFiltered - lpState);
                            int crossLen = 12;
                            int gs = atomSampleStart + s;
                            double fade = 1.0;
                            if (gs < crossLen) fade = (double)gs / crossLen;
                            else if (gs > atomTotalSamples - crossLen)
                                fade = (double)(atomTotalSamples - gs) / crossLen;
                            floatBuffer.push_back(lpState * s1.velocity * fr.intensity * wordAccent * fade);
                            continue;
                        }
                        // pos >= 0.36: vowel onset — full voiced
                    } else if (isOffsetStop) {
                        // Vowel → closure: fade to COMPLETE SILENCE — starts at 80% (was 72%)
                        // Keeps more vowel audible before closure
                        if (pos > 0.80) {
                            double closeFactor = (pos - 0.80) / 0.20;
                            stopAmp = 1.0 - closeFactor;
                        }
                        // No burst here — occurs in next atom's onset
                    }

                    // ═══════════════════════════════════════════════════════
                    // KLATT MIXED EXCITATION (Phase 29 v29.1)
                    // voiced_component   = HarmonicSource × voicing × glottal_env
                    // unvoiced_component = shaped_noise × a_noise (from baked atom)
                    // excitation = voiced_component + unvoiced_component
                    // This models the periodic+noise mix scientifically:
                    //   - Pure vowels: voicing=1.0, a_noise~0.02 → almost pure buzz
                    //   - Voiced fricatives (v,z): voicing=0.7, a_noise=0.2 → buzz+turbulence
                    //   - Unvoiced fricatives (s,f): voicing=0.0, a_noise=0.7 → pure noise
                    //   - Plosive burst: voicing=0.0, a_noise=0.4 → shaped burst
                    //   - Transitions: a_noise peaks at V-UV boundary (baked crossPeak)
                    // ═══════════════════════════════════════════════════════
                    // Prosody 2.0: Sentence Declination Model
                    // Pitch falls by ~20% across the full utterance for naturalism.
                    double declination = 1.0 - (0.2 * pos); 
                    double profileF0   = (double)g_ActiveProfile.pitch_base * declination;
                    
                    // Profile-aware Jitter (Spectral Entropy)
                    double jitter = 1.0 + (g_Dist(g_RNG) * g_ActiveProfile.jitter_depth);
                    harmSrc.SetF0(profileF0 * jitter);

                    double pulseGain  = 1.0 - iTension * 0.6;
                    double gate       = std::min(1.0, iVelocity / 0.04);
                    double voicedBuzz = harmSrc.GetNextSample() * pulseGain * stopAmp * gate;

                    // Read aspiration noise amplitude from atom frame (Klatt v2 field)
                    // If old v1 format: fr.a_noise will be 0 (default), fallback to iVoice-derived
                    double baked_a_noise = (double)fr.a_noise;
                    if (baked_a_noise < 0.001) {
                        // v1 vocab fallback: derive from voicing ratio
                        baked_a_noise = (1.0 - iVoice) * 0.15;
                    }

                    // Shape noise source per consonant class (spectral coloring)
                    double noiseRaw = g_Dist(g_RNG);
                    double shapedNoise = 0.0;
                    if      (c1 == 's' || c2 == 's') shapedNoise = sFilter.BPF(noiseRaw, 5500.0, 1200.0);
                    else if (c1 == 'X' || c2 == 'X') shapedNoise = sFilter.BPF(noiseRaw, 4000.0, 1500.0);
                    else if (c1 == 'f' || c2 == 'f') shapedNoise = sFilter.HPF(noiseRaw, 2500.0);
                    else if (c1 == '4' || c2 == '4') shapedNoise = sFilter.HPF(noiseRaw, 3500.0);
                    else if (c1 == '3' || c2 == '3') shapedNoise = sFilter.HPF(noiseRaw, 2000.0);
                    else if (c1 == 'v' || c2 == 'v' || c1 == 'z' || c2 == 'z' ||
                             c1 == '2' || c2 == '2') shapedNoise = sFilter.HPF(noiseRaw, 1500.0);
                    else if (c1 == 't' || c2 == 't' || c1 == 'k' || c2 == 'k' ||
                             c1 == 'C' || c2 == 'C') {
                        shapedNoise = (atomSampleStart + s < nSamples/4) ?
                            sFilter.HPF(noiseRaw, 3500.0) : noiseRaw * 0.01;
                    }
                    else shapedNoise = noiseRaw * 0.003;

                    // MIXED EXCITATION: voiced + aspiration (Klatt Eq.)
                    // voiced_fraction  modulated by iVoice (coarticulation source strength)
                    // unvoiced_fraction scaled by baked a_noise (per-frame aspiration amplitude)
                    double excitation = (voicedBuzz * iVoice) + (shapedNoise * baked_a_noise * 0.8);

                    // IIR cascade: r1 → r2 → r3 → r4 → r5
                    double vt = r5.Process(r4.Process(r3.Process(r2.Process(r1.Process(excitation)))));
                    bool isNasal = (c1=='n'||c2=='n'||c1=='m'||c2=='m'||c1=='Q'||c2=='Q');
                    if (isNasal) {
                        vt = rN.Process(vt);  // Nasal pole (murmur resonance)

                        // NASAL ANTI-RESONANCE (spectral zero)
                        // ANATOMY: The nasal passage is coupled to the oral cavity.
                        // This coupling creates spectral ZEROS (anti-resonances) —
                        // the hallmark of nasal consonants. Each place of articulation
                        // produces a different zero frequency:
                        //   /m/ bilabial: oral cavity closed at lips → null ~1000Hz
                        //   /n/ alveolar: oral cavity closed at ridge → null ~1400Hz
                        //   /ng/ velar:   oral cavity closed at velum → null ~2000Hz
                        if (c1=='m'||c2=='m')       nasalNotch.SetNull(1000.0, 250.0);
                        else if (c1=='n'||c2=='n')  nasalNotch.SetNull(1400.0, 300.0);
                        else                        nasalNotch.SetNull(2000.0, 350.0); // /ng/
                        vt = nasalNotch.Process(vt);
                    }

                    // Lip radiation: true 1st-order differentiator (positive - negative)
                    // Models the +6dB/oct characteristic of lip radiation
                    // lastVT is the PREVIOUS unfiltered vt (not LP-filtered)
                    double radiated = vt - 0.97 * lastVT;
                    lastVT = vt;

                    // 5kHz LP post-radiation
                    lpState += lpAlpha * (radiated - lpState);

                    // Short cross-fade at atom start/end only (no per-frame windowing)
                    int crossLen = 20;
                    int globalS  = atomSampleStart + s;
                    double fade  = 1.0;
                    if (globalS < crossLen)
                        fade = (double)globalS / crossLen;
                    else if (globalS > atomTotalSamples - crossLen)
                        fade = (double)(atomTotalSamples - globalS) / crossLen;

                    // INTENSITY SILENCE RULE: mirrors the formantAlpha fix.
                    // fr.intensity fades 0→1 for _→X and 1→0 for X→_ (from vocab).
                    // iVelocity ALSO fades 0→0.75 for _→X and 0.75→0 for X→_.
                    // Together: quadratic output (0²) — vowel never sustains, sounds like blip.
                    // Fix: hold non-silence phoneme's intensity; let iVelocity control fade.
                    double frameIntensity = fr.intensity;
                    if (c1 == ' ') frameIntensity = atom->frames[31].intensity; // _→X: hold target intensity
                    if (c2 == ' ') frameIntensity = atom->frames[0].intensity;  // X→_: hold source intensity

                    floatBuffer.push_back(lpState * iVelocity * frameIntensity * wordAccent * fade);





                }

                time += (double)nSamples / 44100.0;
            }
        }

        // --- NORMALIZE & OUTPUT ---
        double peak = 0.0001;
        for (double s : floatBuffer) peak = std::max(peak, std::abs(s));
        std::vector<int16_t> pcm;
        pcm.reserve(floatBuffer.size());
        for (double s : floatBuffer) {
            pcm.push_back((int16_t)std::clamp(s * (30000.0 / peak), -32768.0, 32767.0));
        }

        WriteWAV("C:\\GENESIS\\Sovereign_Voice.wav", pcm);
        PlayHardwareWAV("C:\\GENESIS\\Sovereign_Voice.wav");
    }

    void SovereignAcoustics::WriteWAV(const std::string& filename,
                                       const std::vector<int16_t>& pcmData) {
        WavHeader hdr;
        hdr.subchunk2Size = (uint32_t)(pcmData.size() * sizeof(int16_t));
        hdr.chunkSize     = 36 + hdr.subchunk2Size;
        std::ofstream file(filename, std::ios::binary);
        if (file) {
            file.write(reinterpret_cast<const char*>(&hdr), sizeof(WavHeader));
            file.write(reinterpret_cast<const char*>(pcmData.data()), hdr.subchunk2Size);
        }
    }

    void SovereignAcoustics::PlayHardwareWAV(const std::string& filename) {
        PlaySoundA(filename.c_str(), NULL, SND_FILENAME | SND_SYNC);
    }

} // namespace Sovereign
