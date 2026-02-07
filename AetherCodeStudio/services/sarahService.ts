
import { ModelType } from "../types";
import { SYSTEM_INSTRUCTION } from "../constants";

const GATEWAY_URL = "http://127.0.0.1:8001/api/chat";

export const streamLocalSarahResponse = async (
    model: ModelType,
    history: { role: string; text: string }[],
    currentFileContext: string,
    onChunk: (text: string) => void
): Promise<string> => {

    const contextMessage = `\n\n[CONTEXT - CURRENTLY OPEN FILE]\n${currentFileContext}\n\n[END CONTEXT]`;
    const lastMsg = history[history.length - 1];
    const fullPrompt = lastMsg.text + contextMessage;

    try {
        const response = await fetch(GATEWAY_URL, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({
                message: fullPrompt,
                user_id: "architect",
                model: model
            })
        });

        if (!response.ok) {
            throw new Error(`Gateway Error: ${response.statusText}`);
        }

        const data = await response.json();
        const result = data.content || "SYSTEM_ERROR: No Response Content";

        // Local responses are currently non-streaming in our gateway
        // We simulate a chunk for the UI
        onChunk(result);
        return result;

    } catch (error) {
        console.error("Sarah Local Bridge Error:", error);
        return `[LOCALBREACH] Sarah Backend Unreachable. Ensure Python Gateway is active on Port 8001.`;
    }
};
