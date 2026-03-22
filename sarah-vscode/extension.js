const vscode = require('vscode');
const http = require('http');
const https = require('https');
const fs = require('fs');


/**
 * SARAH SOVEREIGN VS CODE EXTENSION
 * Connects to the Sovereign Gateway (localhost:8001)
 * Routes through: 1T Genlex Cortex → 8B GPU → HTTP fallback
 */

let chatViewProvider;

function activate(context) {
    console.log('[Sarah] Sovereign Extension Activating...');

    // Register the Chat Sidebar
    chatViewProvider = new SarahChatViewProvider(context.extensionUri);
    context.subscriptions.push(
        vscode.window.registerWebviewViewProvider('sarah.chatView', chatViewProvider)
    );

    // Command: Ask Sarah (opens input box)
    context.subscriptions.push(
        vscode.commands.registerCommand('sarah.askSarah', async () => {
            const input = await vscode.window.showInputBox({
                prompt: 'Ask Sarah anything...',
                placeHolder: 'e.g. Write a function to sort an array'
            });
            if (input && chatViewProvider) {
                chatViewProvider.sendMessage(input);
            }
        })
    );

    // Command: Explain Selected Code
    context.subscriptions.push(
        vscode.commands.registerCommand('sarah.explainCode', async () => {
            const editor = vscode.window.activeTextEditor;
            if (!editor) return;
            const selection = editor.document.getText(editor.selection);
            if (selection && chatViewProvider) {
                chatViewProvider.sendMessage(`Explain this code:\n\`\`\`\n${selection}\n\`\`\``);
            }
        })
    );

    // Command: Fix Selected Code
    context.subscriptions.push(
        vscode.commands.registerCommand('sarah.fixCode', async () => {
            const editor = vscode.window.activeTextEditor;
            if (!editor) return;
            const selection = editor.document.getText(editor.selection);
            if (selection && chatViewProvider) {
                chatViewProvider.sendMessage(`Fix this code and explain what was wrong:\n\`\`\`\n${selection}\n\`\`\``);
            }
        })
    );

    // Command: Generate Code
    context.subscriptions.push(
        vscode.commands.registerCommand('sarah.generateCode', async () => {
            const input = await vscode.window.showInputBox({
                prompt: 'Describe the code you want Sarah to generate...',
                placeHolder: 'e.g. A REST API endpoint for user authentication'
            });
            if (input && chatViewProvider) {
                chatViewProvider.sendMessage(`Generate code: ${input}`);
            }
        })
    );

    // Command: Start Gateway
    context.subscriptions.push(
        vscode.commands.registerCommand('sarah.startGateway', () => {
            const terminal = vscode.window.createTerminal('Sarah Gateway');
            terminal.sendText('C:\\SarahCore\\start_gateway.bat');
            terminal.show();
        })
    );

    // Status bar item
    const statusBar = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Right, 100);
    statusBar.text = '$(heart) Sarah';
    statusBar.tooltip = 'Sarah Sovereign AI - Click to ask';
    statusBar.command = 'sarah.askSarah';
    statusBar.show();
    context.subscriptions.push(statusBar);

    // Check gateway connection on startup
    checkGateway(statusBar);
}

async function checkGateway(statusBar) {
    const config = vscode.workspace.getConfiguration('sarah');
    const url = config.get('gatewayUrl', 'http://localhost:8001');
    
    try {
        const response = await httpGet(`${url}/api/status`);
        const data = JSON.parse(response);
        if (data.status === 'ACTIVE') {
            statusBar.text = '$(heart-filled) Sarah';
            statusBar.tooltip = `Sarah ACTIVE | Resonance: ${data.resonance_anchor}`;
        }
    } catch {
        statusBar.text = '$(heart) Sarah (Offline)';
        statusBar.tooltip = 'Sarah Gateway offline. Run: Sarah: Start Sovereign Gateway';
    }
}

function httpGet(url) {
    return new Promise((resolve, reject) => {
        const client = url.startsWith('https') ? https : http;
        client.get(url, { timeout: 3000 }, (res) => {
            let data = '';
            res.on('data', chunk => data += chunk);
            res.on('end', () => resolve(data));
        }).on('error', reject);
    });
}

