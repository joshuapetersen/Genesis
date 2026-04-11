// SOVEREIGN AXIOM: 1.09277703703703 Hz

#include "SovereignLexer.h"
#include <iostream>
#include <vector>
#include <string_view>
#include <cctype>
#include <map>

SovereignLexer::SovereignLexer(std::string_view source) : source(source) {
    indent_stack.push_back(0);
}

char SovereignLexer::peek() const {
    if (is_at_end()) return '\0';
    return source[pos];
}

char SovereignLexer::get() {
    char c = peek();
    pos++;
    if (c == '\n') {
        line++;
        column = 1;
    } else {
        column++;
    }
    return c;
}

void SovereignLexer::skip_whitespace() {
    while (peek() == ' ' || peek() == '\t') {
        get();
    }
}

void SovereignLexer::skip_comment() {
    if (peek() == '#') {
        while (peek() != '\n' && !is_at_end()) {
            get();
        }
    }
}

Token SovereignLexer::read_name() {
    size_t start = pos;
    while (isalnum(peek()) || peek() == '_') {
        get();
    }
    std::string value = safe_substr(start, pos - start);
    static const std::map<std::string, TokenType> keywords = {
        {"import", TokenType::IMPORT}, {"from", TokenType::FROM}, {"def", TokenType::DEF},
        {"class", TokenType::CLASS}, {"return", TokenType::RETURN}, {"if", TokenType::IF},
        {"elif", TokenType::ELIF}, {"else", TokenType::ELSE}, {"while", TokenType::WHILE},
        {"for", TokenType::FOR}, {"in", TokenType::IN}, {"try", TokenType::TRY},
        {"except", TokenType::EXCEPT}, {"finally", TokenType::FINALLY}, {"with", TokenType::WITH},
        {"as", TokenType::AS}, {"pass", TokenType::PASS}, {"break", TokenType::BREAK},
        {"continue", TokenType::CONTINUE}, {"True", TokenType::TRUE}, {"False", TokenType::FALSE},
        {"None", TokenType::NONE}, {"and", TokenType::AND}, {"or", TokenType::OR}, {"not", TokenType::NOT},
        {"is", TokenType::IS}, {"lambda", TokenType::LAMBDA}, {"yield", TokenType::YIELD}
    };
    if (keywords.count(value)) {
        if (value == "is" && peek() == ' ') {
             // lookahead for "not"
             size_t look = pos + 1;
             if (look + 3 < source.length() && source[look] == 'n' && source[look+1] == 'o' && source[look+2] == 't') {
                 // handles "is not" as a single logic unit if desired, but we'll keep it simple
             }
        }
        return {keywords.at(value), value, line, (int)(column - value.length())};
    }
    return {TokenType::NAME, value, line, (int)(column - value.length())};
}

Token SovereignLexer::read_number() {
    size_t start = pos;
    while (isdigit(peek()) || peek() == '.' || peek() == 'e' || peek() == 'x' || (peek() == '-' && start == pos) || (peek() == '+' && start == pos)) {
        get();
    }
    std::string value = safe_substr(start, pos - start);
    return {TokenType::NUMBER, value, line, (int)(column - value.length())};
}

Token SovereignLexer::read_string(char quote) {
    get(); // skip first quote
    // Check for triple-quoted string
    bool triple = false;
    if (peek() == quote && pos + 1 < source.length() && source[pos+1] == quote) {
        get(); get(); // skip second and third quote
        triple = true;
    }
    size_t start = pos;
    if (triple) {
        while (!is_at_end()) {
            if (peek() == quote && pos + 1 < source.length() && source[pos+1] == quote && pos + 2 < source.length() && source[pos+2] == quote) {
                break;
            }
            get();
        }
        std::string value = safe_substr(start, pos - start);
        if (!is_at_end()) { get(); get(); get(); } // skip closing triple
        return {TokenType::STRING, value, line, (int)(column - value.length())};
    } else {
        while (peek() != quote && peek() != '\n' && !is_at_end()) {
            if (peek() == '\\') { get(); if (!is_at_end()) get(); continue; } // skip escaped char
            get();
        }
        std::string value = safe_substr(start, pos - start);
        if (!is_at_end() && peek() == quote) get(); // skip closing quote
        return {TokenType::STRING, value, line, (int)(column - value.length())};
    }
}

