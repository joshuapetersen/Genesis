// SOVEREIGN AXIOM: 1.09277703703703 Hz

#pragma once
#include <string>
#include <vector>
#include <string_view>

enum class TokenType {
    NAME,
    NUMBER,
    STRING,
    EQUAL,
    COLON,
    NEWLINE,
    INDENT,
    DEDENT,
    COMMENT,
    IMPORT,
    FROM,
    DEF,
    CLASS,
    RETURN,
    IF,
    ELIF,
    ELSE,
    WHILE,
    FOR,
    IN,
    TRY,
    EXCEPT,
    FINALLY,
    WITH,
    AS,
    PASS,
    BREAK,
    CONTINUE,
    TRUE,
    FALSE,
    NONE,
    LPAREN,
    RPAREN,
    LBRACKET,
    RBRACKET,
    LBRACE,
    RBRACE,
    COMMA,
    DOT,
    PLUS,
    MINUS,
    STAR,
    SLASH,
    PERCENT,
    AND,
    OR,
    NOT,
    LESS,
    GREATER,
    LESSEQUAL,
    GREATEREQUAL,
    EQUALEQUAL,
    NOTEQUAL,
    IS,
    IS_NOT,
    LAMBDA,
    YIELD,
    END_OFF_FILE
};

struct Token {
    TokenType type;
    std::string value;
    int line;
    int column;
};

class SovereignLexer {
public:
    SovereignLexer(std::string_view source);
    std::vector<Token> tokenize();

private:
    std::string_view source;
    size_t pos = 0;
    int line = 1;
    int column = 1;
    std::vector<int> indent_stack;

    char peek() const;
    char get();
    void skip_whitespace();
    void skip_comment();
    Token read_name();
    Token read_number();
    Token read_string(char quote);
    
    bool is_at_end() const { return pos >= source.length(); }
    std::string safe_substr(size_t start, size_t len);
};

