"""
Google Developer Knowledge Ingester - Focused on Google Technologies
Ingests Firebase, Android, and Google Cloud documentation.
"""
import os
import json
import requests
from typing import List, Dict
from coding_encyclopedia_indexer import CodingEncyclopediaIndexer

VAR_10 = 10
VAR_20 = 20
VAR_500 = 500

class GoogleTechIngester:
    """
    Focused ingester for Google developer technologies.
    """
    
    def __init__(self, api_key: str = None, db_path="c:\\SarahCore\\vault\\coding_encyclopedia"):
        self.api_key = api_key or os.environ.get('GOOGLE_DEV_API_KEY')
        self.indexer = CodingEncyclopediaIndexer(db_path)
        self.base_url = "https://developerknowledge.googleapis.com/v1alpha"
        
        print(f"[Sarah] Google Tech Ingester initialized")
        print(f"[Sarah] Targeting: Firebase, Android, Google Cloud")
    
    def search_docs(self, query: str, max_results: int = VAR_20) -> List[Dict]:
        """Search Google developer documentation."""
        endpoint = f"{self.base_url}/documents:searchDocumentChunks"
        
        params = {
            'key': self.api_key,
            'query': query,
            'pageSize': max_results
        }
        
        try:
            response = requests.get(endpoint, params=params, timeout=VAR_10)
            response.raise_for_status()
            
            data = response.json()
            results = data.get('results', [])
            
            print(f"[Sarah] ✓ Found {len(results)} chunks for '{query}'")
            return results
            
        except Exception as e:
            print(f"[Sarah] ✗ Error for '{query}': {e}")
            return []
    
    def ingest_topic(self, topic: str, category: str) -> int:
        """Ingest a specific topic."""
        results = self.search_docs(topic)
        
        if not results:
            return 0
        
        entries = []
        for result in results:
            content = result.get('content', '')
            parent = result.get('parent', '')
            chunk_id = result.get('id', '')
            
            entry = {
                'term': f"{topic} ({parent.split('/')[-1] if parent else 'Google'})",
                'description': content[:VAR_500] if len(content) > VAR_500 else content,
                'category': category,
                'language': 'general',
                'complexity': 'N/A',
                'use_cases': json.dumps([topic, category]),
                'implementation': content,
                'related': parent
            }
            
            ace_fp = self.indexer.generate_ace_fingerprint(entry['term'])
            entry['ace_fingerprint'] = ace_fp
            entries.append(entry)
        
        # Add to database
        try:
            table = self.indexer.db.open_table(self.indexer.table_name)
            table.add(entries)
            print(f"[Sarah] ✓ Indexed {len(entries)} entries for {topic}")
            return len(entries)
        except Exception as e:
            print(f"[Sarah] ✗ Error indexing {topic}: {e}")
            return 0
    
    def run_ingestion(self):
        """Run focused ingestion on Google technologies."""
        
        # Google-specific topics that the API actually has
        google_topics = {
            "Firebase": [
                "Firebase Authentication",
                "Cloud Firestore",
                "Realtime Database",
                "Cloud Storage",
                "Cloud Functions",
                "Firebase Hosting",
                "Firebase Analytics",
                "Cloud Messaging",
                "Remote Config",
                "Firebase Security Rules"
            ],
            "Android": [
                "Android Activities",
                "Android Fragments",
                "Android Services",
                "Android Jetpack",
                "Jetpack Compose",
                "Android ViewModel",
                "Android LiveData",
                "Android Room Database",
                "Android Navigation",
                "Material Design Android"
            ],
            "Google Cloud": [
                "Google Cloud Storage",
                "Google Cloud Functions",
                "Google Cloud Run",
                "Google Kubernetes Engine",
                "Google Cloud SQL",
                "Google BigQuery",
                "Google Cloud Pub/Sub",
                "Google Cloud IAM",
                "Google Cloud Logging",
                "Google Cloud Monitoring"
            ]
        }
        
        print(f"\n[Sarah] ═══════════════════════════════════════")
        print(f"[Sarah] GOOGLE TECHNOLOGIES INGESTION")
        print(f"[Sarah] ═══════════════════════════════════════\n")
        
        total_entries = 0
        
        for category, topics in google_topics.items():
            print(f"\n[Sarah] === {category} ===")
            for topic in topics:
                count = self.ingest_topic(topic, category)
                total_entries += count
        
        print(f"\n[Sarah] ═══════════════════════════════════════")
        print(f"[Sarah] INGESTION COMPLETE")
        print(f"[Sarah] Total entries: {total_entries}")
        print(f"[Sarah] ═══════════════════════════════════════\n")
        
        return total_entries


def main():
    """Run Google tech ingestion."""
    ingester = GoogleTechIngester()
    total = ingester.run_ingestion()
    
    print(f"[Sarah] Successfully ingested {total} Google developer documentation entries!")
    print(f"[Sarah] Sarah now has deep knowledge of Firebase, Android, and Google Cloud!")


if __name__ == "__main__":
    main()
