const { Menu, shell } = require('electron');

function buildMenu(mainWindow) {
    const template = [
        {
            label: 'File',
            submenu: [
                { label: 'New Workspace', click: () => mainWindow.webContents.send('menu-task', 'new-workspace') },
                { label: 'Open Hive...', click: () => mainWindow.webContents.send('menu-task', 'open-hive') },
                { type: 'separator' },
                { label: 'Save State', accelerator: 'CmdOrCtrl+S', click: () => mainWindow.webContents.send('menu-task', 'save-state') },
                { role: 'quit' }
            ]
        },
        {
            label: 'Edit',
            submenu: [
                { role: 'undo' }, { role: 'redo' }, { type: 'separator' },
                { role: 'cut' }, { role: 'copy' }, { role: 'paste' }
            ]
        },
        {
            label: 'Run',
            submenu: [
                { label: 'Ignite Hive (All Crates)', accelerator: 'F5', click: () => mainWindow.webContents.send('run-task', 'ignite-all') },
                { label: 'Run Benchmark', click: () => mainWindow.webContents.send('run-task', 'benchmark') },
                { label: 'ASH-Swarm Audit', click: () => mainWindow.webContents.send('run-task', 'audit') }
            ]
        },
        {
            label: 'Terminal',
            submenu: [
                { label: 'New Terminal', click: () => mainWindow.webContents.send('terminal-task', 'new') },
                { label: 'Flush KV-Cache', click: () => mainWindow.webContents.send('terminal-task', 'flush') }
            ]
        },
        {
            label: 'Help',
            submenu: [
                { label: 'Sovereign Documentation', click: () => shell.openExternal('https://github.com/joshuapetersen/Genesis') }
            ]
        }
    ];

    const menu = Menu.buildFromTemplate(template);
    Menu.setApplicationMenu(menu);
}

module.exports = { buildMenu };