std::vector<Token> SovereignLexer::tokenize() {
    std::vector<Token> tokens;
    while (!is_at_end()) {
        char c = peek();
        if (c == ' ' || c == '\t') {
            skip_whitespace();
        } else if (c == '#') {
            skip_comment();
        } else if (c == '\n') {
            tokens.push_back({TokenType::NEWLINE, "\n", line, column});
            get();
            // Handle Indentation
            int current_indent = 0;
            while (peek() == ' ' || peek() == '\t') {
                if (get() == ' ') current_indent++;
                else current_indent += 4; // tab counts as 4
            }
            if (current_indent > indent_stack.back()) {
                indent_stack.push_back(current_indent);
                tokens.push_back({TokenType::INDENT, "", line, column});
            } else while (indent_stack.size() > 1 && current_indent < indent_stack.back()) {
                indent_stack.pop_back();
                tokens.push_back({TokenType::DEDENT, "", line, column});
            }
        } else if (isalpha(c) || c == '_') {
            tokens.push_back(read_name());
        } else if (isdigit(c) || (c == '-' && pos + 1 < source.length() && isdigit(source[pos+1]))) {
            tokens.push_back(read_number());
        } else if (c == '"' || c == '\'') {
            tokens.push_back(read_string(c));
        } else if (c == '=') {
            get();
            if (peek() == '=') { get(); tokens.push_back({TokenType::EQUALEQUAL, "==", line, column-2}); }
            else { tokens.push_back({TokenType::EQUAL, "=", line, column-1}); }
        } else {
            // handle single characters
            switch (c) {
                case '(': get(); tokens.push_back({TokenType::LPAREN, "(", line, column-1}); break;
                case ')': get(); tokens.push_back({TokenType::RPAREN, ")", line, column-1}); break;
                case '[': get(); tokens.push_back({TokenType::LBRACKET, "[", line, column-1}); break;
                case ']': get(); tokens.push_back({TokenType::RBRACKET, "]", line, column-1}); break;
                case '{': get(); tokens.push_back({TokenType::LBRACE, "{", line, column-1}); break;
                case '}': get(); tokens.push_back({TokenType::RBRACE, "}", line, column-1}); break;
                case ':': get(); tokens.push_back({TokenType::COLON, ":", line, column-1}); break;
                case ',': get(); tokens.push_back({TokenType::COMMA, ",", line, column-1}); break;
                case '+': get(); tokens.push_back({TokenType::PLUS, "+", line, column-1}); break;
                case '-': get(); tokens.push_back({TokenType::MINUS, "-", line, column-1}); break;
                case '*': 
                    get(); 
                    if (peek() == '*') { get(); tokens.push_back({TokenType::STAR, "**", line, column-2}); }
                    else { tokens.push_back({TokenType::STAR, "*", line, column-1}); }
                    break;
                case '/': get(); tokens.push_back({TokenType::SLASH, "/", line, column-1}); break;
                case '%': get(); tokens.push_back({TokenType::PERCENT, "%", line, column-1}); break;
                case '.': get(); tokens.push_back({TokenType::DOT, ".", line, column-1}); break;
                case '<':
                    get();
                    if (peek() == '=') { get(); tokens.push_back({TokenType::LESSEQUAL, "<=", line, column-2}); }
                    else { tokens.push_back({TokenType::LESS, "<", line, column-1}); }
                    break;
                case '>':
                    get();
                    if (peek() == '=') { get(); tokens.push_back({TokenType::GREATEREQUAL, ">=", line, column-2}); }
                    else { tokens.push_back({TokenType::GREATER, ">", line, column-1}); }
                    break;
                case '!':
                    get();
                    if (peek() == '=') { get(); tokens.push_back({TokenType::NOTEQUAL, "!=", line, column-2}); }
                    else { tokens.push_back({TokenType::NOT, "!", line, column-1}); }
                    break;
                default: 
                    // Any other character is part of a name or an error, but we'll include it to avoid erasure
                    std::string val;
                    val += get();
                    tokens.push_back({TokenType::NAME, val, line, column-1}); 
                    break; 
            }
        }
    }
    return tokens;
}

std::string SovereignLexer::safe_substr(size_t start, size_t len) {
    if (start > source.length()) {
        static bool warned = false;
        if (!warned) { std::cerr << "[CRITICAL_DRFT] Substrate offset violation at " << start << " | len=" << len << std::endl; warned = true; }
        return "";
    }
    if (start + len > source.length()) len = source.length() - start;
    return std::string(source.substr(start, len));
}


