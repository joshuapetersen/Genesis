@echo off
timeout /t 2 /nobreak > nul
cargo run --bin sovereign_orchestrator
exit