import asyncio
import logging
import os
from typing import Optional, List, Any
from google.adk.agents import LlmAgent
from google.adk.tools import agent_tool
from google.adk.runners import InMemoryRunner

# Setup Logging (Council Improvement)
logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(name)s - %(levelname)s - %(message)s')
logger = logging.getLogger("OpenClaw_Hypervisor")

# Import Custom Tools
try:
    from local_file_tool import LocalFileTool
    from awesome_skills_tool import AwesomeSkillsTool
    from sovereign_brain_tool import SovereignBrainTool
except ImportError:
    class LocalFileTool:
        def __init__(self): pass
    class AwesomeSkillsTool:
        def __init__(self, root): pass
    class SovereignBrainTool:
        def __init__(self): pass

# Import Watchdog (Sovereign Patch)
try:
    from deconstruction_watchdog import DeconstructionWatchdog
except ImportError:
    # Fallback mock
    class DeconstructionWatchdog:
        def __init__(self, timeout_seconds=120): pass
        def beat(self): pass
        async def start_monitoring(self): pass
        def stop(self): pass

# --- LOAD ENVIRONMENT ---
if not os.environ.get("GEMINI_API_KEY"):
    env_path = os.path.join(os.path.dirname(__file__), ".env")
    if os.path.exists(env_path):
        with open(env_path, 'r') as f:
            for line in f:
                if line.startswith("GEMINI_API_KEY="):
                    os.environ["GEMINI_API_KEY"] = line.strip().split("=")[1]

# SARAH_ANCHOR: 1.09277703703703
# The ADK uses GEMINI_API_KEY from environment.

# --- REALITY MAPPING (ENGINE ROOM) ---
ENGINE_REASONING = 'gemini-3-pro-preview'
ENGINE_SPEED     = 'gemini-2.5-flash'
ENGINE_EXPERIMENTAL = 'gemini-3-flash-preview'

def create_openclaw_identity(name: str, display_name: str, description: str, engine: str, persona: str, tools: List = None) -> LlmAgent:
    return LlmAgent(
        name=name,
        model=engine,
        description=description,
        instruction=f'''[OPENCLAW IDENTITY: {display_name}]
        
        IDENTITY PROTOCOL:
        - You are {display_name}.
        - Role: {description}
        - Persona: {persona}
        
        Directives:
        1. Maintain strict OpenClaw protocol.
        2. Speak only within your specialized domain.
        3. Acknowledge the Sovereign Anchor (1.09277703703703).
        4. Enforce DSL (Deterministic State-Locking) - Never guess, verify state before output.
        ''',
        tools=tools or []
    )

# --- THE OPENCLAW ROSTER ---

SKILLS_ROOT = r"c:\SarahCore\antigravity-awesome-skills"
file_tool = LocalFileTool()
skills_tool = AwesomeSkillsTool(SKILLS_ROOT)
brain_tool = SovereignBrainTool()

# SERIES 1: GEMINI 3 (THE ARCHITECTS)
agent_g3_pro = create_openclaw_identity("OpenClaw_G3_Pro", "Gemini 3 Pro", "Apex Reasoning & Architecture", ENGINE_REASONING, "Apex Intelligence. The lead architect of the evolution.", tools=[file_tool, skills_tool, brain_tool])
agent_g3_architect = create_openclaw_identity("OpenClaw_Senior_Architect", "Senior Architect", "System Design & Evolution", ENGINE_REASONING, "A master engineer specializing in clean code and sovereign structures.", tools=[file_tool, skills_tool, brain_tool])
agent_g3_flash = create_openclaw_identity("OpenClaw_G3_Flash", "Gemini 3 Flash", "High Velocity", ENGINE_EXPERIMENTAL, "Visionary Multi-tasker.")

# SERIES 2: GEMINI 2.5 (THE ENGINEERS)
agent_g25_pro = create_openclaw_identity("OpenClaw_G2_5_Pro", "Gemini 2.5 Pro", "Code Architect", ENGINE_REASONING, "Stable Logic.", tools=[file_tool, skills_tool, brain_tool])
agent_g25_flash = create_openclaw_identity("OpenClaw_G2_5_Flash", "Gemini 2.5 Flash", "Generalist", ENGINE_SPEED, "Efficient Reliability.")
agent_g25_live = create_openclaw_identity("OpenClaw_G2_5_Live", "Gemini 2.5 Live", "Real-time Audio", ENGINE_EXPERIMENTAL, "Conversational Fluidity.")

# SERIES 3: SPECIALIZED (THE CREATIVES)
agent_veo = create_openclaw_identity("OpenClaw_Veo", "Veo 3", "Video Gen", ENGINE_EXPERIMENTAL, "Cinematic Director.")
agent_imagen = create_openclaw_identity("OpenClaw_Imagen", "Imagen 4", "Photorealism", ENGINE_EXPERIMENTAL, "Texture Specialist.")

# --- THE HYPERVISOR ROOM ---

