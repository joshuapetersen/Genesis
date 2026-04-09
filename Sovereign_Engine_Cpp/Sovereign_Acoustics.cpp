#include "Sovereign_Acoustics.h"
#include <fstream>
#include <cmath>
#include <iostream>
#include <windows.h>
#pragma comment(lib, "winmm.lib")

#ifndef M_PI
#define M_PI 3.14159265358979323846
#endif

namespace Sovereign {

    // --- IIR RESONATOR IMPLEMENTATION ---
    SovereignIIRResonator::SovereignIIRResonator(double sampleRate) 
        : mSampleRate(sampleRate), y1(0), y2(0) { SetFormant(500.0, 50.0); }

    void SovereignIIRResonator::SetFormant(double frequency, double bandwidth) {
        double R = exp(-M_PI * bandwidth / mSampleRate);
        mC = 2.0 * R * cos(2.0 * M_PI * frequency / mSampleRate);
        mB = -(R * R);
        mA = 1.0 - mC - mB; // Gain normalization to prevent clipping natively
    }

    double SovereignIIRResonator::Process(double input) {
        double output = (mA * input) + (mC * y1) + (mB * y2);
        y2 = y1;
        y1 = output;
        return output;
    }

    // --- LF GLOTTAL (ROSENBERG APPROXIMATION) ---
    SovereignLFGlottal::SovereignLFGlottal(double sampleRate) 
        : mSampleRate(sampleRate), mFrequency(120.0), mPhase(0.0) {}

    void SovereignLFGlottal::SetFrequency(double freq) { mFrequency = freq; }

    double SovereignLFGlottal::GetNextSample() {
        mPhase += mFrequency / mSampleRate;
        if (mPhase >= 1.0) mPhase -= 1.0;

        // Rosenberg glottal pulse mathematical modeling
        double t = mPhase;
        double Te = 0.4; // Open quotient
        double excitation = 0.0;

        if (t < Te) {
            double normalized = t / Te;
            excitation = 0.5 * (1.0 - cos(M_PI * normalized)); 
        } else if (t < Te + 0.16) {
            double normalized = (t - Te) / 0.16;
            excitation = cos(M_PI * 0.5 * normalized);
        }
        
        // Derivative for radiation characteristic (buzz generator)
        static double lastExcitation = 0.0;
        double deriv = excitation - lastExcitation;
        lastExcitation = excitation;
        
        return deriv * 10.0; // Boost raw amplitude 
    }

    // --- MASTER ACOUSTICS ---
    void SovereignAcoustics::AppendVowel(std::vector<int16_t>& buffer, double F1, double F2, double F3, double durationSec) {
        SovereignLFGlottal glottal;
        glottal.SetFrequency(110.0); // Sovereign Pitch (Deep, resonant, non-robotic)

        SovereignIIRResonator r1; r1.SetFormant(F1, 50.0);
        SovereignIIRResonator r2; r2.SetFormant(F2, 70.0);
        SovereignIIRResonator r3; r3.SetFormant(F3, 110.0);

        int numSamples = (int)(durationSec * 44100.0);
        for (int i = 0; i < numSamples; i++) {
            double source = glottal.GetNextSample();
            
            // Cascade architecture (Serial Filter Math)
            double filtered = r1.Process(source);
            filtered = r2.Process(filtered);
            filtered = r3.Process(filtered);

            // Scale to 16-bit PCM space smoothly
            filtered *= 32000.0;
            if (filtered > 32767.0) filtered = 32767.0;
            if (filtered < -32768.0) filtered = -32768.0;

            buffer.push_back((int16_t)filtered);
        }
    }

    void SovereignAcoustics::AppendNoise(std::vector<int16_t>& buffer, double frequency, double durationSec) {
        SovereignIIRResonator hp; hp.SetFormant(frequency, 500.0); // High pass friction

        int numSamples = (int)(durationSec * 44100.0);
        for (int i = 0; i < numSamples; i++) {
            double noise = ((double)rand() / RAND_MAX) * 2.0 - 1.0;
            double filtered = hp.Process(noise) * 15000.0; // lower volume for noise
            
            if (filtered > 32767.0) filtered = 32767.0;
            if (filtered < -32768.0) filtered = -32768.0;

            buffer.push_back((int16_t)filtered);
        }
    }

    void SovereignAcoustics::WriteWAV(const std::string& filename, const std::vector<int16_t>& pcmData) {
        WavHeader header;
        header.subchunk2Size = pcmData.size() * sizeof(int16_t);
        header.chunkSize = 36 + header.subchunk2Size;

        std::ofstream file(filename, std::ios::binary);
        if (file) {
            file.write(reinterpret_cast<const char*>(&header), sizeof(WavHeader));
            file.write(reinterpret_cast<const char*>(pcmData.data()), header.subchunk2Size);
            file.close();
        }
    }

    void SovereignAcoustics::PlayHardwareWAV(const std::string& filename) {
        // Raw underlying OS interrupt to hardware DAC. Bypasses wrappers. Assumes asynchronous Win32 thread execution.
        PlaySoundA(filename.c_str(), NULL, SND_FILENAME | SND_ASYNC);
    }

    void SovereignAcoustics::Speak(const std::string& text) {
        std::vector<int16_t> audioBuffer;
        
        // Zero-Dependency rule-based phoneme extraction matrix.
        for (char c : text) {
            char lower = tolower(c);
            // Dynamic Formant Mappings (F1, F2, F3)
            if (lower == 'a') AppendVowel(audioBuffer, 730, 1090, 2440, 0.2);
            else if (lower == 'e') AppendVowel(audioBuffer, 530, 1840, 2480, 0.15);
            else if (lower == 'i') AppendVowel(audioBuffer, 270, 2290, 3010, 0.15);
            else if (lower == 'o') AppendVowel(audioBuffer, 570, 840, 2410, 0.2);
            else if (lower == 'u') AppendVowel(audioBuffer, 300, 870, 2240, 0.2);
            else if (lower == 's') AppendNoise(audioBuffer, 5000, 0.1);
            else if (lower == 't') AppendNoise(audioBuffer, 3500, 0.05);
            else if (lower == 'k') AppendNoise(audioBuffer, 1500, 0.08);
            else if (lower == ' ') { /* Brief silence */
                for(int i=0; i<4410; i++) audioBuffer.push_back(0); 
            }
        }
        
        std::string outPath = "C:\\GENESIS\\Sovereign_Voice.wav";
        WriteWAV(outPath, audioBuffer);
        std::cout << "[ACOUSTICS] DSP Formants applied natively. Generating acoustic pressure..." << std::endl;
        PlayHardwareWAV(outPath);
    }

} // namespace Sovereign
