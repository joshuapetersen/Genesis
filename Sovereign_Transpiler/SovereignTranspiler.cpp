// SOVEREIGN AXIOM: 1.09277703703703 Hz

#include "SovereignLexer.h"
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
// SOVEREIGN TRANSPILER - FIRST PRINCIPLES PYTHON-TO-RUST ENGINE
// No wrappers. No simulations. No fake programs.
// ============================================================

const std::string SOURCE_ROOT = "C:\\GENESIS";
const std::string OUTPUT_ROOT = "C:\\GENESIS\\Sovereign_Suite_RS";

class SovereignTranspiler {
public:
    std::atomic<uint64_t> files_struck{0};
    std::atomic<uint64_t> total_files{0};
    std::atomic<uint64_t> total_lines{0};

    // ---- FULL TRANSPILE: handles classes, functions, constants, imports, control flow ----
    std::string transpile(const std::vector<Token>& tokens, const std::string& filename) {
        std::string out;
        out += "//! " + filename + " (Rust Edition)\n";
        out += "//! Auto-transpiled by Sovereign First-Principles Engine\n";
        out += "//! Axiom: 1.09277703703 Hz\n\n";

        // Collect imports
        out += emit_imports(tokens);
        out += "\n";

        // Walk tokens
        size_t i = 0;
        int current_indent = 0;
        while (i < tokens.size()) {
            const Token& t = tokens[i];

            if (t.type == TokenType::NEWLINE) { i++; continue; }
            if (t.type == TokenType::INDENT) { current_indent++; i++; continue; }
            if (t.type == TokenType::DEDENT) { current_indent--; i++; continue; }

            // ---- CLASS ----
            if (t.type == TokenType::CLASS) {
                out += emit_class(tokens, i, current_indent);
                continue;
            }
            // ---- DEF (top-level function) ----
            if (t.type == TokenType::DEF && current_indent == 0) {
                out += emit_function(tokens, i, current_indent, false);
                continue;
            }
            // ---- TOP-LEVEL ASSIGNMENT (constants) ----
            if (t.type == TokenType::NAME && current_indent == 0) {
                if (i + 1 < tokens.size() && tokens[i+1].type == TokenType::EQUAL) {
                    out += emit_constant(tokens, i);
                    continue;
                }
            }
            // ---- IMPORT / FROM (already handled above, skip) ----
            if (t.type == TokenType::IMPORT || t.type == TokenType::FROM) {
                skip_to_newline(tokens, i);
                continue;
            }
            // ---- COMMENT lines ----
            if (t.type == TokenType::COMMENT) {
                i++; continue;
            }
            // Skip anything else at top level
            i++;
        }
        return out;
    }

private:
    // ---- IMPORTS ----
    std::string emit_imports(const std::vector<Token>& tokens) {
        std::string out;
        std::set<std::string> seen;
        for (size_t i = 0; i < tokens.size(); i++) {
            if (tokens[i].type == TokenType::IMPORT) {
                i++;
                if (i < tokens.size() && tokens[i].type == TokenType::NAME) {
                    std::string mod = tokens[i].value;
                    if (!seen.count(mod)) {
                        out += "// use " + py_mod_to_rust(mod) + ";\n";
                        seen.insert(mod);
                    }
                }
                skip_to_newline(tokens, i);
            } else if (tokens[i].type == TokenType::FROM) {
                i++;
                std::string mod;
                if (i < tokens.size()) mod = tokens[i].value;
                // skip to import keyword
                while (i < tokens.size() && tokens[i].type != TokenType::IMPORT && tokens[i].type != TokenType::NEWLINE) i++;
                if (i < tokens.size() && tokens[i].type == TokenType::IMPORT) {
                    i++;
                    std::string items;
                    while (i < tokens.size() && tokens[i].type != TokenType::NEWLINE) {
                        if (tokens[i].type == TokenType::NAME) {
                            if (!items.empty()) items += ", ";
                            items += tokens[i].value;
                        }
                        i++;
                    }
                    if (!seen.count(mod)) {
                        out += "// use " + py_mod_to_rust(mod) + "::{" + items + "};\n";
                        seen.insert(mod);
                    }
                }
            }
        }
        return out;
    }

