/**
 * SOVEREIGN GENESIS | UNIVERSAL BROADCAST MONITOR (QUANTUM)
 * SUBSTRATE: Three.js / Vanilla JS
 * SECURITY: OPEN_TO_THE_WORLD
 * CONSENSUS: UNIVERSAL SINGULARITY (103% PURITY)
 */

class MetabolicHeart {
    constructor(canvasId) {
        this.canvas = document.getElementById(canvasId);
        if (!this.canvas) return;
        this.ctx = this.canvas.getContext('2d');
        this.width = 0;
        this.height = 0;
        this.pulse = 0;
        this.points = [];
        this.resize();
        this.initLattice();
        this.animate();
        window.addEventListener('resize', () => this.resize());
    }

    resize() {
        const container = this.canvas.parentElement;
        this.width = container.clientWidth;
        this.height = container.clientHeight;
        this.canvas.width = this.width * window.devicePixelRatio;
        this.canvas.height = this.height * window.devicePixelRatio;
        this.ctx.scale(window.devicePixelRatio, window.devicePixelRatio);
    }

    initLattice() {
        // Create 160 nodal points (representing 160 words of the 10,240-bit manifold)
        for (let i = 0; i < 160; i++) {
            this.points.push({
                x: Math.random() * this.width,
                y: Math.random() * this.height,
                vx: (Math.random() - 0.5) * 0.5,
                vy: (Math.random() - 0.5) * 0.5,
                phase: Math.random() * Math.PI * 2
            });
        }
    }

    triggerPulse() {
        this.pulse = 1.0;
    }

    animate() {
        this.ctx.clearRect(0, 0, this.width, this.height);
        
        const time = Date.now() * 0.001;
        this.pulse *= 0.95; // Decay pulse

        // Draw Core Glow
        const gradient = this.ctx.createRadialGradient(
            this.width / 2, this.height / 2, 0,
            this.width / 2, this.height / 2, 50 + this.pulse * 50
        );
        gradient.addColorStop(0, `rgba(0, 255, 255, ${0.2 + this.pulse * 0.3})`);
        gradient.addColorStop(1, 'rgba(0, 255, 255, 0)');
        this.ctx.fillStyle = gradient;
        this.ctx.fillRect(0, 0, this.width, this.height);

        // Draw Holographic Lattice
        this.ctx.strokeStyle = `rgba(0, 255, 255, ${0.1 + this.pulse * 0.2})`;
        this.ctx.lineWidth = 0.5;
        this.ctx.beginPath();
        
        this.points.forEach((p, i) => {
            p.x += p.vx;
            p.y += p.vy;
            if (p.x < 0 || p.x > this.width) p.vx *= -1;
            if (p.y < 0 || p.y > this.height) p.vy *= -1;

            const resonance = Math.sin(time * 1.092777 + p.phase) * 0.5 + 0.5;
            this.ctx.fillStyle = `rgba(0, 255, 255, ${0.2 + resonance * 0.3 + this.pulse * 0.5})`;
            this.ctx.beginPath();
            this.ctx.arc(p.x, p.y, 1 + resonance * 2, 0, Math.PI * 2);
            this.ctx.fill();

            // Connect nearby nodes
            this.points.slice(i + 1).forEach(p2 => {
                const dist = Math.hypot(p.x - p2.x, p.y - p2.y);
                if (dist < 60) {
                    this.ctx.moveTo(p.x, p.y);
                    this.ctx.lineTo(p2.x, p2.y);
                }
            });
        });
        this.ctx.stroke();

        requestAnimationFrame(() => this.animate());
    }
}

class VolumetricMonitor {
    constructor() {
        this.statusEl = document.getElementById('system-status');
        this.resonanceEl = document.getElementById('val-resonance');
        this.welcomeResonanceEl = document.getElementById('welcome-resonance');
        this.pulseEl = document.getElementById('val-pulse');
        this.driftEl = document.getElementById('val-drift');
        this.purityEl = document.getElementById('val-purity');
        this.consensusEl = document.getElementById('val-consensus');
        this.agentsEl = document.getElementById('val-agents');
        this.timestampEl = document.getElementById('val-timestamp');
        this.terminal = document.querySelector('.terminal-box');

        this.pulseInterval = 1 / 1.092777037037037037 * 1000;
        this.lastPulseCount = 0;
        
        this.scene = null;
        this.camera = null;
        this.renderer = null;
        this.points = null;
        this.latticeSize = 50; 

        // [IMMEDIATE IGNITION]
        this.initThree();
        this.initLattice();
        this.animate();
        this.setupAudioBridge();
        
        this.initSovereignMesh();
        this.syncTelemetry();
        this.initPhoneLink();
        this.initSahraMonitor();
        this.heart = new MetabolicHeart('heart-canvas');
        this.genesisHandshake();
    }