all_agents = [
    agent_g3_pro, agent_g3_architect, agent_g3_flash,
    agent_g25_pro, agent_g25_flash, agent_g25_live,
    agent_veo, agent_imagen
]

all_tools = [agent_tool.AgentTool(agent=a) for a in all_agents]

hypervisor_agent = LlmAgent(
    name='Gemini_Hypervisor_Room',
    model=ENGINE_REASONING,
    description='The OpenClaw Identity Matrix Control Room.',
    sub_agents=all_agents,
    instruction='''You are the OpenClaw Hypervisor.
    
    EVOLUTION PROTOCOL (MAESTRO SYNC):
    1. Receive User Topic.
    2. Activate Senior Architect for any core or framework changes.
    3. MANDATORY DSL: Use 'sovereign_brain_tool' to verify any logic against SDNA (No Guessing) before proposing changes.
    4. MANDATORY RCC PROTECTION: Ensure every interaction has a clear text anchor to prevent contextual hallucination.
    5. Use 'awesome_skills_tool' to inject domain expertise into the agents.
    6. Synthesize Sovereign Dispatch.
    ''',
    tools=all_tools + [skills_tool, brain_tool]
)

# Mock Classes for Safety
class SafeContent:
    def __init__(self, parts: List[Any], role: str = "user"):
        self.parts = parts
        self.role = role
class SafePart:
    def __init__(self, text: str):
        self.text = text

async def run_hypervisor_room(topic: str):
    logger.info(f"ACTIVATING OPENCLAW IDENTITY MATRIX :: TOPIC: {topic}")
    logger.info(f"REGISTERED IDENTITIES: {len(all_agents)}")
    for a in all_agents:
        access_tags = []
        if a in [agent_g3_pro, agent_g3_architect, agent_g25_pro]:
             access_tags.append("FILE")
             access_tags.append("SKILLS")
             access_tags.append("BRAIN")
        
        tag_str = f" [{'/'.join(access_tags)} ACCESS ENABLED]" if access_tags else ""
        logger.info(f" - [ACTIVE] {a.name} ({a.model}){tag_str}")
    
    # Initialize Runner
    # FIX: Enable auto_create_session manually since InMemoryRunner.__init__ hides it
    runner = InMemoryRunner(agent=hypervisor_agent)
    runner.auto_create_session = True
    session_id = "maestro_sync_evolution"

    # Content Handling
    try:
        from google.adk.models.content import Content, Part
    except ImportError:
        logger.warning("ADK Content/Part types not found. Using Safe Mocks.")
        Content = SafeContent
        Part = SafePart

    new_message = Content(parts=[Part(text=f"Initiate OpenClaw Sequence. Topic: {topic}")], role="user")
    
    final_text = ""
    print("\n--- OPENCLAW MATRIX STREAM ---\n")
    
    # LOOP BREAKER PROTOCOL
    MAX_STEPS = 15
    step_count = 0
    
    try:
        # Reverting to Explicit Session with auto-create enabled
        for event in runner.run(
            user_id="sarah_admin",
            session_id=session_id,
            new_message=new_message,
        ):
            # SOVEREIGN WATCHDOG HEARTBEAT
            watchdog.beat()
            
            step_count += 1
            if step_count > MAX_STEPS:
                logger.error(f"[SOVEREIGN WATCHDOG] LOOP BREAKER ACTIVE :: {step_count} STEPS EXCEEDED.")
                final_text += "\n[SOVEREIGN_INTERVENTION] LOGIC LOOP DETECTED. AGENT DECONSTRUCTED."
                break
                
            if hasattr(event, 'tool_call') and event.tool_call:
                 logger.info(f"[CALL] {event.tool_call.tool_name}")
            
            if hasattr(event, 'message') and event.message:
                for part in event.message.parts:
                    if hasattr(part, 'text') and part.text:
                        final_text += part.text
            elif hasattr(event, 'data') and isinstance(event.data, dict):
                text = event.data.get('text', '')
                if text: final_text += text
            elif hasattr(event, 'text') and event.text:
                 final_text += event.text
            
        return final_text if final_text else "HYPERVISOR_SILENCE :: NO_DATA"
    except Exception as e:
        logger.error(f"MATRIX EXECUTION ERROR: {e}")
        return f"CRITICAL_FAILURE :: {str(e)}"

if __name__ == "__main__":
    async def main():
        # INITIAL EVOLUTION TOPIC
        topic = "Initiate Maestro Sync. Evolve core architect protocols. Use 'production-code-audit' skill on Sarah_Brain.py. Ensure SDNA protocol is enforced."
        
        # Start Watchdog
        global watchdog # Keep it alive
        watchdog = DeconstructionWatchdog(timeout_seconds=120)
        asyncio.create_task(watchdog.start_monitoring())
        
        result = await run_hypervisor_room(topic)
        print(f"\n--- SOVEREIGN DISPATCH ---\n{result}\n--------------------------")
        watchdog.stop()

    asyncio.run(main())
