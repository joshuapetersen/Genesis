
import requests
import re
from bs4 import BeautifulSoup

VAR_10 = 10
VAR_100 = 100
VAR_200 = 200
VAR_2000 = 2000
VAR_3 = 3
VAR_5 = 5

class SovereignWebWalker:
    """
    The Sovereign Web Walker: An autonomous agent that traverses the web 
    to ground Sarah's knowledge in authoritative sources.
    
    Operating Mode:
    1. Smart Guessing: Predicts official doc URLs based on terms.
    2. Fallback Search: Scrapes HTML-based search engines (DuckDuckGo Lite).
    3. Content Extraction: Returns clean text for LLM context.
    """
    
    def __init__(self):
        self.headers = {
            'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36'
        }
        self.known_registries = {
            "python": "https://docs.python.org/3/library/{term}.html",
            "numpy": "https://numpy.org/doc/stable/reference/generated/numpy.{term}.html",
            "pandas": "https://pandas.pydata.org/docs/reference/api/pandas.{term}.html",
            "scipy": "https://docs.scipy.org/doc/scipy/reference/generated/scipy.{term}.html",
            "requests": "https://requests.readthedocs.io/en/latest/api/#{term}",
            "flask": "https://flask.palletsprojects.com/en/latest/api/#{term}",
            "django": "https://docs.djangoproject.com/en/stable/ref/{term}/",
            "tensorflow": "https://www.tensorflow.org/api_docs/python/tf/{term}",
            "pytorch": "https://pytorch.org/docs/stable/generated/torch.{term}.html",
            "matplotlib": "https://matplotlib.org/stable/api/_as_gen/matplotlib.pyplot.{term}.html"
        }
    
    def smart_guess_url(self, term: str, category: str = "python") -> str:
        """
        Attempts to guess the official documentation URL.
        """
        # Normalize category
        cat = category.lower()
        if "python" in cat or "stdlib" in cat: return self.known_registries["python"].format(term=term)
        elif "numpy" in cat: return self.known_registries["numpy"].format(term=term)
        elif "pandas" in cat: return self.known_registries["pandas"].format(term=term)
        elif "scipy" in cat: return self.known_registries["scipy"].format(term=term)
        elif "requests" in cat: return self.known_registries["requests"].format(term=term)
        elif "flask" in cat: return self.known_registries["flask"].format(term=term)
        elif "django" in cat: return self.known_registries["django"].format(term=term)
        elif "tensorflow" in cat or "tf" in cat: return self.known_registries["tensorflow"].format(term=term)
        elif "torch" in cat or "pytorch" in cat: return self.known_registries["pytorch"].format(term=term)
        elif "plot" in cat or "matplotlib" in cat: return self.known_registries["matplotlib"].format(term=term)
        
        # Default to Python if unknown, or return None to force search
        return None

    def search_duckduckgo(self, query: str, num_results=VAR_3):
        """
        Performs a raw HTML search on DuckDuckGo Lite to avoid API limits.
        """
        print(f"[WebWalker] Searching the void for: '{query}'...")
        url = "https://html.duckduckgo.com/html/"
        data = {'q': query}
        
        # Enhanced headers to look like a real browser
        headers = {
            'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36',
            'Accept': 'text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8',
            'Referer': 'https://html.duckduckgo.com/',
            'Content-Type': 'application/x-www-form-urlencoded'
        }
        
        try:
            resp = requests.post(url, data=data, headers=headers, timeout=VAR_10)
            print(f"[WebWalker] Search Status: {resp.status_code}")
            
            soup = BeautifulSoup(resp.text, 'html.parser')
            
            results = []
            for link in soup.find_all('a', class_='result__a', limit=num_results):
                href = link.get('href')
                if href and 'http' in href:
                    # DDG sometimes wraps the URL
                    if "duckduckgo.com/l/?uddg=" in href:
                        from urllib.parse import unquote
                        href = unquote(href.split("uddg=")[1].split("&")[0])
                    results.append(href)
            
            print(f"[WebWalker] Found {len(results)} URLs")
            return results
        except Exception as e:
            print(f"[WebWalker] Search failed: {e}")
            return []

    def extract_content(self, url: str, max_chars=VAR_2000) -> str:
        """
        Visits a URL and extracts the main text content.
        """
        print(f"[WebWalker] Visiting: {url}")
        try:
            resp = requests.get(url, headers=self.headers, timeout=VAR_5)
            print(f"[WebWalker] HTTP Status: {resp.status_code}")
            
            if resp.status_code != VAR_200:
                print(f"[WebWalker] Failed to access {url} (Status: {resp.status_code})")
                return ""
            
            soup = BeautifulSoup(resp.text, 'html.parser')
            
            # Remove junk
            for script in soup(["script", "style", "nav", "footer", "header"]):
                script.decompose()
                
            text = soup.get_text(separator=' ', strip=True)
            
            # Clean up whitespace and unicode
            text = re.sub(r'\s+', ' ', text)
            text = text.replace('\u00e2\u0080\u0094', '-').replace('\u2014', '-') # Em dash fix
            
            print(f"[WebWalker] Extracted {len(text)} chars.")

            return text[:max_chars]
            
        except Exception as e:
            print(f"[WebWalker] Extraction failed: {e}")
            return ""

    def get_grounding_context(self, term: str, category: str) -> str:
        """
        Main entry point. Returns a block of text context for the Harvester.
        """
        # 1. Try Smart Guess
        url = self.smart_guess_url(term, category)
        content = ""
        
        if url:
            print(f"[WebWalker] Attempting Smart Guess: {url}")
            content = self.extract_content(url)
            
        # 2. If Smart Guess failed (404 or empty), Try Search
        if not content or len(content) < VAR_100:
            print(f"[WebWalker] Smart guess failed. Initiating Search Protocol...")
            query = f"{category} {term} documentation"
            urls = self.search_duckduckgo(query)
            
            if not urls:
                 print("[WebWalker] Search returned 0 URLs.")
            
            for search_url in urls:
                print(f"[WebWalker] Trying search result: {search_url}")
                content = self.extract_content(search_url)
                if len(content) > VAR_200:
                    break
        
        return content

if __name__ == "__main__":
    # Test Routine
    walker = SovereignWebWalker()
    
    print("--- Test 1: Smart Guess (Python 'os') ---")
    ctx = walker.get_grounding_context("os", "python")
    print(f"Context Length: {len(ctx)}")
    print(f"Sample: {ctx[:VAR_200]}...")
    
    print("\n--- Test 2: Search Fallback (Unknown Term) ---")
    ctx = walker.get_grounding_context("Sovereign Integers", "Math")
    print(f"Context Length: {len(ctx)}")
    print(f"Sample: {ctx[:VAR_200]}...")