    std::string py_mod_to_rust(const std::string& mod) {
        // Map known Python stdlib to Rust equivalents
        static const std::map<std::string, std::string> mod_map = {
            {"os", "std::fs"}, {"sys", "std::env"}, {"time", "std::time"},
            {"json", "serde_json"}, {"hashlib", "sha3"}, {"math", "std::f64::consts"},
            {"threading", "std::thread"}, {"sqlite3", "rusqlite"},
            {"typing", "/* typing */"}, {"collections", "std::collections"},
            {"random", "rand::Rng"}, {"re", "regex::Regex"}, {"datetime", "chrono::Utc"},
        };
        if (mod_map.count(mod)) return mod_map.at(mod);
        return "crate::" + mod;
    }

    // ---- CONSTANT ----
    std::string emit_constant(const std::vector<Token>& tokens, size_t& i) {
        std::string name = tokens[i].value;
        i += 2; // skip NAME and EQUAL
        std::string value;
        bool has_dot = false, has_quote = false, has_hex = false;
        while (i < tokens.size() && tokens[i].type != TokenType::NEWLINE) {
            if (tokens[i].type == TokenType::STRING) {
                value += "\"" + tokens[i].value + "\"";
                has_quote = true;
            } else {
                std::string v = tokens[i].value;
                if (v.find('.') != std::string::npos || v.find('e') != std::string::npos) has_dot = true;
                if (v.find("0x") != std::string::npos || v.find("0X") != std::string::npos) has_hex = true;
                if (!value.empty()) value += " ";
                value += v;
            }
            i++;
        }
        // Type inference
        std::string type;
        if (has_quote) type = "&str";
        else if (has_dot) type = "f64";
        else if (has_hex) type = "u64";
        else {
            // Check if it's a numeric literal or a reference
            bool is_num = !value.empty() && (isdigit(value[0]) || value[0] == '-');
            type = is_num ? "u64" : "f64"; // references to other consts default to f64
            // If the value is purely a name reference, match its type
            if (!is_num && !value.empty() && isalpha(value[0])) {
                type = "/* inferred */";
            }
        }
        return "pub const " + name + ": " + type + " = " + value + ";\n";
    }

    // ---- CLASS ----
    std::string emit_class(const std::vector<Token>& tokens, size_t& i, int indent) {
        std::string out;
        i++; // skip CLASS
        std::string class_name = (i < tokens.size()) ? tokens[i].value : "Unknown";
        i++; // skip name

        // Skip inheritance: (Base, Other)
        if (i < tokens.size() && tokens[i].type == TokenType::LPAREN) {
            while (i < tokens.size() && tokens[i].type != TokenType::RPAREN) i++;
            if (i < tokens.size()) i++; // skip RPAREN
        }
        // Skip colon
        if (i < tokens.size() && tokens[i].type == TokenType::COLON) i++;

        out += "pub struct " + class_name + " {\n";

        // Collect fields from __init__ by scanning for self.xxx = 
        std::vector<std::string> fields;
        std::vector<std::string> methods;
        
        // Find the class body (between INDENT/DEDENT at class level)
        int class_indent = indent + 1;
        int cur = class_indent;

        // Scan ahead to find all self.X assignments and method defs
        size_t scan = i;
        // Skip NEWLINE/INDENT to enter class body
        while (scan < tokens.size() && (tokens[scan].type == TokenType::NEWLINE || tokens[scan].type == TokenType::INDENT)) {
            if (tokens[scan].type == TokenType::INDENT) cur++;
            scan++;
        }

        // Collect fields from __init__ and method signatures
        size_t body_start = scan;
        std::set<std::string> field_set;
        
        // First pass: find self.x = patterns for fields
        for (size_t s = body_start; s < tokens.size(); s++) {
            // Track indent level
            if (tokens[s].type == TokenType::INDENT) cur++;
            if (tokens[s].type == TokenType::DEDENT) {
                cur--;
                if (cur <= indent) break; // exited class
            }
            // self.field = ...
            if (tokens[s].type == TokenType::NAME && tokens[s].value == "self" &&
                s + 2 < tokens.size() && tokens[s+1].type == TokenType::DOT &&
                tokens[s+2].type == TokenType::NAME) {
                if (s + 3 < tokens.size() && tokens[s+3].type == TokenType::EQUAL) {
                    std::string field = tokens[s+2].value;
                    if (!field_set.count(field)) {
                        field_set.insert(field);
                        fields.push_back(field);
                    }
                }
            }
        }

        // Emit struct fields
        for (auto& f : fields) {
            out += "    pub " + f + ": String, // TODO: infer type\n";
        }
        out += "}\n\n";

        // Second pass: emit methods
        out += "impl " + class_name + " {\n";

        cur = class_indent;
        i = body_start;
        // Skip to enter class body properly
        while (i < tokens.size()) {
            if (tokens[i].type == TokenType::INDENT) { cur++; i++; continue; }
            if (tokens[i].type == TokenType::DEDENT) {
                cur--;
                i++;
                if (cur <= indent) break;
                continue;
            }
            if (tokens[i].type == TokenType::NEWLINE) { i++; continue; }

            // Method definition
            if (tokens[i].type == TokenType::DEF && cur == class_indent) {
                out += emit_function(tokens, i, cur, true);
                continue;
            }
            // Skip other class-level statements
            i++;
        }
        out += "}\n\n";
        return out;
    }

