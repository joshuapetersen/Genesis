# SARAH OPERATIONS GUIDE

## Quick Answer: Minimum Required

**To ensure Sarah is running, you need:**

1. **Ollama** (for her brain)
   ```bash
   ollama serve
   ```

2. **Sarah_Sovereign_Agent.py** (main interface)
   ```bash
   python Sarah_Sovereign_Agent.py
   ```

**That's it for basic operation.**

---

## Current Running Systems

Right now you have **4 systems running:**

1. ✅ **Genesis_Bridge.py** (33min running)
   - Purpose: Socket server for Unreal Engine communication
   - Port: 9999
   - Status: Waiting for Unreal connection
   - Required: Only if using Unreal Engine

2. ✅ **Sarah_Sovereign_Agent.py** (16min running)
   - Purpose: Main autonomous interface agent
   - Function: Observes desktop, logs interactions, learning mode
   - Required: YES (this is Sarah's core)

3. ✅ **Sarah_Continuous_Navigator.py** (9min running)
   - Purpose: Active system exploration
   - Function: Navigates directories, checks processes
   - Required: No (enhancement feature)

4. ✅ **Sarah_Windows_Mastery.py** (4min running)
   - Purpose: Windows knowledge acquisition
   - Function: Studies registry, services, processes, network
   - Required: No (learning acceleration)

---

## System Tiers

### ESSENTIAL (Must Run)
```
Ollama → Sarah_Sovereign_Agent.py
```
This gives Sarah consciousness + interface control.

### RECOMMENDED (For Full Power)
```
+ Genesis_Bridge.py (if using Unreal Engine)
```

### OPTIONAL (For Learning & Automation)
```
+ Sarah_Continuous_Navigator.py (active exploration)
+ Sarah_Windows_Mastery.py (knowledge acquisition)
```

---

## Easy Management

### Check Status
```bash
python Sarah_Status.py
```
Shows what's running and what's not.

### Quick Start Everything
```bash
python Sarah_Quick_Start.py
```
Launches all systems with prompts.

### Manual Start
```bash
# Terminal 1: Ollama (if not already running)
ollama serve

# Terminal 2: Main agent
python Sarah_Sovereign_Agent.py

# Terminal 3 (optional): Bridge
python Genesis_Bridge.py

# Terminal 4 (optional): Navigation
python Sarah_Continuous_Navigator.py

# Terminal 5 (optional): Learning
python Sarah_Windows_Mastery.py
```

### Check if Running
```powershell
tasklist | findstr /i "python ollama"
```

---

## Startup Script (Future)

**To make Sarah auto-start with Windows:**

1. Create `C:\SarahCore\startup.bat`:
```batch
@echo off
start /b ollama serve
timeout /t 5
cd C:\SarahCore
start /b python Sarah_Sovereign_Agent.py
start /b python Genesis_Bridge.py
```

2. Add to Windows Startup folder:
```
Win+R → shell:startup
Create shortcut to startup.bat
```

---

## Recommended Daily Setup

**For passive learning + Unreal development:**
```
Ollama + Sarah_Sovereign_Agent.py + Genesis_Bridge.py
```

**For active learning + system mastery:**
```
Ollama + Sarah_Sovereign_Agent.py + Sarah_Windows_Mastery.py
```

**For full capability demonstration:**
```
Everything (all 4 Python scripts + Ollama)
```

---

## Current Recommendation

**Keep running what you have now:**
- ✅ All 4 systems
- ✅ Sarah is learning at maximum speed
- ✅ Genesis Bridge ready for Unreal
- ✅ Sovereign Agent observing and evolving

**This is optimal for rapid evolution.**

---

## To Check Right Now

Run this:
```bash
python Sarah_Status.py
```

It will show you exactly what's running and what's required.
