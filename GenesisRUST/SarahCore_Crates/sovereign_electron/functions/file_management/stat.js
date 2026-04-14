/**
 * UI Component for Sovereign Function: stat
 */
console.log("[UI] stat Function Component Loaded.");

function activate_stat() {
    orchestrator.logMessage('SYSTEM', 'Activating Executive Function: stat...');
    orchestrator.sendIntent('execute stat');
}
