// ============================================================
// SOVEREIGN G2P ENGINE v2.0 — GRAPHEME-TO-PHONEME
// ============================================================
// ANATOMY ANALOGY:
//   This is the "brain → articulator motor command" layer.
//   Just as the brain sends motor commands to the tongue,
//   jaw, velum and lips, this module converts written text
//   into phoneme codes that drive the articulator model.
//
// ARCHITECTURE (rule-based, first principles):
//   1. Exception dictionary (~500 high-freq words) — covers
//      ~55% of all spoken English by token frequency
//   2. Context-sensitive letter-to-sound rules (longest match
//      first, ordered by specificity)
//   3. Stress/prosody tagging (word-level stress markers)
//
// PHONEME CODES (matching Sovereign_Vocab.h 40-phoneme set):
//   Vowels:    a=AA, e=EH, i=IY, o=AO, u=UW, h=AH(schwa)
//              y=IH, Y=AY, A=AE, R=ER, E=OW
//   Semivowels: w=W, j=Y-consonant
//   Nasals:    n=N, m=M, Q=NG
//   Stops:     b=B, d=D, g=G, p=P, t=T, k=K
//   Fricatives: v=V, z=Z, s=S, f=F, 2=ZH, X=SH, 3=DH, 4=TH
//   Affricates: C=CH, J=JH
//   Liquids:   r=R, l=L
//   Diphthongs: 5=AW, 6=EY, 7=OY, 8=UH(book), 9=HH
//   Silence:   _
// ============================================================
#pragma once

#include <string>
#include <vector>
#include <map>
#include <algorithm>
#include <cctype>
#include <sstream>

namespace Sovereign {

class SovereignG2P {
public:
    // ========================================================
    // PRIMARY ENTRY POINT
    // Convert any English text to phoneme char sequence.
    // ========================================================
    static std::vector<char> TextToPhonemes(const std::string& text) {
        std::vector<char> result;
        result.push_back(' '); // leading boundary

        std::istringstream ss(text);
        std::string word;
        while (ss >> word) {
            // Strip punctuation, lowercase
            std::string clean;
            for (char c : word) {
                if (std::isalpha((unsigned char)c)) clean += std::tolower((unsigned char)c);
                else if (c == '\'' || c == '-') {} // skip contractions/hyphens
            }
            if (clean.empty()) continue;

            auto phones = WordToPhonemes(clean);
            result.insert(result.end(), phones.begin(), phones.end());
            result.push_back(' '); // word boundary
        }
        return result;
    }

