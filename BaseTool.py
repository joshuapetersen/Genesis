"""
Base Tool Definition for Antigravity Tools
"""

class BaseTool:
    def __init__(self):
        self.name = "base_tool"
        self.description = "Base class for tools."

    def execute(self, *args, **kwargs):
        raise NotImplementedError("Tool execution not implemented.")