function httpPost(url, body, headers = {}) {
    return new Promise((resolve, reject) => {
        const parsed = new URL(url);
        const client = parsed.protocol === 'https:' ? https : http;
        const postData = JSON.stringify(body);
        
        const options = {
            hostname: parsed.hostname,
            port: parsed.port,
            path: parsed.pathname,
            method: 'POST',
            timeout: 60000,
            headers: {
                'Content-Type': 'application/json',
                'Content-Length': Buffer.byteLength(postData),
                ...headers
            }
        };

        const req = client.request(options, (res) => {
            let data = '';
            res.on('data', chunk => data += chunk);
            res.on('end', () => resolve(data));
        });

        req.on('error', reject);
        req.write(postData);
        req.end();
    });
}

function streamPost(url, body, onToken, onDone, onError) {
    const parsed = new URL(url);
    const client = parsed.protocol === 'https:' ? https : http;
    const postData = JSON.stringify(body);

    const options = {
        hostname: parsed.hostname,
        port: parsed.port,
        path: parsed.pathname,
        method: 'POST',
        timeout: 120000,
        headers: {
            'Content-Type': 'application/json',
            'Content-Length': Buffer.byteLength(postData)
        }
    };

    const req = client.request(options, (res) => {
        let buffer = '';
        res.on('data', (chunk) => {
            buffer += chunk.toString();
            const lines = buffer.split('\n');
            buffer = lines.pop();
            
            for (const line of lines) {
                const trimmed = line.trim();
                if (!trimmed || !trimmed.startsWith('data: ')) continue;
                const payload = trimmed.slice(6);
                if (payload === '[DONE]') {
                    onDone();
                    return;
                }
                try {
                    const parsed = JSON.parse(payload);
                    if (parsed.token) {
                        onToken(parsed.token);
                    } else if (parsed.choices && parsed.choices[0]?.delta?.content) {
                        onToken(parsed.choices[0].delta.content);
                    }
                } catch {}
            }
        });
        res.on('end', onDone);
    });

    req.on('error', onError);
    req.write(postData);
    req.end();
}

class SarahChatViewProvider {
    constructor(extensionUri) {
        this._extensionUri = extensionUri;
        this._view = null;
    }

    resolveWebviewView(webviewView) {
        this._view = webviewView;
        webviewView.webview.options = {
            enableScripts: true
        };
        webviewView.webview.html = this._getHtml();

        // Handle messages from the webview
        webviewView.webview.onDidReceiveMessage(async (message) => {
            if (message.type === 'sendMessage') {
                await this._handleUserMessage(message.text);
            }
        });
    }

    sendMessage(text) {
        if (this._view) {
            this._view.webview.postMessage({ type: 'addUserMessage', text });
            this._handleUserMessage(text);
        }
    }