    // ---- FUNCTION / METHOD ----
    std::string emit_function(const std::vector<Token>& tokens, size_t& i, int indent, bool is_method) {
        std::string out;
        i++; // skip DEF
        std::string fn_name = (i < tokens.size()) ? tokens[i].value : "unknown";
        i++; // skip name

        // Parse params
        std::vector<std::string> params;
        if (i < tokens.size() && tokens[i].type == TokenType::LPAREN) {
            i++; // skip (
            while (i < tokens.size() && tokens[i].type != TokenType::RPAREN) {
                if (tokens[i].type == TokenType::NAME) {
                    std::string p = tokens[i].value;
                    if (p != "self" && p != "cls") {
                        params.push_back(p);
                    }
                }
                i++;
            }
            if (i < tokens.size()) i++; // skip )
        }

        // Skip colon and optional return type hint
        while (i < tokens.size() && tokens[i].type != TokenType::COLON && tokens[i].type != TokenType::NEWLINE) i++;
        if (i < tokens.size() && tokens[i].type == TokenType::COLON) i++;

        // Determine if __init__
        bool is_init = (fn_name == "__init__");
        if (is_init) fn_name = "new";

        // Build signature
        std::string indent_str(4, ' ');
        if (is_method && !is_init) {
            out += indent_str + "pub fn " + fn_name + "(&self";
        } else if (is_init) {
            out += indent_str + "pub fn " + fn_name + "(";
        } else {
            out += "pub fn " + fn_name + "(";
        }

        for (size_t p = 0; p < params.size(); p++) {
            if (is_method && !is_init && p == 0) out += ", ";
            else if (p > 0) out += ", ";
            else if (is_init && p == 0) { /* first param, no comma */ }
            out += params[p] + ": &str";
        }
        out += ")";
        if (is_init) out += " -> Self";
        out += " {\n";

        // Emit body: collect all tokens until we DEDENT back to the function's level
        int fn_indent = indent + 1;
        int cur = fn_indent;
        std::string body;
        bool in_body = false;

        // Skip initial NEWLINE/INDENT
        while (i < tokens.size() && (tokens[i].type == TokenType::NEWLINE || tokens[i].type == TokenType::INDENT)) {
            if (tokens[i].type == TokenType::INDENT) cur++;
            i++;
            in_body = true;
        }

        // Collect body lines
        std::string line;
        while (i < tokens.size()) {
            if (tokens[i].type == TokenType::INDENT) { cur++; i++; continue; }
            if (tokens[i].type == TokenType::DEDENT) {
                cur--;
                i++;
                if (cur <= indent) break;
                continue;
            }
            if (tokens[i].type == TokenType::NEWLINE) {
                if (!line.empty()) {
                    body += "        " + translate_statement(line) + "\n";
                    line.clear();
                }
                i++;
                continue;
            }
            // Build line from tokens
            if (!line.empty()) line += " ";
            if (tokens[i].type == TokenType::STRING) {
                line += "\"" + tokens[i].value + "\"";
            } else {
                line += tokens[i].value;
            }
            i++;
        }
        if (!line.empty()) {
            body += "        " + translate_statement(line) + "\n";
        }

        if (body.empty()) {
            body = "        // pass\n";
        }

        out += body;
        if (is_method || is_init) {
            out += indent_str + "}\n\n";
        } else {
            out += "}\n\n";
        }
        return out;
    }

