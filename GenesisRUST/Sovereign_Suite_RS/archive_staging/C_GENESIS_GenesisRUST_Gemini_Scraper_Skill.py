import requests
from bs4 import BeautifulSoup
import os
import json

class GeminiScraperSkill:
    """
    [SCRAPE_0x0S]: GEMINI KNOWLEDGE ACQUISITION
    Scrapes Gemini documentation and help centers to acquire 'every word' for the vault.
    """
    def __init__(self):
        self.targets = [
            "https://ai.google.dev/gemini-api/docs",
            "https://support.google.com/gemini/answer/13275745", # Gemini Apps FAQ
            "https://blog.google/technology/ai/google-gemini-next-generation-model-announcement/"
        ]
        self.output_dir = os.path.join(os.path.dirname(__file__), "vault", "scraped_content")
        os.makedirs(self.output_dir, exist_ok=True)

    def scrape_all(self):
        """Iterates through targets and extracts text content."""
        results = []
        for url in self.targets:
            print(f"[Scraper] Accessing: {url}...")
            try:
                response = requests.get(url, timeout=10)
                if response.status_code == 200:
                    soup = BeautifulSoup(response.text, 'html.parser')
                    
                    # Remove scripts and styles
                    for script in soup(["script", "style"]):
                        script.decompose()

                    # Extract text
                    text = soup.get_text(separator=' ')
                    # Clean up whitespace
                    lines = (line.strip() for line in text.splitlines())
                    chunks = (phrase.strip() for line in lines for phrase in line.split("  "))
                    text = '\n'.join(chunk for chunk in chunks if chunk)

                    filename = url.replace("https://", "").replace("/", "_").replace(".", "_") + ".txt"
                    path = os.path.join(self.output_dir, filename)
                    
                    with open(path, "w", encoding="utf-8") as f:
                        f.write(text)
                    
                    results.append({"url": url, "path": path, "status": "success"})
                else:
                    results.append({"url": url, "status": f"failed_{response.status_code}"})
            except Exception as e:
                print(f"[Scraper Error] {url}: {e}")
                results.append({"url": url, "status": f"error_{e}"})
        
        return results

if __name__ == "__main__":
    scraper = GeminiScraperSkill()
    summary = scraper.scrape_all()
    print(json.dumps(summary, indent=2))
