use anyhow::Result;
use async_trait::async_trait;
use crate::factory_core::SovereignFactory;

/// THE META-FACTORY TRAIT
/// V-41.0 META-STRIKE
#[async_trait]
pub trait MetaFactory {
    /// Meta-domain (e.g., "Research", "Development")
    fn meta_domain(&self) -> &str;

    /// Node ID (e.g., "Alpha", "Beta")
    fn node_id(&self) -> &str;

    /// Orchestrate a group of agent factories
    async fn orchestrate(&self, task_id: &str) -> Result<()>;

    /// Meta-Reflection: Analyzing the results from Alpha vs Beta branches
    async fn reflect_branches(&self) -> Result<()>;
}

pub struct MetaOrcestrator {
    pub factories: Vec<Box<dyn SovereignFactory + Send + Sync>>,
}
