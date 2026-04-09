// ============================================================
// GODSEYE ENGINE v11.0 - THE SOVEREIGN ACOUSTIC SYNTHESIZER
// First-Principles Mathematical Vocal Cord Emulator
// ============================================================
#pragma once

#include <vector>
#include <string>
#include <cstdint>

namespace Sovereign {

    // 16-bit 44.1kHz standard PCM struct layout
    #pragma pack(push, 1)
    struct WavHeader {
        char chunkId[4] = {'R', 'I', 'F', 'F'};
        uint32_t chunkSize;
        char format[4] = {'W', 'A', 'V', 'E'};
        
        char subchunk1Id[4] = {'f', 'm', 't', ' '};
        uint32_t subchunk1Size = 16;
        uint16_t audioFormat = 1; // PCM
        uint16_t numChannels = 1; // Mono
        uint32_t sampleRate = 44100;
        uint32_t byteRate = 44100 * 1 * 2;
        uint16_t blockAlign = 2;
        uint16_t bitsPerSample = 16;

        char subchunk2Id[4] = {'d', 'a', 't', 'a'};
        uint32_t subchunk2Size;
    };
    #pragma pack(pop)

    // Mathematical Infinite Impulse Response (IIR) Formant Filter
    class SovereignIIRResonator {
    public:
        SovereignIIRResonator(double sampleRate = 44100.0);
        void SetFormant(double frequency, double bandwidth);
        double Process(double input);

    private:
        double mA, mB, mC;
        double y1, y2;
        double mSampleRate;
    };

    // Liljencrants-Fant (LF) Derivative Glottal Pulse Model
    class SovereignLFGlottal {
    public:
        SovereignLFGlottal(double sampleRate = 44100.0);
        void SetFrequency(double freq);
        double GetNextSample(); 

    private:
        double mSampleRate;
        double mFrequency;
        double mPhase;
    };

    // Master Voice Synthesizer
    class SovereignAcoustics {
    public:
        // Converts mathematical sequence native strings into biological soundwaves
        static void Speak(const std::string& text);
        
    private:
        static void AppendVowel(std::vector<int16_t>& buffer, double F1, double F2, double F3, double durationSec);
        static void AppendNoise(std::vector<int16_t>& buffer, double frequency, double durationSec);
        static void WriteWAV(const std::string& filename, const std::vector<int16_t>& pcmData);
        static void PlayHardwareWAV(const std::string& filename);
    };

} // namespace Sovereign
