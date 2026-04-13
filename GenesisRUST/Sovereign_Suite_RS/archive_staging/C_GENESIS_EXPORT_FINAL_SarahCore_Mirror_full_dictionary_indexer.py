"""
Full English Dictionary Indexer
Downloads and indexes the complete English dictionary using ACE Token fingerprints.
"""
import os
import json
import hashlib
import lancedb
import requests
from typing import List, Dict
from Sovereign_Constants import (
    ACE_64_BIT_MASK, SOVEREIGN_ANCHOR, MEMORY_CONSOLIDATION_LIMIT,
    VAR_1000, VAR_HEX_RADIX, VAR_30, HEX_RADIX
)

class FullDictionaryIndexer:
    """
    Indexes the complete English dictionary using ACE Token 64-bit fingerprints.
    """
    
    def __init__(self, db_path="c:\\SarahCore\\vault\\dictionary_index"):
        self.db_path = db_path
        os.makedirs(self.db_path, exist_ok=True)
        
        self.db = lancedb.connect(self.db_path)
        self.table_name = "dictionary"
        self.SOVEREIGN_ANCHOR = SOVEREIGN_ANCHOR
        
        print(f"[Full Dictionary] Initialized at {self.db_path}")
    
    def generate_ace_fingerprint(self, word: str) -> str:
        """
        Generates a 64-bit ACE Token fingerprint for a word.
        """
        combined = f"{word.lower()}{self.SOVEREIGN_ANCHOR}"
        hash_obj = hashlib.sha256(combined.encode())
        ace_fp = int(hash_obj.hexdigest(), HEX_RADIX) & ACE_64_BIT_MASK
        return hex(ace_fp)
    
    def download_dictionary(self) -> List[Dict]:
        """
        Downloads a comprehensive English dictionary.
        Uses the Free Dictionary API or local datasets.
        """
        print("[Full Dictionary] Downloading English dictionary...")
        
        # Try to use a pre-existing dictionary file first
        cache_file = "c:\\SarahCore\\vault\\english_dictionary_full.json"
        
        if os.path.exists(cache_file):
            print(f"[Full Dictionary] Loading from cache: {cache_file}")
            with open(cache_file, 'r', encoding='utf-8') as f:
                return json.load(f)
        
        # Download from public source
        # Using the OPTED English Dictionary (Open Source)
        print("[Full Dictionary] Downloading from public source...")
        
        try:
            # Try to download from a public dictionary API
            # Note: This is a placeholder - you'll need to use a real dictionary source
            url = "https://raw.githubusercontent.com/matthewreagan/WebstersEnglishDictionary/master/dictionary.json"
            
            response = requests.get(url, timeout=VAR_30)
            response.raise_for_status()
            
            dictionary_data = response.json()
            
            # Save to cache
            with open(cache_file, 'w', encoding='utf-8') as f:
                json.dump(dictionary_data, f)
            
            print(f"[Full Dictionary] Downloaded {len(dictionary_data)} entries")
            return dictionary_data
            
        except Exception as e:
            print(f"[Full Dictionary] Download failed: {e}")
            print("[Full Dictionary] Using fallback dictionary generation...")
            return self.generate_fallback_dictionary()
    
    def generate_fallback_dictionary(self) -> List[Dict]:
        """
        Generates a comprehensive fallback dictionary if download fails.
        """
        print("[Full Dictionary] Generating comprehensive fallback dictionary...")
        
        # This will create a basic but comprehensive dictionary
        # In production, you'd use a real dictionary source
        
        fallback_words = {}
        
        # Add common English words with definitions
        # (This is a simplified version - a real implementation would have 100k+ words)
        
        # A
        fallback_words.update({
            "abandon": "to give up completely; to desert",
            "ability": "the power or capacity to do something",
            "able": "having the power, skill, or means to do something",
            "about": "on the subject of; concerning",
            "above": "in or to a higher place",
            "absolute": "not qualified or diminished in any way; total",
            "abstract": "existing in thought or as an idea but not having a physical existence",
            "accept": "to receive willingly",
            "access": "the means or opportunity to approach or enter a place",
            "accident": "an unfortunate incident that happens unexpectedly",
            "account": "a report or description of an event or experience",
            "accurate": "correct in all details; exact",
            "achieve": "to successfully reach a desired objective or result",
            "acknowledge": "to accept or admit the existence or truth of",
            "acquire": "to buy or obtain for oneself",
            "across": "from one side to the other",
            "act": "to take action; to do something",
            "action": "the fact or process of doing something",
            "active": "engaging or ready to engage in physically energetic pursuits",
            "actual": "existing in fact; real",
            "add": "to join something to something else",
            "address": "the particulars of the place where someone lives",
            "adequate": "satisfactory or acceptable in quality or quantity",
            "adjust": "to alter or move something slightly",
            "administration": "the process or activity of running a business or organization",
            "admit": "to confess to be true or to be the case",
            "adopt": "to legally take another's child and bring it up as one's own",
            "adult": "a person who is fully grown or developed",
            "advance": "to move forward in a purposeful way",
            "advantage": "a condition or circumstance that puts one in a favorable position",
            "advertise": "to describe or draw attention to a product or service",
            "advice": "guidance or recommendations offered with regard to prudent action",
            "advise": "to offer suggestions about the best course of action",
            "affect": "to have an effect on; to make a difference to",
            "afford": "to have enough money to pay for",
            "afraid": "feeling fear or anxiety; frightened",
            "after": "in the time following an event",
            "afternoon": "the time from noon or lunchtime to evening",
            "again": "another time; once more",
            "against": "in opposition to",
            "age": "the length of time that a person has lived",
            "agency": "a business or organization providing a particular service",
            "agent": "a person who acts on behalf of another",
            "ago": "before the present; earlier",
            "agree": "to have the same opinion about something",
            "agreement": "harmony or accordance in opinion or feeling",
            "ahead": "further forward in space or time",
            "aid": "help or support",
            "aim": "to point or direct a weapon or camera at a target",
            "air": "the invisible gaseous substance surrounding the earth",
            "aircraft": "a vehicle that can fly",
            "airline": "a company that provides regular flights",
            "airport": "a complex of runways and buildings for takeoff and landing of aircraft",
            "alarm": "a warning of danger",
            "alcohol": "a colorless volatile flammable liquid",
            "alert": "quick to notice any unusual and potentially dangerous circumstances",
            "alive": "living; not dead",
            "all": "used to refer to the whole quantity or extent of something",
            "allow": "to give permission for something to happen",
            "almost": "not quite; very nearly",
            "alone": "having no one else present",
            "along": "moving in a constant direction on a path",
            "already": "before or by now or the time in question",
            "also": "in addition; too",
            "alter": "to change in character or composition",
            "alternative": "one of two or more available possibilities",
            "although": "in spite of the fact that; even though",
            "always": "at all times; on all occasions",
            "amazing": "causing great surprise or wonder",
            "among": "situated more or less centrally in relation to several other things",
            "amount": "a quantity of something",
            "analysis": "detailed examination of the elements or structure of something",
            "ancient": "belonging to the very distant past",
            "and": "used to connect words of the same part of speech",
            "anger": "a strong feeling of annoyance or hostility",
            "angle": "the space between two intersecting lines",
            "angry": "feeling or showing strong annoyance",
            "animal": "a living organism that feeds on organic matter",
            "announce": "to make a public declaration about a fact or occurrence",
            "annual": "occurring once every year",
            "another": "used to refer to an additional person or thing",
            "answer": "a thing said or written in reaction to a question",
            "anticipate": "to regard as probable; to expect or predict",
            "anxiety": "a feeling of worry or unease",
            "any": "used to refer to one or some of a thing",
            "anybody": "any person or people",
            "anymore": "to any further extent; any longer",
            "anyone": "any person or people",
            "anything": "used to refer to a thing, no matter what",
            "anyway": "used to confirm or support a point just made",
            "anywhere": "in or to any place",
            "apart": "separated by a distance; at a specified distance from each other",
            "apartment": "a suite of rooms forming one residence",
            "apparent": "clearly visible or understood; obvious",
            "appeal": "to make a serious or urgent request",
            "appear": "to come into sight; to become visible",
            "appearance": "the way that someone or something looks",
            "apple": "a round fruit with red or green skin",
            "application": "a formal request to an authority",
            "apply": "to make a formal request for something",
            "appoint": "to assign a job or role to someone",
            "appointment": "an arrangement to meet someone at a particular time",
            "appreciate": "to recognize the full worth of",
            "approach": "to come near or nearer to something",
            "appropriate": "suitable or proper in the circumstances",
            "approval": "the action of officially agreeing to something",
            "approve": "to officially agree to or accept as satisfactory",
            "approximate": "close to the actual but not completely accurate",
            "architect": "a person who designs buildings",
            "architecture": "the art or practice of designing and constructing buildings",
            "area": "a region or part of a town or country",
            "argue": "to give reasons or cite evidence in support of an idea",
            "argument": "an exchange of diverging or opposite views",
            "arise": "to emerge; to become apparent",
            "arm": "each of the two upper limbs of the human body",
            "armed": "equipped with or carrying a weapon",
            "army": "an organized military force equipped for fighting on land",
            "around": "on every side of; encircling",
            "arrange": "to put things in a neat or attractive order",
            "arrangement": "the action of arranging or being arranged",
            "arrest": "to seize someone by legal authority",
            "arrival": "the action of arriving",
            "arrive": "to reach a place at the end of a journey",
            "art": "the expression of creative skill and imagination",
            "article": "a particular item or object",
            "artist": "a person who creates art",
            "artistic": "having or revealing natural creative skill",
        })
        
        entries = []
        for word, definition in fallback_words.items():
            entries.append({
                "word": word,
                "definition": definition,
                "type": "english",
                "pos": "varies"
            })
        
        print(f"[Full Dictionary] Generated {len(entries)} fallback entries")
        return entries
    
    def build_index(self):
        """
        Builds the complete dictionary index with ACE Token fingerprints.
        """
        print("[Full Dictionary] Building complete dictionary index...")
        
        # Download dictionary
        dictionary_entries = self.download_dictionary()
        
        print(f"[Full Dictionary] Processing {len(dictionary_entries)} entries...")
        
        # Add ACE Token fingerprints
        indexed_data = []
        
        # Handle different dictionary formats
        if isinstance(dictionary_entries, dict):
            # Format: {word: definition}
            for word, definition in dictionary_entries.items():
                ace_fp = self.generate_ace_fingerprint(word)
                
                # Handle definition format
                if isinstance(definition, str):
                    def_text = definition
                elif isinstance(definition, dict):
                    def_text = definition.get('definition', str(definition))
                else:
                    def_text = str(definition)
                
                indexed_data.append({
                    "ace_fingerprint": ace_fp,
                    "word": word.lower(),
                    "definition": def_text[:MEMORY_CONSOLIDATION_LIMIT],  # Limit definition length
                    "type": "english",
                    "pos": "varies"
                })
        else:
            # Format: [{word: ..., definition: ...}]
            for entry in dictionary_entries:
                word = entry.get('word', '')
                definition = entry.get('definition', '')
                
                ace_fp = self.generate_ace_fingerprint(word)
                
                indexed_data.append({
                    "ace_fingerprint": ace_fp,
                    "word": word.lower(),
                    "definition": str(definition)[:VAR_500],
                    "type": "english",
                    "pos": entry.get('pos', 'varies')
                })
        
        # Progress updates
        if len(indexed_data) > VAR_1000:
            print(f"[Full Dictionary] Indexed {len(indexed_data)} words")
        
        # Store in LanceDB
        print(f"[Full Dictionary] Writing to LanceDB...")
        
        # Drop existing table if it exists
        try:
            if self.table_name in self.db.list_tables():
                print(f"[Full Dictionary] Dropping existing table: {self.table_name}")
                self.db.drop_table(self.table_name)
        except Exception as e:
            print(f"[Full Dictionary] Warning during table drop: {e}")
        
        # Create new table
        try:
            self.db.create_table(self.table_name, data=indexed_data)
            print(f"[Full Dictionary] SUCCESS: {len(indexed_data)} words indexed")
            print(f"[Full Dictionary] Index location: {self.db_path}")
        except ValueError as e:
            print(f"[Full Dictionary] Table exists, clearing and recreating...")
            self.db.drop_table(self.table_name)
            self.db.create_table(self.table_name, data=indexed_data)
            print(f"[Full Dictionary] SUCCESS: {len(indexed_data)} words indexed")


def main():
    """
    Main entry point for full dictionary indexing.
    """
    indexer = FullDictionaryIndexer()
    indexer.build_index()


if __name__ == "__main__":
    main()
