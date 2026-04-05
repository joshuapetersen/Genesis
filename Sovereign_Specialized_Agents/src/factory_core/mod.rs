use anyhow::Result;
use async_trait::async_trait;

pub mod meta;

/// THE SOVEREIGN FACTORY TRAIT
/// V-39.0 SYMBIO-FORGE
#[async_trait]
pub trait SovereignFactory {
    /// Domain-specific name (e.g., "Internet", "Hacking")
    fn domain(&self) -> &str;

    /// Spawn a specialized sub-agent for a task
    async fn spawn_agent(&self, task_id: &str) -> Result<u32>;

    /// Decommission an agent after task completion
    async fn destroy_agent(&self, pid: u32) -> Result<()>;

    /// Recursive Self-Evolution Strike
    async fn evolve_factory(&self) -> Result<()>;
}
