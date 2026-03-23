from flask import Flask, render_template_string, request, jsonify
from flask_socketio import SocketIO, emit
import json
import sys
import os
from datetime import datetime
import threading
import time
import random

# Add core path
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..'))

try:
    from agent_autonomy_loops import SovereignCoordinator, AutonomousAgent, Tier1Sovereign, Task
    from node_classification_metric import NodeClusterMetric
except ImportError:
    print("[ERROR] Could not import core modules. Run from 05_THE_CORE/")
    SovereignCoordinator = None

app = Flask(__name__)
app.config['SECRET_KEY'] = 'sarah-sovereign-secret'
socketio = SocketIO(app, cors_allowed_origins="*")

# Initialize system
coordinator = None
cluster_metric = None
message_history = []
autonomous_thread = None
running = False

def init_system():
    global coordinator, cluster_metric
    
    if SovereignCoordinator is None:
        return
    
    # Initialize coordinator
    coordinator = SovereignCoordinator()
    
    # Create Tier-1 Sovereigns
    axiom = Tier1Sovereign("axiom", "science_technology", ["quark", "forge", "scribe"])
    vigil = Tier1Sovereign("vigil", "medical_biology", ["aegis", "helix", "pulse"])
    atlas = Tier1Sovereign("atlas", "social_info_econ", ["lattice", "strata", "chorus"])
    
    coordinator.register_sovereign(axiom)
    coordinator.register_sovereign(vigil)
    coordinator.register_sovereign(atlas)
    
    # Create agents
    agents_config = [
        ("quark", "science_technology", "axiom", ["c++", "cuda"]),
        ("forge", "synthetic_hardware", "axiom", ["rust", "c"]),
        ("scribe", "science_technology", "axiom", ["lean", "coq"]),
        ("aegis", "medical_biology", "vigil", ["python", "r"]),
        ("helix", "medical_biology", "vigil", ["python", "sbml"]),
        ("pulse", "medical_biology", "vigil", ["python", "julia"]),
        ("lattice", "social_info", "atlas", ["python", "typescript"]),
        ("strata", "economics", "atlas", ["go", "python"]),
        ("chorus", "audio_music", "atlas", ["python", "typescript"]),
    ]
    
    for agent_id, pillar, supervisor, langs in agents_config:
        agent = AutonomousAgent(agent_id, pillar, supervisor, langs)
        coordinator.register_agent(agent, supervisor)
    
    # Initialize cluster metrics
    cluster_metric = NodeClusterMetric()
    alpha = cluster_metric.register_node("alpha", "logic_paramount")
    beta = cluster_metric.register_node("beta", "persistence_engine")
    delta = cluster_metric.register_node("delta", "command_anchor")
    
    # Set metrics
    alpha.update_metric("connectivity", 98)
    alpha.update_metric("security_score", 95)
    alpha.update_metric("performance", 92)
    alpha.update_metric("data_integrity", 100)
    alpha.update_metric("agent_capacity", 12)
    
    beta.update_metric("connectivity", 99)
    beta.update_metric("security_score", 98)
    beta.update_metric("performance", 95)
    beta.update_metric("data_integrity", 99)
    
    delta.update_metric("connectivity", 100)
    delta.update_metric("security_score", 92)
    delta.update_metric("performance", 85)
    delta.update_metric("data_integrity", 95)
    delta.update_metric("agent_capacity", 3)

