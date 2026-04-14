/**
 * UI Component for Sovereign Function: alarm
 */
console.log("[UI] alarm Function Component Loaded.");

function activate_alarm() {
    orchestrator.logMessage('SYSTEM', 'Activating Executive Function: alarm...');
    orchestrator.sendIntent('execute alarm');
}