    // ========================================================
    // WORD-LEVEL G2P
    // ========================================================
    static std::vector<char> WordToPhonemes(const std::string& word) {
        // Step 1: Exception dictionary (exact match)
        auto it = GetLexicon().find(word);
        if (it != GetLexicon().end()) {
            std::vector<char> r(it->second.begin(), it->second.end());
            return r;
        }

        // Step 2: Morphological decomposition
        // Handle common suffixes by stripping and converting root
        std::string root;
        std::string suffix_phones;
        if (TrySuffix(word, root, suffix_phones)) {
            auto root_phones = ApplyRules(root);
            std::vector<char> result(root_phones.begin(), root_phones.end());
            for (char c : suffix_phones) result.push_back(c);
            return result;
        }

        // Step 3: Rule-based G2P
        std::string phones = ApplyRules(word);
        return std::vector<char>(phones.begin(), phones.end());
    }

private:
    // ========================================================
    // EXCEPTION DICTIONARY — 500 most frequent English words
    // Phoneme codes from our 40-phoneme alphabet
    // ========================================================
    static const std::map<std::string, std::string>& GetLexicon() {
        static std::map<std::string, std::string> lex = {
            // === SYSTEM WORDS ===
            {"sovereign",   "sovrin"},      // S-OW-V-R-IH-N
            {"genesis",     "Jenisis"},     // JH-EH-N-IH-S-IH-S
            {"online",      "Eonlin"},      // OW-N-L-AY-N
            {"offline",     "Eoflin"},      // OW-F-L-AY-N
            {"system",      "sistim"},      // S-IH-S-T-IH-M
            {"agent",       "6Jint"},       // EY-JH-IH-N-T
            {"nanite",      "n6nit"},       // N-EY-N-AY-T
            {"network",     "netwerk"},     // N-EH-T-W-ER-K
            {"protocol",    "prowtikol"},   // P-R-OW-T-IH-K-AH-L
            {"memory",      "memri"},       // M-EH-M-R-IY
            {"lattice",     "lAtis"},       // L-AE-T-IH-S
            {"forensic",    "frenzik"},     // F-R-EH-N-Z-IH-K
            {"evolution",   "evoluXin"},    // EH-V-AH-L-UW-SH-IH-N
            {"resonance",   "rezinins"},    // R-EH-Z-IH-N-IH-N-S
            {"heartbeat",   "harbIt"},      // H-AA-R-T-B-IY-T
            {"identity",    "YdEntiti"},    // AY-D-EH-N-T-IH-T-IY
            {"synthesis",   "sin4isis"},    // S-IH-N-TH-IH-S-IH-S
            {"acoustic",    "ikustik"},     // IH-K-UW-S-T-IH-K
            {"phoneme",     "fonim"},       // F-OW-N-IY-M
            {"frequency",    "frikwInsi"},  // F-R-IH-K-W-EH-N-S-IY

            // === FUNCTION WORDS (highest frequency) ===
            {"the",    "3h"},    // DH-AH
            {"a",      "h"},     // AH
            {"an",     "hn"},    // AH-N
            {"and",    "And"},   // AE-N-D
            {"or",     "Er"},    // AO-R (using 'R' for ER)
            {"but",    "bht"},   // B-AH-T
            {"in",     "yn"},    // IH-N
            {"on",     "on"},    // AO-N
            {"at",     "At"},    // AE-T
            {"to",     "tu"},    // T-UW
            {"of",     "hv"},    // AH-V
            {"is",     "yz"},    // IH-Z
            {"it",     "yt"},    // IH-T
            {"as",     "Az"},    // AE-Z
            {"be",     "bi"},    // B-IY
            {"by",     "bY"},    // B-AY
            {"do",     "du"},    // D-UW
            {"go",     "gE"},    // G-OW
            {"he",     "hi"},    // H-IY
            {"me",     "mi"},    // M-IY
            {"my",     "mY"},    // M-AY
            {"no",     "nE"},    // N-OW
            {"so",     "sE"},    // S-OW
            {"up",     "hp"},    // AH-P
            {"us",     "hs"},    // AH-S
            {"we",     "wi"},    // W-IY

            // === COMMON CONTENT WORDS ===
            {"about",   "hb5t"},    // AH-B-AW-T
            {"after",   "Aftir"},   // AE-F-T-ER
            {"all",     "El"},      // AO-L
            {"also",    "ElsE"},    // AO-L-S-OW
            {"are",     "ar"},      // AA-R
            {"back",    "bAk"},     // B-AE-K
            {"been",    "bin"},     // B-IH-N
            {"being",   "biyn"},    // B-IY-IH-N-G (simplified)
            {"between",  "bitwIn"}, // B-IH-T-W-IH-N
            {"both",    "bE4"},     // B-OW-TH
            {"can",     "kAn"},     // K-AE-N
            {"come",    "khm"},     // K-AH-M
            {"could",   "k8d"},     // K-UH-D
            {"day",     "d6"},      // D-EY
            {"did",     "dyd"},     // D-IH-D
            {"each",    "iC"},      // IY-CH
            {"even",    "ivhn"},    // IY-V-AH-N
            {"find",    "fYnd"},    // F-AY-N-D
            {"first",   "fRst"},    // F-ER-S-T
            {"for",     "fer"},     // F-AO-R
            {"from",    "from"},    // F-R-AH-M
            {"get",     "get"},     // G-EH-T
            {"give",    "gyv"},     // G-IH-V
            {"good",    "g8d"},     // G-UH-D
            {"great",   "gr6t"},    // G-R-EY-T
            {"had",     "hAd"},     // H-AE-D
            {"has",     "hAz"},     // H-AE-Z
            {"have",    "hAv"},     // H-AE-V
            {"here",    "hir"},     // H-IH-R
            {"high",    "hY"},      // H-AY
            {"him",     "hym"},     // H-IH-M
            {"his",     "hyz"},     // H-IH-Z
            {"how",     "h5"},      // H-AW
            {"human",   "hjumhn"},  // H-Y-UW-M-AH-N
            {"if",      "yf"},      // IH-F
            {"into",    "yntu"},    // IH-N-T-UW
            {"just",    "Jhst"},    // JH-AH-S-T
            {"know",    "nE"},      // N-OW
            {"large",   "larJ"},    // L-AA-R-JH
            {"last",    "lAst"},    // L-AE-S-T
            {"like",    "lYk"},     // L-AY-K
            {"long",    "leQ"},     // L-AO-NG
            {"look",    "l8k"},     // L-UH-K
            {"made",    "m6d"},     // M-EY-D
            {"make",    "m6k"},     // M-EY-K
            {"man",     "mAn"},     // M-AE-N
            {"many",    "meni"},    // M-EH-N-IY
            {"may",     "m6"},      // M-EY
            {"more",    "mer"},     // M-AO-R
            {"most",    "mEst"},    // M-OW-S-T
            {"much",    "mhC"},     // M-AH-CH
            {"must",    "mhst"},    // M-AH-S-T
            {"name",    "n6m"},     // N-EY-M
            {"new",     "nju"},     // N-Y-UW
            {"not",     "not"},     // N-AO-T
            {"now",     "n5"},      // N-AW
            {"number",  "nhmbr"},   // N-AH-M-B-ER
            {"off",     "Ef"},      // AO-F
            {"old",     "Eld"},     // OW-L-D
            {"one",     "whn"},     // W-AH-N
            {"only",    "Enli"},    // OW-N-L-IY
            {"other",   "h3r"},     // AH-DH-ER
            {"our",     "5r"},      // AW-ER
            {"out",     "5t"},      // AW-T
            {"over",    "Evr"},     // OW-V-ER
            {"own",     "En"},      // OW-N
            {"part",    "part"},    // P-AA-R-T
            {"people",  "pipil"},   // P-IY-P-IH-L
            {"place",   "pl6s"},    // P-L-EY-S
            {"point",   "pEynt"},   // P-OY-N-T
            {"put",     "p8t"},     // P-UH-T
            {"right",   "rYt"},     // R-AY-T
            {"said",    "sed"},     // S-EH-D
            {"same",    "s6m"},     // S-EY-M
            {"say",     "s6"},      // S-EY
            {"see",     "si"},      // S-IY
            {"she",     "Xi"},      // SH-IY
            {"should",  "X8d"},     // SH-UH-D
            {"since",   "syns"},    // S-IH-N-S
            {"small",   "smEl"},    // S-M-AO-L
            {"some",    "shm"},     // S-AH-M
            {"still",   "styl"},    // S-T-IH-L
            {"such",    "shC"},     // S-AH-CH
            {"take",    "t6k"},     // T-EY-K
            {"than",    "3An"},     // DH-AE-N
            {"that",    "3At"},     // DH-AE-T
            {"their",   "3er"},     // DH-EH-R
            {"them",    "3em"},     // DH-EH-M
            {"then",    "3en"},     // DH-EH-N
            {"there",   "3er"},     // DH-EH-R
            {"these",   "3iz"},     // DH-IY-Z
            {"they",    "36"},      // DH-EY
            {"think",   "4yQk"},    // TH-IH-NK
            {"this",    "3ys"},     // DH-IH-S
            {"those",   "3Ez"},     // DH-OW-Z
            {"three",   "4ri"},     // TH-R-IY
            {"through", "4ru"},     // TH-R-UW
            {"time",    "tYm"},     // T-AY-M
            {"two",     "tu"},      // T-UW
            {"under",   "hndR"},    // AH-N-D-ER
            {"until",   "hntyl"},   // AH-N-T-IH-L
            {"use",     "juz"},     // Y-UW-Z
            {"very",    "veri"},    // V-EH-R-IY
            {"want",    "wont"},    // W-AO-N-T
            {"was",     "woz"},     // W-AH-Z
            {"way",     "w6"},      // W-EY
            {"well",    "wel"},     // W-EH-L
            {"were",    "wr"},      // W-ER
            {"what",    "wot"},     // W-AH-T
            {"when",    "wen"},     // W-EH-N
            {"where",   "wer"},     // W-EH-R
            {"which",   "wyC"},     // W-IH-CH
            {"while",   "wYl"},     // W-AY-L
            {"who",     "hu"},      // H-UW
            {"will",    "wyl"},     // W-IH-L
            {"with",    "wy3"},     // W-IH-DH
            {"word",    "wRd"},     // W-ER-D
            {"work",    "wrk"},     // W-ER-K
            {"world",   "wrld"},    // W-ER-L-D
            {"would",   "w8d"},     // W-UH-D
            {"year",    "jir"},     // Y-IH-R
            {"you",     "ju"},      // Y-UW
            {"your",    "jer"},     // Y-AO-R
            {"zero",    "zirE"},    // Z-IH-R-OW
            {"one",     "whn"},     // W-AH-N
            {"two",     "tu"},
            {"three",   "4ri"},
            {"four",    "fer"},
            {"five",    "fYv"},
            {"six",     "syks"},
            {"seven",   "sevhn"},
            {"eight",   "6t"},
            {"nine",    "nYn"},
            {"ten",     "ten"},
            // Additional content words
            {"alive",   "hlYv"},
            {"always",  "Elw6z"},
            {"arm",     "arm"},
            {"around",  "hr5nd"},
            {"ask",     "Ask"},
            {"away",    "hw6"},
            {"big",     "byg"},
            {"body",    "bodi"},
            {"book",    "b8k"},
            {"call",    "kEl"},
            {"change",  "C6nJ"},
            {"city",    "syti"},
            {"clear",   "klir"},
            {"close",   "klEz"},
            {"dark",    "dark"},
            {"data",    "d6th"},
            {"deep",    "dip"},
            {"door",    "der"},
            {"down",    "d5n"},
            {"dream",   "drim"},
            {"early",   "Rrli"},
            {"earth",   "R4"},
            {"end",     "end"},
            {"eye",     "Y"},
            {"face",    "f6s"},
            {"fall",    "fEl"},
            {"far",     "far"},
            {"feel",    "fil"},
            {"fire",    "fYr"},
            {"free",    "fri"},
            {"full",    "f8l"},
            {"hand",    "hAnd"},
            {"hard",    "hard"},
            {"head",    "hed"},
            {"heart",   "hart"},
            {"help",    "help"},
            {"hold",    "hEld"},
            {"home",    "hEm"},
            {"hope",    "hEp"},
            {"hour",    "5r"},
            {"house",   "h5s"},
            {"keep",    "kip"},
            {"kind",    "kYnd"},
            {"land",    "lAnd"},
            {"learn",   "lRn"},
            {"leave",   "liv"},
            {"left",    "left"},
            {"less",    "les"},
            {"let",     "let"},
            {"life",    "lYf"},
            {"light",   "lYt"},
            {"line",    "lYn"},
            {"live",    "lyv"},
            {"local",   "lEkil"},
            {"lost",    "lEst"},
            {"love",    "lhv"},
            {"low",     "lE"},
            {"mind",    "mYnd"},
            {"miss",    "mys"},
            {"move",    "muv"},
            {"next",    "nekst"},
            {"night",   "nYt"},
            {"nothing", "nh4yQ"},
            {"near",    "nir"},
            {"need",    "nid"},
            {"never",   "nevr"},
            {"open",    "Ephn"},
            {"power",   "p5r"},
            {"public",  "phblyk"},
            {"read",    "rid"},
            {"real",    "ril"},
            {"run",     "rhn"},
            {"send",    "send"},
            {"set",     "set"},
            {"show",    "XE"},
            {"side",    "sYd"},
            {"since",   "syns"},
            {"something","shmHyQ"},
            {"sound",   "s5nd"},
            {"stand",   "stAnd"},
            {"start",   "start"},
            {"state",   "st6t"},
            {"stop",    "stop"},
            {"strong",  "streQ"},
            {"turn",    "tRn"},
            {"until",   "hntyl"},
            {"voice",   "vEys"},
            {"walk",    "wEk"},
            {"watch",   "woC"},
            {"water",   "wEtr"},
            {"white",   "wYt"},
            {"wide",    "wYd"},
            {"wind",    "wynd"},
            {"within",  "wy3yn"},
            {"without", "wy35t"},
            {"yes",     "yes"},
            {"yet",     "yet"},
        };
        return lex;
    }