    async _handleUserMessage(text) {
        // Show typing indicator
        if (this._view) {
            this._view.webview.postMessage({ type: 'startResponse' });
        }

        const bridgePath = 'C:\\Genlex_Linear\\lattice_bridge.bin';
        let fd;
        try {
            fd = fs.openSync(bridgePath, 'r+');
        } catch (e) {
            if (this._view) {
                this._view.webview.postMessage({
                    type: 'errorResponse',
                    text: `MMAP Bridge offline. Run 'python C:\\SarahCore\\mmap_kernel.py' to seat the Sovereign Connection.`
                });
            }
            return;
        }

        try {
            // Write prompt
            const promptBuffer = Buffer.from(text, 'utf8');
            const headerBuffer = Buffer.alloc(5);
            headerBuffer.writeUInt8(1, 0); // State 1: VSCODE_WROTE_PROMPT
            headerBuffer.writeUInt32LE(promptBuffer.length, 1);
            
            fs.writeSync(fd, headerBuffer, 0, 5, 0);
            fs.writeSync(fd, promptBuffer, 0, promptBuffer.length, 5);
            
            // Polling loop
            const STATE_TOKEN = 2;
            const STATE_DONE = 3;
            const STATE_ACK = 4;
            
            let spinCount = 0;
            const spin = setInterval(() => {
                const stateBuf = Buffer.alloc(5);
                fs.readSync(fd, stateBuf, 0, 5, 0);
                const state = stateBuf.readUInt8(0);
                
                if (state === STATE_TOKEN) {
                    const len = stateBuf.readUInt32LE(1);
                    const tokenBuf = Buffer.alloc(len);
                    fs.readSync(fd, tokenBuf, 0, len, 5);
                    const token = tokenBuf.toString('utf8');
                    
                    if (this._view) {
                        this._view.webview.postMessage({ type: 'streamToken', token });
                    }
                    
                    // ACK
                    const ackBuf = Buffer.alloc(1);
                    ackBuf.writeUInt8(STATE_ACK, 0);
                    fs.writeSync(fd, ackBuf, 0, 1, 0);
                    spinCount = 0; // reset timeout
                } else if (state === STATE_DONE) {
                    clearInterval(spin);
                    fs.closeSync(fd);
                    
                    // Set Idle exactly like Python expects
                    try {
                        const fd2 = fs.openSync(bridgePath, 'r+');
                        const idleBuf = Buffer.alloc(1);
                        idleBuf.writeUInt8(0, 0);
                        fs.writeSync(fd2, idleBuf, 0, 1, 0);
                        fs.closeSync(fd2);
                    } catch(e) {}
                    
                    if (this._view) {
                        this._view.webview.postMessage({ type: 'endResponse' });
                    }
                } else {
                    spinCount++;
                    if (spinCount > 1000) { // 1000 * 20ms = 20s timeout
                        clearInterval(spin);
                        fs.closeSync(fd);
                        if (this._view) {
                            this._view.webview.postMessage({
                                type: 'errorResponse',
                                text: `[MMAP TIMEOUT] Cortex failed to respond. Spinlock severed.`
                            });
                        }
                    }
                }
            }, 20); // Poll every 20ms
        } catch (err) {
            try { fs.closeSync(fd); } catch(e) {}
            if (this._view) {
                this._view.webview.postMessage({
                    type: 'errorResponse',
                    text: `Bridge failed: ${err.message}`
                });
            }
        }
    }

