use crate::transpiler::uir::{UirNode, UirNodeKind};
use crate::transpiler::traits::SovereignBackend;

pub struct JavascriptBackend;

impl SovereignBackend for JavascriptBackend {
    fn forge(&mut self, ir: Vec<UirNode>) -> String {
        let mut js_code = String::new();
        js_code.push_str("const fs = require('fs');\nconst vscode = require('vscode');\n\n");
        js_code.push_str("/// SARAH CORE — SOVEREIGN MAIN LOOP [REFORGED JS]\n");
        js_code.push_str("/// HEARTBEAT: 1.09277703703703 Hz\n\n");
        js_code.push_str("function activate(context) {\n");
        js_code.push_str("    console.log('[!] SARAH CORE IGNITED — HIGH-PRECISION LOCK ACTIVE');\n\n");

        for node in ir {
            match node.kind {
                UirNodeKind::Bridge { ref target, .. } => {
                     js_code.push_str(&format!("    const bridgePath = '{}';\n", target));
                     js_code.push_str(&format!("    const bridge = fs.openSync(bridgePath, 'r+');\n"));
                     js_code.push_str("    console.log('[BRIDGE] SEATED:', bridgePath);\n\n");
                },
                UirNodeKind::SysCall { ref name, ref args, resonance: _ } => {
                     js_code.push_str(&format!("    // Call: {}\n", name));
                     if name == "RegisterCommand" {
                         for arg in args {
                             js_code.push_str(&format!("    vscode.commands.registerCommand('{}', () => {{}});\n", arg));
                         }
                     }
                     js_code.push_str("\n");
                },
                _ => {}
            }
        }

        js_code.push_str("    // MAIN LOOP (1.0927... Hz)\n");
        js_code.push_str("    setInterval(() => { /* logic */ }, 915);\n");
        js_code.push_str("}\n\nmodule.exports = { activate };\n");
        js_code
    }
}
