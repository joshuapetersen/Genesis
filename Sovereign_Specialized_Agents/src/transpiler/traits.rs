use crate::transpiler::uir::UirNode;

/// Frontend — Ingests source code and extracts UIR nodes.
pub trait SovereignFrontend {
    fn ingest(&mut self, source: &str) -> Vec<UirNode>;
}

/// Backend — Forges source code from UIR nodes.
pub trait SovereignBackend {
    fn forge(&mut self, ir: Vec<UirNode>) -> String;
}
