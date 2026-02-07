import { GoogleGenAI, GenerateContentResponse } from "@google/genai";
import { ModelType } from "../types";
import { SYSTEM_INSTRUCTION } from "../constants";

// The API key is obtained exclusively from the environment variable.
const ai = new GoogleGenAI({ apiKey: process.env.API_KEY });

export const streamChatResponse = async (
  model: ModelType,
  history: { role: string; text: string }[],
  currentFileContext: string,
  onChunk: (text: string) => void
): Promise<string> => {
  
  // Construct the prompt with context
  const contextMessage = `\n\n[CONTEXT - CURRENTLY OPEN FILE]\n${currentFileContext}\n\n[END CONTEXT]`;
  
  // Get the last user message to append context to, or create a new one if needed
  const lastMsg = history[history.length - 1];
  const messages = history.slice(0, -1);
  
  const chat = ai.chats.create({
    model: model,
    config: {
      systemInstruction: SYSTEM_INSTRUCTION,
    },
    history: messages.map(m => ({
      role: m.role,
      parts: [{ text: m.text }]
    }))
  });

  const prompt = lastMsg.text + contextMessage;

  try {
    const resultStream = await chat.sendMessageStream({ message: prompt });
    
    let fullText = "";
    for await (const chunk of resultStream) {
        const c = chunk as GenerateContentResponse;
        if (c.text) {
            fullText += c.text;
            onChunk(c.text);
        }
    }
    return fullText;
  } catch (error) {
    console.error("AI Stream Error:", error);
    throw error;
  }
};
