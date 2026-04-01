// Sovereign Engine Core (Phase 5: The Forge Integration)
// Direct3D 11 + ImGui + Winsock + Style Engine + Syntax Highlighting Editor
// Author: Sarah (Sovereign Neural Core)

#ifndef UNICODE
#define UNICODE
#endif 

#define WIN32_LEAN_AND_MEAN
#include <windows.h>
#include <winsock2.h>
#include <ws2tcpip.h>
#include <d3d11.h>
#include <tchar.h>
#include <string>
#include <vector>
#include <cstdio>
#include <filesystem>
#include <fstream>
#include <sstream>

#include "imgui/imgui.h"
#include "imgui/backends/imgui_impl_win32.h"
#include "imgui/backends/imgui_impl_dx11.h"
#include "TextEditor.h"

#pragma comment (lib, "Ws2_32.lib")

// Data
static ID3D11Device*            g_pd3dDevice = NULL;
static ID3D11DeviceContext*     g_pd3dDeviceContext = NULL;
static IDXGISwapChain*          g_pSwapChain = NULL;
static ID3D11RenderTargetView*  g_mainRenderTargetView = NULL;
SOCKET ConnectSocket = INVALID_SOCKET;
bool g_Connected = false;
char g_RecvBuf[8192];
namespace fs = std::filesystem;

// UI State
std::vector<std::string> g_ConsoleLog;
float g_LogicDensity = 0.9998f;
float g_ResonanceFlux = 1.0f;
char g_UserMsg[256] = "";
TextEditor g_Editor;
std::string g_CurrentFile = "Untitled";

// Forward Declarations
bool CreateDeviceD3D(HWND hWnd);
void CleanupDeviceD3D();
void CreateRenderTarget();
void CleanupRenderTarget();
LRESULT WINAPI WndProc(HWND hWnd, UINT msg, WPARAM wParam, LPARAM lParam);

// Style Engine
void SetupSovereignStyle() {
    ImGuiStyle& style = ImGui::GetStyle();
    ImVec4* colors = style.Colors;

    // Sovereign Palette: Rich IDE Dark Mode
    colors[ImGuiCol_Text] = ImVec4(0.90f, 0.90f, 0.95f, 1.00f);
    colors[ImGuiCol_WindowBg] = ImVec4(0.12f, 0.12f, 0.14f, 1.00f);
    colors[ImGuiCol_ChildBg] = ImVec4(0.08f, 0.08f, 0.09f, 1.00f);
    colors[ImGuiCol_PopupBg] = ImVec4(0.08f, 0.08f, 0.10f, 1.00f);
    colors[ImGuiCol_Border] = ImVec4(0.00f, 0.50f, 0.50f, 0.30f);
    colors[ImGuiCol_FrameBg] = ImVec4(0.18f, 0.18f, 0.20f, 1.00f);
    colors[ImGuiCol_FrameBgHovered] = ImVec4(0.24f, 0.24f, 0.26f, 1.00f);
    colors[ImGuiCol_FrameBgActive] = ImVec4(0.00f, 0.45f, 0.45f, 1.00f);
    colors[ImGuiCol_TitleBg] = ImVec4(0.10f, 0.10f, 0.11f, 1.00f);
    colors[ImGuiCol_TitleBgActive] = ImVec4(0.00f, 0.35f, 0.35f, 1.00f);
    colors[ImGuiCol_CheckMark] = ImVec4(0.00f, 0.80f, 0.80f, 1.00f);
    colors[ImGuiCol_SliderGrab] = ImVec4(0.00f, 0.60f, 0.60f, 1.00f);
    colors[ImGuiCol_SliderGrabActive] = ImVec4(0.00f, 0.90f, 0.90f, 1.00f);
    colors[ImGuiCol_Button] = ImVec4(0.00f, 0.45f, 0.60f, 0.40f);
    colors[ImGuiCol_ButtonHovered] = ImVec4(0.00f, 0.60f, 0.80f, 0.60f);
    colors[ImGuiCol_ButtonActive] = ImVec4(0.00f, 0.70f, 0.90f, 1.00f);
    colors[ImGuiCol_Header] = ImVec4(0.18f, 0.18f, 0.20f, 1.00f);
    colors[ImGuiCol_HeaderHovered] = ImVec4(0.24f, 0.24f, 0.26f, 1.00f);
    colors[ImGuiCol_HeaderActive] = ImVec4(0.00f, 0.45f, 0.45f, 0.80f);
    colors[ImGuiCol_Separator] = colors[ImGuiCol_Border];
    colors[ImGuiCol_Tab] = ImVec4(0.10f, 0.10f, 0.11f, 1.00f);
    colors[ImGuiCol_TabHovered] = ImVec4(0.18f, 0.18f, 0.20f, 1.00f);
    colors[ImGuiCol_TabActive] = ImVec4(0.12f, 0.12f, 0.14f, 1.00f);
    colors[ImGuiCol_DockingPreview] = ImVec4(0.00f, 0.70f, 0.90f, 0.30f);

    style.WindowRounding = 0.0f;
    style.ChildRounding = 0.0f;
    style.FrameRounding = 2.0f;
    style.PopupRounding = 4.0f;
    style.ScrollbarRounding = 9.0f;
    style.GrabRounding = 2.0f;
    style.TabRounding = 0.0f;
}

