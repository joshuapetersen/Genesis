use sovereign_constants::*;
use sovereign_math::{SovereignMath, VolumetricContext};
use tree_sitter::{Parser, Language, Node};
use anyhow::{Result, anyhow};
use std::fs;

/// [DISSECTOR_0x0D]: UNIVERSAL VOLUMETRIC AUDITOR
/// Analyzes AST architecture across multiple languages (Rust, C, C++, etc.).
/// Maps logic nodes into the 15,330³ manifold to detect "Insecure Resonance."
pub struct SovereignDissector {
    pub math: SovereignMath,
    pub parser: Parser,
}

impl SovereignDissector {
    pub fn new(lang_id: &str) -> Result<Self> {
        let mut parser = Parser::new();
        let lang = match lang_id {
            "rust" => tree_sitter_rust::language(),
            "c" => tree_sitter_c::language(),
            "cpp" => tree_sitter_cpp::language(),
            "python" => tree_sitter_python::language(),
            _ => return Err(anyhow!("Language '{}' !supported for First-Principles Auditing", lang_id)),
        };
        
        parser.set_language(lang)?;
        Ok(Self {
            math: SovereignMath::new(),
            parser,
        })
    }

    /// [AUDIT_FILE]: Executes a Volumetric Audit on a file substrate.
    pub fn audit_file(&mut self, path: &std::path::Path) -> Result<Vec<VulnerabilitySignal>> {
        let source = fs::read_to_string(path)?;
        let tree = self.parser.parse(&source, None)
            .ok_or_else(|| anyhow!("Failed to parse file: {:?}", path))?;

        let mut signals = Vec::new();
        self.traverse_and_audit(tree.root_node(), &source, &mut signals);
        Ok(signals)
    }

    /// [RESONANCE_SCAN]: Traverses the AST and maps nodes to the 15,330³ lattice.
    fn traverse_and_audit(&self, node: Node, source: &str, signals: &mut Vec<VulnerabilitySignal>) {
        let kind = node.kind();
        let text = &source[node.start_byte()..node.end_byte()];

        // 1. Project node into Volumetric Manifold
        let ctx = self.math.expand(text);
        
        // 2. Correlation Check: Identify "Desync" signals
        // Example: Identifying "unsafe" blocks or dangerous function calls
        if kind == "unsafe_block" || kind == "call_expression" && (text.contains("malloc") || text.contains("memcpy")) {
            let density = self.math.refract(&ctx);
            signals.push(VulnerabilitySignal {
                location: format!("{}:{}", node.start_position().row, node.start_position().column),
                severity: "CRITICAL".to_string(),
                resonance: density,
                description: format!("High-Entropy Logic Detected: {}", kind),
                node_text: text.to_string(),
            });
        }

        // Recursive descent into the lattice
        for i in 0..node.child_count() {
            if let Some(child) = node.child(i) {
                self.traverse_and_audit(child, source, signals);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct VulnerabilitySignal {
    pub location: String,
    pub severity: String,
    pub resonance: f64,
    pub description: String,
    pub node_text: String,
}
