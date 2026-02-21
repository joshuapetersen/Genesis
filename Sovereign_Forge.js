/**
 * Sovereign Forge v1.0
 * Purpose: Mass discovery and extraction of Gemini chat history.
 * Handles: Virtual Scrolling, DOM Recycling, and Context Preservation.
 */

(async () => {
    console.log("[SovereignForge] Initiating Memory Discovery...");

    const sidebarScroller = document.querySelector('infinite-scroller');
    if (!sidebarScroller) {
        console.error("[SovereignForge] Sidebar scroller not found! Expand the sidebar manually.");
        return;
    }

    const discoveredThreads = new Map(); // id -> {title, href}
    let lastScrollHeight = 0;
    let staleCount = 0;
    const MAX_STALE = 10; // Stop after 10 scrolls with no new content

    // --- PHASE 1: SIDEBAR DISCOVERY ---
    while (staleCount < MAX_STALE) {
        const threads = document.querySelectorAll('a.conversation');
        let newFound = 0;

        threads.forEach(t => {
            const href = t.getAttribute('href');
            if (href && href.startsWith('/app/')) {
                const id = href.split('/').pop();
                if (!discoveredThreads.has(id)) {
                    discoveredThreads.set(id, {
                        id: id,
                        title: t.innerText.trim(),
                        href: "https://gemini.google.com" + href
                    });
                    newFound++;
                }
            }
        });

        if (newFound > 0) {
            staleCount = 0;
            console.log(`[SovereignForge] Discovered ${discoveredThreads.size} threads...`);
        } else {
            staleCount++;
        }

        sidebarScroller.scrollTop += 1500;
        await new Promise(r => setTimeout(r, 1500)); // Wait for lazy load

        if (sidebarScroller.scrollHeight === lastScrollHeight && newFound === 0) {
            staleCount = MAX_STALE; // Reached bottom
        }
        lastScrollHeight = sidebarScroller.scrollHeight;
    }

    const threadList = Array.from(discoveredThreads.values());
    console.log(`[SovereignForge] Discovery Complete. Total unique threads: ${threadList.size}`);

    // Output the discovery list for the agent
    console.log("--- DISCOVERY_DATA_START ---");
    console.log(JSON.stringify(threadList, null, 2));
    console.log("--- DISCOVERY_DATA_END ---");

    // --- PHASE 2: (Optional Auto-Extraction) ---
    // Note: We'll likely let the Python agent handle navigation to avoid page crashes
    // but the logic for Deep Scroll within a thread is here for reference:

    window.sovereignDeepScroll = async () => {
        const mainScroller = document.querySelector('infinite-scroller.chat-history');
        if (!mainScroller) return [];

        let prevHeight = 0;
        for (let i = 0; i < 50; i++) {
            mainScroller.scrollTop = 0;
            await new Promise(r => setTimeout(r, 2000));
            if (mainScroller.scrollHeight === prevHeight) break;
            prevHeight = mainScroller.scrollHeight;
        }

        const history = [];
        const items = document.querySelectorAll('.query-content, .model-response-text');
        items.forEach(el => {
            history.push({
                role: el.classList.contains('query-content') ? 'user' : 'model',
                content: el.innerText.trim()
            });
        });
        return history;
    };
})();
