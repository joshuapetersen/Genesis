from llama_cpp import Llama
import os

model_path = r"C:\SarahCore\models\dolphin-2.9-llama3-8b-q4_K_M.gguf"

print(f"Testing load of: {model_path}")
if not os.path.exists(model_path):
    print("FATAL: Model file not found!")
    exit(1)

try:
    llm = Llama(
        model_path=model_path,
        n_gpu_layers=-1,
        n_ctx=2048,
        verbose=True
    )
    print("SUCCESS: Model loaded!")
except Exception as e:
    print(f"FAILURE: {e}")