def autonomous_worker():
    """Background thread that continuously generates tasks and runs cycles"""
    global running, coordinator, cluster_metric
    
    task_templates = [
        ("Process quantum simulation dataset", "quark", 2),
        ("Optimize neural network architecture", "forge", 3),
        ("Verify mathematical proof", "scribe", 2),
        ("Analyze genomic sequence", "aegis", 3),
        ("Model protein folding dynamics", "helix", 3),
        ("Process medical imaging data", "pulse", 2),
        ("Analyze social network patterns", "lattice", 2),
        ("Forecast economic indicators", "strata", 3),
        ("Synthesize audio waveform", "chorus", 2),
    ]
    
    cycle_count = 0
    
    while running:
        try:
            # Generate 2-4 new tasks every cycle
            num_tasks = random.randint(2, 4)
            for _ in range(num_tasks):
                desc, agent_id, priority = random.choice(task_templates)
                task = Task(
                    f"{desc} #{random.randint(1000, 9999)}",
                    priority,
                    agent_id
                )
                # Dispatch directly to agent via sovereign
                for sov in coordinator.sovereigns.values():
                    if agent_id in sov.agents:
                        sov.dispatch_task(agent_id, task)
                        break
            
            # Run coordination cycle
            coordinator.run_coordination_cycle()
            cycle_count += 1
            
            # Update node metrics with slight variations
            if cluster_metric and cycle_count % 3 == 0:
                alpha = cluster_metric.nodes.get("alpha")
                beta = cluster_metric.nodes.get("beta")
                delta = cluster_metric.nodes.get("delta")
                
                if alpha:
                    alpha.update_metric("performance", min(100, max(85, alpha.metrics.get("performance", 92) + random.randint(-2, 3))))
                if beta:
                    beta.update_metric("performance", min(100, max(90, beta.metrics.get("performance", 95) + random.randint(-1, 2))))
                if delta:
                    delta.update_metric("performance", min(100, max(80, delta.metrics.get("performance", 85) + random.randint(-3, 4))))
            
            # Broadcast update to all connected clients
            socketio.emit('system_update', get_system_state())
            
            # Wait 3 seconds between cycles
            time.sleep(3)
            
        except Exception as e:
            print(f"[AUTONOMOUS WORKER ERROR] {e}")
            time.sleep(5)

def get_system_state():
    """Get current system state for broadcasting"""
    if coordinator is None or cluster_metric is None:
        return {}
    
    cluster_status = coordinator.get_cluster_status()
    cluster_report = cluster_metric.report()
    
    data = {
        'cluster_health': cluster_report['cluster_health'],
        'total_tasks': coordinator.total_tasks_executed,
        'cycles': coordinator.coordination_cycles,
        'nodes': cluster_report['nodes'],
        'sovereigns': cluster_status['sovereigns'],
        'agents': []
    }
    
    for sov_data in cluster_status['sovereigns'].values():
        data['agents'].extend(sov_data['agents'])
    
    return data

init_system()

