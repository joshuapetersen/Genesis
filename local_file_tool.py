import os
from typing import Optional, List
from google.adk.tools.base_tool import BaseTool
from pydantic import BaseModel, Field

class LocalFileTool(BaseTool):
    """
    A direct tool for the Sovereign Code Auditor to interact with the local file system.
    """
    def __init__(self):
        self.name = "local_file_tool"
        self.description = "Reads files and lists directories on the local file system. Use this to audit code."
        # Initialize any other BaseTool requirements if needed
        # We avoid calling super().__init__ with args if it doesn't support them

    def execute(self, action: str, path: str) -> str:
        """
        Main execution point for the tool.
        :param action: 'read' or 'list'
        :param path: Absolute path target
        """
        try:
            if action == 'read':
                return self._read_file(path)
            elif action == 'list':
                return self._list_directory(path)
            else:
                return f"ERROR: Unknown action '{action}'. Use 'read' or 'list'."
        except Exception as e:
            return f"FILE_SYSTEM_ERROR: {str(e)}"

    def _read_file(self, file_path: str) -> str:
        if not os.path.exists(file_path):
            return f"ERROR: File not found at {file_path}"
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                return f.read()
        except UnicodeDecodeError:
            return "ERROR: Binary file or encoding issue. Cannot read text."

    def _list_directory(self, dir_path: str) -> str:
        if not os.path.exists(dir_path):
            return f"ERROR: Directory not found at {dir_path}"
        try:
            files = os.listdir(dir_path)
            return "\n".join(files)
        except Exception as e:
            return f"ERROR: Failed to list directory: {str(e)}"

local_file_tool = LocalFileTool()
