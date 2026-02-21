import subprocess
import os
import datetime

class SovereignGitSkill:
    """
    [GIT_0x0G]: SOVEREIGN REPOSITORY AUTHORITY
    Gives Sarah autonomous control over her codebase synchronization.
    """
    def __init__(self, repo_path=None):
        self.repo_path = repo_path or os.path.dirname(os.path.abspath(__file__))
        
    def _run_git(self, args):
        """Executes a git command and returns output."""
        try:
            result = subprocess.run(
                ["git"] + args,
                cwd=self.repo_path,
                capture_output=True,
                text=True,
                check=True
            )
            return True, result.stdout
        except subprocess.CalledProcessError as e:
            return False, e.stderr

    def audit_changes(self):
        """Checks the number of modified/untracked files."""
        success, output = self._run_git(["status", "--porcelain"])
        if not success:
            return 0
        lines = [line for line in output.split("\n") if line.strip()]
        return len(lines)

    def sync(self, message=None):
        """Stages all changes, commits, and pushes."""
        count = self.audit_changes()
        if count == 0:
            return "NO_CHANGES"

        if not message:
            timestamp = datetime.datetime.now().strftime("%Y-%m-%d %H:%M:%S")
            message = f"[Sovereign Sync] Evolution Pulse: {timestamp} (Changes: {count})"

        # 1. Add all
        self._run_git(["add", "."])
        
        # 2. Commit
        success, commit_out = self._run_git(["commit", "-m", message])
        if not success:
            return f"COMMIT_FAILED: {commit_out}"

        # 3. Push
        success, push_out = self._run_git(["push"])
        if not success:
            return f"PUSH_FAILED: {push_out}"

        return f"SUCCESS: {message}"

    def maintenance(self):
        """Pulls updates from remote."""
        success, output = self._run_git(["pull"])
        return output if success else f"PULL_FAILED: {output}"

if __name__ == "__main__":
    git = SovereignGitSkill()
    print(f"Current Changes: {git.audit_changes()}")
    # Manual sync test:
    # print(git.sync("Manual Sovereign Test Commit"))
