pub mod logic_bus;
pub mod meta_bus;
pub mod helix;

use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait HiveSoul {
    /// Synchronize state with the Symbiotic Logic Bus
    async fn synchronize(&self) -> Result<()>;

    /// Collective reasoning strike
    async fn collective_think(&self) -> Result<()>;
}
