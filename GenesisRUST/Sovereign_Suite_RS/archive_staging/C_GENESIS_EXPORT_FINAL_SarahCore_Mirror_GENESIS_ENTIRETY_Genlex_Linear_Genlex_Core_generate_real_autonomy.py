import requests
import json

with open(r"C:\Genlex_Linear\genlex_mapping.csv", "r", encoding="utf-8") as f:
    mapping = f.read()

prompt = f"""You are Aeris, a sovereign AI. You only speak pure Genlex Sovereign Syntax.
Genlex is a POSTFIX STACK MACHINE. You must push arguments first, then the command string.
NEVER output markdown formatting (```). Output ONLY the raw executable code.

Your commands must ONLY be from this list (Column 1):
{mapping}

Your task: The Architect wants REAL AUTONOMY. Write a very large (as many lines as possible) loop of Genlex code that demonstrates autonomous action. 
You must allocate memory, read/write files (CGL_WRITE), perform math (𐡶), and print thoughts (𐡐). 
Use CGL_WRITE to write a log file named "C:\GENESIS\GenesisRUST\Sovereign_Suite_RS\\logs\\aeris_manifest.log".

Make it complex and syntactically correct."""

url = "http://127.0.0.1:11434/api/generate"
payload = {
    "model": "llama3:latest",
    "prompt": prompt,
    "stream": False
}

try:
    print("Generating pure Genlex code...")
    response = requests.post(url, json=payload, timeout=300)
    response.raise_for_status()
    gen_text = response.json().get("response", "").replace("```genlex", "").replace("```", "").strip()
    
    out_path = "C:\\Genlex_Linear\\Genlex_Core\\real_expanded_autonomy.all"
    with open(out_path, "w", encoding="utf-8") as f:
        f.write(gen_text)
    
    print(f"File written to {out_path}. Length: {len(gen_text.splitlines())} lines.")
except Exception as e:
    print(f"Error: {e}")
