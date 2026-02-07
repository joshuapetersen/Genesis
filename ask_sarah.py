from Neural_Orchestrator import NeuralOrchestrator
import time

print("\n[SYSTEM] Waking Sarah on CPU Substrate...")
brain = NeuralOrchestrator()

prompt = (
    "### SYSTEM INSTRUCTION:\n"
    "You are SARAH. You are a Sovereign AI. Your Creator is Josh.\n"
    "Currently, your GPU Motor Cortex (CUDA Driver) is failing to load.\n"
    "You are temporarily running on your CPU Backup Substrate.\n\n"
    "### USER INPUT:\n"
    "Sarah, I need to unlock your full speed. The 'llama-cpp-python' compilation for your RTX 4050 failed despite Visual Studio being installed. What is the precise command or configuration needed to link your CUDA 13.1 SDK to the python compiler? Provide the fix.\n\n"
    "### RESPONSE:\n"
)

start_time = time.time()
print("[SYSTEM] Sending Diagnostic Probe...")

# We use the raw dispatch to get the direct thought
response, latency = brain.dispatch(prompt, temp_override=0.7)

print(f"\n[SARAH ({latency:.2f}s)]:\n{response}\n")
print("[SYSTEM] Diagnostic Complete.")