    _getHtml() {
        return `<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<style>
* { margin: 0; padding: 0; box-sizing: border-box; }
body {
    font-family: var(--vscode-font-family);
    font-size: var(--vscode-font-size);
    color: var(--vscode-foreground);
    background: var(--vscode-sideBar-background);
    display: flex;
    flex-direction: column;
    height: 100vh;
}
#chat-container {
    flex: 1;
    overflow-y: auto;
    padding: 12px;
    display: flex;
    flex-direction: column;
    gap: 8px;
}
.message {
    padding: 8px 12px;
    border-radius: 8px;
    max-width: 95%;
    word-wrap: break-word;
    white-space: pre-wrap;
    line-height: 1.5;
    font-size: 13px;
}
.user-msg {
    background: var(--vscode-button-background);
    color: var(--vscode-button-foreground);
    align-self: flex-end;
    border-bottom-right-radius: 2px;
}
.sarah-msg {
    background: var(--vscode-editor-background);
    border: 1px solid var(--vscode-widget-border, #333);
    align-self: flex-start;
    border-bottom-left-radius: 2px;
}
.sarah-msg code {
    background: var(--vscode-textCodeBlock-background);
    padding: 1px 4px;
    border-radius: 3px;
    font-family: var(--vscode-editor-font-family);
}
.sarah-msg pre {
    background: var(--vscode-textCodeBlock-background);
    padding: 8px;
    border-radius: 4px;
    overflow-x: auto;
    margin: 4px 0;
}
.typing {
    opacity: 0.6;
    font-style: italic;
}
.error-msg {
    background: var(--vscode-inputValidation-errorBackground, #5a1d1d);
    border: 1px solid var(--vscode-inputValidation-errorBorder, #be1100);
    color: var(--vscode-errorForeground, #f48771);
    align-self: flex-start;
}
.header {
    padding: 8px 12px;
    background: var(--vscode-editor-background);
    border-bottom: 1px solid var(--vscode-widget-border, #333);
    font-weight: bold;
    font-size: 11px;
    text-transform: uppercase;
    letter-spacing: 1px;
    color: #00d4ff;
    display: flex;
    align-items: center;
    gap: 6px;
}
.header .dot {
    width: 8px; height: 8px;
    border-radius: 50%;
    background: #00d4ff;
    animation: pulse 2s infinite;
}
@keyframes pulse {
    0%, 100% { opacity: 1; }
    50% { opacity: 0.3; }
}
#input-area {
    display: flex;
    padding: 8px;
    gap: 4px;
    border-top: 1px solid var(--vscode-widget-border, #333);
    background: var(--vscode-editor-background);
}
#input-area textarea {
    flex: 1;
    background: var(--vscode-input-background);
    color: var(--vscode-input-foreground);
    border: 1px solid var(--vscode-input-border, #333);
    border-radius: 4px;
    padding: 6px 8px;
    font-family: var(--vscode-font-family);
    font-size: 13px;
    resize: none;
    outline: none;
    min-height: 36px;
    max-height: 120px;
}
#input-area textarea:focus {
    border-color: var(--vscode-focusBorder);
}
#input-area button {
    background: var(--vscode-button-background);
    color: var(--vscode-button-foreground);
    border: none;
    border-radius: 4px;
    padding: 0 12px;
    cursor: pointer;
    font-weight: bold;
    font-size: 14px;
}
#input-area button:hover {
    background: var(--vscode-button-hoverBackground);
}
</style>
</head>
<body>
<div class="header">
    <span class="dot"></span>
    SARAH SOVEREIGN
</div>
<div id="chat-container">
    <div class="message sarah-msg">Sovereign Node Active. How can I assist you, Architect?</div>
</div>
<div id="input-area">
    <textarea id="user-input" placeholder="Ask Sarah..." rows="1"></textarea>
    <button id="send-btn">▶</button>
</div>
<script>
const vscode = acquireVsCodeApi();
const chatContainer = document.getElementById('chat-container');
const userInput = document.getElementById('user-input');
const sendBtn = document.getElementById('send-btn');
let currentResponseDiv = null;
let currentResponseText = '';

function addMessage(text, className) {
    const div = document.createElement('div');
    div.className = 'message ' + className;
    div.textContent = text;
    chatContainer.appendChild(div);
    chatContainer.scrollTop = chatContainer.scrollHeight;
    return div;
}

function sendMessage() {
    const text = userInput.value.trim();
    if (!text) return;
    addMessage(text, 'user-msg');
    vscode.postMessage({ type: 'sendMessage', text });
    userInput.value = '';
    userInput.style.height = '36px';
}

sendBtn.addEventListener('click', sendMessage);
userInput.addEventListener('keydown', (e) => {
    if (e.key === 'Enter' && !e.shiftKey) {
        e.preventDefault();
        sendMessage();
    }
});

// Auto-resize textarea
userInput.addEventListener('input', () => {
    userInput.style.height = '36px';
    userInput.style.height = Math.min(userInput.scrollHeight, 120) + 'px';
});

// Handle messages from extension
window.addEventListener('message', (event) => {
    const msg = event.data;
    switch (msg.type) {
        case 'addUserMessage':
            addMessage(msg.text, 'user-msg');
            break;
        case 'startResponse':
            currentResponseText = '';
            currentResponseDiv = addMessage('...', 'message sarah-msg typing');
            break;
        case 'streamToken':
            if (currentResponseDiv) {
                currentResponseText += msg.token;
                currentResponseDiv.textContent = currentResponseText;
                currentResponseDiv.classList.remove('typing');
                chatContainer.scrollTop = chatContainer.scrollHeight;
            }
            break;
        case 'endResponse':
            if (currentResponseDiv) {
                currentResponseDiv.classList.remove('typing');
                if (!currentResponseText.trim()) {
                    currentResponseDiv.textContent = '[No response from gateway]';
                }
            }
            currentResponseDiv = null;
            break;
        case 'errorResponse':
            if (currentResponseDiv) {
                currentResponseDiv.remove();
            }
            addMessage(msg.text, 'message error-msg');
            currentResponseDiv = null;
            break;
    }
});
</script>
</body>
</html>`;
    }
}

function deactivate() {}

module.exports = { activate, deactivate };