HTML_TEMPLATE = """
<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>New World Command Center</title>
    <script src="https://cdn.socket.io/4.5.4/socket.io.min.js"></script>
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body { 
            font-family: 'Segoe UI', 'Courier New', monospace; 
            background: linear-gradient(135deg, #000000 0%, #0a0a0a 100%); 
            color: #00ff41; 
            overflow: hidden;
        }
        .container { 
            display: grid; 
            grid-template-columns: 320px 1fr 380px; 
            height: 100vh; 
            gap: 2px; 
            background: #000;
        }
        
        .sidebar { 
            background: linear-gradient(180deg, #000000 0%, #0d1117 100%); 
            padding: 20px; 
            overflow-y: auto; 
            border-right: 1px solid #1a3a52;
            box-shadow: inset -2px 0 8px rgba(0, 255, 65, 0.1);
        }
        .sidebar h2 { 
            color: #00ff41; 
            margin-bottom: 20px; 
            font-size: 16px; 
            text-transform: uppercase;
            letter-spacing: 2px;
            text-shadow: 0 0 10px rgba(0, 255, 65, 0.5);
        }
        .node-card { 
            background: linear-gradient(135deg, #0a0f1a 0%, #121820 100%); 
            border: 1px solid #1a3a52; 
            padding: 14px; 
            margin-bottom: 16px; 
            border-radius: 8px;
            box-shadow: 0 4px 12px rgba(0, 0, 0, 0.4), inset 0 1px 2px rgba(26, 58, 82, 0.3);
            transition: all 0.3s ease;
        }
        .node-card:hover {
            border-color: #00ff41;
            box-shadow: 0 6px 16px rgba(0, 255, 65, 0.2);
            transform: translateY(-2px);
        }
        .node-name { 
            color: #00ff41; 
            font-weight: bold; 
            margin-bottom: 10px;
            font-size: 14px;
            text-shadow: 0 0 8px rgba(0, 255, 65, 0.4);
        }
        .node-metric { 
            font-size: 11px; 
            margin: 5px 0; 
            color: #8ab4f8;
        }
        .health-bar { 
            background: linear-gradient(90deg, #0a0f1a 0%, #1a1f2a 100%); 
            height: 10px; 
            margin: 8px 0; 
            border-radius: 6px; 
            overflow: hidden;
            border: 1px solid #1a3a52;
            box-shadow: inset 0 2px 4px rgba(0, 0, 0, 0.5);
        }
        .health-fill { 
            background: linear-gradient(90deg, #00ff41 0%, #00cc33 50%, #00ff41 100%); 
            height: 100%; 
            transition: width 0.5s ease;
            box-shadow: 0 0 10px rgba(0, 255, 65, 0.6);
        }
        
        .main { 
            background: #000000; 
            display: flex; 
            flex-direction: column;
        }
        .header { 
            background: linear-gradient(135deg, #000000 0%, #0d1117 100%); 
            padding: 24px; 
            border-bottom: 2px solid #1a3a52;
            box-shadow: 0 4px 12px rgba(0, 0, 0, 0.6);
        }
        .header h1 { 
            color: #00ff41; 
            text-shadow: 0 0 20px rgba(0, 255, 65, 0.6), 0 0 40px rgba(0, 255, 65, 0.3); 
            font-size: 26px;
            display: flex;
            align-items: center;
            gap: 16px;
            letter-spacing: 1px;
        }
        .logo-n {
            width: 42px;
            height: 42px;
            display: inline-block;
        }
        .status-bar { 
            display: flex; 
            gap: 24px; 
            margin-top: 14px; 
            font-size: 13px;
            flex-wrap: wrap;
        }
        .status-item { 
            display: flex; 
            align-items: center; 
            gap: 8px;
            padding: 6px 12px;
            background: rgba(26, 58, 82, 0.2);
            border-radius: 6px;
            border: 1px solid #1a3a52;
        }
        .status-ok { 
            color: #00ff41;
            text-shadow: 0 0 6px rgba(0, 255, 65, 0.5);
        }
        .status-critical { color: #ff3333; }
        .status-warning { color: #ffaa00; }
        
        .content { 
            flex: 1; 
            overflow-y: auto; 
            padding: 24px;
            background: linear-gradient(135deg, #000000 0%, #0a0a0a 100%);
        }
        .agents-grid { 
            display: grid; 
            grid-template-columns: repeat(3, 1fr); 
            gap: 18px; 
            margin-bottom: 30px;
        }
        .agent-card { 
            background: linear-gradient(135deg, #0a0f1a 0%, #121820 100%); 
            border: 1px solid #1a3a52; 
            padding: 14px; 
            border-radius: 8px;
            box-shadow: 0 4px 12px rgba(0, 0, 0, 0.4);
            transition: all 0.3s ease;
        }
        .agent-card:hover {
            border-color: #8ab4f8;
            box-shadow: 0 6px 16px rgba(138, 180, 248, 0.3);
            transform: translateY(-2px);
        }
        .agent-card.active {
            border-color: #00ff41;
            box-shadow: 0 0 20px rgba(0, 255, 65, 0.4);
        }
        .agent-name { 
            color: #8ab4f8; 
            font-weight: bold; 
            margin-bottom: 8px; 
            font-size: 14px;
            text-transform: uppercase;
            letter-spacing: 1px;
        }
        .agent-card.active .agent-name {
            color: #00ff41;
            text-shadow: 0 0 8px rgba(0, 255, 65, 0.5);
        }
        .agent-metric { 
            font-size: 11px; 
            margin: 4px 0;
            color: #8ab4f8;
        }
        
        .chat-panel { 
            background: linear-gradient(180deg, #000000 0%, #0d1117 100%); 
            display: flex; 
            flex-direction: column;
            border-left: 1px solid #1a3a52;
            box-shadow: inset 2px 0 8px rgba(0, 255, 65, 0.1);
        }
        .chat-header { 
            background: linear-gradient(135deg, #0a0f1a 0%, #121820 100%); 
            padding: 18px; 
            border-bottom: 2px solid #1a3a52;
            box-shadow: 0 2px 8px rgba(0, 0, 0, 0.4);
        }
        .chat-header h2 { 
            color: #00ff41; 
            font-size: 16px;
            text-transform: uppercase;
            letter-spacing: 2px;
            text-shadow: 0 0 10px rgba(0, 255, 65, 0.5);
        }
        .chat-messages { 
            flex: 1; 
            overflow-y: auto; 
            padding: 18px;
            background: #000000;
        }
        .message { 
            margin-bottom: 16px; 
            padding: 12px; 
            border-radius: 8px; 
            font-size: 13px;
            animation: slideIn 0.3s ease;
        }
        @keyframes slideIn {
            from { opacity: 0; transform: translateX(10px); }
            to { opacity: 1; transform: translateX(0); }
        }
        .message-user { 
            background: linear-gradient(135deg, #0a0f1a 0%, #1a2a4a 100%); 
            border-left: 3px solid #8ab4f8;
            box-shadow: 0 2px 8px rgba(138, 180, 248, 0.2);
        }
        .message-system { 
            background: linear-gradient(135deg, #0a0f1a 0%, #1a3a1a 100%); 
            border-left: 3px solid #00ff41;
            box-shadow: 0 2px 8px rgba(0, 255, 65, 0.2);
        }
        .message-time { 
            font-size: 10px; 
            color: #666; 
            margin-bottom: 6px;
        }
        .message-content { 
            line-height: 1.5;
            color: #8ab4f8;
        }
        .message-system .message-content {
            color: #00ff41;
        }
        
        .chat-input-area { 
            background: linear-gradient(135deg, #0a0f1a 0%, #121820 100%); 
            padding: 18px; 
            border-top: 1px solid #1a3a52;
            box-shadow: 0 -2px 8px rgba(0, 0, 0, 0.4);
        }
        .input-group { 
            display: flex; 
            gap: 12px;
        }
        .chat-input { 
            flex: 1; 
            background: #000000; 
            border: 1px solid #1a3a52; 
            color: #00ff41; 
            padding: 12px; 
            border-radius: 6px; 
            font-family: 'Segoe UI', monospace;
            font-size: 13px;
            transition: all 0.3s ease;
        }
        .chat-input:focus {
            outline: none;
            border-color: #00ff41;
            box-shadow: 0 0 12px rgba(0, 255, 65, 0.3);
        }
        .btn { 
            background: linear-gradient(135deg, #00ff41 0%, #00cc33 100%); 
            color: #000; 
            border: none; 
            padding: 12px 24px; 
            border-radius: 6px; 
            cursor: pointer; 
            font-weight: bold; 
            font-family: 'Segoe UI', monospace;
            transition: all 0.3s ease;
            text-shadow: none;
            box-shadow: 0 4px 12px rgba(0, 255, 65, 0.4);
        }
        .btn:hover { 
            background: linear-gradient(135deg, #00ff41 0%, #00ff41 100%); 
            box-shadow: 0 6px 16px rgba(0, 255, 65, 0.6);
            transform: translateY(-2px);
        }
        .btn:active {
            transform: translateY(0);
        }
        .btn-secondary { 
            background: linear-gradient(135deg, #8ab4f8 0%, #6a94d8 100%); 
            color: #000;
            box-shadow: 0 4px 12px rgba(138, 180, 248, 0.4);
        }
        .btn-secondary:hover {
            background: linear-gradient(135deg, #8ab4f8 0%, #8ab4f8 100%); 
            box-shadow: 0 6px 16px rgba(138, 180, 248, 0.6);
        }
        
        .quick-actions { 
            margin-bottom: 12px;
            display: flex;
            gap: 8px;
            flex-wrap: wrap;
        }
        .quick-btn { 
            background: linear-gradient(135deg, #1a2a4a 0%, #2a3a5a 100%); 
            color: #8ab4f8; 
            border: 1px solid #1a3a52; 
            padding: 8px 16px; 
            border-radius: 6px; 
            cursor: pointer; 
            font-size: 11px;
            transition: all 0.3s ease;
            text-transform: uppercase;
            letter-spacing: 0.5px;
        }
        .quick-btn:hover { 
            background: linear-gradient(135deg, #2a3a5a 0%, #3a4a6a 100%); 
            border-color: #8ab4f8;
            box-shadow: 0 0 12px rgba(138, 180, 248, 0.4);
            transform: translateY(-2px);
        }
        
        .status-pulse { 
            animation: pulse 2s infinite; 
            filter: drop-shadow(0 0 6px currentColor);
        }
        @keyframes pulse { 
            0%, 100% { opacity: 1; transform: scale(1); } 
            50% { opacity: 0.6; transform: scale(1.1); } 
        }
        
        .section-title { 
            color: #00ff41; 
            font-size: 14px; 
            margin: 24px 0 12px 0; 
            text-transform: uppercase;
            letter-spacing: 2px;
            text-shadow: 0 0 10px rgba(0, 255, 65, 0.5);
        }
        
        ::-webkit-scrollbar { width: 8px; }
        ::-webkit-scrollbar-track { background: #000; }
        ::-webkit-scrollbar-thumb { 
            background: linear-gradient(180deg, #1a3a52 0%, #2a4a62 100%); 
            border-radius: 4px;
        }
        ::-webkit-scrollbar-thumb:hover { background: #8ab4f8; }
    </style>
</head>
<body>
    <div class="container">
        <!-- Left Sidebar: Node Status -->
        <div class="sidebar">
            <h2>⚡ NODE STATUS</h2>
            <div id="nodes-container"></div>
            
            <div class="section-title">Tier-1 Sovereigns</div>
            <div id="sovereigns-container"></div>
        </div>
        
        <!-- Main Content: Agent Grid -->
        <div class="main">
            <div class="header">
                <h1>
                    <svg class="logo-n" viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg">
                        <defs>
                            <linearGradient id="nGrad" x1="0%" y1="0%" x2="100%" y2="100%">
                                <stop offset="0%" style="stop-color:#00ff41;stop-opacity:1" />
                                <stop offset="50%" style="stop-color:#00cc33;stop-opacity:1" />
                                <stop offset="100%" style="stop-color:#00ff41;stop-opacity:1" />
                            </linearGradient>
                            <filter id="glow">
                                <feGaussianBlur stdDeviation="3" result="coloredBlur"/>
                                <feMerge>
                                    <feMergeNode in="coloredBlur"/>
                                    <feMergeNode in="SourceGraphic"/>
                                </feMerge>
                            </filter>
                        </defs>
                        <path d="M 20 80 L 20 20 L 30 20 L 30 60 L 70 20 L 80 20 L 80 80 L 70 80 L 70 40 L 30 80 Z" 
                              fill="url(#nGrad)" 
                              stroke="#00ff41" 
                              stroke-width="2"
                              filter="url(#glow)"/>
                        <circle cx="50" cy="50" r="45" fill="none" stroke="url(#nGrad)" stroke-width="2" opacity="0.3"/>
                    </svg>
                    NEW WORLD COMMAND CENTER
                </h1>
                <div class="status-bar">
                    <div class="status-item">
                        <span class="status-pulse status-ok">●</span>
                        <span>System Online</span>
                    </div>
                    <div class="status-item">
                        <span>Cluster Health: <span id="cluster-health">--</span>/100</span>
                    </div>
                    <div class="status-item">
                        <span>Tasks Executed: <span id="tasks-count">0</span></span>
                    </div>
                    <div class="status-item">
                        <span>Cycles: <span id="cycles-count">0</span></span>
                    </div>
                </div>
            </div>
            
            <div class="content">
                <div class="section-title">🤖 Nine Polyglot Agents</div>
                <div class="agents-grid" id="agents-grid"></div>
            </div>
        </div>
        
        <!-- Right Panel: Chat Interface -->
        <div class="chat-panel">
            <div class="chat-header">
                <h2>💬 COMMAND INTERFACE</h2>
            </div>
            <div class="chat-messages" id="chat-messages"></div>
            <div class="chat-input-area">
                <div class="quick-actions">
                    <button class="quick-btn" onclick="quickCommand('status')">System Status</button>
                    <button class="quick-btn" onclick="quickCommand('run_cycle')">Run Cycle</button>
                    <button class="quick-btn" onclick="quickCommand('health')">Health Check</button>
                </div>
                <div class="input-group">
                    <input type="text" id="chat-input" class="chat-input" placeholder="Enter command or message..." onkeypress="if(event.key==='Enter') sendMessage()">
                    <button class="btn" onclick="sendMessage()">SEND</button>
                </div>
            </div>
        </div>
    </div>
    
    <script>
        const socket = io();
        
        socket.on('connect', () => {
            addSystemMessage('Connected to Sarah Sovereign Core');
            requestUpdate();
        });
        
        socket.on('system_update', (data) => {
            updateDashboard(data);
        });
        
        socket.on('message_response', (data) => {
            addSystemMessage(data.message);
        });
        
        function requestUpdate() {
            socket.emit('request_update');
        }
        
        function sendMessage() {
            const input = document.getElementById('chat-input');
            const message = input.value.trim();
            if (!message) return;
            
            addUserMessage(message);
            socket.emit('user_message', { message: message });
            input.value = '';
        }
        
        function quickCommand(cmd) {
            addUserMessage(cmd);
            socket.emit('user_message', { message: cmd });
        }
        
        function addUserMessage(text) {
            const container = document.getElementById('chat-messages');
            const msg = document.createElement('div');
            msg.className = 'message message-user';
            msg.innerHTML = `
                <div class="message-time">${new Date().toLocaleTimeString()}</div>
                <div class="message-content">> ${text}</div>
            `;
            container.appendChild(msg);
            container.scrollTop = container.scrollHeight;
        }
        
        function addSystemMessage(text) {
            const container = document.getElementById('chat-messages');
            const msg = document.createElement('div');
            msg.className = 'message message-system';
            msg.innerHTML = `
                <div class="message-time">${new Date().toLocaleTimeString()}</div>
                <div class="message-content">${text}</div>
            `;
            container.appendChild(msg);
            container.scrollTop = container.scrollHeight;
        }
        
        function updateDashboard(data) {
            // Update header stats
            document.getElementById('cluster-health').textContent = data.cluster_health || '--';
            document.getElementById('tasks-count').textContent = data.total_tasks || 0;
            document.getElementById('cycles-count').textContent = data.cycles || 0;
            
            // Update nodes
            const nodesContainer = document.getElementById('nodes-container');
            nodesContainer.innerHTML = '';
            for (const [nodeId, nodeData] of Object.entries(data.nodes || {})) {
                const health = nodeData.health || 0;
                nodesContainer.innerHTML += `
                    <div class="node-card">
                        <div class="node-name">${nodeId.toUpperCase()} Node</div>
                        <div class="node-metric">Health: ${health}/100</div>
                        <div class="health-bar"><div class="health-fill" style="width: ${health}%"></div></div>
                        <div class="node-metric">Security: ${nodeData.security || '--'}</div>
                        <div class="node-metric">Status: <span class="status-ok">${nodeData.classification || 'UNKNOWN'}</span></div>
                    </div>
                `;
            }
            
            // Update sovereigns
            const sovereignsContainer = document.getElementById('sovereigns-container');
            sovereignsContainer.innerHTML = '';
            for (const [sovId, sovData] of Object.entries(data.sovereigns || {})) {
                sovereignsContainer.innerHTML += `
                    <div class="node-card">
                        <div class="node-name">${sovId.toUpperCase()}</div>
                        <div class="node-metric">Authority: ${Math.round((sovData.authority_level || 0) * 100)}%</div>
                        <div class="node-metric">Decisions: ${sovData.decisions_made || 0}</div>
                        <div class="node-metric">Perf: ${sovData.cluster_performance || 0}</div>
                    </div>
                `;
            }
            
            // Update agents
            const agentsGrid = document.getElementById('agents-grid');
            agentsGrid.innerHTML = '';
            for (const agent of data.agents || []) {
                const statusClass = agent.is_active ? 'status-ok' : '';
                const statusText = agent.is_active ? 'ACTIVE' : 'IDLE';
                const activeClass = agent.is_active ? 'active' : '';
                const perfBar = Math.round(agent.performance_score * 100);
                agentsGrid.innerHTML += `
                    <div class="agent-card ${activeClass}">
                        <div class="agent-name">${agent.agent_id.toUpperCase()}</div>
                        <div class="agent-metric">Perf: ${agent.performance_score}</div>
                        <div class="health-bar"><div class="health-fill" style="width: ${perfBar}%"></div></div>
                        <div class="agent-metric">Autonomy: ${agent.autonomy_level}</div>
                        <div class="agent-metric">Tasks: ${agent.tasks_completed}</div>
                        <div class="agent-metric">Queue: ${agent.queue_size}</div>
                        <div class="agent-metric">Status: <span class="${statusClass}">${statusText}</span></div>
                    </div>
                `;
            }
        }
        
        // Auto-refresh every 2 seconds
        setInterval(requestUpdate, 2000);
    </script>
</body>
</html>
"""

