
import asyncio
from coding_encyclopedia_indexer import CodingEncyclopediaIndexer

async def main():
    indexer = CodingEncyclopediaIndexer()
    await indexer.build_index()
    print("[Success] Sovereign Knowledge Vault populated with 100 categories.")

if __name__ == "__main__":
    asyncio.run(main())
