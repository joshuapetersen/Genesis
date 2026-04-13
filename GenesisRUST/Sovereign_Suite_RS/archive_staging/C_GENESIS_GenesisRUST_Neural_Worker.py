import os
import time

VAR_0_1 = 0.1
VAR_0_9 = 0.9
VAR_2048 = 2048
VAR_512 = 512
try:
    from llama_cpp import Llama
except ImportError:
    Llama = None

class NeuralWorker:
    """
    [NODE BETA]: The Errand Runner (1B Parameter Model).
    Role: High-speed execution of routine tasks (formatting, searching, summarizing).
    Constraint: Minimal VRAM usage (~800MB).
    """
    def __init__(self, model_path=r"C:\SarahCore\models\Llama-3.2-1B-Instruct-Q4_K_M.gguf"):
        self.output_queue = []
        self.model_path = model_path
        self.llm = None
        self.active = False
        
        # Initialize immediately if file exists
        if os.path.exists(model_path):
            self.load_model()
        else:
            print(f"[Neural Worker] Model not found: {model_path}")
            print(f"[Neural Worker] Run 'python download_worker.py' to provision Node Beta.")

    def load_model(self):
        """Function: load_model"""
        if not Llama:
            print("[Neural Worker] llama-cpp-python not installed.")
            return

        try:
            print(f"[Neural Worker] Initializing Node Beta (1B)...")
            # 1B Model fits easily in VRAM even with 8B loaded, 
            # but to be safe we can use partial offload or decent CPU threads.
            # Given user's "Master-Slave" prompt, we want SPEED.
            # 1B model is tiny. Full GPU offload is negligible (~600MB).
            self.llm = Llama(
                model_path=self.model_path,
                n_gpu_layers=-1, # Full Offload
                n_ctx=VAR_2048, # Smaller context for speed
                n_batch=VAR_512,
                verbose=False
            )
            self.active = True
            print(f"[Neural Worker] Node Beta ONLINE. Ready for errands.")
        except Exception as e:
            print(f"[Neural Worker] Initialization Failed: {e}")
            self.active = False

    def run_errand(self, task, context=None):
        """
        Executes a specific low-level task.
        """
        if not self.active or not self.llm:
            return None

        # Format prompt for Llama-3 Instruct
        prompt = f"<|begin_of_text|><|start_header_id|>system<|end_header_id|>\n\nYou are a high-speed worker node. Your job is to format data, summarize text, and organize files. You do not have a personality. You are a tool. Output only the requested result.<|eot_id|><|start_header_id|>user<|end_header_id|>\n\n{task}"
        
        if context:
            prompt += f"\n\nCONTEXT:\n{context}"
            
        prompt += "<|eot_id|><|start_header_id|>assistant<|end_header_id|>\n\n"

        try:
            start_time = time.time()
            output = self.llm.create_completion(
                prompt=prompt,
                max_tokens=VAR_512, # Short bursts
                temperature=VAR_0_1, # Strict instruction following
                top_p=VAR_0_9,
                stop=["<|eot_id|>"],
                echo=False
            )
            result = output['choices'][0]['text'].strip()
            latency = time.time() - start_time
            return {"result": result, "latency": latency, "worker": "Node Beta (1B)"}
        except Exception as e:
            print(f"[Neural Worker] Errand Failed: {e}")
            return None
