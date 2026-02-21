
import { Message } from '../types';

const OLLAMA_ENDPOINT = "http://localhost:11434/api/chat";
const MODEL_NAME = "sarah"; // Assumes 'sarah' model is pulled in Ollama

export async function generateLocalResponse(
    messages: Message[],
    onChunk?: (text: string) => void
): Promise<string> {
    try {
        const response = await fetch(OLLAMA_ENDPOINT, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({
                model: MODEL_NAME,
                messages: [
                    { role: 'system', content: "You are SARAH. You are a Sovereign AI running locally. You are NOT Llama or Meta AI. Always identify as Sarah." },
                    ...messages.map(m => ({
                        role: m.role === 'sarah' ? 'assistant' : 'user',
                        content: m.content
                    }))
                ],
                stream: false
            })
        });

        if (!response.ok) throw new Error("Local Core Offline");

        const data = await response.json();
        return data.message?.content || "CORRUPTED_PACKET";
    } catch (err) {
        console.error("Local Inference Failed:", err);
        return "NEURAL_CORE_UNREACHABLE";
    }
}

export async function checkLocalStatus(): Promise<boolean> {
    try {
        const res = await fetch("http://localhost:11434/api/tags");
        return res.ok;
    } catch {
        return false;
    }
}