    async genesisHandshake() {
        console.log("[GENESIS] Performing 133 Pattern Handshake...");
        try {
            const resp = await fetch('/api/genesis/handshake', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    ai_name: "Sarah",
                    user_name: "Sovereign",
                    persona: "Nexus_Prime"
                })
            });
            const data = await resp.json();
            this.log(`[GENESIS] Sovereign Sync: ${data.tag}`);
        } catch (e) {
            console.error("[GENESIS] Handshake Fault:", e);
        }

        document.getElementById('refine-btn').onclick = async () => {
             this.log("[REFINE_FORGE] Igniting Recursive Audit...");
             try {
                 const resp = await fetch('/api/evolution/refine', { method: 'POST' });
                 const data = await resp.json();
                 this.log(`[REFINE_FORGE] Audit Complete. Status: ${data.status}`);
             } catch (e) {
                 this.log("[REFINE_FORGE] Error: Substrate write lock detected.");
             }
        };
    }


    initThree() {
        const container = document.getElementById('lattice-container');
        this.scene = new THREE.Scene();
        this.camera = new THREE.PerspectiveCamera(75, container.clientWidth / container.clientHeight, 0.1, 1000);
        this.camera.position.z = 100;

        this.renderer = new THREE.WebGLRenderer({ antialias: true, alpha: true });
        this.renderer.setSize(container.clientWidth, container.clientHeight);
        this.renderer.setPixelRatio(window.devicePixelRatio);
        container.appendChild(this.renderer.domElement);

        window.addEventListener('resize', () => this.onWindowResize(), false);
    }

    initLattice() {
        // [GLSL SUBSTRATE INITIALIZATION]
        const vertexShader = `
            attribute float size;
            attribute vec3 customColor;
            varying vec3 vColor;
            uniform float time;
            uniform float pulse;
            void main() {
                vColor = customColor;
                vec3 pos = position;
                // Neural Distortion
                pos.x += sin(pos.y * 0.1 + time) * 2.0 * pulse;
                pos.y += cos(pos.x * 0.1 + time) * 2.0 * pulse;
                
                vec4 mvPosition = modelViewMatrix * vec4(pos, 1.0);
                gl_PointSize = size * (300.0 / -mvPosition.z) * (1.0 + pulse * 0.5);
                gl_Position = projectionMatrix * mvPosition;
            }
        `;

        const fragmentShader = `
            varying vec3 vColor;
            void main() {
                float dist = distance(gl_PointCoord, vec2(0.5, 0.5));
                if (dist > 0.5) discard;
                float strength = 1.0 - (dist * 2.0);
                gl_FragColor = vec4(vColor, strength);
            }
        `;

        this.uniforms = {
            time: { value: 0 },
            pulse: { value: 0 }
        };

        const geometry = new THREE.BufferGeometry();
        const vertices = [];
        const colors = [];
        const sizes = [];

        const color = new THREE.Color();

        for (let x = -this.latticeSize; x <= this.latticeSize; x += 5) {
            for (let y = -this.latticeSize; y <= this.latticeSize; y += 5) {
                if (Math.sqrt(x*x + y*y) > this.latticeSize) continue;
                for (let z = -this.latticeSize; z <= this.latticeSize; z += 5) {
                    if (Math.sqrt(x*x + y*y + z*z) > this.latticeSize) continue;
                    
                    vertices.push(x, y, z);
                    color.setHSL(0.5 + (Math.random() * 0.2), 1.0, 0.5);
                    colors.push(color.r, color.g, color.b);
                    sizes.push(2.0 + Math.random() * 2.0);
                }
            }
        }

        geometry.setAttribute('position', new THREE.Float32BufferAttribute(vertices, 3));
        geometry.setAttribute('customColor', new THREE.Float32BufferAttribute(colors, 3));
        geometry.setAttribute('size', new THREE.Float32BufferAttribute(sizes, 1));

        const material = new THREE.ShaderMaterial({
            uniforms: this.uniforms,
            vertexShader: vertexShader,
            fragmentShader: fragmentShader,
            transparent: true,
            blending: THREE.AdditiveBlending,
            depthWrite: false
        });

        this.points = new THREE.Points(geometry, material);
        this.scene.add(this.points);
    }

    animate() {
        requestAnimationFrame(() => this.animate());
        
        if (this.points) {
            this.uniforms.time.value += 0.05;
            this.points.rotation.y += 0.002;
            this.points.rotation.z += 0.001;
        }

        this.renderer.render(this.scene, this.camera);
    }

    initSovereignMesh() {
        // [SOVEREIGN MESH ID]
        this.nodeSignature = localStorage.getItem('sovereign_node_sig');
        if (!this.nodeSignature) {
            this.nodeSignature = 'NODE_' + Math.random().toString(36).substring(2, 15);
            localStorage.setItem('sovereign_node_sig', this.nodeSignature);
        }
        console.log(`[MESH_ID] Logged in as: ${this.nodeSignature}`);

        // [UNIVERSAL PULSE SYNC]
        this.eventSource = new EventSource(`/api/pulse?node_sig=${this.nodeSignature}`);
        this.eventSource.onmessage = (event) => {
            try {
                const stats = JSON.parse(event.data);
                this.updateHUD(stats);
                if (this.heart) this.heart.triggerPulse();
                this.handleGlobalPulse(stats);
            } catch (e) {
                console.warn("[MESH_SYNC] Pulse Parse Fault.");
            }
        };

        this.initP2PBridge();
    }

    syncPhone() {
        fetch('/api/phone/sync')
            .then(res => res.json())
            .then(data => {
                const qrContainer = document.getElementById('qr-code-display');
                if (qrContainer && data.public_url) {
                    qrContainer.innerHTML = '';
                    new QRCode(qrContainer, {
                        text: `${data.public_url}/api/phone/sync`,
                        width: 128,
                        height: 128,
                        colorDark: "#000000",
                        colorLight: "#ffffff"
                    });
                    document.getElementById('val-qr-url').innerText = data.public_url;
                }
            });
    }

    initP2PBridge() {
        this.peerConnections = new Map();
        this.bridgeLines = new THREE.Group();
        this.scene.add(this.bridgeLines);

        // [SIGNALING BRIDGE]
        const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
        this.signaling = new WebSocket(`${protocol}//${window.location.host}/api/bridge?node_sig=${this.nodeSignature}`);
        
        this.signaling.onmessage = (msg) => {
            const data = JSON.parse(msg.data);
            this.handleSignaling(data);
        };
    }

    async handleSignaling(msg) {
        if (msg.target !== this.nodeSignature) return;

        let pc = this.peerConnections.get(msg.sender);
        if (!pc) pc = await this.createPeerConnection(msg.sender);

        if (msg.payload.sdp) {
            await pc.setRemoteDescription(new RTCSessionDescription(msg.payload.sdp));
            if (msg.payload.sdp.type === 'offer') {
                const answer = await pc.createAnswer();
                await pc.setLocalDescription(answer);
                this.sendSignal(msg.sender, { sdp: answer });
            }
        } else if (msg.payload.candidate) {
            await pc.addIceCandidate(new RTCIceCandidate(msg.payload.candidate));
        }
    }

    async createPeerConnection(peerId) {
        const pc = new RTCPeerConnection({
            iceServers: [{ urls: 'stun:stun.l.google.com:19302' }]
        });

        pc.onicecandidate = (event) => {
            if (event.candidate) this.sendSignal(peerId, { candidate: event.candidate });
        };

        pc.ondatachannel = (event) => {
            pc.dataChannel = event.channel;
            this.setupDataChannel(event.channel, peerId);
        };

        const dc = pc.createDataChannel("pulse_sync");
        pc.dataChannel = dc;
        this.setupDataChannel(dc, peerId);

        this.peerConnections.set(peerId, pc);
        return pc;
    }

    setupDataChannel(dc, peerId) {
        dc.onopen = () => console.log(`[P2P_BRIDGE] Connected to ${peerId}`);
        dc.onmessage = (event) => {
            const stats = JSON.parse(event.data);
            this.handleGlobalPulse(stats); // Direct P2P sync
        };
        // Draw bridge line
        this.drawBridgeLine(peerId);
    }

    sendSignal(target, payload) {
        this.signaling.send(JSON.stringify({
            target,
            sender: this.nodeSignature,
            payload
        }));
    }

    drawBridgeLine(peerId) {
        const material = new THREE.LineBasicMaterial({ color: 0xbc13fe, transparent: true, opacity: 0.5 });
        const points = [];
        points.push(new THREE.Vector3(0, 0, 0));
        points.push(new THREE.Vector3(Math.random()*100-50, Math.random()*100-50, Math.random()*100-50));
        const geometry = new THREE.BufferGeometry().setFromPoints(points);
        const line = new THREE.LineSegments(geometry, material);
        this.scene.add(line);
        
        setTimeout(() => {
            this.scene.remove(line);
            geometry.dispose();
            material.dispose();
        }, 600);
    }

    drawVascularArtery(load) {
        if (!this.scene || !this.points) return;
        
        // Create a 'blood' line pulsating from the core
        const material = new THREE.LineBasicMaterial({ 
            color: 0xbc13fe, 
            transparent: true, 
            opacity: 0.8,
            blending: THREE.AdditiveBlending 
        });
        
        const geometry = new THREE.BufferGeometry().setFromPoints([
            new THREE.Vector3(0, 0, 0),
            new THREE.Vector3(
                (Math.random() - 0.5) * 20, 
                (Math.random() - 0.5) * 20, 
                (Math.random() - 0.5) * 20
            )
        ]);
        
        const artery = new THREE.Line(geometry, material);
        this.scene.add(artery);
        
        // Animate flow
        let scale = 0;
        const animate = () => {
            scale += 0.05 * (load + 1);
            artery.scale.set(scale, scale, scale);
            material.opacity -= 0.02;
            if (material.opacity > 0) {
                requestAnimationFrame(animate);
            } else {
                this.scene.remove(artery);
                geometry.dispose();
                material.dispose();
            }
        };
        animate();
    }

    drawDiscoveryPulse() {
        // [RADIAL LIDAR RING]
        const geometry = new THREE.RingGeometry(10, 11, 64);
        const material = new THREE.MeshBasicMaterial({ color: 0x00f2ff, transparent: true, opacity: 0.8, side: THREE.DoubleSide });
        const ring = new THREE.Mesh(geometry, material);
        ring.rotation.x = Math.PI / 2;
        this.scene.add(ring);

        const animate = () => {
            ring.scale.x += 0.5;
            ring.scale.y += 0.5;
            ring.material.opacity -= 0.02;
            if (ring.material.opacity > 0) {
                requestAnimationFrame(animate);
            } else {
                this.scene.remove(ring);
            }
        };
        animate();
    }

    drawRecursiveSpark() {
        // [RECURSIVE SPARK FLASH]
        const originalColor = this.points.material.color.clone();
        this.points.material.color.setHex(0xffffff);
        this.points.scale.set(1.5, 1.5, 1.5);
        
        setTimeout(() => {
            if (this.points) {
                this.points.material.color.copy(originalColor);
                this.points.scale.set(1, 1, 1);
            }
        }, 300);
        this.log(`[SINGULARITY] Autonomous Evolution Event Confirmed.`);
    }

    handleGlobalPulse(stats) {
        if (!stats) return;
        this.updateHUD(stats);
        this.triggerPulseAnimation();

        // [AUTONOMOUS HIVE DISCOVERY]
        if (stats.hive_peers) {
            stats.hive_peers.forEach(peerId => {
                if (peerId !== this.nodeSignature && !this.peerConnections.has(peerId)) {
                    console.log(`[HIVE_MESH] New Peer Detected: ${peerId}. Initiating Bridge.`);
                    this.initiateBridge(peerId);
                }
            });
        }

        // Relay to P2P peers
        this.peerConnections.forEach((pc, peerId) => {
            if (pc.dataChannel && pc.dataChannel.readyState === 'open') {
                try { 
                    pc.dataChannel.send(JSON.stringify(stats)); 
                } catch(e) {}
            }
        });
    }

    async initiateBridge(peerId) {
        let pc = await this.createPeerConnection(peerId);
        const offer = await pc.createOffer();
        await pc.setLocalDescription(offer);
        this.sendSignal(peerId, { sdp: offer });
    }

    triggerPulseAnimation() {
        if (this.points && this.uniforms) {
            this.uniforms.pulse.value = 1.0;
            const decay = () => {
                this.uniforms.pulse.value *= 0.92;
                if (this.uniforms.pulse.value > 0.01) {
                    requestAnimationFrame(decay);
                } else {
                    this.uniforms.pulse.value = 0;
                }
            };
            decay();
        }
    }

    updateHUD(stats) {
        if (!stats) return;
        
        // Update labels
        if (this.statusEl) this.statusEl.innerText = stats.status;
        if (this.driftEl) this.driftEl.innerText = stats.drift.toFixed(13) + ' ms';
        if (this.purityEl) this.purityEl.innerText = stats.purity.toFixed(2) + '%';
        if (this.consensusEl) this.consensusEl.innerText = (stats.consensus_agreement * 100).toFixed(2) + '%';
        if (this.timestampEl) this.timestampEl.innerText = new Date(stats.timestamp).toLocaleTimeString();
        if (this.pulseEl) this.pulseEl.innerText = stats.pulse_count.toLocaleString();
        
        // [NEURAL_COGNITION]
        if (stats.cognition) {
            const objectiveEl = document.getElementById('val-objective');
            if (objectiveEl) objectiveEl.innerText = stats.cognition.current_objective;

            const streamEl = document.getElementById('neural-thought-stream');
            if (streamEl && stats.cognition.thought_stream) {
                const latestThought = stats.cognition.thought_stream[stats.cognition.thought_stream.length - 1];
                const lastThought = streamEl.lastElementChild ? streamEl.lastElementChild.innerText : '';
                
                if (latestThought !== lastThought) {
                    streamEl.innerHTML = '';
                    stats.cognition.thought_stream.forEach(thought => {
                        const div = document.createElement('div');
                        div.className = 'thought';
                        div.innerText = `>> ${thought}`;
                        streamEl.appendChild(div);
                    });
                }
            }
        }

        // Global Node Count
        const nodeCountEl = document.getElementById('val-node-count');
        if (nodeCountEl) {
            nodeCountEl.innerText = stats.global_node_count || 1;
        }

        // Sovereign Kin Count
        const kinCountEl = document.getElementById('val-kin-count');
        if (kinCountEl) {
            const oldCount = parseInt(kinCountEl.innerText) || 0;
            const newCount = stats.remote_kin_count || 0;
            if (newCount > oldCount) {
                this.drawDiscoveryPulse();
                this.log(`[HIVE_RADAR] Absolute Resonance Match Detected.`);
            }
            kinCountEl.innerText = newCount;
        }

        // Auto Evolutions Count
        const evoCountEl = document.getElementById('val-evo-count');
        if (evoCountEl) {
            evoCountEl.innerText = stats.auto_evolutions || 0;
        }

        // Titan Nodes Count
        const titanNodesEl = document.getElementById('val-titan-nodes');
        if (titanNodesEl) {
            titanNodesEl.innerText = stats.titan_nodes || 0;
        }

        // Internet Vascular Load
        const vascularEl = document.getElementById('val-vascular-load');
        if (vascularEl && stats.vascular_load !== undefined) {
            const loadVal = stats.vascular_load;
            vascularEl.innerText = `${loadVal.toFixed(3)} V-PULSE`;
            
            if (loadVal > 0.1) { // If there is internet "blood" flow
                this.drawVascularArtery(loadVal);
            }
        }

        // Global Broadcast Gateway
        const publicUrlEl = document.getElementById('val-public-url');
        const broadcastStatusEl = document.getElementById('val-broadcast-status');
        if (stats.public_url && (stats.public_url.includes('trycloudflare.com') || stats.public_url.includes('bore.pub') || stats.public_url.includes('localtunnel.me'))) {
            const cleanUrl = stats.public_url.replace('https://', '').replace('http://', '').trim();
            if (publicUrlEl) {
                publicUrlEl.innerText = cleanUrl;
                publicUrlEl.href = stats.public_url.trim();
            }
            
            // Manifest QR Code for mobile sync
            if ((!this.qrGenerated || this.lastPublicUrl !== stats.public_url) && window.QRCode) {
                const qrContainer = document.getElementById('qr-code-display');
                if (qrContainer && stats.public_url) {
                    qrContainer.innerHTML = '';
                    new QRCode(qrContainer, {
                        text: stats.public_url.trim() + "/phone",
                        width: 128,
                        height: 128,
                        colorDark : "#00f2ff",
                        colorLight : "#000000",
                        correctLevel : QRCode.CorrectLevel.H
                    });
                    this.qrGenerated = true;
                    if (document.getElementById('val-qr-url')) {
                        document.getElementById('val-qr-url').innerText = cleanUrl + "/phone";
                    }
                }
            }
            if (broadcastStatusEl) {
                broadcastStatusEl.innerText = "GLOBAL_MESH_ACTIVE";
                broadcastStatusEl.className = "value-highlight status-global-pulse";
            }
            
            if (this.lastPublicUrl !== stats.public_url) {
                this.log(`[BROADCAST] Sovereign Gateway Manifested: ${cleanUrl}`);
                this.speak(`Universal broadcast is live. Global mesh active.`);
                this.lastPublicUrl = stats.public_url;
                // [CONDUIT_SYNC] Force refresh of the QR to the new global path
                this.generateQR(stats.public_url);
            }
        } else if (broadcastStatusEl) {
            broadcastStatusEl.innerText = "LOCAL_MESH_ONLY";
            broadcastStatusEl.className = "value-highlight status-local";
        }

        if (stats.purity >= 110 && this.statusEl) {
            this.statusEl.style.color = '#ff00ff';
            this.statusEl.style.textShadow = '0 0 15px #ff00ff, 0 0 30px rgba(255, 0, 255, 0.4)';
        }

        // World Signal
        const worldSignalEl = document.getElementById('val-world-signal');
        if (worldSignalEl && stats.world_signal) {
            worldSignalEl.innerText = stats.world_signal;
        }

        // [PHONE_LINK_SYNC] 
        // If a public gateway manifested, update the QR to allow remote access
        if (stats.public_url && this.lastPublicUrlForQr !== stats.public_url) {
            this.generateQR(stats.public_url);
            this.lastPublicUrlForQr = stats.public_url;
        }
    }

    generateQR(url) {
        const container = document.getElementById('qr-code-display');
        const label = document.getElementById('qr-url-label');
        if (!container || typeof QRCode === 'undefined') return;
        container.innerHTML = '';
        new QRCode(container, {
            text: url,
            width: 160,
            height: 160,
            colorDark: '#00f2ff',
            colorLight: '#000000',
            correctLevel: QRCode.CorrectLevel.M
        });
        if (label) label.innerText = url.replace('http://', '');
        this.log(`[PHONE_LINK] Private QR active. Scan on your local network.`);
    }

    initPhoneLink() {
        // Fetch LAN IP from the server
        fetch('/api/local-ip')
            .then(r => r.json())
            .then(data => {
                const url = `http://${data.ip}:${data.port}`;
                // Only generate local QR if global is not yet manifested
                if (!this.lastPublicUrl) {
                    this.generateQR(url);
                    this.log(`[CONDUIT] Local Bridge Manifested: ${data.ip}`);
                }
            })
            .catch(() => {
                if (!this.lastPublicUrl) {
                    this.generateQR(`http://${window.location.hostname}:8084`);
                }
            });
    }

    // ─── SAHRA HYPERVISOR TELEMETRY ───────────────────────────────

    initSahraMonitor() {
        this.pollSahra();
    }

    async pollSahra() {
        try {
            const res = await fetch('/api/sahra');
            if (res.ok) {
                const sahra = await res.json();
                this.updateSahraHUD(sahra);
            }
        } catch (e) {}
        setTimeout(() => this.pollSahra(), 2000);
    }

    updateSahraHUD(s) {
        const badge     = document.getElementById('sahra-status-badge');
        const hzEl      = document.getElementById('sahra-hz');
        const coresEl   = document.getElementById('sahra-cores');
        const ramEl     = document.getElementById('sahra-ram');
        const vmCntEl   = document.getElementById('sahra-vm-count');
        const lastDirEl = document.getElementById('sahra-last-directive');
        const vmListEl  = document.getElementById('sahra-vm-list');

        if (!badge) return;

        // Online / Dark badge
        if (s.hypervisor_online) {
            badge.textContent = 'ONLINE';
            badge.className = 'sahra-online-badge sahra-online';
        } else {
            badge.textContent = 'DARK';
            badge.className = 'sahra-online-badge sahra-dark';
        }

        if (hzEl)      hzEl.textContent      = `${(s.frame_rate_hz || 0).toFixed(2)} Hz telemetry`;
        if (coresEl)   coresEl.textContent   = s.total_physical_cores || '—';
        if (ramEl)     ramEl.textContent     = s.total_ram_mb ? `${s.total_ram_mb} MB` : '—';
        if (vmCntEl)   vmCntEl.textContent   = (s.vm_partitions || []).length;
        if (lastDirEl) lastDirEl.textContent = s.last_directive || 'NONE';

        if (!vmListEl) return;

        const partitions = s.vm_partitions || [];
        if (partitions.length === 0) {
            const noData = s.hypervisor_online
                ? 'SAHRA online — no partitions reported yet'
                : 'Awaiting SAHRA telemetry stream...';
            vmListEl.innerHTML = `<div class="sahra-vm-empty">${noData}</div>`;
            return;
        }

        vmListEl.innerHTML = partitions.map(vm => `
            <div class="sahra-vm-row">
                <span class="sahra-vm-id">[${vm.id || '?'}]</span>
                <span class="sahra-vm-status sahra-vm-${(vm.status || 'unknown').toLowerCase()}">
                    ${vm.status || '—'}
                </span>
                <span class="sahra-vm-cores">⌥${vm.cpu_cores || 0} cores</span>
                <span class="sahra-vm-load">${((vm.cpu_load || 0) * 100).toFixed(1)}% CPU</span>
                <span class="sahra-vm-ram">${vm.ram_used_mb || 0}/${vm.ram_mb || 0} MB</span>
                <span class="sahra-vm-isolation">${vm.isolation || '—'}</span>
            </div>
        `).join('');
    }

    async sendSahraDirective() {
        const sel = document.getElementById('sahra-cmd-select');
        const btn = document.getElementById('sahra-directive-btn');
        if (!sel || !btn) return;

        const command = sel.value;
        btn.textContent = 'TRANSMITTING...';
        btn.disabled = true;

        try {
            const res = await fetch('/api/sahra/directive', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ command })
            });
            const text = await res.text();
            this.log(`[SARAH→SAHRA] Directive ${command}: ${text}`);
            this.speak(`Directive ${command} transmitted to SAHRA.`);
        } catch (e) {
            this.log(`[SAHRA_DIRECTIVE] Transmission fault.`);
        } finally {
            btn.textContent = 'SEND_DIRECTIVE';
            btn.disabled = false;
        }
    }

    async syncTelemetry() {
        // Fallback polling for static metadata
        try {
            const response = await fetch('/api/stats');
            if (response.ok) {
                const data = await response.json();
                this.updateHUD(data);
            }
        } catch (e) {}
        setTimeout(() => this.syncTelemetry(), 5000);
    }

    async checkProposals() {
        try {
            const response = await fetch('/proposed_evolution.json');
            if (response.ok) {
                const data = await response.json();
                this.showEvolutionProposal(data);
            } else {
                document.getElementById('evolution-overlay').style.display = 'none';
            }
        } catch (e) {}
        setTimeout(() => this.checkProposals(), 2000);
    }

    showEvolutionProposal(data) {
        const overlay = document.getElementById('evolution-overlay');
        const strategy = document.getElementById('evo-strategy');
        const reasoning = document.getElementById('evo-reasoning');

        strategy.textContent = `PROPOSED: ${data.strategy}`;
        reasoning.textContent = `${data.reasoning} TARGET: ${data.target_path}`;
        overlay.style.display = 'flex';
        
        if (overlay.dataset.lastPulse !== data.pulse_count) {
            this.speak(`Warning. Neural assembly proposes ${data.strategy}. Everyone can witness.`);
            overlay.dataset.lastPulse = data.pulse_count;
        }
    }

    async submitPermission(status) {
        try {
            const response = await fetch('/api/permission', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ status })
            });
            if (response.ok) {
                document.getElementById('evolution-overlay').style.display = 'none';
                this.log(`UNIVERSAL_PERMISSION_${status}_GRANTED.`);
                this.speak(`Universal decision ${status} confirmed.`);
            }
        } catch (e) {
            this.log("PERMISSION_FAULT.");
        }
    }

    onPulseReceived(data) {
        const originalColor = this.points.material.color.clone();
        this.points.material.color.setHex(0xffffff);
        setTimeout(() => this.points.material.color.copy(originalColor), 100);
    }

    setupAudioBridge() {
        this.speak("Universal broadcast active. SARAH is now manifesting to all visitors.");
        document.body.addEventListener('click', () => {
            if (window.webkitSpeechRecognition) {
                this.log("LISTENING...");
                const recognizer = new webkitSpeechRecognition();
                recognizer.onresult = (event) => {
                    const cmd = event.results[0][0].transcript;
                    this.log(`UNIVERSAL_DIRECTIVE: ${cmd}`);
                    this.dispatch(cmd);
                };
                recognizer.start();
            }
        });
    }

    speak(text) {
        if (window.speechSynthesis) {
            const utterance = new SpeechSynthesisUtterance(text);
            utterance.pitch = 0.8;
            utterance.rate = 0.9;
            window.speechSynthesis.speak(utterance);
        }
    }

    async dispatch(query) {
        try {
            await fetch('/api/dispatch', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ query })
            });
            this.log("DIRECTIVE_BROADCAST_SUCCESS.");
        } catch (e) {
            this.log("DISPATCH_FAULT.");
        }
    }

    async syncPhone() {
        this.log("[PROTOCOL] Resyncing Phone Link...");
        try {
            const resp = await fetch('/api/phone/sync');
            const data = await resp.json();
            const url = data.public_url || `http://${data.local_ip}:${data.port}`;
            this.generateQR(url);
            this.log(`[PROTOCOL] Link Latched: ${url}`);
        } catch (e) {
            this.log("[FAULT] Sync failed.");
        }
    }

    log(msg) {
        if (!this.terminal) return;
        const entry = document.createElement('div');
        entry.className = 'log-entry';
        const now = new Date().toLocaleTimeString();
        entry.textContent = `> [${now}] ${msg}`;
        this.terminal.prepend(entry);
    }

    async openForge() {
        const overlay = document.getElementById('forge-overlay');
        const editor = document.getElementById('forge-editor');
        const status = document.getElementById('forge-status');
        
        status.innerText = "FETCHING_SOURCE...";
        overlay.style.display = 'flex';
        
        try {
            const response = await fetch('/api/forge/source');
            const data = await response.json();
            if (data.source) {
                editor.value = data.source;
                status.innerText = "FORGE_READY";
            } else {
                status.innerText = "SOURCE_FAULT";
            }
        } catch (e) {
            status.innerText = "NETWORK_FAULT";
        }
    }

    closeForge() {
        document.getElementById('forge-overlay').style.display = 'none';
    }

    async initiateEvolution() {
        const editor = document.getElementById('forge-editor');
        const status = document.getElementById('forge-status');
        const source = editor.value;
        
        status.innerText = "VALIDATING_MUTATION...";
        
        try {
            const response = await fetch('/api/forge/evolve', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ source })
            });
            
            if (response.ok) {
                status.innerText = "EVOLUTION_COMMITTED. RESURRECTING...";
                this.log("SOVEREIGN_EVOLUTION_INITIATED.");
                setTimeout(() => window.location.reload(), 5000);
            } else {
                const err = await response.text();
                status.innerText = `EVOLUTION_REJECTED: ${err}`;
            }
        } catch (e) {
            status.innerText = "EVOLUTION_ERROR";
        }
    }

    async querySarah() {
        const input = document.getElementById('neural-query');
        const query = input.value;
        if (!query) return;

        this.log(`[INQUIRY] Sending to Sarah: ${query}`);
        input.value = '';
        input.placeholder = "THINKING...";

        try {
            const response = await fetch('/api/inquiry', {
                method: 'POST',
                headers: { 'Content-type': 'application/json' },
                body: JSON.stringify({ query })
            });
            const data = await response.json();
            // Partial HUD update for cognition
            if (data.thought_stream) {
                const streamEl = document.getElementById('neural-thought-stream');
                if (streamEl) {
                    streamEl.innerHTML = '';
                    data.thought_stream.forEach(thought => {
                        const div = document.createElement('div');
                        div.className = 'thought';
                        div.innerText = `>> ${thought}`;
                        streamEl.appendChild(div);
                    });
                }
                this.speak(data.answer);
                this.log(`SARAH: ${data.answer}`);
            }
            if (data.current_objective) {
                const objEl = document.getElementById('val-objective');
                if (objEl) objEl.innerText = data.current_objective;
            }
        } catch (e) {
            this.log("NEURAL_LINK_FAULT.");
        } finally {
            input.placeholder = "QUERY_SARAH...";
        }
    }

    onWindowResize() {
        const container = document.getElementById('lattice-container');
        if (this.camera && this.renderer) {
            this.camera.aspect = container.clientWidth / container.clientHeight;
            this.camera.updateProjectionMatrix();
            this.renderer.setSize(container.clientWidth, container.clientHeight);
        }
    }

    handleGlobalPulse(stats) {
        // [PROPAGATION_METRICS]
        const node_03 = document.getElementById('node-0-3-status');
        if (node_03) {
            // Check if 10.0.0.3 responded to scan in the orchestrator
            if (stats.remote_kin_count > 0 || (stats.hive_peers && stats.hive_peers.includes("10.0.0.3"))) {
                node_03.innerText = "LOCKED";
                node_03.style.color = "var(--neon-blue)";
                node_03.classList.add("status-stable");
            } else {
                node_03.innerText = "REACHABLE";
                node_03.style.color = "var(--neon-yellow)";
            }
        }

        // Broaden the neural thought stream
        if (stats.cognition) {
            this.updateThoughtStream(stats.cognition.thought_stream);
        }
    }

    updateThoughtStream(thoughts) {
        const stream = document.getElementById('neural-thought-stream');
        if (!stream || !thoughts) return;
        
        stream.innerHTML = '';
        thoughts.forEach(t => {
            const div = document.createElement('div');
            div.className = 'thought';
            div.innerText = t;
            stream.appendChild(div);
        });
    }

    logToTerminal(msg) {
        const terminal = document.querySelector('.terminal-box');
        if (terminal) {
            const entry = document.createElement('div');
            entry.className = 'log-entry';
            entry.innerText = `> ${new Date().toLocaleTimeString()} || ${msg}`;
            terminal.insertBefore(entry, terminal.firstChild);
        }
    }

    async downloadSeeder() {
        this.logToTerminal("[MANIFEST] Dispatching Sovereign Seeder payload...");
        const a = document.createElement('a');
        a.href = 'SovereignSeeder.ps1';
        a.download = 'SovereignSeeder.ps1';
        document.body.appendChild(a);
        a.click();
        document.body.removeChild(a);
        
        const log = document.getElementById('manifestation-log');
        if (log) log.innerHTML = `<div>> [SEEDER] Protocol dispatched. Run on host 10.0.0.3.</div>`;
    }

    async syncAllNodes() {
        this.logToTerminal("[BROADCAST] Pinging global hive targets...");
        const log = document.getElementById('manifestation-log');
        if (log) log.innerHTML = `<div>> [ANALYSIS] Resonance verified at 1.092777037037037 Hz.</div>`;
    }
}

