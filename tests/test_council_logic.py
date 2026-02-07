import asyncio
import os
import sys
import time
from council_simulation import CouncilOfWisdom
from dotenv import load_dotenv

async def test_real_council():
    print("--- STARTING REAL COUNCIL TEST ---")
    load_dotenv()
    api_key = os.getenv("GEMINI_API_KEY")
    if not api_key:
        print("FAIL: GEMINI_API_KEY not found in .env")
        return

    task = "Add a docstring to Sovereign_Math.py"
    council = CouncilOfWisdom(api_key=api_key)
    
    try:
        success, proposal, logs = await council.run_simulation(task)
        print(f"\nRESULTS:\nSuccess: {success}")
        print(f"Proposal Snippet: {proposal[:100] if proposal else 'None'}")
        print(f"Debate Log Length: {len(logs)}")
        
        # Check if logs saved
        for entry in os.listdir("data/council_logs"):
            print(f"Found Log File: {entry}")
            
    except Exception as e:
        print(f"CRITICAL ERROR: {e}")
    finally:
        council.cleanup()

if __name__ == "__main__":
    asyncio.run(test_real_council())
