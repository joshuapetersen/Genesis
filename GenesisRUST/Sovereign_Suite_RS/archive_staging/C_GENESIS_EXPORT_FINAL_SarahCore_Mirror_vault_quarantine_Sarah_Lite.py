"""
Sarah Lite Mode - 2GB RAM Target
Ultra-minimal startup for maximum compatibility.

This mode disables heavy modules and enforces a hard 2GB RAM limit.
If Sarah can run here, she runs ANYWHERE.
"""

import os
import sys
import psutil

# STEP 1: Apply hard 2GB RAM cap immediately
def enforce_2gb_cap():
    """Hard cap this process to 2GB RAM."""
    try:
        import win32job
        import win32api
        
        h_job = win32job.CreateJobObject(None, "SarahLite_2GB")
        
        # Hard 2GB limit (2048 MB = 2,147,483,648 bytes)
        mem_limit = 2 * 1024 * 1024 * 1024  # 2GB
        
        info = win32job.QueryInformationJobObject(h_job, win32job.JobObjectExtendedLimitInformation)
        info['ProcessMemoryLimit'] = mem_limit
        info['BasicLimitInformation']['LimitFlags'] |= win32job.JOB_OBJECT_LIMIT_PROCESS_MEMORY
        
        win32job.SetInformationJobObject(h_job, win32job.JobObjectExtendedLimitInformation, info)
        
        process_handle = win32api.GetCurrentProcess()
        win32job.AssignProcessToJobObject(h_job, process_handle)
        
        print("[Sarah Lite] HARD 2GB RAM CAP ENFORCED")
        return True
        
    except Exception as e:
        print(f"[Sarah Lite] Cap failed (will use soft limit): {e}")
        return False

# STEP 2: Set process to lowest priority and limit cores
def set_low_priority():
    """Minimize resource usage."""
    try:
        p = psutil.Process()
        p.nice(psutil.BELOW_NORMAL_PRIORITY_CLASS)
        
        # Use only 2 cores
        total_cores = psutil.cpu_count(logical=True)
        p.cpu_affinity([0, 1])  # First 2 cores only
        
        print(f"[Sarah Lite] Priority: BELOW_NORMAL, Cores: 2/{total_cores}")
        return True
    except Exception as e:
        print(f"[Sarah Lite] Priority set failed: {e}")
        return False


# STEP 3: Lite imports only - NO heavy modules
def boot_lite():
    """Boot Sarah in minimal mode."""
    print("\n" + "="*50)
    print(" SARAH LITE MODE - 2GB RAM TARGET")
    print("="*50 + "\n")
    
    # Enforce limits
    enforce_2gb_cap()
    set_low_priority()
    
    # Set offline mode
    os.environ['HF_HUB_OFFLINE'] = '1'
    os.environ['TRANSFORMERS_OFFLINE'] = '1'
    os.environ['CUDA_VISIBLE_DEVICES'] = ''  # No GPU (saves VRAM)
    
    # Check current RAM
    p = psutil.Process()
    print(f"[Sarah Lite] Current RAM: {p.memory_info().rss / 1024 / 1024:.0f} MB")
    
    # Import ONLY essential modules
    print("[Sarah Lite] Loading minimal core...")
    
    from Sovereign_Constants import SOVEREIGN_ANCHOR
    from Sovereign_Math import SovereignMath
    print(f"[Sarah Lite] Anchor verified: {SOVEREIGN_ANCHOR}")
    
    # TinyRuntime for inference (no heavy LLM)
    try:
        from TinyRuntime import TinyRuntime
        runtime = TinyRuntime(model_name="smollm")
        print("[Sarah Lite] TinyRuntime ready (will use cache + SmolLM)")
    except Exception as e:
        print(f"[Sarah Lite] TinyRuntime skipped: {e}")
        runtime = None
    
    # TheoryLab for logic (no LLM needed)
    try:
        from TheoryLab import TheoryLab
        lab = TheoryLab()
        print("[Sarah Lite] TheoryLab ready (Sovereign Vault)")
    except Exception as e:
        print(f"[Sarah Lite] TheoryLab skipped: {e}")
        lab = None
    
    # Check RAM after loading
    ram_mb = p.memory_info().rss / 1024 / 1024
    print(f"\n[Sarah Lite] RAM after boot: {ram_mb:.0f} MB")
    
    if ram_mb <= 2048:
        print(f"[Sarah Lite] ✅ WITHIN 2GB TARGET!")
    else:
        print(f"[Sarah Lite] ⚠️ Over by {ram_mb - 2048:.0f} MB")
    
    return runtime, lab


def interactive_lite():
    """Minimal chat loop."""
    runtime, lab = boot_lite()
    
    print("\n" + "-"*50)
    print(" Sarah Lite Interactive (type 'exit' to quit)")
    print("-"*50 + "\n")
    
    while True:
        try:
            user_input = input("You: ").strip()
            if user_input.lower() in ('exit', 'quit', 'q'):
                break
            
            if not user_input:
                continue
            
            # Try TheoryLab first (no LLM needed)
            if lab and any(kw in user_input.lower() for kw in ['how', 'solve', 'algorithm', 'approach']):
                candidates = lab.theorize(user_input, num_candidates=2)
                if candidates:
                    print(f"\nSarah: I found these approaches:")
                    print(lab.compare_candidates(candidates))
                    continue
            
            # Use TinyRuntime for general questions
            if runtime:
                response = runtime.generate(user_input, max_tokens=200)
                print(f"\nSarah: {response}\n")
            else:
                print("\nSarah: [Lite mode - no model loaded. Ask algorithm questions.]\n")
                
        except KeyboardInterrupt:
            break
        except Exception as e:
            print(f"\nSarah: Error - {e}\n")
    
    print("\n[Sarah Lite] Shutdown complete.")


if __name__ == "__main__":
    if len(sys.argv) > 1 and sys.argv[1] == "chat":
        interactive_lite()
    else:
        boot_lite()
        print("\n[Sarah Lite] Run with 'chat' argument for interactive mode.")