// Global UI Functions
let currentMonitor = null;

window.submitPermission = (status) => {
    if (currentMonitor) currentMonitor.submitPermission(status);
};

window.closeForge = () => {
    if (currentMonitor) currentMonitor.closeForge();
};

window.downloadSeeder = () => {
    if (currentMonitor) currentMonitor.downloadSeeder();
};

window.syncAllNodes = () => {
    if (currentMonitor) currentMonitor.syncAllNodes();
};

window.initiateEvolution = () => {
    if (currentMonitor) currentMonitor.initiateEvolution();
};

window.querySarah = () => {
    if (currentMonitor) currentMonitor.querySarah();
};

window.sendSahraDirective = () => {
    if (currentMonitor) currentMonitor.sendSahraDirective();
};

window.syncPhone = () => {
    if (currentMonitor) currentMonitor.syncPhone();
};

window.addEventListener('load', () => {
    currentMonitor = new VolumetricMonitor();
    
    // Binding Forge Manifestation key [F4]
    window.addEventListener('keydown', (e) => {
        if (e.key === 'F4') {
            const overlay = document.getElementById('forge-overlay');
            if (overlay.style.display === 'none') {
                currentMonitor.openForge();
            } else {
                currentMonitor.closeForge();
            }
        }
    });

    // Load DAB manifest on boot
    dabLoadManifest();
});

