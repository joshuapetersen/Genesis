// ============================================================
// SOVEREIGN ASR v1.0 — Windows SAPI5 Microphone Input
// NO ATL dependency — raw COM SAPI5 interfaces only
// ============================================================
#include "Sovereign_ASR.h"
#include <windows.h>
#include <sapi.h>
#include <iostream>
#include <string>
#include <algorithm>

#pragma comment(lib, "sapi.lib")

// ISpNotifySource is not always in sapi.h automatically; declare what we need
#ifndef SPEI_RECOGNITION
#define SPEI_RECOGNITION 53
#endif

namespace Sovereign {

    static std::string WideToUtf8(const WCHAR* wide) {
        if (!wide) return "";
        int len = WideCharToMultiByte(CP_UTF8, 0, wide, -1, nullptr, 0, nullptr, nullptr);
        if (len <= 0) return "";
        std::string result(len - 1, '\0');
        WideCharToMultiByte(CP_UTF8, 0, wide, -1, &result[0], len, nullptr, nullptr);
        return result;
    }

    // Pull one recognition result text from the event queue. Returns "" if none.
    static std::string DrainRecoEvents(ISpRecoContext* pCtx) {
        std::string found;
        SPEVENT evt;
        ULONG fetched = 0;
        while (SUCCEEDED(pCtx->GetEvents(1, &evt, &fetched)) && fetched > 0) {
            if (evt.eEventId == SPEI_RECOGNITION && evt.elParamType == SPET_LPARAM_IS_OBJECT) {
                ISpRecoResult* pResult = reinterpret_cast<ISpRecoResult*>(evt.lParam);
                if (pResult) {
                    WCHAR* pText = nullptr;
                    if (SUCCEEDED(pResult->GetText(SP_GETWHOLEPHRASE, SP_GETWHOLEPHRASE,
                                                    TRUE, &pText, nullptr)) && pText) {
                        if (found.empty()) found = WideToUtf8(pText);
                        CoTaskMemFree(pText);
                    }
                    pResult->Release();
                }
            }
            // Release any other param objects
            if (evt.elParamType == SPET_LPARAM_IS_OBJECT && evt.eEventId != SPEI_RECOGNITION) {
                IUnknown* pUnk = reinterpret_cast<IUnknown*>(evt.lParam);
                if (pUnk) pUnk->Release();
            }
        }
        return found;
    }

    SovereignASR::SovereignASR()
        : mRecognizer(nullptr), mContext(nullptr),
          mGrammar(nullptr), mNotifyEvent(nullptr), mRunning(false) {
    }

    SovereignASR::~SovereignASR() {
        Stop();
        if (mGrammar)    { ((ISpRecoGrammar*)mGrammar)->Release();    mGrammar    = nullptr; }
        if (mContext)    { ((ISpRecoContext*)mContext)->Release();     mContext    = nullptr; }
        if (mRecognizer) { ((ISpRecognizer*)mRecognizer)->Release();   mRecognizer = nullptr; }
        if (mNotifyEvent){ CloseHandle((HANDLE)mNotifyEvent);          mNotifyEvent = nullptr; }
        CoUninitialize();
    }

