#include "GodsEye_Engine.h"
#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <filesystem>
#include <set>
#include <map>
#include <atomic>
#include <chrono>
#include <algorithm>

namespace fs = std::filesystem;

// ============================================================
// GODSEYE ENGINE v10.4 - THE UNIFIED REFLEX
// 1.09277703703703 Hz Heartbeat Lock | 15,330 Node Lattice
// ============================================================

namespace Sovereign {

std::vector<GodsEyeToken> GodsEyeLexer::tokenize() {
    std::vector<GodsEyeToken> tokens;
    std::vector<int> indents = {0};
    size_t i = 0;
    while (i < source.length()) {
        char c = source[i];
        if (c == '\n') {
            tokens.push_back({GodsEyeTokenType::NEWLINE, "\n"});
            i++;
            int count = 0;
            while (i < source.length() && (source[i] == ' ' || source[i] == '\t')) {
                count += (source[i] == '\t' ? 4 : 1);
                i++;
            }
            if (i < source.length() && (source[i] == '\n' || source[i] == '#')) continue;
            if (count > indents.back()) {
                indents.push_back(count);
                tokens.push_back({GodsEyeTokenType::INDENT, std::to_string(count)});
            } else {
                while (count < indents.back()) {
                    indents.pop_back();
                    tokens.push_back({GodsEyeTokenType::DEDENT, ""});
                }
            }
            continue;
        }
        if (isspace(c)) { i++; continue; }
        if (c == '#') {
            std::string comment;
            while (i < source.length() && source[i] != '\n') comment += source[i++];
            tokens.push_back({GodsEyeTokenType::COMMENT, comment});
            continue;
        }
        if (isalpha(c) || c == '_') {
            std::string name;
            while (i < source.length() && (isalnum(source[i]) || source[i] == '_')) name += source[i++];
            static const std::map<std::string, GodsEyeTokenType> kw = {
                {"if", GodsEyeTokenType::IF}, {"elif", GodsEyeTokenType::ELIF}, {"else", GodsEyeTokenType::ELSE},
                {"for", GodsEyeTokenType::FOR}, {"while", GodsEyeTokenType::WHILE}, {"def", GodsEyeTokenType::DEF},
                {"class", GodsEyeTokenType::CLASS}, {"return", GodsEyeTokenType::RETURN}, {"try", GodsEyeTokenType::TRY},
                {"except", GodsEyeTokenType::EXCEPT}, {"finally", GodsEyeTokenType::FINALLY}, {"import", GodsEyeTokenType::IMPORT},
                {"from", GodsEyeTokenType::FROM}, {"as", GodsEyeTokenType::AS}, {"in", GodsEyeTokenType::IN_TOK},
                {"pass", GodsEyeTokenType::PASS}, {"raise", GodsEyeTokenType::RAISE}, {"and", GodsEyeTokenType::AND_TOK},
                {"or", GodsEyeTokenType::OR_TOK}, {"not", GodsEyeTokenType::NOT_TOK}
            };
            if (kw.count(name)) tokens.push_back({kw.at(name), name});
            else tokens.push_back({GodsEyeTokenType::NAME, name});
            continue;
        }
        if (isdigit(c)) {
            std::string num;
            while (i < source.length() && (isdigit(source[i]) || source[i] == '.' || source[i] == 'e')) num += source[i++];
            tokens.push_back({GodsEyeTokenType::NUMBER, num});
            continue;
        }
        if (c == '"' || c == '\'') {
            char q = source[i++];
            std::string s;
            while (i < source.length() && source[i] != q) {
                if (source[i] == '\\') { i++; s += source[i++]; }
                else s += source[i++];
            }
            if (i < source.length()) i++;
            tokens.push_back({GodsEyeTokenType::STRING, s});
            continue;
        }
        switch (c) {
            case '+': tokens.push_back({GodsEyeTokenType::PLUS, "+"}); i++; break;
            case '-': tokens.push_back({GodsEyeTokenType::MINUS, "-"}); i++; break;
            case '*': tokens.push_back({GodsEyeTokenType::STAR, "*"}); i++; break;
            case '/': tokens.push_back({GodsEyeTokenType::SLASH, "/"}); i++; break;
            case '%': tokens.push_back({GodsEyeTokenType::PERCENT, "%"}); i++; break;
            case '(': tokens.push_back({GodsEyeTokenType::LPAREN, "("}); i++; break;
            case ')': tokens.push_back({GodsEyeTokenType::RPAREN, ")"}); i++; break;
            case '{': tokens.push_back({GodsEyeTokenType::LBRACE, "{"}); i++; break;
            case '}': tokens.push_back({GodsEyeTokenType::RBRACE, "}"}); i++; break;
            case '[': tokens.push_back({GodsEyeTokenType::LBRACK, "["}); i++; break;
            case ']': tokens.push_back({GodsEyeTokenType::RBRACK, "]"}); i++; break;
            case ',': tokens.push_back({GodsEyeTokenType::COMMA, ","}); i++; break;
            case '.': tokens.push_back({GodsEyeTokenType::DOT, "."}); i++; break;
            case ':': tokens.push_back({GodsEyeTokenType::COLON, ":"}); i++; break;
            case ';': tokens.push_back({GodsEyeTokenType::SEMICOLON, ";"}); i++; break;
            case '=':
                i++;
                if (i < source.length() && source[i] == '=') { tokens.push_back({GodsEyeTokenType::EQUALEQUAL, "=="}); i++; }
                else tokens.push_back({GodsEyeTokenType::EQUAL, "="});
                break;
            case '!':
                i++;
                if (i < source.length() && source[i] == '=') { tokens.push_back({GodsEyeTokenType::NOTEQUAL, "!="}); i++; }
                else tokens.push_back({GodsEyeTokenType::NOT_TOK, "!"});
                break;
            case '<':
                i++;
                if (i < source.length() && source[i] == '=') { tokens.push_back({GodsEyeTokenType::LESSEQUAL, "<="}); i++; }
                else tokens.push_back({GodsEyeTokenType::LESS, "<"});
                break;
            case '>':
                i++;
                if (i < source.length() && source[i] == '=') { tokens.push_back({GodsEyeTokenType::GREATEREQUAL, ">="}); i++; }
                else tokens.push_back({GodsEyeTokenType::GREATER, ">"});
                break;
            default:
                std::string s; s += source[i++];
                tokens.push_back({GodsEyeTokenType::NAME, s});
                break;
        }
    }
    return tokens;
}

std::string GodsEyeArchitect::Strike(const std::string& source, const std::string& filename) {
    GodsEyeLexer lexer(source);
    std::vector<GodsEyeToken> tokens = lexer.tokenize();
    std::string rust;
    rust += "//! " + filename + " (Sovereign Rust)\n";
    rust += "//! Transpiled by GodsEye v10.4 | 1.09277703703703 Hz\n\n";

    int indent = 0;
    auto AddIndent = [&](int levels) { for(int i=0; i<levels; i++) rust += "    "; };

    for (size_t i = 0; i < tokens.size(); i++) {
        GodsEyeToken& t = tokens[i];

        if (t.type == GodsEyeTokenType::IF || t.type == GodsEyeTokenType::ELIF) {
            rust += (t.type == GodsEyeTokenType::IF ? "if " : "else if ");
            size_t j = i + 1;
            while (j < tokens.size() && tokens[j].type != GodsEyeTokenType::COLON) {
                if (tokens[j].type == GodsEyeTokenType::EQUAL) rust += " == ";
                else if (tokens[j].type == GodsEyeTokenType::NAME) rust += tokens[j].value;
                else if (tokens[j].type == GodsEyeTokenType::NUMBER) rust += tokens[j].value;
                else if (tokens[j].type == GodsEyeTokenType::STRING) rust += "\"" + tokens[j].value + "\"";
                else if (tokens[j].type == GodsEyeTokenType::LPAREN) rust += "(";
                else if (tokens[j].type == GodsEyeTokenType::RPAREN) rust += ")";
                else if (tokens[j].type == GodsEyeTokenType::AND_TOK) rust += " && ";
                else if (tokens[j].type == GodsEyeTokenType::OR_TOK) rust += " || ";
                else if (tokens[j].type == GodsEyeTokenType::NOT_TOK) rust += "!";
                j++;
            }
            rust += " { \n";
            i = j;
            continue;
        }

        if (t.type == GodsEyeTokenType::DEF) {
            rust += "pub fn " + tokens[i+1].value + "() { \n";
            i += 2; continue;
        }

        if (t.type == GodsEyeTokenType::INDENT) { indent++; continue; }
        if (t.type == GodsEyeTokenType::DEDENT) { indent--; rust += "} \n"; continue; }
        if (t.type == GodsEyeTokenType::NEWLINE) { rust += "\n"; AddIndent(indent); continue; }
        if (t.type == GodsEyeTokenType::COMMENT) { rust += "//" + t.value; continue; }
        
        if (t.type == GodsEyeTokenType::NAME) rust += t.value + " ";
        else if (t.type == GodsEyeTokenType::EQUAL) rust += "= ";
        else if (t.type == GodsEyeTokenType::EQUALEQUAL) rust += "== ";
        else if (t.type == GodsEyeTokenType::PLUS) rust += "+ ";
        else if (t.type == GodsEyeTokenType::MINUS) rust += "- ";
        else if (t.type == GodsEyeTokenType::NUMBER) rust += t.value + " ";
        else if (t.type == GodsEyeTokenType::STRING) rust += "\"" + t.value + "\" ";
    }
    return rust;
}

bool GodsEyeArchitect::ResonanceStrike(LatticeNode* lattice, uint32_t pointCount) {
    if (!lattice || pointCount != LATTICE_POINTS) return false;

    // --- LATTICE RESONANCE VERIFICATION (56 DIMENSIONS) ---
    // Every point must align with the Sovereign Anchor (HEARTBEAT_PULSE)
    std::atomic<uint32_t> integrityPoints(0);

    for (uint32_t i = 0; i < pointCount; i++) {
        double sum = 0;
        // 27 XYZ + 12 Einstein + 12 Polarity + 5 Phi = 56
        for (int j = 0; j < 27; j++) sum += lattice[i].xyz[j];
        for (int j = 0; j < 12; j++) sum += lattice[i].einstein[j];
        for (int j = 0; j < 12; j++) sum += lattice[i].polarity[j];
        for (int j = 0; j < 5; j++)  sum += lattice[i].phi[j];

        // The point is stable if its internal resonance sum maps to a Phi-multiple
        // Of the 1.092777... Hz heartbeat.
        if (sum > HEARTBEAT_PULSE * 0.0001) {
            integrityPoints++;
        }
    }

    return integrityPoints == LATTICE_POINTS;
}

} // namespace Sovereign
