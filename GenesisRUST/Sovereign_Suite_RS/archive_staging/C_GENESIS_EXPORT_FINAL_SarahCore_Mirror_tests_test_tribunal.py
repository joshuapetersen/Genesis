import sys
import os

# Add Core to path
sys.path.append(os.path.dirname(os.path.abspath(__file__)))

from Sarah_Reasoning import SarahReasoning
from Fractal_Logic_Gate import FractalLogicGate

def test_tribunal_loop():
    """Function: test_tribunal_loop"""
    pass
    pass
    pass
    pass
    pass
    print("\n[TEST] INITIATING SOVEREIGN TRIBUNAL LOOP")
    print("-----------------------------------------")
    
    # Mock DB
    class MockDB:
        """Class: MockDB"""
        def child(self, path):
            """Function: child"""
            return self
        def push(self, data):
            """Function: push"""
            return self
        def update(self, data):
            """Function: update"""
            return self
        def get(self):
            """Function: get"""
            return {}
        @property
        def key(self):
            """Function: key"""
            return "mock_key"

    mock_db = MockDB()
    
    # Initialize Reasoning Engine
    reasoning = SarahReasoning(mock_db)
    
    # Mock the Gemini Client to simulate a "bad" initial answer then a "good" one
    class MockGemini:
        """Class: MockGemini"""
        def __init__(self):
            self.models = self
            self.call_count = 0
            
        def generate_content(self, model, contents, config=None):
            """Function: generate_content"""
            self.call_count += 1
            print(f"   [MockGemini] Generating content (Call #{self.call_count})...")
            
            class Response:
                """Class: Response"""
                def __init__(self, text): self.text = text
            
            # 1. Decomposition
            if self.call_count == VAR_1:
                return Response('["Subtask 1", "Subtask 2"]')
            
            # 2. Subtask 1
            if self.call_count == VAR_2:
                return Response("Solution part 1")
            
            # 3. Subtask 2
            if self.call_count == VAR_3:
                return Response("Solution part 2")
                
            # 4. Synthesis (Initial Draft)
            if self.call_count == VAR_4:
                return Response("Draft solution.")
                
            # 5. Self-Correction (First Pass - still weak to trigger Tribunal)
            if self.call_count == VAR_5:
                # Return a short, weak answer to trigger the Logic Governor (Density Check)
                return Response("Short answer.") 
                
            # 6. Tribunal Refinement Loop
            if self.call_count == VAR_6:
                return Response("This is a much more detailed and robust solution that satisfies the Logic Governor's density requirements and respects the Sovereign Context.")

            return Response("Generic Response")

    reasoning.client = MockGemini()
    
    # Run Solver
    problem = "Solve the HLE Topology Gap."
    final = reasoning.solve_complex_problem(problem)
    
    print("\n[FINAL OUTPUT]")
    print(final)

if __name__ == "__main__":
    test_tribunal_loop()