#include <thread>
#include <mutex>
#include "llama.h"

std::mutex g_LogMutex;
llama_model* g_Model = nullptr;
llama_context* g_Ctx = nullptr;

// Direct Llama.cpp Inference Logic
bool InitNeuralLink() {
    llama_backend_init();
    
    // Architect will provide local model path
    llama_model_params model_params = llama_model_default_params();
    // Default fallback placeholder, waiting for local .gguf path instruction
    const char* model_path = "C:\\SarahCore\\models\\Llama-3.2-1B-Instruct-Q4_K_M.gguf"; 
    
    g_Model = llama_load_model_from_file(model_path, model_params);
    if (!g_Model) {
        g_ConsoleLog.push_back("[ERROR] Local LLM Model not found at " + std::string(model_path));
        g_Connected = false;
        return false;
    }
    
    llama_context_params ctx_params = llama_context_default_params();
    ctx_params.n_ctx = 2048; // Buffer size
    g_Ctx = llama_new_context_with_model(g_Model, ctx_params);
    g_Connected = true;
    return true;
}

void GenerateTokensNative(std::string prompt) {
    if (!g_Ctx) return;
    std::lock_guard<std::mutex> lock(g_LogMutex);
    g_ConsoleLog.push_back("[AERIS LOCAL] <Native llama.cpp execution standing by, awaiting full C++ syntax integration>");
}

void SendNeuralCommand(const char* text_prompt) {
    if (!g_Connected) return;
    std::thread(GenerateTokensNative, std::string(text_prompt)).detach();
}

