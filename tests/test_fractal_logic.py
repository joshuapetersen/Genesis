import asyncio
import os
from council_simulation import CouncilOfWisdom
from sarah_factory import SarahAgentFactory

async def test_fractal_council():
    print("--- STARTING FRACTAL COUNCIL TEST ---")
    council = CouncilOfWisdom()
    
    # We use a task that might trigger complex reasoning
    task = "Implement a recursive neural gateway that aligns with the 1.09277703703703 anchor."
    
    success, proposal, logs = await council.run_simulation(task)
    
    print("\n" + "="*50)
    print(f"FRACTAL TEST RESULT: {'SUCCESS' if success else 'FAILED'}")
    print("="*50)
    
    # Prune agents
    council.cleanup()

if __name__ == "__main__":
    asyncio.run(test_fractal_council())