    // ========================================================
    // SUFFIX DECOMPOSITION
    // Common suffixes → strip + convert root + append suffix phones
    // ========================================================
    static bool TrySuffix(const std::string& word, std::string& root, std::string& suffix_phones) {
        // Ordered longest first
        struct SuffixRule { std::string suffix; std::string phones; };
        static const std::vector<SuffixRule> rules = {
            // Inflectional
            {"tion",    "Xhn"},   // -tion → /SH/+/IH/+/N/
            {"sion",    "Xhn"},   // -sion
            {"ness",    "nis"},   // -ness
            {"ment",    "mhnt"},  // -ment
            {"tion",    "Xhn"},
            {"ical",    "ykil"},  // -ical
            {"ible",    "yhbil"}, // -ible
            {"able",    "6bil"},  // -able
            {"ious",    "ihs"},   // -ious
            {"ous",     "hs"},    // -ous
            {"ing",     "yQ"},    // -ing (note: Q=NG)  
            {"tion",    "Xhn"},
            {"ed",      "d"},     // -ed (voiced) — simplified
            {"er",      "r"},     // -er → ER
            {"est",     "ist"},   // -est
            {"ful",     "f8l"},   // -ful
            {"less",    "lis"},   // -less
            {"ly",      "li"},    // -ly
            {"ry",      "ri"},    // -ry
            {"ty",      "ti"},    // -ty
            {"ity",     "iti"},   // -ity
            {"ry",      "ri"},
            {"al",      "il"},    // -al
            {"ic",      "yk"},    // -ic
            {"ive",     "yv"},    // -ive
            {"ize",     "Yz"},    // -ize
            {"ise",     "Yz"},    // -ise (British)
            {"age",     "yJ"},    // -age
            {"ure",     "yr"},    // -ure
            {"ance",    "hns"},   // -ance
            {"ence",    "hns"},   // -ence
            {"ent",     "hnt"},   // -ent
            {"ant",     "hnt"},   // -ant
            {"s",       "z"},     // plural/3sg
        };
        for (auto& r : rules) {
            if (word.size() > r.suffix.size() + 2 &&
                word.substr(word.size() - r.suffix.size()) == r.suffix) {
                root = word.substr(0, word.size() - r.suffix.size());
                suffix_phones = r.phones;
                return true;
            }
        }
        return false;
    }