    // ---- STATEMENT-LEVEL TRANSLATION ----
    std::string translate_statement(const std::string& line_src) {
        std::string line = line_src;

        // 1. GLOBAL OPERATOR & LITERAL MAPPING (Priority)
        // Note: Do NOT scrub colons here, as handlers need them to identify blocks
        replace_all(line, " && ", " and "); // Temporarily revert to standard for easy detection
        replace_all(line, " || ", " or "); 
        replace_all(line, " and ", " && ");
        replace_all(line, " or ", " || ");
        replace_all(line, " not ", " !");
        replace_all(line, "True", "true");
        replace_all(line, "False", "false");
        replace_all(line, "None", "None /* Option */");

        // 2. STATEMENT HANDLERS
        // print(...) -> println!(...)
        if (line.find("print") == 0 && line.find("(") != std::string::npos) {
            size_t start_paren = line.find("(");
            if (start_paren != std::string::npos && start_paren < line.length()) {
                std::string inner = safe_substr(line, start_paren);
                if (inner.length() >= 2) {
                    return "println!(" + safe_substr(inner, 1, inner.length()-2) + ");";
                }
            }
            return "println!();";
        }
        // return X -> return X;
        if (line.find("return") == 0) {
            if (line.length() <= 7) return "return;";
            std::string val = safe_substr(line, 6);
            if (val.empty() || val == " None /* Option */") return "return;";
            return "return " + val + ";";
        }
        // def -> pub fn
        if (line.find("def ") == 0) {
            if (line.length() <= 4) return "pub fn unknown() {";
            std::string sig_full = safe_substr(line, 4);
            size_t colon_pos = sig_full.find(":");
            
            if (colon_pos != std::string::npos && colon_pos < sig_full.length() - 1) {
                std::string sig = safe_substr(sig_full, 0, colon_pos);
                std::string action = safe_substr(sig_full, colon_pos + 1);
                replace_all(sig, "self ,", "&self,");
                replace_all(sig, "self)", "&self)");
                return "pub fn " + sig + " { " + translate_statement(action) + " }";
            }
            std::string sig = (colon_pos != std::string::npos) ? safe_substr(sig_full, 0, colon_pos) : sig_full;
            replace_all(sig, "self ,", "&self,");
            replace_all(sig, "self)", "&self)");
            return "pub fn " + sig + " {";
        }
        // if __name__ == "__main__" -> main()
        if (line.find("if __name__") != std::string::npos) {
            return "fn main() {";
        }
        // self.x = y -> self.x = y;
        if (line.find("self .") == 0 || line.find("self.") == 0) {
            return line + ";";
        }
        // pass -> // pass
        if (line == "pass") return "// pass";
        // raise -> panic!
        if (line.find("raise") == 0) {
            return "panic!(\"" + safe_substr(line, 6) + "\");";
        }
        // if / elif / else
        if (line.find("if ") == 0) {
            std::string rest = safe_substr(line, 3);
            size_t colon_pos = rest.find(" :");
            if (colon_pos == std::string::npos) colon_pos = rest.find(":");
            
            if (colon_pos != std::string::npos && colon_pos < rest.length() - 2) {
                // One-liner: if cond: action
                std::string cond = safe_substr(rest, 0, colon_pos);
                std::string action = safe_substr(rest, colon_pos + 1);
                return "if " + cond + " { " + translate_statement(action) + " }";
            }
            std::string cond = (colon_pos != std::string::npos) ? safe_substr(rest, 0, colon_pos) : rest;
            return "if " + cond + " {";
        }
        if (line.find("elif ") == 0) {
            std::string rest = safe_substr(line, 5);
            size_t colon_pos = rest.find(" :");
            if (colon_pos == std::string::npos) colon_pos = rest.find(":");
            std::string cond = (colon_pos != std::string::npos) ? safe_substr(rest, 0, colon_pos) : rest;
            return "} else if " + cond + " {";
        }
        if (line.find("else") == 0 && (line.find(":") != std::string::npos || line.length() < 6)) {
            return "} else {";
        }
        // for x in y: -> for x in y.iter() {
        if (line.find("for ") == 0) {
            std::string rest = safe_substr(line, 4);
            if (!rest.empty() && rest.back() == ':') rest.pop_back();
            size_t in_pos = rest.find(" in ");
            if (in_pos != std::string::npos) {
                std::string var = safe_substr(rest, 0, in_pos);
                std::string iter = safe_substr(rest, in_pos + 4);
                return "for " + var + " in " + iter + ".iter() {";
            }
        }
        // while
        if (line.find("while ") == 0) {
            std::string cond = safe_substr(line, 6);
            if (!cond.empty() && cond.back() == ':') cond.pop_back();
            if (cond == "true" || cond == "True") return "loop {";
            return "while " + cond + " {";
        }
        // with ... as ...:
        if (line.find("with ") == 0) {
            std::string rest = safe_substr(line, 5);
            if (!rest.empty() && rest.back() == ':') rest.pop_back();
            return "// with scope: " + rest + " {";
        }
        // break / continue
        if (line == "break") return "break;";
        if (line == "continue") return "continue;";
        
        // try: -> // try {
        if (line.find("try") == 0 && (line.find(":") != std::string::npos || line.length() < 5)) return "// try {";
        // except -> // } catch
        if (line.find("except") == 0) {
            size_t start_ex = 6;
            std::string rest = (line.length() > 6) ? safe_substr(line, start_ex) : "";
            if (!rest.empty() && rest.back() == ':') rest.pop_back();
            return "// } catch " + rest + " {";
        }
        // lambda -> |...| { ... }
        if (line.find("lambda ") != std::string::npos) {
            size_t l_pos = line.find("lambda ");
            size_t c_pos = line.find(":", l_pos);
            if (c_pos != std::string::npos) {
                std::string args = safe_substr(line, l_pos + 7, c_pos - (l_pos + 7));
                std::string expr = safe_substr(line, c_pos + 1);
                replace_all(line, "lambda " + args + ":" + expr, "|" + args + "| { " + expr + " }");
            }
        }
        // yield -> yield
        if (line.find("yield ") == 0) return "yield " + safe_substr(line, 6) + ";";

        if (line.find("finally") == 0) return "// } finally {";
        
        // Final Global Scrub for non-blocks
        if (!line.empty() && line.back() == ':') line.pop_back();

        // 3. EXPRESSION-LEVEL MAPPING (is, in, f-strings, list comps)
        // [x for x in y] -> vec![...]
        if (line.find("[") != std::string::npos && line.find(" for ") != std::string::npos && line.find(" in ") != std::string::npos) {
            replace_all(line, "[", "vec![");
            replace_all(line, " for ", ".iter().map(|");
            replace_all(line, " in ", "| ");
            if (!line.empty() && line.back() == ']') { line.pop_back(); line += ").collect()"; }
        }
        // f"..." -> format!("...")
        if (line.find("f \"") != std::string::npos || line.find("f\"") != std::string::npos) {
            replace_all(line, "f \"", "format!(\"");
            replace_all(line, "f\"", "format!(\"");
            line += ")";
        }
        // "is" -> "==" (approximate)
        replace_all(line, " is ", " == ");
        replace_all(line, " is not ", " != ");

        // Default: treat as expression statement
        return line + ";";
    }

