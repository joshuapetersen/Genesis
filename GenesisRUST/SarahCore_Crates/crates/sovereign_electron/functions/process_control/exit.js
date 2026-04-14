/**
 * UI Component for Sovereign Function: exit
 */
console.log("[UI] exit Function Component Loaded.");

function activate_exit() {
    orchestrator.logMessage('SYSTEM', 'Activating Executive Function: exit...');
    orchestrator.sendIntent('execute exit');
}
