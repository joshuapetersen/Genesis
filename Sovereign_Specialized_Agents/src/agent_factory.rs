use std::process::{Command, Child, Stdio};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use anyhow::Result;

pub struct SovereignAgentFactory {
    active_agents: Arc<Mutex<HashMap<u32, Child>>>,
}

impl SovereignAgentFactory {
    pub fn new() -> Self {
        Self {
            active_agents: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// SPAWN A STANDALONE BRAIN (Skill #222)
    /// Each brain is its own OS process and its own agent factory.
    pub fn spawn_brain(&self, brain_name: &str) -> Result<u32> {
        println!("[ FACTORY ] Spawning Sovereign Brain: {}", brain_name);
        
        let exe_dir = std::env::current_exe()?.parent().unwrap().to_path_buf();
        let brain_path = exe_dir.join(format!("{}.exe", brain_name));
        
        let child = Command::new(brain_path)
            .stdin(Stdio::null())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .expect(&format!("Failed to spawn brain: {}", brain_name));
        
        let pid = child.id();
        self.active_agents.lock().unwrap().insert(pid, child);
        
        println!("[ FACTORY ] Brain Online | PID: {}", pid);
        Ok(pid)
    }

    /// DESTROY A STANDALONE BRAIN (Skill #222)
    pub fn destroy_brain(&self, pid: u32) -> Result<()> {
        println!("[ FACTORY ] Destroying Brain Target | PID: {}", pid);
        
        let mut agents = self.active_agents.lock().unwrap();
        if let Some(mut child) = agents.remove(&pid) {
            child.kill()?;
            println!("[ FACTORY ] Brain Neutralized | PID: {}", pid);
        } else {
            println!("[ WARNING ] Target PID: {} not found in local hive index.", pid);
        }
        
        Ok(())
    }

    pub fn list_active_pids(&self) -> Vec<u32> {
        self.active_agents.lock().unwrap().keys().cloned().collect()
    }
}

unsafe impl Send for SovereignAgentFactory {}
unsafe impl Sync for SovereignAgentFactory {}
