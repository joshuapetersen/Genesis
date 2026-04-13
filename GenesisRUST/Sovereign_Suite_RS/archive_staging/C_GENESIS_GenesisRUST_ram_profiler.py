"""
RAM Profiler for 2GB Optimization Target
Identifies what's consuming memory and suggests optimizations.
"""

import psutil
import os
import sys


def profile_system():
    """Profile system RAM and identify heavy processes."""
    print("=" * 60)
    print("SARAH RAM PROFILER - 2GB Target Optimization")
    print("=" * 60)
    
    mem = psutil.virtual_memory()
    print(f"\n[SYSTEM MEMORY]")
    print(f"  Total RAM:     {mem.total / 1024 / 1024 / 1024:.1f} GB")
    print(f"  Used:          {mem.used / 1024 / 1024 / 1024:.2f} GB ({mem.percent}%)")
    print(f"  Available:     {mem.available / 1024 / 1024 / 1024:.2f} GB")
    print(f"  Target (2GB):  2.00 GB")
    print(f"  Over Target:   {max(0, mem.used / 1024 / 1024 / 1024 - 2):.2f} GB")
    
    print(f"\n[TOP MEMORY CONSUMERS]")
    procs = []
    for proc in psutil.process_iter(['pid', 'name', 'memory_info', 'cmdline']):
        try:
            info = proc.info
            rss = info['memory_info'].rss / 1024 / 1024 if info['memory_info'] else 0
            if rss > 50:  # Only show processes using > 50MB
                cmdline = ' '.join(info['cmdline'] or [])[:60] if info['cmdline'] else ''
                procs.append((rss, info['pid'], info['name'], cmdline))
        except (psutil.NoSuchProcess, psutil.AccessDenied):
            pass
    
    procs.sort(reverse=True)
    
    total_big = 0
    python_total = 0
    for rss, pid, name, cmdline in procs[:15]:
        is_python = 'python' in name.lower()
        marker = " [SARAH?]" if is_python else ""
        print(f"  {pid:>6}  {name:<20} {rss:>8.0f} MB{marker}")
        if 'SarahCore' in cmdline:
            print(f"          └─ {cmdline}")
        total_big += rss
        if is_python:
            python_total += rss
    
    print(f"\n[ANALYSIS]")
    print(f"  Total from top 15:  {total_big:.0f} MB")
    print(f"  Python processes:   {python_total:.0f} MB")
    
    # 2GB target check
    target_mb = 2048
    current_mb = mem.used / 1024 / 1024
    if current_mb <= target_mb:
        print(f"\n  ✅ WITHIN 2GB TARGET! ({current_mb:.0f} MB)")
    else:
        print(f"\n  ❌ OVER TARGET by {current_mb - target_mb:.0f} MB")
        print(f"     Need to free: {current_mb - target_mb:.0f} MB")
    
    return procs


def suggest_optimizations(procs):
    """Suggest optimizations based on profile."""
    print(f"\n[OPTIMIZATION SUGGESTIONS]")
    
    python_procs = [(rss, pid, name, cmd) for rss, pid, name, cmd in procs 
                    if 'python' in name.lower()]
    
    if python_procs:
        total_py = sum(p[0] for p in python_procs)
        print(f"  1. Python processes using {total_py:.0f} MB")
        if total_py > 500:
            print(f"     → Kill idle Python processes")
            print(f"     → Use lazy loading for modules")
            print(f"     → Switch to TinyRuntime (SmolLM: 200MB)")
    
    print(f"\n  2. Module Optimizations:")
    print(f"     → Genesis Core: Use 'volumetric_lite' mode")
    print(f"     → Neural Orchestrator: Defer loading until needed")
    print(f"     → Audio Core: Disable if not using voice")
    print(f"     → Disable speculative decoding")
    
    print(f"\n  3. Governor Settings:")
    print(f"     → RAM cap: 0.45 (45%) OR hard 2048 MB")
    print(f"     → CPU: BELOW_NORMAL priority")
    print(f"     → Cores: 4/12 (33%)")


if __name__ == "__main__":
    procs = profile_system()
    suggest_optimizations(procs)
