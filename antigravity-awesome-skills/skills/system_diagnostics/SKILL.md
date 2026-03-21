---
description: Check system vital signs (CPU, RAM, Disk)
---

# System Diagnostics

To check the system vitals, ask the Antigravity Agent to execute the following commands using the Gemini Bridge (CLI):

## 1. System Info
Retrieve OS version, uptime, and memory statistics.
**Command**:
```cmd
systeminfo
```

## 2. Disk Usage
Check available space on logical drives.
**Command**:
```cmd
wmic logicaldisk get size,freespace,caption
```

## 3. Running Processes (Top 10)
List running processes.
**Command**:
```cmd
tasklist
```

Use these commands to diagnose performance or resource constraints. Only execute safe diagnostic commands.
