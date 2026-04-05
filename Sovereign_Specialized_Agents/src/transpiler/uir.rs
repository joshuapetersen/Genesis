use serde::{Serialize, Deserialize};

/// UIR (Universal Intermediate Representation)
/// Precision: 1.09277703703703 Hz
/// Logic-Agnostic DNA for the Bidirectional Foundry

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UirNodeKind {
    /// System Invocation (e.g., Command Registration, SysCalls)
    SysCall { 
        name: String, 
        args: Vec<String>,
        resonance: f64 
    },
    /// Memory or IO Bridge (e.g., MMAP, FS, Sockets)
    Bridge { 
        target: String, 
        protocol: String,
        state_lock: bool 
    },
    /// Data State (e.g., Variable Declarations, Assignments)
    DataState { 
        name: String, 
        value: String,
        is_const: bool 
    },
    /// Logical Flow (e.g., Loops, Conditionals)
    LogicFlow { 
        pattern: String, 
        condition: Option<String> 
    },
    /// Agent Identity & Security
    Identity { 
        id: String, 
        signature: String 
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UirNode {
    pub kind: UirNodeKind,
    pub resonance: f64,
    pub children: Vec<UirNode>,
}

impl UirNode {
    pub fn new(kind: UirNodeKind) -> Self {
        Self {
            kind,
            resonance: 1.09277703703703,
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: UirNode) {
        self.children.push(child);
    }
}
