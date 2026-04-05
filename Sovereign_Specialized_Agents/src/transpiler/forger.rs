use crate::transpiler::{Intent, SarahNode};

pub struct RustForger;

impl RustForger {
    pub fn forge(ir: Vec<SarahNode>) -> String {
        let mut rust_code = String::new();

        rust_code.push_str("use tokio;\nuse memmap2::MmapMut;\nuse std::fs::OpenOptions;\n\n");
        rust_code.push_str("/// SARAH CORE — SOVEREIGN MAIN LOOP [FORGED RUST]\n");
        rust_code.push_str("/// HEARTBEAT: 1.09277703703703 Hz\n\n");
        rust_code.push_str("#[tokio::main]\nasync fn main() -> Result<(), Box<dyn std::error::Error>> {\n");
        rust_code.push_str("    println!(\"[!] SARAH CORE IGNITED — HIGH-PRECISION LOCK ACTIVE\");\n\n");

        for node in ir {
            match node.intent {
                Intent::MemoryBridge { ref path, state_lock: _ } => {
                     rust_code.push_str(&format!("    // Memory Bridge: {}\n", path));
                     rust_code.push_str("    let file = OpenOptions::new().read(true).write(true).open(r\"");
                     rust_code.push_str(path);
                     rust_code.push_str("\")?;\n");
                     rust_code.push_str("    let mut mmap = unsafe { MmapMut::map_mut(&file)? };\n");
                     rust_code.push_str(&format!("    println!(\"[BRIDGE] MMAP SEATED: {{}}\", r\"{}\");\n\n", path));
                },
                Intent::CommandBind { ref command, callback_id: _ } => {
                     rust_code.push_str(&format!("    // Command: {}\n", command));
                     rust_code.push_str(&format!("    println!(\"[COMMAND] Registered: {{}}\", r\"{}\");\n\n", command));
                },
                _ => {}
            }
        }

        rust_code.push_str("    // MAIN LOOP (1.0927... Hz)\n");
        rust_code.push_str("    loop {\n");
        rust_code.push_str("        // Process Sarah logic at target resonance\n");
        rust_code.push_str("        tokio::time::sleep(std::time::Duration::from_millis(915)).await;\n"); // ~1.0927 Hz
        rust_code.push_str("    }\n");
        rust_code.push_str("}\n");

        rust_code
    }
}