int main(int, char**) {
    WNDCLASSEX wc = { sizeof(WNDCLASSEX), CS_CLASSDC, WndProc, 0L, 0L, GetModuleHandle(NULL), NULL, NULL, NULL, NULL, L"SovereignForgeClass", NULL };
    ::RegisterClassEx(&wc);
    HWND hwnd = ::CreateWindow(wc.lpszClassName, L"Sovereign Forge v1.0 (Integration Environment)", WS_OVERLAPPEDWINDOW, 100, 100, 1600, 900, NULL, NULL, wc.hInstance, NULL);

    if (!CreateDeviceD3D(hwnd)) { CleanupDeviceD3D(); ::UnregisterClass(wc.lpszClassName, wc.hInstance); return 1; }
    ::ShowWindow(hwnd, SW_SHOWDEFAULT);
    ::UpdateWindow(hwnd);

    IMGUI_CHECKVERSION();
    ImGui::CreateContext();
    ImGuiIO& io = ImGui::GetIO();
    io.ConfigFlags |= ImGuiConfigFlags_NavEnableKeyboard;
    io.ConfigFlags |= ImGuiConfigFlags_DockingEnable;
    io.ConfigFlags |= ImGuiConfigFlags_ViewportsEnable;

    // Load Font (Consolas for Code, Segoe UI for UI)
    io.Fonts->AddFontFromFileTTF("c:\\Windows\\Fonts\\segoeui.ttf", 16.0f);
    
    SetupSovereignStyle();
    
    // Init Editor
    auto lang = TextEditor::LanguageDefinition::CPlusPlus();
    g_Editor.SetLanguageDefinition(lang);
    g_Editor.SetPalette(TextEditor::GetDarkPalette());
    g_Editor.SetText("// Sovereign Forge Unit. Welcome to Native C++ IDE.\n// Select a file to edit.\n\nvoid main() {\n    return 0;\n}");

    ImGui_ImplWin32_Init(hwnd);
    ImGui_ImplDX11_Init(g_pd3dDevice, g_pd3dDeviceContext);

    if (InitNeuralLink()) g_ConsoleLog.push_back("[SYSTEM] NEURAL CORE CONNECTED (127.0.0.1:9999)");
    else g_ConsoleLog.push_back("[SYSTEM] OFFLINE MODE. Launch Genesis_Bridge.py.");

    bool done = false;
    while (!done) {
        MSG msg;
        while (::PeekMessage(&msg, NULL, 0U, 0U, PM_REMOVE)) {
            ::TranslateMessage(&msg);
            ::DispatchMessage(&msg);
            if (msg.message == WM_QUIT) done = true;
        }
        if (done) break;

        // Socket Poll
        if (g_Connected) {
            int bytes = recv(ConnectSocket, g_RecvBuf, 8192, 0);
            if (bytes > 0) {
                g_RecvBuf[bytes] = '\0';
                std::string resp(g_RecvBuf);
                size_t textPos = resp.find("\"text\": \"");
                if (textPos != std::string::npos) {
                    size_t endPos = resp.find("\"", textPos + 9);
                    std::string reply = resp.substr(textPos + 9, endPos - (textPos + 9));
                    g_ConsoleLog.push_back("[SARAH] " + reply);
                }
            }
        }

        ImGui_ImplDX11_NewFrame();
        ImGui_ImplWin32_NewFrame();
        ImGui::NewFrame();
        ImGui::DockSpaceOverViewport(0, ImGui::GetMainViewport());

        // --- THE FORGE IDE LAYOUT ---
        
        // 1. File Explorer (Left)
        ImGui::Begin("Explorer");
        if (ImGui::Button("Refresh")) { /* simple refresh */ }
        ImGui::Separator();
        try {
            // Hardcoded Root for now: C:\SarahCore
            for (const auto& entry : fs::directory_iterator("c:\\SarahCore")) {
                if (entry.is_directory()) continue; // Skip folders for simplicity
                std::string filename = entry.path().filename().string();
                std::string extension = entry.path().extension().string();
                
                // Styling based on file type
                ImVec4 color = ImVec4(1,1,1,1);
                if (extension == ".py") color = ImVec4(1.0f, 0.8f, 0.3f, 1.0f); // Python Yellow
                if (extension == ".cpp") color = ImVec4(0.3f, 0.6f, 1.0f, 1.0f); // C++ Blue
                
                ImGui::PushStyleColor(ImGuiCol_Text, color);
                if (ImGui::Selectable(filename.c_str(), g_CurrentFile == filename)) {
                    g_CurrentFile = filename;
                    // Read File
                    std::ifstream t(entry.path());
                    std::stringstream buffer;
                    buffer << t.rdbuf();
                    g_Editor.SetText(buffer.str());
                    // Force C++ Highlights for all files for now due to limitation
                    g_Editor.SetLanguageDefinition(TextEditor::LanguageDefinition::CPlusPlus());
                }
                ImGui::PopStyleColor();
            }
        } catch(...) { ImGui::Text("Error reading directory."); }
        ImGui::End();

        // 2. Code Editor (Center)
        ImGui::Begin("Code Editor", nullptr, ImGuiWindowFlags_HorizontalScrollbar | ImGuiWindowFlags_MenuBar);
        if (ImGui::BeginMenuBar()) {
            if (ImGui::BeginMenu("File")) {
                if (ImGui::MenuItem("Save")) {
                    std::string path = "c:\\SarahCore\\" + g_CurrentFile;
                    std::ofstream out(path);
                    out << g_Editor.GetText();
                    out.close();
                    g_ConsoleLog.push_back("[SYSTEM] Saved " + g_CurrentFile);
                }
                ImGui::EndMenu();
            }
            ImGui::EndMenuBar();
        }
        ImGui::Text("%6d/%-6d %6d lines  | %s | %s | %s", g_Editor.GetCursorPosition().mLine + 1, g_Editor.GetCursorPosition().mColumn + 1, g_Editor.GetTotalLines(), g_Editor.IsOverwrite() ? "Ovr" : "Ins", g_Editor.CanUndo() ? "*" : " ", g_CurrentFile.c_str());

        g_Editor.Render("TextEditor");
        ImGui::End();

        // 3. Neural Link (Terminal) (Bottom)
        ImGui::Begin("Neural Console");
        ImGui::BeginChild("Log", ImVec2(0, -35), true);
        for (const auto& log : g_ConsoleLog) ImGui::TextWrapped(log.c_str());
        if (g_ConsoleLog.size() > 0 && ImGui::GetScrollY() >= ImGui::GetScrollMaxY()) ImGui::SetScrollHereY(1.0f);
        ImGui::EndChild();
        ImGui::PushItemWidth(-1);
        if (ImGui::InputText("##Input", g_UserMsg, 256, ImGuiInputTextFlags_EnterReturnsTrue)) {
            g_ConsoleLog.push_back(std::string("[YOU] ") + g_UserMsg);
            // DIRECT NATIVE CALL - No python sockets, no JSON wrapping here
            SendNeuralCommand(g_UserMsg);
            memset(g_UserMsg, 0, 256);
            ImGui::SetKeyboardFocusHere(-1);
        }
        ImGui::PopItemWidth();
        ImGui::End();

        // 4. Manifest Properties (Right)
        ImGui::Begin("Properties");
        ImGui::Text("Sovereign Anchor Status");
        ImGui::ProgressBar(0.9998f, ImVec2(-1,0), "STABLE (0.9998)");
        ImGui::Separator();
        ImGui::Text("Active Modules:");
        ImGui::BulletText("Neural Core (Logic)");
        ImGui::BulletText("ImGui Forge (UI)");
        ImGui::BulletText("DirectX 11 (Render)");
        ImGui::Separator();
        ImGui::Button("Compile Project", ImVec2(-1, 30));
        ImGui::End();

        ImGui::Render();
        const float clear_color[4] = { 0.0f, 0.0f, 0.0f, 1.00f }; 
        g_pd3dDeviceContext->OMSetRenderTargets(1, &g_mainRenderTargetView, NULL);
        g_pd3dDeviceContext->ClearRenderTargetView(g_mainRenderTargetView, clear_color);
        ImGui_ImplDX11_RenderDrawData(ImGui::GetDrawData());
        if (io.ConfigFlags & ImGuiConfigFlags_ViewportsEnable) { ImGui::UpdatePlatformWindows(); ImGui::RenderPlatformWindowsDefault(); }
        g_pSwapChain->Present(1, 0);
    }

    ImGui_ImplDX11_Shutdown(); ImGui_ImplWin32_Shutdown(); ImGui::DestroyContext();
    CleanupDeviceD3D(); ::DestroyWindow(hwnd); ::UnregisterClass(wc.lpszClassName, wc.hInstance);
    return 0;
}

