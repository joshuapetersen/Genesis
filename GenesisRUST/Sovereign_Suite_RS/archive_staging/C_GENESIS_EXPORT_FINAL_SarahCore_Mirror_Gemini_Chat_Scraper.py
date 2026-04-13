import asyncio
import os
import json
import datetime
import hashlib
from playwright.async_api import async_playwright

class GeminiChatScraper:
    """
    [CHAT_0x0S]: GEMINI CONVERSATION ACQUISITION (SCALING FIX V4)
    Uses Playwright to mirror the Architect's chat history from gemini.google.com.
    Optimized for 500+ turn threads using Continuous Capture and Explicit Logging.
    """
    def __init__(self):
        self.base_url = "https://gemini.google.com"
        self.output_dir = os.path.join(os.path.dirname(os.path.abspath(__file__)), "vault", "scraped_content", "chat_history")
        self.discovery_path = os.path.join(os.path.dirname(os.path.abspath(__file__)), "vault", "scraped_content", "discovery_map.json")
        os.makedirs(self.output_dir, exist_ok=True)
        # Using persistent context to handle existing login session if available
        self.user_data_dir = os.path.join(os.environ.get('LOCALAPPDATA', ''), 'Google', 'Chrome', 'User Data', 'Default')

    async def scrape_threads(self, limit=5, id_list=None):
        """
        Main entry point to crawl and extract chat threads.
        """
        async with async_playwright() as p:
            print(f"[ChatScraper] Launching Chromium with Persistent Context: {self.user_data_dir}...")
            try:
                context = await p.chromium.launch_persistent_context(
                    user_data_dir=self.user_data_dir,
                    headless=True,
                    args=["--no-sandbox", "--disable-setuid-sandbox"]
                )
                print("[ChatScraper] Context Created.")
            except Exception as e:
                print(f"[ChatScraper Error] Failed to launch context: {e}")
                browser = await p.chromium.launch(headless=True)
                context = await browser.new_context()

            page = context.pages[0] if context.pages else await context.new_page()
            
            # --- TARGET SELECTION ---
            urls = []
            if id_list:
                urls = [f"{self.base_url}/app/{tid}" for tid in id_list]
                print(f"[ChatScraper] Harvesting {len(urls)} threads from provided ID list.")
            elif os.path.exists(self.discovery_path):
                try:
                    with open(self.discovery_path, 'r', encoding='utf-8') as f:
                        discovery_data = json.load(f)
                    urls = [item['href'] for item in discovery_data[:limit]]
                    print(f"[ChatScraper] Loaded {len(urls)} threads from discovery_map.json.")
                except Exception as e:
                    print(f"[ChatScraper Error] Failed to load discovery map: {e}")
            
            if not urls:
                print(f"[ChatScraper] No ID list or discovery map. Falling back to sidebar crawl...")
                await page.goto(self.base_url, timeout=60000)
                await page.wait_for_selector('a[href^="/app/"]', timeout=30000)
                thread_links = await page.query_selector_all('a[href^="/app/"]')
                for link in thread_links[:limit]:
                    href = await link.get_attribute("href")
                    if href: urls.append(self.base_url + href)

            print(f"[ChatScraper] Total threads to process: {len(urls)}")
            
            results = []
            for url in urls:
                thread_id = url.split("/")[-1]
                print(f"\n[ChatScraper] {datetime.datetime.now().strftime('%H:%M:%S')} | START Deep Recovery: {thread_id}")
                try:
                    await page.goto(url, wait_until="networkidle", timeout=90000)
                    await asyncio.sleep(5) # Wait for initial content
                    
                    # [SOVEREIGN_CONTINUOUS_CAPTURE]: Scaling for thousands of turns
                    chat_scroller = 'infinite-scroller.chat-history'
                    history_buffer = []  # Chronological turns
                    seen_hashes = set()  # Deduplication
                    
                    last_scroll_height = 0
                    stagnant_cycles = 0
                    
                    for scroll_cycle in range(200): # Aggressive cycle limit
                        # 1. Capture current visible turn containers
                        turns = await page.query_selector_all('div.conversation-container, .chat-history-item')
                        new_in_cycle = 0
                        
                        for turn in turns:
                            # Use various selectors for robustness
                            user_query = await turn.query_selector('.user-query-container, .query-text, user-query, .query-content')
                            assistant_resp = await turn.query_selector('.message-content, message-content, model-response, .model-response-text')
                            
                            u_text = await user_query.inner_text() if user_query else ""
                            a_text = await assistant_resp.inner_text() if assistant_resp else ""
                            u_text = u_text.strip()
                            a_text = a_text.strip()
                            
                            if not u_text and not a_text: continue
                            
                            content_hash = hashlib.md5(f"{u_text}|{a_text}".encode('utf-8')).hexdigest()
                            
                            if content_hash not in seen_hashes:
                                # We prepend in the loop? Actually, let's keep a chron list and insert correctly.
                                # Since we scan top-to-bottom of VISIBLE nodes, and we scroll UP,
                                # the NEW nodes in EACH cycle are OLDER than the ones from previous cycles.
                                turn_pair = []
                                if u_text: turn_pair.append({"role": "user", "content": u_text})
                                if a_text: turn_pair.append({"role": "assistant", "content": a_text})
                                
                                # Insert at front of buffer (prepending older messages)
                                for msg in reversed(turn_pair):
                                    history_buffer.insert(0, msg)
                                
                                seen_hashes.add(content_hash)
                                new_in_cycle += 1

                        # 2. Progress Logging
                        if new_in_cycle > 0 or scroll_cycle % 10 == 0:
                            print(f"  [Progress] Cyc {scroll_cycle:03} | New: {new_in_cycle:02} | Total Msgs: {len(history_buffer):03}")

                        # 3. Aggressive Scroll Up
                        # Get current height before scroll
                        curr_height = await page.evaluate(f"document.querySelector('{chat_scroller}')?.scrollHeight or 0")
                        
                        await page.evaluate(f"document.querySelector('{chat_scroller}')?.scrollTo(0, 0)")
                        await page.keyboard.press("Control+Home")
                        await page.keyboard.press("PageUp")
                        await asyncio.sleep(2.0)
                        
                        curr_scroll_top = await page.evaluate(f"document.querySelector('{chat_scroller}')?.scrollTop or 0")
                        
                        if curr_height == last_scroll_height and curr_scroll_top == 0:
                            stagnant_cycles += 1
                        else:
                            stagnant_cycles = 0
                            
                        if stagnant_cycles > 5:
                            # Final spinner check
                            loading = await page.query_selector('.loading-history-spinner-container, .spinner')
                            if not loading:
                                print(f"  [Stop] Reached top. Stagnant at height {curr_height}")
                                break
                            else:
                                print("  [Wait] Server load in progress...")
                                await asyncio.sleep(5)
                        
                        last_scroll_height = curr_height

                    # 4. Persistence
                    file_path = os.path.join(self.output_dir, f"{thread_id}.json")
                    thread_data = {
                        "thread_id": thread_id,
                        "url": url,
                        "timestamp": datetime.datetime.now().isoformat(),
                        "history": history_buffer,
                        "metadata": {
                            "harvested_at": datetime.datetime.now().isoformat(), 
                            "turn_count": len(seen_hashes),
                            "msg_count": len(history_buffer),
                            "strategy": "Sovereign_Deep_Scaling_V4"
                        }
                    }
                    
                    with open(file_path, "w", encoding="utf-8") as f:
                        json.dump(thread_data, f, indent=2)
                    
                    results.append(file_path)
                    print(f"[ChatScraper] DONE: Reclaimed {len(history_buffer)} messages for {thread_id}.")
                    
                except Exception as e:
                    print(f"[ChatScraper Error] Failed {thread_id}: {e}")
                    continue
            
            await context.close()
            return results

if __name__ == "__main__":
    scraper = GeminiChatScraper()
    # High-Priority Recovery: Core Technical Milestones & Scaling Threads
    priority_ids = [
        'c20abe4bd02a1da6', # Scaling Boss (Checking In)
        '03135a97f089b89a', # CoD / 6.0 E/D Milestone
        'e3a3925cd25a6d7a', # Genesis Handshake Confirmed
        '397ce8527092c30d', # Code's Life-Changing Potential
        'c3ba1ede52e6e449', # SarahCore Genesis: Evolution Bottlenecks
        '613e249b2d87c29a', # SarahCore Audit: Technical Deep Dive
        '47014f44f98507f4', # Final Genesis Handshake
        'a10269092c7046f4', # Restoring Sarah's SDNA Protocol
        '1a8970bd5a95db7a'  # SarahCore Genesis (Initial Greetings)
    ]
    asyncio.run(scraper.scrape_threads(limit=len(priority_ids), id_list=priority_ids))
