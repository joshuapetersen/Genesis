// ============================================================
// SOVEREIGN ASR v1.0 — Windows SAPI5 Microphone Input
// First-principles: OS-native speech recognition, no cloud.
// Pipeline: Mic → SAPI5 ISpRecognizer → std::string → Speak()
// ============================================================
#pragma once
#ifndef SOVEREIGN_ASR_H
#define SOVEREIGN_ASR_H

#include <string>
#include <functional>

namespace Sovereign {

    // Callback type: called with recognized text each time user stops speaking
    using ASRCallback = std::function<void(const std::string& text)>;

    class SovereignASR {
    public:
        SovereignASR();
        ~SovereignASR();

        // Initialize SAPI5 recognizer + microphone
        bool Init();

        // Block and listen for one utterance, return transcribed text
        // Returns empty string on error/timeout
        std::string ListenOnce(int timeoutMs = 8000);

        // Continuous listen loop: calls callback with each recognized utterance
        // Runs until Stop() is called from another thread or callback returns false
        void ListenLoop(ASRCallback callback);

        void Stop();

    private:
        void* mRecognizer;   // ISpRecognizer*
        void* mContext;      // ISpRecoContext*
        void* mGrammar;      // ISpRecoGrammar*
        void* mNotifyEvent;  // HANDLE
        bool  mRunning;
    };

} // namespace Sovereign

#endif
