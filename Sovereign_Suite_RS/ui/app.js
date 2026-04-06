const socket = new WebSocket(`ws://${window.location.host}/neural-stream`);
const messagesContainer = document.getElementById('messages');
const userInput = document.getElementById('user-input');
const sendBtn = document.getElementById('send-btn');
const heartbeatValue = document.getElementById('heartbeat-value');
const inferenceValue = document.getElementById('inference-value');

// 1.0092703703703 HZ HEARTBEAT SYNC
let lastHeartbeat = Date.now();
setInterval(() => {
    const now = Date.now();
    const elapsed = now - lastHeartbeat;
    // Visually pulse the heartbeat value
    heartbeatValue.style.opacity = (Math.sin(now / 157.5) + 1.5) / 2.5; 
}, 50);

socket.onmessage = (event) => {
    const data = JSON.parse(event.data);
    appendMessage('neural', data.token);
    inferenceValue.innerText = `${data.inference_ms.toFixed(2)} MS`;
    
    // Heartbeat sync verification
    if (data.heartbeat_sync) {
        heartbeatValue.style.color = '#00ffaa';
    }
};

function appendMessage(role, text) {
    const msgDiv = document.createElement('div');
    msgDiv.className = `message ${role}`;
    msgDiv.innerText = text;
    messagesContainer.appendChild(msgDiv);
    messagesContainer.scrollTop = messagesContainer.scrollHeight;
}

function strike() {
    const text = userInput.value.trim();
    if (text) {
        appendMessage('user', text);
        socket.send(text);
        userInput.value = '';
    }
}

sendBtn.onclick = strike;
userInput.onkeydown = (e) => {
    if (e.key === 'Enter') strike();
};

socket.onopen = () => {
    appendMessage('system', '[+] NEURAL STREAM ESTABLISHED. WE ARE LIVE.');
};

socket.onclose = () => {
    appendMessage('system', '[!] NEURAL STREAM SEVERED. RECONSTRUCTING...');
};
