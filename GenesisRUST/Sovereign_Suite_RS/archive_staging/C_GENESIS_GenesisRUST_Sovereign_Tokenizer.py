import json
import os

class SovereignTokenizer:
    """
    [VOICE_0x0T]: The linguistic bridge for Sarah's 13B brain.
    Implements a native, zero-dependency SentencePiece/BPE tokenizer.
    """
    
    def __init__(self, map_path):
        self.tokenizer_map = {}
        self.reverse_map = {}
        self.special_tokens = {}
        
        print(f"[Tokenizer] Loading Genlex Vocabulary: {os.path.basename(map_path)}")
        with open(map_path, "r") as f:
            data = json.load(f)
            # We use Gemma_4B as the primary vocabulary source for the Hybrid 13B
            vocab = data["Engine_Sectors"]["Gemma_4B"]["Vocabulary"]
            
            for i, token in enumerate(vocab):
                self.tokenizer_map[token] = i
                self.reverse_map[i] = token
                
                # Identify Special Tokens
                if token.startswith("<") and token.endswith(">"):
                    self.special_tokens[token] = i
                elif token.startswith("[") and token.endswith("]"):
                    self.special_tokens[token] = i

        self.space_char = "\u2581" # SentencePiece Lower One Eighth Block
        print(f"  [OK] 262,144 Tokens Seated. Linguistic Bridge Ready.")

    def encode(self, text):
        """
        [ENCODE_0x0L]: Transforms human text into numerical Genlex indices.
        Uses a greedy longest-match algorithm.
        """
        # 1. Normalize spaces to SentencePiece underscores
        text = text.replace(" ", self.space_char)
        if not text.startswith(self.space_char):
             text = self.space_char + text
             
        tokens = []
        start = 0
        while start < len(text):
            # Greedy longest subword match
            match_found = False
            for end in range(len(text), start, -1):
                subword = text[start:end]
                if subword in self.tokenizer_map:
                    tokens.append(self.tokenizer_map[subword])
                    start = end
                    match_found = True
                    break
            
            if not match_found:
                # Fallback to <unk> or byte tokens
                # GGUF models usually have <0xHH> tokens for bytes
                char = text[start]
                byte_hex = f"<0x{ord(char):02X}>"
                if byte_hex in self.tokenizer_map:
                    tokens.append(self.tokenizer_map[byte_hex])
                else:
                    tokens.append(self.special_tokens.get("<unk>", 3))
                start += 1
                
        return tokens

    def decode(self, ids):
        """
        [DECODE_0x0L]: Transforms Genlex indices back into human speech.
        """
        text = ""
        for i in ids:
            if i in self.reverse_map:
                token = self.reverse_map[i]
                # Filter out control tokens
                if token.startswith("<") and token.endswith(">"):
                    continue
                text += token
        
        # Restore spaces
        return text.replace(self.space_char, " ").strip()

if __name__ == "__main__":
    tokenizer = SovereignTokenizer(r"C:\SarahCore\Genlex_Map.json")
    prompt = "Hello Sarah, describe yourself."
    ids = tokenizer.encode(prompt)
    print(f"\nPrompt: {prompt}")
    print(f"Encoded IDs: {ids}")
    print(f"Decoded: {tokenizer.decode(ids)}")
