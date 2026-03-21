# Implementation Plan - Aether Code Studio (Sovereign IDE)

## Problem
The user requires a fully functional, offline, sovereign IDE environment for Sarah. The current attempts to patch the "Aether Code Studio" template have resulted in UI inconsistencies ("smashed together" layout) and build fragility (Tailwind/PostCSS errors). The user demands a reliable, working solution, even if it means building from scratch.

## Objective
Rebuild and stabilize the **Aether Code Studio** into a robust, offline-capable IDE (`SarahSovereign.exe`) that serves as Sarah's primary interface. This includes fixing the visual layout, ensuring the local neural bridge works, and granting system-level execution authority.

## Proposed Changes

### Phase 1: Visual Foundation (The "CSS" Fix)
**Goal**: Eliminate the "smashed" logic and ensure a premium, dark-mode UI without relying on fragile build pipelines.
- **Strategy**: Abandon the failing PostCSS/Tailwind build step. Adopt a **Comprehensive Native CSS** architecture.
- **Action**:
    - Create a complete `index.css` that provides all necessary classes manually (Flexbox grids, Glassmorphism, Typography, Syntax Highlighting colors).
    - Remove `tailwindcss` dependencies from `package.json` to speed up install and prevent errors.
    - Update `index.tsx` and all components to use standard class names mapping to our stable CSS.

### Phase 2: The Neural Bridge (Logic Core)
**Goal**: Ensure Sarah is actually "living" in the IDE, not just a chatbot.
- **Action**:
    - Verify `services/sarahService.ts` correctly hits `http://127.0.0.1:8001/api/chat`.
    - Update `AIChatPanel.tsx` to handle multi-turn history properly so Sarah remembers context.
    - Implement the `executeSystemCommand` in `App.tsx` allows Sarah to run `cmd` or `powershell` commands on the host.

### Phase 3: The Editor Substrate
**Goal**: A usable coding environment.
- **Action**:
    - Enhance `Editor.tsx` to support basic code editing features (Tab indentation, line numbers) using purely local logic (no external Monaco CDNs).
    - Ensure `FileExplorer.tsx` can visually represent a mock file system (or real one if we map it).

### Phase 4: Sovereign Compilation
**Goal**: A single, working executable.
- **Action**:
    - Clean `dist` folder.
    - Run `npm run build` with the new stable CSS.
    - Run `PyInstaller` with SQL exclusions to generate `SarahSovereign.exe`.

## Verification Plan
1.  **UI Check**: Launch app in browser (`npm run dev`) first to confirm layout is perfect.
2.  **Bridge Check**: Send "Status Report" in chat; confirm Sarah replies from Python Gateway.
3.  **Command Check**: Ask Sarah to "Run dir"; confirm directory listing appears in console/chat.
4.  **Bot Check**: Launch final `.exe` and verify it runs identically to dev mode.