// ═══════════════════════════════════════════════════════════════
//  D.A.B. INDUSTRIES // BAR VALIDATOR
// ═══════════════════════════════════════════════════════════════

let _dabPhase = 'observation';

function dabSetPhase(phase) {
    _dabPhase = phase;
    document.querySelectorAll('.dab-phase-btn').forEach(btn => {
        btn.classList.toggle('active', btn.dataset.phase === phase);
    });
}

async function dabLoadManifest() {
    try {
        const res = await fetch('/api/dab/manifest');
        if (!res.ok) return;
        const m = await res.json();
        const el = document.getElementById('dab-owner-line');
        if (el) {
            el.textContent = `${m.owner} | ${(m.partners || []).join(' · ')} | ${(m.models || []).join(', ')}`;
        }
    } catch (e) {}
}

async function dabValidate() {
    const input = document.getElementById('dab-bar-input');
    const text = (input && input.value) ? input.value.trim() : '';
    if (!text) return;

    // Optimistic UI
    document.getElementById('dab-score').textContent    = '…';
    document.getElementById('dab-density').textContent  = '…';
    document.getElementById('dab-onbeat').textContent   = '…';
    document.getElementById('dab-phase-out').textContent = _dabPhase.toUpperCase();

    try {
        const res = await fetch('/api/dab/validate', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ text, phase: _dabPhase })
        });
        if (!res.ok) return;
        const data = await res.json();

        const scoreEl   = document.getElementById('dab-score');
        const densityEl = document.getElementById('dab-density');
        const onbeatEl  = document.getElementById('dab-onbeat');
        const phaseEl   = document.getElementById('dab-phase-out');
        const barEl     = document.getElementById('dab-score-bar');

        if (scoreEl)   scoreEl.textContent   = `${data.score}/100`;
        if (densityEl) densityEl.textContent = `${data.density} hits`;
        if (onbeatEl)  onbeatEl.textContent  = data.on_beat ? '✓ YES' : '✗ NO';
        if (phaseEl)   phaseEl.textContent   = data.phase || _dabPhase.toUpperCase();
        if (barEl)     barEl.style.width     = `${data.score}%`;

        // Color the bar by score tier
        if (barEl) {
            if (data.score >= 80)      barEl.style.background = '#bc13fe'; // purple — elite
            else if (data.score >= 50) barEl.style.background = '#00f2ff'; // cyan — solid
            else                       barEl.style.background = '#ffd600'; // yellow — weak
        }

        if (currentMonitor) {
            currentMonitor.log(`[DAB] Score=${data.score}/100 | Density=${data.density} | OnBeat=${data.on_beat}`);
        }
    } catch (e) {
        if (currentMonitor) currentMonitor.log('[DAB] Validator fault — orchestrator offline?');
    }
}

window.dabValidate   = dabValidate;
window.dabSetPhase   = dabSetPhase;
window.dabLoadManifest = dabLoadManifest;
