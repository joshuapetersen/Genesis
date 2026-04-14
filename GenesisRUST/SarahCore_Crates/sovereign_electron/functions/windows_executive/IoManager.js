/**
 * UI Component for Sovereign Function: IoManager
 */
console.log("[UI] IoManager Function Component Loaded.");

function activate_IoManager() {
    orchestrator.logMessage('SYSTEM', 'Activating Executive Function: IoManager...');
    orchestrator.sendIntent('execute IoManager');
}