    bool SovereignASR::Init() {
        HRESULT hr = CoInitialize(nullptr);
        if (FAILED(hr) && hr != S_FALSE) {
            std::cerr << "[ASR] CoInitialize failed: 0x" << std::hex << hr << std::endl;
            return false;
        }

        // Shared recognizer = uses the system default microphone (already selected in Windows)
        ISpRecognizer* pRec = nullptr;
        hr = CoCreateInstance(CLSID_SpSharedRecognizer, nullptr,
                              CLSCTX_ALL, IID_ISpRecognizer, (void**)&pRec);
        if (FAILED(hr)) {
            std::cerr << "[ASR] SpSharedRecognizer failed: 0x" << std::hex << hr << std::endl;
            std::cerr << "[ASR] Enable Windows Speech Recognition: Settings > Time & Language > Speech" << std::endl;
            CoUninitialize();
            return false;
        }
        mRecognizer = pRec;

        // Reco context: where events come through
        ISpRecoContext* pCtx = nullptr;
        hr = pRec->CreateRecoContext(&pCtx);
        if (FAILED(hr)) {
            std::cerr << "[ASR] CreateRecoContext failed: 0x" << std::hex << hr << std::endl;
            return false;
        }
        mContext = pCtx;

        // Win32 event notification: SAPI signals this when recognition fires
        HANDLE hEvent = CreateEvent(nullptr, FALSE, FALSE, nullptr);
        if (!hEvent) { std::cerr << "[ASR] CreateEvent failed" << std::endl; return false; }
        mNotifyEvent = hEvent;

        hr = pCtx->SetNotifyWin32Event();
        if (FAILED(hr)) {
            std::cout << "[ASR] SetNotifyWin32Event not available, using polling mode." << std::endl;
        } else {
            // GetNotifyEventHandle returns the HANDLE directly (no args)
            HANDLE hSapiEvent = pCtx->GetNotifyEventHandle();
            if (hSapiEvent && hSapiEvent != INVALID_HANDLE_VALUE) {
                CloseHandle(hEvent);
                mNotifyEvent = hSapiEvent;
            }
        }

        // Subscribe to recognition events only
        ULONGLONG interest = SPFEI(SPEI_RECOGNITION) | SPFEI(SPEI_FALSE_RECOGNITION);
        pCtx->SetInterest(interest, interest);

        // Open dictation grammar (accepts any speech — no word list constraint)
        ISpRecoGrammar* pGram = nullptr;
        hr = pCtx->CreateGrammar(0, &pGram);
        if (FAILED(hr)) {
            std::cerr << "[ASR] CreateGrammar failed: 0x" << std::hex << hr << std::endl;
            return false;
        }
        mGrammar = pGram;

        hr = pGram->LoadDictation(nullptr, SPLO_STATIC);
        if (FAILED(hr)) {
            std::cerr << "[ASR] LoadDictation failed: 0x" << std::hex << hr << std::endl;
            return false;
        }

        hr = pGram->SetDictationState(SPRS_ACTIVE);
        if (FAILED(hr)) {
            std::cerr << "[ASR] SetDictationState failed: 0x" << std::hex << hr << std::endl;
            return false;
        }

        std::cout << "[ASR] SAPI5 microphone initialized. Ready." << std::endl;
        return true;
    }

    std::string SovereignASR::ListenOnce(int timeoutMs) {
        if (!mContext) return "";
        ISpRecoContext* pCtx = (ISpRecoContext*)mContext;

        int elapsed = 0;
        while (elapsed < timeoutMs) {
            if (mNotifyEvent) {
                DWORD r = WaitForSingleObject((HANDLE)mNotifyEvent, 100);
                if (r != WAIT_OBJECT_0 && r != WAIT_TIMEOUT) break;
            } else {
                Sleep(100);
            }
            elapsed += 100;
            std::string text = DrainRecoEvents(pCtx);
            if (!text.empty()) return text;
        }
        return "";
    }

    void SovereignASR::ListenLoop(ASRCallback callback) {
        if (!mContext) return;
        mRunning = true;
        ISpRecoContext* pCtx = (ISpRecoContext*)mContext;

        std::cout << "[ASR] Listening... (say 'stop' or 'exit' to end)" << std::endl;

        while (mRunning) {
            if (mNotifyEvent) {
                WaitForSingleObject((HANDLE)mNotifyEvent, 300);
            } else {
                Sleep(300);
            }

            std::string text = DrainRecoEvents(pCtx);
            if (!text.empty() && mRunning) {
                bool keepGoing = true;
                try {
                    callback(text);
                } catch (...) {
                    keepGoing = false;
                }
                if (!keepGoing) mRunning = false;
            }
        }
        std::cout << "[ASR] Stopped." << std::endl;
    }

    void SovereignASR::Stop() {
        mRunning = false;
        if (mGrammar) ((ISpRecoGrammar*)mGrammar)->SetDictationState(SPRS_INACTIVE);
    }

} // namespace Sovereign
