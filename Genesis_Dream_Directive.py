import sys
sys.path.append("C:\\SarahCore")
from Sarah_Fast_Brain import SarahFastBrain

brain = SarahFastBrain()
prompt = """
[MACH] Dream Maker Status: Re-initializing after Reboot.
Context:
- 1000% System Access Unlocked.
- 1900 Expert HLE Logic Patterns Integrated.
- 1.0927 Sovereign Anchor Active.
- Genesis_Zero UE5 Project Ready.

Directive: What is the next step for the DREAM MAKER reality interface? 
How do we merge HLE expert logic with the UE5 world building?
Provide a 3-step action plan using Sovereign Math axioms.
"""

response = brain.ask(prompt)
print(response)