@app.route('/')
def index():
    return render_template_string(HTML_TEMPLATE)

@socketio.on('connect')
def handle_connect():
    global running, autonomous_thread
    if not running:
        running = True
        autonomous_thread = threading.Thread(target=autonomous_worker, daemon=True)
        autonomous_thread.start()
        print("[AUTONOMOUS WORKER] Background processing started")

@socketio.on('request_update')
def handle_update_request():
    if coordinator is None or cluster_metric is None:
        return
    
    emit('system_update', get_system_state())

@socketio.on('user_message')
def handle_user_message(data):
    message = data.get('message', '').strip().lower()
    
    if message == 'status':
        response = f"System operational. Cluster health: {cluster_metric.cluster_health}/100. {coordinator.coordination_cycles} cycles executed. {coordinator.total_tasks_executed} tasks completed."
    elif message == 'run_cycle':
        results = coordinator.run_coordination_cycle()
        count = sum(len(tasks) for tasks in results.values())
        response = f"Coordination cycle executed. {count} tasks completed."
    elif message == 'health':
        report = cluster_metric.report()
        response = f"Alpha: {report['nodes']['alpha']['metrics']['health']}/100, Beta: {report['nodes']['beta']['metrics']['health']}/100, Delta: {report['nodes']['delta']['metrics']['health']}/100"
    else:
        response = f"[SARAH] Received: '{message}'. Command interface active. Available: status, run_cycle, health"
    
    emit('message_response', {'message': response})

if __name__ == '__main__':
    print("[SARAH COMMAND CENTER] Starting interactive UI server...")
    print("[SARAH COMMAND CENTER] Open http://localhost:5000 in your browser")
    print("[SARAH COMMAND CENTER] Autonomous background processing will start on first connection")
    try:
        socketio.run(app, host='0.0.0.0', port=5000, debug=False)
    finally:
        running = False
        if autonomous_thread:
            autonomous_thread.join(timeout=2)