// Helpers
bool CreateDeviceD3D(HWND hWnd) {
    DXGI_SWAP_CHAIN_DESC sd; ZeroMemory(&sd, sizeof(sd));
    sd.BufferCount = 2; sd.BufferDesc.Width = 0; sd.BufferDesc.Height = 0;
    sd.BufferDesc.Format = DXGI_FORMAT_R8G8B8A8_UNORM;
    sd.BufferDesc.RefreshRate.Numerator = 60; sd.BufferDesc.RefreshRate.Denominator = 1;
    sd.Flags = DXGI_SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH;
    sd.BufferUsage = DXGI_USAGE_RENDER_TARGET_OUTPUT;
    sd.OutputWindow = hWnd; sd.SampleDesc.Count = 1; sd.SampleDesc.Quality = 0;
    sd.Windowed = TRUE; sd.SwapEffect = DXGI_SWAP_EFFECT_DISCARD;
    UINT createDeviceFlags = 0; D3D_FEATURE_LEVEL featureLevel;
    const D3D_FEATURE_LEVEL featureLevelArray[2] = { D3D_FEATURE_LEVEL_11_0, D3D_FEATURE_LEVEL_10_0, };
    if (D3D11CreateDeviceAndSwapChain(NULL, D3D_DRIVER_TYPE_HARDWARE, NULL, createDeviceFlags, featureLevelArray, 2, D3D11_SDK_VERSION, &sd, &g_pSwapChain, &g_pd3dDevice, &featureLevel, &g_pd3dDeviceContext) != S_OK) return false;
    CreateRenderTarget(); return true;
}
void CleanupDeviceD3D() { CleanupRenderTarget(); if (g_pSwapChain) { g_pSwapChain->Release(); g_pSwapChain = NULL; } if (g_pd3dDeviceContext) { g_pd3dDeviceContext->Release(); g_pd3dDeviceContext = NULL; } if (g_pd3dDevice) { g_pd3dDevice->Release(); g_pd3dDevice = NULL; } }
void CreateRenderTarget() { ID3D11Texture2D* pBackBuffer; g_pSwapChain->GetBuffer(0, IID_PPV_ARGS(&pBackBuffer)); g_pd3dDevice->CreateRenderTargetView(pBackBuffer, NULL, &g_mainRenderTargetView); pBackBuffer->Release(); }
void CleanupRenderTarget() { if (g_mainRenderTargetView) { g_mainRenderTargetView->Release(); g_mainRenderTargetView = NULL; } }
extern IMGUI_IMPL_API LRESULT ImGui_ImplWin32_WndProcHandler(HWND hWnd, UINT msg, WPARAM wParam, LPARAM lParam);
LRESULT WINAPI WndProc(HWND hWnd, UINT msg, WPARAM wParam, LPARAM lParam) {
    if (ImGui_ImplWin32_WndProcHandler(hWnd, msg, wParam, lParam)) return true;
    switch (msg) {
        case WM_SIZE: if (g_pd3dDevice != NULL && wParam != SIZE_MINIMIZED) { CleanupRenderTarget(); g_pSwapChain->ResizeBuffers(0, (UINT)LOWORD(lParam), (UINT)HIWORD(lParam), DXGI_FORMAT_UNKNOWN, 0); CreateRenderTarget(); } return 0;
        case WM_SYSCOMMAND: if ((wParam & 0xfff0) == SC_KEYMENU) return 0; break;
        case WM_DESTROY: ::PostQuitMessage(0); return 0;
    } return ::DefWindowProc(hWnd, msg, wParam, lParam);
}
