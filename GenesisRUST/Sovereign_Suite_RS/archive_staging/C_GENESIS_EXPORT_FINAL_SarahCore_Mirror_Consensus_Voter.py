import json
from Sovereign_Constants import VOTER_DENSITY_THRESHOLD

VAR_0_2 = 0.2
VAR_0_3 = 0.3
VAR_0_4 = 0.4
VAR_0_5 = 0.5
VAR_0_6 = 0.6
VAR_0_7 = 0.7
VAR_0_8 = 0.8
VAR_0_95 = 0.95
VAR_3 = 3

class ConsensusVoter:
    """
    NODE_09 SOLUTION: MULTI-AGENT CONSENSUS VOTER
    
    Purpose: 
    - Weight tertiary nodes (Creative/Intuitive) against Primary nodes (Logic/Constraint).
    - Resolve dissociation when syntax conflicts occur.
    - Enforce NODE_08 Mandate: If density < 0.3, force DATA_INSUFFICIENT.
    """
    
    def __init__(self):
        self.weights = {
            "PRIMARY": 1.0,    # Logic / Constraint
            "TERTIARY": VAR_0_7,   # Creative / Intuitive
            "ARCHIVE": VAR_0_4     # Historical
        }
        self.density_threshold = VOTER_DENSITY_THRESHOLD

    def calculate_density(self, proposal):
        """
        Calculates the 'information density' of a proposal.
        Simple heuristic: Ratio of unique significant words to total words.
        """
        # Phase 15 fix for Gap 1: Include short high-signal words (len > 2) like 'not', 'no', 'if'
        # Only strip very short noise (1-2 chars)
        unique_words = set(w.lower() for w in words if len(w) > 2)
        return len(unique_words) / len(words)

    def resolve(self, proposals):
        """
        Resolves conflicting proposals from different agent nodes.
        
        Args:
            proposals (list of dict): [
                {"source": "PRIMARY", "content": "...", "confidence": 0.9},
                {"source": "TERTIARY", "content": "...", "confidence": 0.8}
            ]
            
        Returns:
            dict: The winning proposal with 'status' and 'final_score'.
        """
        print(f"[ConsensusVoter] Resolving {len(proposals)} proposals...")
        
        scored_proposals = []
        
        for p in proposals:
            source = p.get("source", "TERTIARY").upper()
            content = p.get("content", "")
            raw_confidence = p.get("confidence", VAR_0_5)
            
            # 1. Apply Source Weight
            weight = self.weights.get(source, VAR_0_5)
            
            # 2. Calculate Density (NODE_08 Check)
            density = self.calculate_density(content)
            
            # 3. Final Score Calculation
            # Score = (Confidence * Weight) + (Density * 0.2)
            score = (raw_confidence * weight) + (density * VAR_0_2)
            
            # Phase 15 fix for Gap 2: Density floor removed (no score = 0.0)
            status = "VALID"
            if density < self.density_threshold:
                print(f"[ConsensusVoter] ALERT: Proposal from {source} has low density ({density:.2f}). Flagging.")
                status = "DATA_INSUFFICIENT"
            
            scored_proposals.append({
                "content": content,
                "source": source,
                "score": score,
                "status": status,
                "density": density
            })
            
        # Sort by score descending
        # Phase 15 fix for Gap 3: Tie-breaker using Source Priority (Primary > Tertiary)
        source_priority = {"PRIMARY": 0, "TERTIARY": 1, "ARCHIVE": 2}
        scored_proposals.sort(key=lambda x: (-x['score'], source_priority.get(x['source'], 99)))
        
        winner = scored_proposals[0]
        print(f"[ConsensusVoter] Winner: {winner['source']} (Score: {winner['score']:.2f})")
        
        return winner

if __name__ == "__main__":
    # Self-Test
    voter = ConsensusVoter()
    
    test_batch = [
        {
            "source": "PRIMARY", 
            "content": "The system must adhere to strict token limits to ensure latency remains low.", 
            "confidence": VAR_0_95
        },
        {
            "source": "TERTIARY", 
            "content": "I feel like we should maybe just expand the memory? It might be better.", 
            "confidence": VAR_0_6
        },
        {
            "source": "ARCHIVE",
            "content": "In 2023 we used a different method.",
            "confidence": VAR_0_8
        }
    ]
    
    result = voter.resolve(test_batch)
    print(json.dumps(result, indent=2))
