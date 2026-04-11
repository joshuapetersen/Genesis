//! ZHTP Network Node - Complete Internet Replacement System
//! 
//! This is the main orchestrator binary that coordinates all ZHTP packages
//! into a unified network node capable of:
//! 
//! - Complete ISP replacement through mesh networking
//! - Zero-knowledge privacy for all communications  
//! - Universal Basic Income through network participation
//! - Quantum-resistant cryptographic security
//! - Decentralized governance through DAO integration
//! - Web4 protocol stack
//! 
//! The ZHTP node can operate in two modes:
//! - Pure Mesh: Complete  using only mesh protocols
//! - Hybrid: Mesh networking with TCP/IP fallback for transition

use anyhow::Result;
use tracing::info;

// Import new orchestrator modules
use zhtp::{
    cli::run_cli,
    // api::{start_api_server, ApiConfig}, // TODO: Re-enable when API handlers are implemented
};

// Stack size: did_startup::handle_startup_wallet_flow is deeply nested (68KB of code).
// Post-quantum key generation (Dilithium/Kyber) during genesis overflows the OS-default
// main-thread stack (1 MB on Windows in PE header, 8 MB on Linux).
//
// Fix: spawn a dedicated std::thread with 64 MB stack and DRIVE the Tokio runtime
// from there. `block_on` drives the async future on the CALLING thread, so both
// the driver thread AND all tokio worker threads get the enlarged stack.
fn main() -> Result<()> {
    std::thread::Builder::new()
        .name("zhtp-main".to_string())
        .stack_size(64 * 1024 * 1024) // 64 MB — generous headroom for genesis init
        .spawn(|| {
            let runtime = tokio::runtime::Builder::new_multi_thread()
                .thread_stack_size(64 * 1024 * 1024) // Worker threads also get 64 MB
                .enable_all()
                .build()
                .expect("Failed to build Tokio runtime");
            runtime.block_on(async_main())
        })
        .expect("Failed to spawn zhtp-main thread")
        .join()
        .expect("zhtp-main thread panicked")
}

async fn async_main() -> Result<()> {
    // Install rustls crypto provider before any TLS/QUIC operations
    // This MUST be done before any rustls usage to avoid panic
    let _ = rustls::crypto::ring::default_provider().install_default();

    // Initialize logging system with INFO level by default.
    // Add explicit filter directives to silence noisy third-party targets
    // (mdns_sd) and to suppress firewall rule WARNs from network_isolation.
    let mut filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));

    // Silence mdns-sd internal noisy messages (these are harmless channel-close warnings)
    filter = filter.add_directive("mdns_sd=error".parse().expect("invalid directive"));
    filter = filter.add_directive("mdns_sd::service_daemon=error".parse().expect("invalid directive"));

    // Suppress firewall-rule failure warnings coming from network isolation on Windows
    filter = filter.add_directive("zhtp::config::network_isolation=error".parse().expect("invalid directive"));

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .init();

    info!(" ZHTP Orchestrator v{}", env!("CARGO_PKG_VERSION"));
    info!("Level 1 Orchestrator - Coordinates protocols, blockchain, network");

    // Check if this is a special server mode
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() > 1 && args[1] == "--server" {
        info!("ZHTP Orchestrator Server mode not yet implemented");
        info!("Falling back to CLI mode");
        run_cli().await?;
    } else {
        // Default: Use the full CLI structure with all subcommands
        info!("Starting ZHTP Orchestrator CLI");
        run_cli().await?;
    }

    info!(" ZHTP Orchestrator shutdown complete");
    Ok(())
}


// Note: The legacy run_node function and local modules have been replaced
// with the new zhtp::cli and zhtp::api architecture.
//
// The new architecture properly implements:
// Level 1: zhtp (orchestrator) - coordinates Level 2 components
// Level 2: (protocols, blockchain, network) - manage Level 3 components  
// Level 3: (consensus, storage, economy) - utilize Level 4 utilities
// Level 4: (proofs, identity, crypto) - core utilities
//
// To actually run Level 2 components with implementations:
// 1. Start lib-protocols server on port 8001
// 2. Start lib-blockchain server on port 8002  
// 3. Start lib-network server on port 8003
// 4. ZHTP orchestrator coordinates them via HTTP calls
