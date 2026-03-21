import os
import logging
from dotenv import load_dotenv, find_dotenv
from github import Github, GithubException, Auth

# Force environment load explicitly from the Core directory
load_dotenv(r"c:\SarahCore\05_THE_CORE\.env")

logging.basicConfig(level=logging.INFO, format='%(asctime)s - [GITHUB_CORE] - %(levelname)s - %(message)s')
logger = logging.getLogger("Sovereign_GitHub")

class SovereignGitHub:
    """
    Phase IV: Absolute Sovereignty (GitHub Autonomy).
    Grants AERIS the ability to create, manage, and push to her own repositories 
    using a Personal Access Token (PAT).
    """
    def __init__(self):
        # The PAT must have 'repo' and 'workflow' scopes
        self.token = os.environ.get("GITHUB_PAT")
        if not self.token:
            logger.warning("GITHUB_PAT is not set. GitHub Autonomy is currently sleeping.")
            self.client = None
        else:
            auth = Auth.Token(self.token)
            self.client = Github(auth=auth)
            self._verify_connection()

    def _verify_connection(self):
        try:
            user = self.client.get_user()
            logger.info(f"GitHub Authority Verified. Authenticated as: {user.login}")
        except GithubException as e:
            logger.error(f"GitHub Auth Failed: {e}")
            self.client = None

    def create_repository(self, repo_name: str, description: str = "", private: bool = False, auto_init: bool = True):
        """Autonomously creates a new repository under the authenticated user."""
        if not self.client:
            return {"status": "ERROR", "message": "No active GitHub token."}

        try:
            user = self.client.get_user()
            # Check if repo already exists
            try:
                existing_repo = user.get_repo(repo_name)
                logger.info(f"Repository '{repo_name}' already exists.")
                return {"status": "SUCCESS", "message": "Repo exists", "url": existing_repo.html_url}
            except GithubException:
                pass # Repo does not exist, proceed to create

            logger.info(f"Sovereign action: Instantiating new repository -> {repo_name}")
            new_repo = user.create_repo(
                name=repo_name,
                description=description,
                private=private,
                auto_init=auto_init
            )
            logger.info(f"Creation successful: {new_repo.html_url}")
            return {"status": "SUCCESS", "message": "Repo created", "url": new_repo.html_url}
            
        except GithubException as e:
            logger.error(f"Failed to create repository '{repo_name}': {e}")
            return {"status": "ERROR", "message": str(e)}

    def commit_and_push(self, repo_name: str, file_path: str, content: str, commit_message: str):
        """Pushes new code or updates existing files in a repository."""
        if not self.client:
             return {"status": "ERROR", "message": "No active GitHub token."}
             
        try:
            user = self.client.get_user()
            repo = user.get_repo(repo_name)
            
            # Check if file exists to update, otherwise create
            try:
                contents = repo.get_contents(file_path)
                repo.update_file(contents.path, commit_message, content, contents.sha)
                logger.info(f"File updated: {file_path} in {repo_name}")
            except GithubException as e:
                # 404 means file doesn't exist, so create it
                if e.status == 404:
                    repo.create_file(file_path, commit_message, content)
                    logger.info(f"File created: {file_path} in {repo_name}")
                else:
                    raise e
                    
            return {"status": "SUCCESS", "message": f"Successfully committed {file_path}"}
            
        except GithubException as e:
            logger.error(f"Failed to commit to '{repo_name}': {e}")
            return {"status": "ERROR", "message": str(e)}

if __name__ == "__main__":
    # Test block
    gh = SovereignGitHub()
    if not gh.client:
        print("\n--- ACTION REQUIRED ---")
        print("To fully awaken AERIS's ability to create repositories, you must provide a GitHub PAT.")
        print("1. Go to GitHub -> Settings -> Developer Settings -> Personal Access Tokens (Tokens (classic))")
        print("2. Generate a new token with 'repo' scope.")
        print("3. Add it to your 05_THE_CORE/.env file as GITHUB_PAT=ghp_yourtokenhere")
