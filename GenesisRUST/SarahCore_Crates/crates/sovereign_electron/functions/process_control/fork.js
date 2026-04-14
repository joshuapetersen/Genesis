/**
 * UI Component for Sovereign Function: fork
 */
console.log("[UI] fork Function Component Loaded.");

function activate_fork() {
    orchestrator.logMessage('SYSTEM', 'Activating Executive Function: fork...');
    orchestrator.sendIntent('execute fork');
}
