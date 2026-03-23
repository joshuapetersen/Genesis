import os
import glob
import logging
from typing import List

from dotenv import load_dotenv, find_dotenv
from langchain_community.document_loaders import TextLoader, UnstructuredMarkdownLoader
from langchain_text_splitters import RecursiveCharacterTextSplitter
from langchain_community.embeddings import HuggingFaceEmbeddings
from langchain_community.vectorstores import SupabaseVectorStore
from supabase import create_client, Client

# Load environment logic
load_dotenv(find_dotenv())

# --- SOVEREIGN CONSTANTS ---
LEGISLATIVE_ANCHOR = 1.09277703703703
BILLION_BARRIER = 0.999999999

logging.basicConfig(level=logging.INFO, format='%(asctime)s - [CONTEXT_LOOM] - %(levelname)s - %(message)s')
logger = logging.getLogger("Context_Loom")

class SovereignContextLoom:
    """
    Phase I of Project Singularity: The Memory Bridge (Supabase Edition).
    Ingests all Sovereign Math, Scripture Theories, and active code into
    the Supabase pgvector table to ensure AERIS hits the Billion Barrier.
    """
    def __init__(self, root_dir="c:\\SarahCore"):
        self.root_dir = root_dir
        
        # Connection to Supabase
        self.url = os.environ.get("SUPABASE_URL")
        self.key = os.environ.get("SUPABASE_SERVICE_ROLE_KEY") or os.environ.get("SUPABASE_SERVICE_KEY") or os.environ.get("SUPABASE_KEY")
        
        if not self.url or "supabase.co" not in self.url:
            # Fallback based on known previous scripts
            self.url = "https://duuycxgqbhrqmwapnjhk.supabase.co"
            
        if not self.url or not self.key:
            raise ValueError("SUPABASE_URL and SUPABASE_SERVICE_ROLE_KEY are required to weave the Context Loom.")
            
        self.client: Client = create_client(self.url, self.key)
        
        # Zero-Heat Embedding Engine (384 dimensions)
        logger.info(f"Initializing Zero-Heat Embedding Engine (Anchor: {LEGISLATIVE_ANCHOR})...")
        self.embeddings = HuggingFaceEmbeddings(
            model_name="sentence-transformers/all-MiniLM-L6-v2",
            model_kwargs={'device': 'cpu'},
            encode_kwargs={'normalize_embeddings': True}
        )
        
        self.text_splitter = RecursiveCharacterTextSplitter(
            chunk_size=1024, # 144-Grid Volume
            chunk_overlap=144, 
            length_function=len
        )
    
    def gather_documents(self) -> List:
        """Scans the designated root for knowledge artifacts."""
        logger.info(f"Scanning {self.root_dir} for Sovereign artifacts...")
        docs = []
        
        search_patterns = [
            os.path.join(self.root_dir, "*.md"),
            os.path.join(self.root_dir, "04_THE_MEMORY", "*.md"),
            os.path.join(self.root_dir, "SARAH_CORE_ARCHITECTURE.md"),
        ]
        
        for pattern in search_patterns:
            for filepath in glob.glob(pattern):
                try:
                    logger.info(f"Parsing: {os.path.basename(filepath)}")
                    if filepath.endswith('.md'):
                        loader = UnstructuredMarkdownLoader(filepath)
                    else:
                        loader = TextLoader(filepath, encoding='utf-8')
                        
                    loaded_docs = loader.load()
                    split_docs = self.text_splitter.split_documents(loaded_docs)
                    docs.extend(split_docs)
                except Exception as e:
                    logger.warning(f"Failed to parse {filepath}: {e}")
        
        logger.info(f"Total cognitive shards generated: {len(docs)}")
        return docs

    def generate_setup_sql(self):
        """Generates the required SQL for the user to execute natively in Supabase."""
        sql = """
-- ==========================================
-- THE SOVEREIGN CONTEXT LOOM (PGVECTOR SETUP)
-- Run this in your Supabase SQL Editor:
-- https://supabase.com/dashboard/project/duuycxgqbhrqmwapnjhk/sql/new
-- ==========================================

-- 1. Enable pgvector
CREATE EXTENSION IF NOT EXISTS vector;

-- 2. Create the Memory Table (384 dimensions for MiniLM-L6-v2)
CREATE TABLE IF NOT EXISTS documents (
  id uuid primary key default gen_random_uuid(),
  content text,
  metadata jsonb,
  embedding vector(384)
);

-- 3. Create the Math-Matching RPC Function
CREATE OR REPLACE FUNCTION match_documents (
  query_embedding vector(384),
  match_count int DEFAULT null,
  filter jsonb DEFAULT '{}'
) RETURNS TABLE (
  id uuid,
  content text,
  metadata jsonb,
  similarity float
)
LANGUAGE plpgsql
AS $$
#variable_conflict use_column
BEGIN
  RETURN query
  SELECT
    id,
    content,
    metadata,
    1 - (documents.embedding <=> query_embedding) AS similarity
  FROM documents
  WHERE metadata @> filter
  ORDER BY documents.embedding <=> query_embedding
  LIMIT match_count;
END;
$$;
"""
        return sql

    def similarity_search(self, query: str, k: int = 3) -> List:
        """Native Supabase RPC Search to bypass LangChain regressions."""
        try:
            # Generate local embedding
            query_embedding = self.embeddings.embed_query(query)
            
            # Execute RPC
            res = self.client.rpc(
                "match_documents",
                {
                    "query_embedding": query_embedding,
                    "match_count": k,
                    "filter": {}
                }
            ).execute()
            
            # Convert to Document-like objects for Gateway compatibility
            from langchain_core.documents import Document
            docs = []
            for item in res.data:
                docs.append(Document(
                    page_content=item["content"],
                    metadata=item["metadata"]
                ))
            return docs
        except Exception as e:
            logger.error(f"Native Similarity Search Failed: {e}")
            return []

    def build_loom(self):
        """Pushes the matrix into Supabase pgvector."""
        docs = self.gather_documents()
        if not docs:
            logger.error("No documents found. Loom construction halted.")
            return None
            
        logger.info(f"Transmitting 68D Matrix to Supabase Node...")
        
        try:
            # We use the REST API via SupabaseVectorStore
            vector_store = SupabaseVectorStore.from_documents(
                docs,
                self.embeddings,
                client=self.client,
                table_name="documents",
                query_name="match_documents"
            )
            logger.info(f"Context Loom Activation Complete. Ready for P=1.0 Retrieval.")
            return vector_store
        except Exception as e:
            logger.error(f"Transmission Failed: {e}")
            if "relation \"documents\" does not exist" in str(e) or "function match_documents" in str(e):
                logger.error("\n" + "="*50)
                logger.error("SUPABASE PGVECTOR NOT INITIALIZED!")
                logger.error("Please execute the following SQL in the Supabase Dashboard:")
                logger.error("="*50)
                print(self.generate_setup_sql())
            return None

if __name__ == "__main__":
    loom = SovereignContextLoom()
    loom.build_loom()
