
import { Message } from '../types';

const GATEWAY_ENDPOINT = "http://localhost:8001/api/chat";
const MODEL_NAME = "aeris";

export async function generateLocalResponse(
    messages: Message[],
    onChunk?: (text: string) => void
): Promise<string> {
    try {
        const response = await fetch(GATEWAY_ENDPOINT, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({
                model: MODEL_NAME,
                message: messages[messages.length - 1].content,
                user_id: "architect_local"
            })
        });

        if (!response.ok) throw new Error("Sovereign Gateway Offline");

        const data = await response.json();
        return data.content || "CORRUPTED_PACKET";
    } catch (err) {
        console.error("Local Inference Failed:", err);
        return "NEURAL_CORE_UNREACHABLE";
    }
}

export async function checkLocalStatus(): Promise<boolean> {
    try {
        const res = await fetch("http://localhost:8001/api/chat/status");
        return res.ok;
    } catch {
        return false;
    }
}
