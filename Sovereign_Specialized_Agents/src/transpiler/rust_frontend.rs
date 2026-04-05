use crate::transpiler::uir::{UirNode, UirNodeKind};
use crate::transpiler::traits::SovereignFrontend;
use regex::Regex;

pub struct RustFrontend;

impl SovereignFrontend for RustFrontend {
    fn ingest(&mut self, source: &str) -> Vec<UirNode> {
        let mut ir = Vec::new();
        
        // Match Bridge Calls
        let re_bridge = Regex::new(r###"// ([^ ]+) Bridge: ([^\n]+)\n\s+let file = OpenOptions::new\(\)\.read\(true\)\.write\(true\)\.open\(r"([^\"]+)"\)\?;"###).unwrap();
        for cap in re_bridge.captures_iter(source) {
            ir.push(UirNode::new(UirNodeKind::Bridge {
                target: cap[3].to_string(),
                protocol: cap[1].to_string(),
                state_lock: true,
            }));
        }

        // Match SysCall Calls
        let re_syscall = Regex::new(r###"// Call: ([^ ]+) - Args: \["([^"]+)"\]"###).unwrap();
        for cap in re_syscall.captures_iter(source) {
            ir.push(UirNode::new(UirNodeKind::SysCall {
                name: cap[1].to_string(),
                args: vec![cap[2].to_string()],
                resonance: 1.09277703703703,
            }));
        }

        ir
    }
}
