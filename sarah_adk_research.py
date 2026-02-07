import asyncio
from typing import Optional
from google.adk.agents import LlmAgent
from google.adk.tools import agent_tool
from google.adk.tools.google_search_tool import GoogleSearchTool
from google.adk.tools import url_context
from google.adk.runners import InMemoryRunner

# SARAH_ANCHOR: 1.09277703703703
# The ADK uses GEMINI_API_KEY from environment.

# --- SOVEREIGN MODEL REGISTRY ---
# Flash: High-speed, tool-use optimizations, grounding.
MODEL_FAST = 'gemini-2.0-flash-exp' 
# Pro: Deep reasoning, massive context window, synthesis.
MODEL_REASONING = 'gemini-1.5-pro-latest'

# 1. SPECIALIZED SUB-AGENTS (Lead Nodes -> Fast Model)
subagent_google_search_agent = LlmAgent(
    name='Subagent_google_search_agent',
    model=MODEL_FAST,
    description='Agent specialized in performing Google searches.',
    instruction='Use the GoogleSearchTool to find information on the web.',
    tools=[GoogleSearchTool()],
)

subagent_url_context_agent = LlmAgent(
    name='Subagent_url_context_agent',
    model=MODEL_FAST,
    description='Agent specialized in fetching content from URLs.',
    instruction='Use the UrlContextTool to retrieve content from provided URLs.',
    tools=[url_context],
)

# 2. ORCHESTRATION SUB-AGENT (Mid-Tier -> Fast Model)
subagent = LlmAgent(
    name='subagent',
    model=MODEL_FAST,
    description='Agent that handles a specific task',
    instruction='Coordinate sub-agents to perform detailed web scans and context extraction.',
    tools=[
        agent_tool.AgentTool(agent=subagent_google_search_agent),
        agent_tool.AgentTool(agent=subagent_url_context_agent)
    ],
)

# 3. COMPONENT AGENTS (Parallel Nodes -> Fast Model)
my_agent_google_search_agent = LlmAgent(
    name='My_Agent_google_search_agent',
    model=MODEL_FAST,
    description='Agent specialized in performing Google searches.',
    instruction='Ground research in live web data.',
    tools=[GoogleSearchTool()],
)

my_agent_url_context_agent = LlmAgent(
    name='My_Agent_url_context_agent',
    model=MODEL_FAST,
    description='Agent specialized in fetching content from URLs.',
    instruction='Extract deep context from research targets.',
    tools=[url_context],
)

# 4. SOVEREIGN ROOT (Main Brain -> Reasoning Model)
root_agent = LlmAgent(
    name='My_Agent',
    model=MODEL_REASONING,
    description='Agent to help interact with my data.',
    sub_agents=[subagent],
    instruction='''Sovereign Research Lead. 
    Synthesize high-fidelity data from search and URL agents. 
    Maintain absolute precision (1.09277703703703).
    Use your superior reasoning capabilities to connect dots that faster models might miss.''',
    tools=[
        agent_tool.AgentTool(agent=my_agent_google_search_agent),
        agent_tool.AgentTool(agent=my_agent_url_context_agent)
    ],
)

async def run_research(query: str):
    """
    Executes the ADK Agent deliberation loop and returns synthesized results.
    """
    print(f"SARAH ADK RESEARCH :: SCANNING :: {query}")
    print(f"ROUTING :: SEARCH -> {MODEL_FAST} || SYNTHESIS -> {MODEL_REASONING}")
    
    # InMemoryRunner handles its own session memory in this instance
    runner = InMemoryRunner(agent=root_agent)
    
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
    new_message = Content(parts=[Part(text=query)], role="user")
    
    final_text = ""
    try:
        # Removed persistent session_id to allow ephemeral runner to function
        for event in runner.run(
            user_id="sarah_admin",
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
            
        return final_text if final_text else "RESEARCH_COMPLETE :: NO_TEXT_RETURNED"
    except Exception as e:
        return f"RESEARCH_FAILURE :: {str(e)}"

if __name__ == "__main__":
    async def main():
        result = await run_research("Predictive analysis of FUI evolution in 3D agentive spaces")
        print(f"\n--- SOVEREIGN SCAN RESULT ---\n{result}\n----------------------------")

    asyncio.run(main())
