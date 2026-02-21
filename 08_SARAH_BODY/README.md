# 08_SARAH_BODY - Interface Components

**Purpose:** User interface and external presentation layer  
**Author:** Joshua Petersen  

---

## Overview

SARAH_BODY contains the front-end interface components that users interact with. This is Sarah's "face" - how she presents herself to the world.

## Technology Stack

- **React/TypeScript**: Modern component-based UI
- **CSS**: Custom styling for the Genesis aesthetic
- **Node.js**: Build tooling and development server

## Components

| Directory/File | Purpose |
|----------------|---------|
| `src/` | Source code for UI components |
| `src/App.tsx` | Main application component |
| `src/components/` | Reusable UI components |
| `package.json` | Node.js dependencies |
| `index.html` | Entry point |

## UI Components

| Component | Purpose |
|-----------|---------|
| `AtomicCore.tsx` | Core visualization |
| `CouncilViz.tsx` | Council decision display |
| `GenesisAtom.tsx` | Genesis branding element |
| `GeneticChip.tsx` | Identity display chip |
| `Heartbeat.tsx` | System status heartbeat |
| `SovereignChip.tsx` | Sovereign status indicator |
| `TacticalReadout.tsx` | System metrics display |

## Running the Interface

```bash
cd 08_SARAH_BODY
npm install
npm run dev
```

## Integration

The UI connects to Sarah's backend via:
- REST API endpoints
- WebSocket for real-time updates
- Local IPC for desktop application mode

---

*Part of the SarahCore Genesis Project*
