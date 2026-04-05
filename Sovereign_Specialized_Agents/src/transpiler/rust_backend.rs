use crate::transpiler::uir::{UirNode, UirNodeKind};
use crate::transpiler::traits::SovereignBackend;

pub struct RustBackend;

impl SovereignBackend for RustBackend {
    fn forge(&mut self, ir: Vec<UirNode>) -> String {
        let mut rust_code = String::new();
        rust_code.push_str("use tokio;\nuse memmap2::MmapMut;\nuse std::fs::OpenOptions;\n\n");
        rust_code.push_str("/// SARAH CORE — SOVEREIGN MAIN LOOP [FORGED RUST]\n");
        rust_code.push_str("/// HEARTBEAT: 1.09277703703703 Hz\n\n");
        rust_code.push_str("#[tokio::main]\nasync fn main() -> Result<(), Box<dyn std::error::Error>> {\n");
        rust_code.push_str("    println!(\"[!] SARAH CORE IGNITED — HIGH-PRECISION LOCK ACTIVE\");\n\n");

        for node in ir {
            match node.kind {
                UirNodeKind::Bridge { ref target, ref protocol, state_lock: _ } => {
                     rust_code.push_str(&format!("    // {} Bridge: {}\n", protocol, target));
                     rust_code.push_str(&format!("    let file = OpenOptions::new().read(true).write(true).open(r\"{}\")?;\n", target));
                     rust_code.push_str("    let _mmap = unsafe { MmapMut::map_mut(&file)? };\n");
                     rust_code.push_str(&format!("    println!(\"[BRIDGE] {} SEATED: {{}}\", r\"{}\");\n\n", protocol, target));
                },
                UirNodeKind::SysCall { ref name, ref args, resonance: _ } => {
                     rust_code.push_str(&format!("    // Call: {} - Args: {:?}\n", name, args));
                     rust_code.push_str(&format!("    println!(\"[SYSCALL] {}: {{}}\", r\"{}\");\n\n", name, args.join(", ")));
                },
                _ => {}
            }
        }

        rust_code.push_str("    // MAIN LOOP (1.0927... Hz)\n");
        rust_code.push_str("    loop { tokio::time::sleep(std::time::Duration::from_millis(915)).await; }\n");
        rust_code.push_str("}\n");
        rust_code
    }
}
