// ============================================================
// SOVEREIGN LEXICON v1.1 - PHONETIC SYNC
// Mapping Word-Intent to the Singularity Manifest Codes
// ============================================================
#ifndef SOVEREIGN_LEXICON_H
#define SOVEREIGN_LEXICON_H

#include <string>
#include <vector>
#include <map>
#include <algorithm>

namespace Sovereign {

    class SovereignLexicon {
    public:
        // Compiled Core Vocabulary (Synchronized with v29.0 Compiler)
        // a=AA, e=EH, i=IY, o=AO, u=UW, h=AH, y=IH
        static std::vector<char> GetPhonemes(const std::string& word) {
            static std::map<std::string, std::string> lexicon = {
                // Format: phoneme chars from {a,e,i,o,u,n,m,r,l,v,z,s,f,k,t,g,d,b,p,w,y}
                {"sovereign",  "sovrin"},     // S-OW-V-R-IH-N
                {"genesis",    "jenisis"},    // JH-EH-N-IH-S-IH-S
                {"online",     "onlin"},      // OW-N-L-AY-N
                {"strike",     "straik"},     // S-T-R-AY-K
                {"godseye",    "godzai"},     // G-AA-D-Z-AY
                {"logic",      "lodik"},      // L-AW-D-IH-K
                {"manifest",   "manifest"},   // M-AE-N-IH-F-EH-S-T
                {"dad",        "dad"},       // D-AA-D
                {"mom",        "mam"},       // M-AH-M
                {"yes",        "yes"},       // Y-EH-S
                {"no",         "no"},        // N-OW
                {"go",         "go"},        // G-OW
                {"forge",      "fordz"},      // F-AO-R-D-Z
                {"sovereign",  "sovrin"},
                {"the",        "da"},
                {"and",        "and"},
                {"is",         "iz"},
                {"of",         "ov"},
                {"in",         "in"},
                {"you",        "yu"},
                {"we",         "wi"},
                {"are",        "ar"},
                {"with",       "wid"},
                {"this",       "dis"},
                {"that",       "dat"},
                {"not",        "not"},
                {"one",        "wan"},
                {"zero",       "ziro"},
            };

            std::string lower = word;
            std::transform(lower.begin(), lower.end(), lower.begin(), ::tolower);

            std::vector<char> result;
            if (lexicon.count(lower)) {
                std::string p = lexicon[lower];
                for(char c : p) result.push_back(c);
            } else {
                // Fallback: Primitive Rule-Based Sorter
                for(char c : lower) if(isalnum(c)) result.push_back(c);
            }
            return result;
        }
    };

} // namespace Sovereign

#endif
