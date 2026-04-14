/**
 * UI Component for Sovereign Function: getppid
 */
console.log("[UI] getppid Function Component Loaded.");

function activate_getppid() {
    orchestrator.logMessage('SYSTEM', 'Activating Executive Function: getppid...');
    orchestrator.sendIntent('execute getppid');
}
