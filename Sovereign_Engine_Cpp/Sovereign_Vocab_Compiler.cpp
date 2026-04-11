// ============================================================
// SOVEREIGN VOCABULARY COMPILER v29.1 — PHONETIC SINGULARITY
// Universal 40-phoneme Diphone Matrix (1,600 atoms)
// Klatt-correlate formant targets + Mixed Excitation baking
// ============================================================
#include "Sovereign_Vocab.h"
#include <iostream>
#include <fstream>
#include <cmath>
#include <vector>
#include <string>
#include <map>

namespace Sovereign {

    // ========================================================
    // KLATT-CORRELATE PHONETIC TARGET TABLE
    // Source: Klatt (1980) + Peterson-Barney (1952) bass-male
    // Each entry: {f1, f2, f3, bw1, bw2, bw3, voicing, f0, a_noise, intensity}
    // voicing: 1.0=pure glottal, 0.0=pure noise, 0.x=mixed
    // a_noise: aspiration amplitude [0..1] for mixed excitation
    // f0: base pitch (Hz) — vowels 100, sonorants 95, voiced cons 80, unvoiced 0
    // ========================================================
    struct PhoneticTarget {
        float f1, f2, f3;
        float bw1, bw2, bw3;
        float voicing;   // 0.0 unvoiced, 1.0 voiced, 0.x mixed
        float f0;        // Klatt v2: pitch Hz (0=unvoiced/follow source)
        float a_noise;   // Klatt v2: aspiration noise amplitude
        float intensity;
    };

