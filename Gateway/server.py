import asyncio
import json
import logging
import os
import time
import sys
from typing import Dict, Any, List, Optional
from aiohttp import web
import aiohttp
from dotenv import load_dotenv

# Add core dir to path for imports
sys.path.append(os.getcwd())

try:
    from Sarah_Brain import SarahBrain
except ImportError:
    SarahBrain = None

load_dotenv()

# --- Logging Configuration ---
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s [%(levelname)s] %(message)s'
)
logger = logging.getLogger("SarahGateway")

class SarahGateway:
    def __init__(self, host="127.0.0.1", port=18789):
        self.host = host
        self.port = port
        self.app = web.Application()
        self.clients: Dict[str, web.WebSocketResponse] = {}
        self.protocol_version = 3
        
        # Initialize Brain
        if SarahBrain:
            logger.info("Initializing Sarah Brain for Gateway...")
            self.brain = SarahBrain()
        else:
            logger.warning("SarahBrain not found. Operating in MOCK mode.")
            self.brain = None
        
    async def start(self):
        self.app.router.add_get('/ws', self.websocket_handler)
        self.app.router.add_get('/health', self.health_handler)
        
        runner = web.AppRunner(self.app)
        await runner.setup()
        site = web.TCPSite(runner, self.host, self.port)
        await site.start()
        
        logger.info(f"🌑 Sarah Gateway ACTIVE at ws://{self.host}:{self.port}")
        
    async def health_handler(self, request):
        return web.json_response({
            "status": "healthy",
            "brain": "online" if self.brain else "mock",
            "clients": len(self.clients)
        })

    async def websocket_handler(self, request):
        ws = web.WebSocketResponse()
        await ws.prepare(request)
        
        conn_id = f"conn_{int(time.time())}_{len(self.clients)}"
        self.clients[conn_id] = ws
        logger.info(f"Client connected: {conn_id}")

        try:
            async for msg in ws:
                if msg.type == aiohttp.WSMsgType.TEXT:
                    await self.handle_message(conn_id, msg.data)
        finally:
            if conn_id in self.clients:
                del self.clients[conn_id]
            logger.info(f"Client disconnected: {conn_id}")
            
        return ws

    async def handle_message(self, conn_id: str, data: str):
        try:
            frame = json.loads(data)
            
            if frame["type"] == "req":
                method = frame.get("method")
                req_id = frame.get("id")
                
                if method == "connect":
                    await self.send_res(conn_id, req_id, {"status": "connected"})
                elif method == "agent.chat":
                    params = frame.get("params", {})
                    message = params.get("message", "")
                    
                    await self.broadcast_event("agent.thinking", {"status": "Sarah is processing..."})
                    
                    if self.brain:
                        # Call brain.chat or similar
                        # Since SarahBrain typically uses interactive_chat, we might need a non-interactive method
                        # For now, we'll use the reasoning engine or a direct chat response
                        try:
                            # Direct check of chat validity
                            if self.brain.chat:
                                response_text = self.brain.chat.generate_response(message)
                            else:
                                response_text = f"Brain Online. Resonance confirmed. Problem: {message}"
                        except Exception as e:
                            logger.error(f"Brain Error: {e}")
                            response_text = f"Resonance Divergence: {e}"
                    else:
                        response_text = f"MOCK: {message}"

                    await self.send_res(conn_id, req_id, {"response": response_text})
                    await self.broadcast_event("agent.response", {"text": response_text})
            
        except Exception as e:
            logger.error(f"Error handling message: {e}")

    async def send_res(self, conn_id: str, req_id: str, payload: Dict):
        if conn_id in self.clients:
            await self.clients[conn_id].send_json({
                "type": "res",
                "id": req_id,
                "ok": True,
                "payload": payload
            })

    async def broadcast_event(self, event: str, payload: Dict):
        for ws in self.clients.values():
            try:
                await ws.send_json({
                    "type": "event",
                    "event": event,
                    "payload": payload
                })
            except:
                pass

if __name__ == "__main__":
    gateway = SarahGateway()
    loop = asyncio.get_event_loop()
    loop.run_until_complete(gateway.start())
    loop.run_forever()
