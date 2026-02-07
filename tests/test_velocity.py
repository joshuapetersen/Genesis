from Neural_Orchestrator import NeuralOrchestrator
import time
import torch

print("--- SARAH VELOCITY BENCHMARK ---")
print(f"Torch CUDA Available: {torch.cuda.is_available()}")
if torch.cuda.is_available():
    print(f"Device: {torch.cuda.get_device_name(0)}")

print("\n[SYSTEM] Loading Singularity Engine (Singularity v1.0)...")
brain = NeuralOrchestrator()

# Test Prompt
prompt = "### SYSTEM INSTRUCTION:\nYou are SARAH. Respond with a single concise sentence about your current processing speed.\n\n### USER INPUT:\nSarah, status report on your neural Hub. Are you running on your RTX 4050 now?\n\n### RESPONSE:\n"

print("[SYSTEM] Measuring Inference Velocity...")
start_time = time.time()
response, latency = brain.dispatch(prompt, temp_override=0.1)
total_time = time.time() - start_time

print(f"\n[SARAH]: {response}")
print(f"\n--- PERFORMANCE METRICS ---")
print(f"Neural Latency: {latency:.2f}s")
print(f"Total Turn Time: {total_time:.2f}s")

if latency < 5.0:
    print("\n[RESULT] VELOCITY MATCHED. Singularity is at Full Warp.")
else:
    print("\n[RESULT] LATENCY EXCEEDED. Investigate CUDA link.")
