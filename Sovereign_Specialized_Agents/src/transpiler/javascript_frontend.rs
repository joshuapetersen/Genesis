use crate::transpiler::uir::{UirNode, UirNodeKind};
use crate::transpiler::traits::SovereignFrontend;
use tree_sitter::{Parser, Language, Node};

pub struct JavascriptFrontend {
    parser: Parser,
}

impl JavascriptFrontend {
    pub fn new(lang: Language) -> Self {
        let mut parser = Parser::new();
        parser.set_language(lang).expect("Error loading JS grammar");
        Self { parser }
    }

    fn extract_string(&self, node: Node, source: &str) -> Option<String> {
        let mut result = String::new();
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            match child.kind() {
                "string_fragment" => { result.push_str(&source[child.byte_range()]); },
                "escape_sequence" => {
                    let escape = &source[child.byte_range()];
                    match escape {
                        "\\\\" => result.push('\\'),
                        "\\\"" => result.push('"'),
                        _ => result.push_str(escape),
                    }
                },
                _ => {}
            }
        }
        if result.is_empty() { None } else { Some(result) }
    }

    fn traverse(&self, node: Node, source: &str, ir: &mut Vec<UirNode>) {
        let kind = node.kind();
        match kind {
            "call_expression" => {
                if let Some(function_node) = node.child_by_field_name("function") {
                    let name = &source[function_node.byte_range()];
                    if name.contains("registerCommand") {
                        if let Some(args_node) = node.child_by_field_name("arguments") {
                            let mut cursor = args_node.walk();
                            for arg in args_node.children(&mut cursor) {
                                if arg.kind() == "string" {
                                    if let Some(cmd) = self.extract_string(arg, source) {
                                        ir.push(UirNode::new(UirNodeKind::SysCall {
                                            name: "RegisterCommand".to_string(),
                                            args: vec![cmd],
                                            resonance: 1.09277703703703,
                                        }));
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "variable_declarator" => {
                if let Some(name_node) = node.child_by_field_name("name") {
                    let name = &source[name_node.byte_range()];
                    if name == "bridgePath" {
                        if let Some(val_node) = node.child_by_field_name("value") {
                            if let Some(path) = self.extract_string(val_node, source) {
                                ir.push(UirNode::new(UirNodeKind::Bridge {
                                    target: path,
                                    protocol: "MMAP".to_string(),
                                    state_lock: true,
                                }));
                            }
                        }
                    }
                }
            },
            _ => {}
        }
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.traverse(child, source, ir);
        }
    }
}

impl SovereignFrontend for JavascriptFrontend {
    fn ingest(&mut self, source: &str) -> Vec<UirNode> {
        let tree = self.parser.parse(source, None).unwrap();
        let mut ir = Vec::new();
        self.traverse(tree.root_node(), source, &mut ir);
        ir
    }
}