    void replace_all(std::string& str, const std::string& from, const std::string& to) {
        size_t pos = 0;
        while ((pos = str.find(from, pos)) != std::string::npos) {
            str.replace(pos, from.length(), to);
            pos += to.length();
        }
    }

    std::string safe_substr(const std::string& s, size_t start, size_t count = std::string::npos) {
        if (start >= s.length()) return "";
        return s.substr(start, count);
    }

    void skip_to_newline(const std::vector<Token>& tokens, size_t& i) {
        while (i < tokens.size() && tokens[i].type != TokenType::NEWLINE) i++;
        if (i < tokens.size()) i++;
    }
};

// ============================================================
// MAIN: RECURSIVE DIRECTORY CRAWLER
// Walks C:\GENESIS\Genesis, transpiles every .py -> .rs
// Deposits into C:\GENESIS\Sovereign_Suite_RS preserving structure
// Original source is NEVER modified.
// ============================================================

#ifndef SOVEREIGN_ENGINE
int main(int argc, char* argv[]) {
    std::string source_root = SOURCE_ROOT;
    std::string output_root = OUTPUT_ROOT;

    // Allow override: SovereignTranspiler.exe [source_dir] [output_dir]
    if (argc >= 2) source_root = argv[1];
    if (argc >= 3) output_root = argv[2];

    auto start_time = std::chrono::high_resolution_clock::now();

    std::cout << "=============================================" << std::endl;
    std::cout << " SOVEREIGN TRANSPILER - FIRST PRINCIPLES" << std::endl;
    std::cout << " Source: " << source_root << std::endl;
    std::cout << " Output: " << output_root << std::endl;
    std::cout << "=============================================" << std::endl;

    // 1. Collect all .py files
    std::vector<fs::path> py_files;
    for (const auto& entry : fs::recursive_directory_iterator(source_root,
             fs::directory_options::skip_permission_denied)) {
        if (!entry.is_regular_file()) continue;
        
        std::string p = entry.path().string();
        
        // EXCLUSIONS: Skip build artifacts, git, and our own Sovereign substrate
        if (p.find("__pycache__") != std::string::npos) continue;
        if (p.find("\\.git\\") != std::string::npos) continue;
        if (p.find("\\.genesis\\") != std::string::npos) continue;
        if (p.find("\\.vs\\") != std::string::npos) continue;
        if (p.find("Sovereign_") != std::string::npos && p.find("Sovereign_Transpiler") == std::string::npos) {
             // Only allow transpiling the transpiler itself if it's meant to be self-hosting, 
             // but usually we skip it. For now, skip ALL Sovereign_ prefixed dirs.
             continue;
        }
        if (p.find(output_root) != std::string::npos) continue;

        if (entry.path().extension() == ".py") {
            py_files.push_back(entry.path());
        }
    }

    uint64_t total = py_files.size();
    std::cout << "[STRIKE] Mapped " << total << " Python nodes." << std::endl;

    if (total == 0) {
        std::cout << "[DONE] No Python files found." << std::endl;
        return 0;
    }

    // Ensure output root exists
    fs::create_directories(output_root);

    SovereignTranspiler transpiler;
    uint64_t struck = 0;
    uint64_t total_lines = 0;
    uint64_t errors = 0;

    for (const auto& py_path : py_files) {
        struck++;

        try {
            // Read source
            std::ifstream file(py_path);
            if (!file.is_open()) {
                errors++;
                continue;
            }
            std::string source((std::istreambuf_iterator<char>(file)),
                               std::istreambuf_iterator<char>());
            file.close();

            // Count lines
            uint64_t lines = std::count(source.begin(), source.end(), '\n') + 1;
            total_lines += lines;

            // Tokenize
            SovereignLexer lexer(source);
            std::vector<Token> tokens = lexer.tokenize();

            // Transpile
            std::string filename = py_path.filename().string();
            std::string rust_code = transpiler.transpile(tokens, filename);

            // Mirror directory structure
            fs::path relative = fs::relative(py_path, source_root);
            fs::path out_path = fs::path(output_root) / relative;
            out_path.replace_extension(".rs");
            fs::create_directories(out_path.parent_path());

            // Write
            std::ofstream out_file(out_path);
            out_file << rust_code;
            out_file.close();

            // Progress
            float pct = (float)struck / total * 100.0f;
            printf("\r[STRIKE] %.1f%% | %llu/%llu | %s -> %s",
                   pct, struck, total,
                   py_path.filename().string().c_str(),
                   out_path.filename().string().c_str());
            std::cout << std::flush;
        } catch (const std::exception& e) {
            errors++;
            std::cerr << "\n[ERROR] " << py_path.filename().string() << ": " << e.what() << std::endl;
        } catch (...) {
            errors++;
            std::cerr << "\n[ERROR] " << py_path.filename().string() << ": Unknown crash" << std::endl;
        }
    }

    auto end_time = std::chrono::high_resolution_clock::now();
    auto duration = std::chrono::duration_cast<std::chrono::milliseconds>(end_time - start_time);

    std::cout << std::endl;
    std::cout << "=============================================" << std::endl;
    std::cout << " STRIKE COMPLETE" << std::endl;
    std::cout << " Files Struck: " << struck << std::endl;
    std::cout << " Lines Processed: " << total_lines << std::endl;
    std::cout << " Errors: " << errors << std::endl;
    std::cout << " Time: " << duration.count() << " ms" << std::endl;
    std::cout << " Output: " << output_root << std::endl;
    std::cout << "=============================================" << std::endl;

    return 0;
}
#endif