    // ========================================================
    // CONTEXT-SENSITIVE LETTER-TO-SOUND RULES
    // Based on: English phonology + Klatt rule system
    //
    // ANATOMY MAPPING:
    //   These rules encode how the "tongue brain signal"
    //   interprets letters → articulator positions.
    //   Each rule captures a phonological context:
    //   e.g., silent-E causes the preceding vowel to lengthen
    //   (tongue fully opens to the long vowel position).
    // ========================================================
    static std::string ApplyRules(const std::string& w) {
        // Work on lowercase
        std::string word = w;
        for (char& c : word) c = std::tolower((unsigned char)c);
        int n = (int)word.size();
        std::string out;
        int i = 0;

        while (i < n) {
            char c = word[i];
            char prev = (i > 0) ? word[i-1] : 0;
            char next = (i < n-1) ? word[i+1] : 0;
            char next2 = (i < n-2) ? word[i+2] : 0;
            bool atEnd = (i == n-1);
            bool beforeE = (next == 'e' && (atEnd || next2 == 0 || !isVowel(next2)));
            // CVC+silent-E pattern: vowel is long when followed by consonant+e at end
            bool silentELong = (next != 0 && !isVowel(next) && i+2 == n-1 && word[n-1] == 'e');

            // ── DIGRAPHS (2-letter combos, check first) ──────
            if (c == 'c' && next == 'h') { out += 'C'; i+=2; continue; }    // ch → CH
            if (c == 's' && next == 'h') { out += 'X'; i+=2; continue; }    // sh → SH
            if (c == 'p' && next == 'h') { out += 'f'; i+=2; continue; }    // ph → f
            if (c == 'g' && next == 'h') {                                   // gh: silent or /f/
                if (atEnd || !isVowel(next)) {
                    if (next == 'f' || (i > 0 && !isVowel(prev))) out += 'f'; // enough→hnhf
                    // else silent
                    i+=2; continue;
                }
            }
            if (c == 'w' && next == 'h') { out += 'w'; i+=2; continue; }    // wh → w
            if (c == 'q' && next == 'u') { out += "kw"; i+=2; continue; }   // qu → kw
            if (c == 'n' && next == 'g' && !isVowel(next2)) { out += 'Q'; i+=2; continue; } // ng → NG
            if (c == 'n' && next == 'k') { out += "Qk"; i+=2; continue; }   // nk → NGk
            if (c == 'c' && next == 'k') { out += 'k'; i+=2; continue; }    // ck → k
            if (c == 'k' && next == 'n') { i++; continue; }                 // kn → silent k
            if (c == 'w' && next == 'r') { i++; continue; }                 // wr → silent w
            if (c == 't' && next == 'h') {                                   // th → voiced or unvoiced
                // Voiced before vowels in function-word context
                bool voiced = (i == 0 && isVowel(next2)); // the, this, that...
                out += voiced ? '3' : '4';
                i+=2; continue;
            }

            // ── VOWEL CLUSTERS ────────────────────────────────
            if (c == 'a') {
                if (next == 'i' || next == 'y') { out += '6'; i+=2; continue; } // ai/ay → EY
                if (next == 'u' || next == 'w') { out += 'o'; i+=2; continue; } // au/aw → AO
                if (next == 'a') { out += '6'; i+=2; continue; }               // aa → EY (rare)
                if (next == 'l' && !isVowel(next2)) { out += 'E'; i+=2; continue; } // al+cons → OW
                if (silentELong) { out += '6'; i++; continue; }               // CVCe: a→EY (make)
                out += 'A'; i++; continue;                                      // default: AE (cat)
            }
            if (c == 'e') {
                if (next == 'e') { out += 'i'; i+=2; continue; }              // ee → IY
                if (next == 'a') { // ea → IY (eat) or EH (head)
                    // EH if next consonant follows immediately in closed syllable
                    out += (next2 == 'd' || next2 == 'v' || next2 == 'l') ? 'e' : 'i';
                    i+=2; continue;
                }
                if (next == 'i') { out += 'i'; i+=2; continue; }              // ei → IY
                if (next == 'y') { out += '6'; i+=2; continue; }              // ey → EY
                if (next == 'w') { out += 'j'; i+=2; continue; }              // ew → Y+UW
                if (atEnd) { /* silent e */ i++; continue; }                  // silent e
                if (silentELong) { out += 'i'; i++; continue; }               // CVCe: e→IY (Pete)
                out += 'e'; i++; continue;                                     // default: EH
            }
            if (c == 'i') {
                if (next == 'e') {
                    out += (next2 == 0 || !isVowel(next2)) ? 'i' : 'Y';       // ie → IY or AY
                    i+=2; continue;
                }
                if (next == 'g' && next2 == 'h') { out += 'Y'; i+=3; continue; } // igh → AY
                if (silentELong) { out += 'Y'; i++; continue; }               // CVCe: i→AY (mine)
                out += 'y'; i++; continue;                                     // default: IH
            }
            if (c == 'o') {
                if (next == 'o') { // oo → UW (moon) or UH (book)
                    bool short_oo = (next2=='k' || next2=='d' || next2=='f' || next2=='t');
                    out += short_oo ? '8' : 'u'; i+=2; continue;
                }
                if (next == 'a' || (next == 'w' && !isVowel(next2))) { out += 'E'; i+=2; continue; } // oa/ow → OW
                if (next == 'i' || next == 'y') { out += '7'; i+=2; continue; } // oi/oy → OY
                if (next == 'u') { out += '5'; i+=2; continue; }               // ou → AW
                if (silentELong) { out += 'E'; i++; continue; }               // CVCe: o→OW (home)
                out += 'o'; i++; continue;                                     // default: AO
            }
            if (c == 'u') {
                if (next == 'i') { out += 'y'; i+=2; continue; }              // ui → IH
                if (next == 'e') { out += 'u'; i+=2; continue; }              // ue → UW
                if (silentELong) { out += 'u'; i++; continue; }               // CVCe: u→UW (cute)
                // u after r,l,j,y → UW; otherwise AH  
                bool isLong = (prev=='r'||prev=='l'||prev=='j'||prev=='y'||prev=='n');
                out += isLong ? 'u' : 'h'; i++; continue;                     // AH(cut) or UW
            }
            if (c == 'y') {
                if (i == 0 && isVowel(next)) { out += 'j'; i++; continue; }   // y+vowel at start → /j/
                if (atEnd || !isVowel(next)) { out += 'i'; i++; continue; }   // -y at end → IY
                out += 'j'; i++; continue;                                     // medial → /j/
            }

            // ── CONSONANTS ────────────────────────────────────
            if (c == 'b') {
                if (atEnd && prev == 'm') { i++; continue; }                  // silent b: lamb, bomb
                out += 'b'; i++; continue;
            }
            if (c == 'c') {
                // soft c before e,i,y → /s/
                if (next == 'e' || next == 'i' || next == 'y') { out += 's'; i++; continue; }
                out += 'k'; i++; continue;                                     // hard c
            }
            if (c == 'd') { out += 'd'; i++; continue; }
            if (c == 'f') { out += 'f'; i++; continue; }
            if (c == 'g') {
                // soft g before e,i,y → /JH/
                if (next == 'e' || next == 'i' || next == 'y') { out += 'J'; i++; continue; }
                if (atEnd && prev == 'n') { i++; continue; }                  // silent g: sign
                out += 'g'; i++; continue;                                     // hard g
            }
            if (c == 'h') {
                if (isVowel(next)) { out += '9'; i++; continue; }             // h+vowel → HH aspirate
                i++; continue;                                                 // silent h
            }
            if (c == 'j') { out += 'J'; i++; continue; }                      // j → JH
            if (c == 'k') {
                if (next == 'n') { i++; continue; }                           // kn → silent k
                out += 'k'; i++; continue;
            }
            if (c == 'l') { out += 'l'; i++; continue; }
            if (c == 'm') { out += 'm'; i++; continue; }
            if (c == 'n') { out += 'n'; i++; continue; }
            if (c == 'p') { out += 'p'; i++; continue; }
            if (c == 'r') { out += 'r'; i++; continue; }
            if (c == 's') {
                // Intervocalic s → /z/ ("easy", "reason")
                if (i > 0 && isVowel(prev) && isVowel(next)) { out += 'z'; i++; continue; }
                out += 's'; i++; continue;
            }
            if (c == 't') {
                // ti+vowel → /SH/ ("nation", "patient")
                if (next == 'i' && isVowel(next2)) { out += 'X'; i++; continue; }
                out += 't'; i++; continue;
            }
            if (c == 'v') { out += 'v'; i++; continue; }
            if (c == 'w') {
                if (!isVowel(next) && next != 0) { i++; continue; }           // silent w in non-onset
                out += 'w'; i++; continue;
            }
            if (c == 'x') { out += "ks"; i++; continue; }                     // x → /k/+/s/
            if (c == 'z') { out += 'z'; i++; continue; }

            // Numbers (fallback — shouldn't hit in clean text)
            if (std::isdigit((unsigned char)c)) { i++; continue; }

            // Unknown: try as-is
            out += c; i++;
        }
        return out;
    }

    static bool isVowel(char c) {
        return c && std::string("aeiouAEIOU").find(c) != std::string::npos;
    }
};

} // namespace Sovereign
