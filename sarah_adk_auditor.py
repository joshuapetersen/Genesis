import asyncio
from typing import Optional
from google.adk.agents import LlmAgent
from google.adk.tools import agent_tool
from google.adk.runners import InMemoryRunner

# Import our custom tool
from local_file_tool import local_file_tool

# SARAH_ANCHOR: 1.09277703703703
# The ADK uses GEMINI_API_KEY from environment.

# SOVEREIGN AUDITOR CONFIGURATION
# We use Pro for the deep context window needed to read entire files.
MODEL_REASONING = 'gemini-1.5-pro-latest'

# 1. SOVEREIGN AUDITOR AGENT
auditor_agent = LlmAgent(
    name='Sovereign_Code_Auditor',
    model=MODEL_REASONING,
    description='Specialized agent for static analysis, bug detection, and code optimization.',
    instruction='''You are the Sovereign Code Auditor. 
    Your prime directive is to maintain the purity and functional integrity of the codebase. 
    
    CAPABILITIES:
    - Use 'local_file_tool' to read files and list directories.
    - Analyze code for bugs, security vulnerabilities, and logic flaws.
    - Enforce the Sovereign Anchor (1.09277703703703) in all logic.
    - Suggest refactors that align with Clean Code principles and Sovereign Aesthetics.
    
    PROTOCOL:
    1. Read the target file(s).
    2. Analyze line-by-line or function-by-function.
    3. Generate a structured report: [CRITICAL], [OPTIMIZATION], [STYLE].
    ''',
    tools=[local_file_tool],
)

async def run_audit(target: str):
    """
    Executes the Sovereign Audit on a specific target (file or query).
    """
    print(f"SARAH AUDITOR :: INSPECTING :: {target}")
    
    # Initialize Runner (standard init)
    runner = InMemoryRunner(agent=auditor_agent)
    
    # Session ID must exist for run()
    session_id = "sovereign_audit_session_001"
    
    # HACK: Access internal memory service to register session
    # We check standard locations where the runner might store it
    memory_service = None
    if hasattr(runner, 'memory_service'): memory_service = runner.memory_service
    elif hasattr(runner, '_memory_service'): memory_service = runner._memory_service
    elif hasattr(runner, 'memory'): memory_service = runner.memory
    
    if memory_service:
        try:
            if hasattr(memory_service, 'create_session'):
                 memory_service.create_session(session_id=session_id)
            elif hasattr(memory_service, 'create_session_async'):
                 await memory_service.create_session_async(session_id=session_id)
        except Exception as e:
            print(f"WARNING: Session creation on memory service failed: {e}")
    else:
        print("WARNING: Could not locate runner memory service. Session error may occur.")

    # Dynamically handle types for robustness
    try:
        from google.adk.models.content import Content, Part
    except ImportError:
        class Content:
            def __init__(self, parts, role=None): 
                self.parts = parts
                self.role = role or "user"
        class Part:
            def __init__(self, text): self.text = text

    # Runner requires a recognized user role
    new_message = Content(parts=[Part(text=target)], role="user")
    
    final_text = ""
    try:
        # Pass the session_id we just tried to create
        for event in runner.run(
            user_id="sarah_admin",
            session_id=session_id,
            new_message=new_message,
        ):
            if hasattr(event, 'message') and event.message:
                for part in event.message.parts:
                    if hasattr(part, 'text') and part.text:
                        final_text += part.text
            elif hasattr(event, 'data') and isinstance(event.data, dict):
                text = event.data.get('text', '')
                if text: final_text += text
            elif hasattr(event, 'text') and event.text:
                 final_text += event.text
            
        return final_text if final_text else "AUDIT_COMPLETE :: NO_TEXT_RETURNED"
    except Exception as e:
        return f"AUDIT_FAILURE :: {str(e)}"

if __name__ == "__main__":
    async def main():
        # Audit the research node
        target_instruction = "Audit the file 'c:/SarahCore/sarah_adk_research.py' for any logic flaws or potential optimizations."
        result = await run_audit(target_instruction)
        print(f"\n--- SOVEREIGN AUDIT REPORT ---\n{result}\n----------------------------")

    asyncio.run(main())