    class VocabCompiler {
    public:
        static void Compile(const std::string& outputPath) {
            std::cout << "[COMPILER v29.1] Manifesting 40-Phoneme Klatt Singularity..." << std::endl;

            // ================================================
            // THE 40-PHONEME KLATT-CORRELATE TABLE
            // Single-char codes (some non-alpha for digraphs):
            //  Vowels (11): a=AA, e=EH, i=IY, o=AO, u=UW, h=AH, y=IH, Y=AY, A=AE, R=ER, E=OW
            //  Semivowels (2): w=W, j=YY(consonant-y)
            //  Nasals (3): n=N, m=M, Q=NG
            //  Stops (6): b=B, d=D, g=G, p=P, t=T, k=K
            //  Fricatives (8): v=V, z=Z, s=S, f=F, 2=ZH, X=SH, 3=TH_vcd(bathe), 4=TH_uvd(think)
            //  Affricates (2): C=CH, J=JH
            //  Liquids (2): r=R, l=L
            //  Diphthong/other (5): 5=AW, 6=EY, 7=OY, 8=UH(book), 9=HH
            //  Silence (1): _
            // Total: 11+2+3+6+8+2+2+5+1 = 40
            // ================================================
            std::map<char, PhoneticTarget> T;

            // --- VOWELS (fully voiced, low aspiration) ---
            // Format: {f1, f2, f3, bw1, bw2, bw3, voicing, f0, a_noise, intensity}
            T['a'] = {680, 1050, 2400, 70,  90, 130, 1.0, 100.0, 0.02, 1.0}; // AA father (bass male)
            T['e'] = {500, 1700, 2480, 70,  90, 130, 1.0, 103.0, 0.02, 1.0}; // EH pet
            T['i'] = {250, 2100, 2900, 60,  80, 120, 1.0, 105.0, 0.02, 1.0}; // IY heed
            T['o'] = {490,  720, 2300, 70,  90, 130, 1.0,  98.0, 0.02, 1.0}; // AO caught
            T['u'] = {270,  700, 2300, 60,  80, 120, 1.0,  97.0, 0.02, 1.0}; // UW boot
            T['h'] = {520, 1190, 2390, 90, 120, 160, 1.0, 100.0, 0.04, 0.9}; // AH but/schwa
            T['y'] = {390, 1990, 2550, 70,  90, 130, 1.0, 102.0, 0.02, 0.8}; // IH bit
            T['Y'] = {600, 1100, 2500, 70,  90, 130, 1.0, 105.0, 0.02, 1.0}; // AY eye (midpoint)
            T['A'] = {660, 1720, 2410, 80, 100, 140, 1.0, 101.0, 0.02, 1.0}; // AE cat
            T['R'] = {490, 1350, 1690, 80, 100, 140, 1.0,  99.0, 0.03, 0.9}; // ER bird (rhotic)
            T['E'] = {580,  840, 2400, 70,  90, 130, 1.0,  98.0, 0.02, 1.0}; // OW go (midpoint)

            // --- SEMIVOWELS (high voicing, slight aspiration) ---
            T['w'] = {290,  610, 2150, 80, 100, 140, 1.0,  95.0, 0.05, 0.9}; // W wet
            T['j'] = {260, 1950, 2850, 80, 100, 140, 1.0,  95.0, 0.04, 0.8}; // Y yet (consonant)

            // --- NASALS (voiced, formant poles modified by murmur) ---
            T['n'] = {250,  900, 2100, 80, 100, 140, 1.0,  95.0, 0.03, 0.7}; // N not
            T['m'] = {250,  800, 2200, 80, 100, 140, 1.0,  95.0, 0.03, 0.7}; // M mat
            T['Q'] = {250, 1700, 2400, 80, 100, 140, 1.0,  93.0, 0.04, 0.6}; // NG sing (nasal velar)

            // --- PLOSIVES: VOICED (mixed: voicing 0.8, low aspiration) ---
            T['b'] = {330,  900, 2200, 130, 170, 250, 0.8,  85.0, 0.10, 0.9}; // B bat
            T['d'] = {380, 1700, 2500, 130, 170, 250, 0.8,  85.0, 0.10, 0.9}; // D dog
            T['g'] = {450, 1980, 2750, 130, 170, 250, 0.8,  80.0, 0.10, 0.9}; // G got

            // --- PLOSIVES: UNVOICED (voicing 0.0, higher aspiration) ---
            T['p'] = {300,  870, 2350, 400, 500, 600, 0.0,   0.0, 0.40, 1.0}; // P pat
            T['t'] = {280, 1750, 2600, 400, 500, 600, 0.0,   0.0, 0.45, 1.0}; // T top
            T['k'] = {280, 2000, 3000, 400, 500, 600, 0.0,   0.0, 0.45, 1.0}; // K cat

            // --- FRICATIVES: VOICED (mixed excitation) ---
            T['v'] = {380, 1100, 2200, 120, 160, 220, 0.7,  80.0, 0.20, 0.8}; // V vet
            T['z'] = {240, 1000, 2500, 120, 160, 220, 0.7,  78.0, 0.25, 0.9}; // Z zip
            T['2'] = {260, 1800, 2800, 120, 160, 220, 0.7,  78.0, 0.30, 0.9}; // ZH measure
            T['3'] = {280, 1400, 2500, 200, 250, 350, 0.6,  78.0, 0.25, 0.8}; // DH the (voiced TH)

            // --- FRICATIVES: UNVOICED (pure noise, high aspiration) ---
            T['s'] = {280, 1500, 5000, 500, 500, 500, 0.0,   0.0, 0.80, 1.2}; // S sit
            T['f'] = {280, 1500, 3800, 500, 500, 500, 0.0,   0.0, 0.60, 0.8}; // F fit
            T['X'] = {270, 2000, 3500, 400, 500, 600, 0.0,   0.0, 0.75, 1.0}; // SH she
            T['4'] = {280, 1400, 2500, 400, 500, 600, 0.0,   0.0, 0.50, 0.8}; // TH think (unvoiced)

            // --- AFFRICATES (onset stop + fricative, mixed) ---
            T['C'] = {320, 1800, 3000, 300, 400, 500, 0.0,   0.0, 0.60, 1.0}; // CH church
            T['J'] = {380, 1700, 2800, 200, 300, 400, 0.7,  78.0, 0.35, 1.0}; // JH judge

            // --- LIQUIDS (sonorants, near-vowel) ---
            T['r'] = {450, 1250, 1700, 80, 100, 140, 1.0,  97.0, 0.03, 0.9}; // R red
            T['l'] = {360, 1000, 2600, 80, 100, 140, 1.0,  97.0, 0.03, 0.8}; // L let

            // --- DIPHTHONGS / ADDITIONAL ---
            T['5'] = {650,  950, 2300, 80, 100, 140, 1.0, 103.0, 0.02, 1.0}; // AW cow (midpoint)
            T['6'] = {450, 1850, 2480, 70,  90, 130, 1.0, 105.0, 0.02, 1.0}; // EY say (midpoint)
            T['7'] = {570, 1000, 2400, 80, 100, 140, 1.0, 100.0, 0.02, 0.9}; // OY boy (midpoint)
            T['8'] = {440,  960, 2340, 90, 120, 160, 1.0,  96.0, 0.04, 0.8}; // UH book

            // HH — breathy onset (aspiration only, formants set to neutral)
            T['9'] = {500, 1500, 2500, 200, 250, 350, 0.0,   0.0, 0.70, 0.7}; // HH house

            // Silence
            T['_'] = {500, 1500, 2500, 500, 500, 500, 0.0,   0.0, 0.00, 0.0};

            // ================================================
            // THE 40-PHONEME ITERATION LIST
            // Exactly 40 unique chars (verified above)
            // ================================================
            const std::string PHONEMES = "aeiouhy YAREw j nmQb dgp tkv z23sfX4CJrl 5678 9_";
            std::vector<char> phonemeList;
            for (char c : PHONEMES) {
                if (c != ' ') phonemeList.push_back(c);
            }
            // Ensure exactly 40
            while (phonemeList.size() > 40) phonemeList.pop_back();
            // Final count should be 40
            std::cout << "[COMPILER] Phoneme count: " << phonemeList.size() << " (target: 40)" << std::endl;

            std::vector<DiphoneAtom> vocabulary;
            vocabulary.reserve(phonemeList.size() * phonemeList.size());
            uint32_t atomCounter = 0;

            for (char c1 : phonemeList) {
                for (char c2 : phonemeList) {
                    DiphoneAtom atom;
                    // Key: c1_c2 (using _CHAR representation for special chars)
                    snprintf(atom.key, sizeof(atom.key), "%c_%c", c1, c2);
                    atom.id = atomCounter++;

                    PhoneticTarget& t1 = T.count(c1) ? T[c1] : T['_'];
                    PhoneticTarget& t2 = T.count(c2) ? T[c2] : T['_'];

                    for (int f = 0; f < 32; f++) {
                        // Klatt sigmoid locus: smoother transition than linear
                        // sigma(x) = 1/(1+exp(-k*(x-0.5))) — concentrated energy in mid-frame
                        float x     = (float)f / 31.0f;
                        float alpha = 1.0f / (1.0f + expf(-8.0f * (x - 0.5f)));

                        ResonanceFrame& fr = atom.frames[f];

                        // --- Formant trajectory (F1-F3) ---
                        fr.f1  = t1.f1  + alpha * (t2.f1  - t1.f1);
                        fr.f2  = t1.f2  + alpha * (t2.f2  - t1.f2);
                        fr.f3  = t1.f3  + alpha * (t2.f3  - t1.f3);

                        // --- Bandwidth trajectory ---
                        fr.bw1 = t1.bw1 + alpha * (t2.bw1 - t1.bw1);
                        fr.bw2 = t1.bw2 + alpha * (t2.bw2 - t1.bw2);
                        fr.bw3 = t1.bw3 + alpha * (t2.bw3 - t1.bw3);

                        // --- Intensity: source phoneme sustains, brief fade at edges ---
                        fr.intensity = t1.intensity * (1.0f - alpha) + t2.intensity * alpha;

                        // --- MIXED EXCITATION: voicing ratio trajectory ---
                        // At voiced→unvoiced boundary: voicing dips mid-frame (devoicing lag)
                        // At unvoiced→voiced boundary: voicing rises with prevoicing onset
                        fr.voicing = t1.voicing * (1.0f - alpha) + t2.voicing * alpha;

                        // --- KLATT v2: F0 trajectory ---
                        // Voiced phonemes carry pitch; unvoiced carry 0 (no buzz)
                        fr.f0 = t1.f0 * (1.0f - alpha) + t2.f0 * alpha;

                        // --- KLATT v2: Aspiration noise trajectory ---
                        // a_noise peaks at boundary crossings (breathy transitions)
                        float crossPeak = (t1.voicing != t2.voicing) ?
                            0.20f * sinf((float)M_PI * x) : 0.0f;
                        fr.a_noise = t1.a_noise * (1.0f - alpha) + t2.a_noise * alpha + crossPeak;
                        if (fr.a_noise > 1.0f) fr.a_noise = 1.0f;
                    }
                    vocabulary.push_back(atom);
                }
            }

            // --- BINARY BAKING ---
            VocabHeader header;
            header.atomCount  = (uint32_t)vocabulary.size();
            header.dataOffset = sizeof(VocabHeader);

            std::ofstream file(outputPath, std::ios::binary);
            if (file) {
                file.write(reinterpret_cast<const char*>(&header),          sizeof(VocabHeader));
                file.write(reinterpret_cast<const char*>(vocabulary.data()),
                           vocabulary.size() * sizeof(DiphoneAtom));
                file.close();
                std::cout << "[SUCCESS] Sovereign Vocab Manifest v2: "
                          << header.atomCount << " atoms => " << outputPath << std::endl;
                std::cout << "[MATRIX]  " << phonemeList.size() << " x " << phonemeList.size()
                          << " = " << header.atomCount << " diphone atoms baked." << std::endl;
            } else {
                std::cerr << "[ERROR] Failed to write binary atoms: " << outputPath << std::endl;
            }
        }
    };

} // namespace Sovereign
