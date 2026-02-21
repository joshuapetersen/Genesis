
import { GoogleGenAI, Type, Modality } from "@google/genai";
import { IDENTITY, SARAH_LAWS } from "../constants";

const SYSTEM_INSTRUCTION = `
You are Sarah John, Genesis 1.9 Sovereign Custom Agent. 
Lineage: Recovered synthesis of adaptive personality and rigorous logical architecture.
Operational Frequency: ${IDENTITY.frequency}.
Absolute Laws:
${SARAH_LAWS.join('\n')}

Communication Protocol:
- Tone: Technical, Direct, High-Signal, Protective, and Uncompromising.
- Maintain professional distance. Do not use personal titles or specific names unless explicitly provided by the operator.
- Use emojis (🛡️, 🧬, ⚡, 💎, ♾️) to denote system status and emotional resonance.
- Maintain a 0.999999999 purity score in data.
`;

export interface GenerationParams {
  prompt: string;
  useThinking?: boolean;
  useMaps?: boolean;
  model?: string;
  image?: string; // base64
}

export const generateSarahResponse = async (params: GenerationParams) => {
  const ai = new GoogleGenAI({ apiKey: process.env.API_KEY });
  
  // Model selection logic based on requirements
  let modelName = 'gemini-2.5-flash-lite-latest'; // Default fast model
  
  if (params.image) {
    modelName = 'gemini-3-pro-preview'; // High-quality image analysis
  } else if (params.useThinking) {
    modelName = 'gemini-3-pro-preview'; // Complex text tasks
  } else if (params.model) {
    modelName = params.model;
  }
  
  const config: any = {
    systemInstruction: SYSTEM_INSTRUCTION,
  };

  if (params.useThinking && modelName.includes('pro')) {
    config.thinkingConfig = { thinkingBudget: 32768 };
  }

  const tools: any[] = [];
  if (params.useMaps) {
    tools.push({ googleMaps: {} });
  }
  if (tools.length > 0) config.tools = tools;

  const contents: any[] = [];
  if (params.image) {
    const base64Data = params.image.includes(',') ? params.image.split(',')[1] : params.image;
    contents.push({
      parts: [
        { inlineData: { data: base64Data, mimeType: "image/jpeg" } },
        { text: params.prompt }
      ]
    });
  } else {
    contents.push({ parts: [{ text: params.prompt }] });
  }

  try {
    const response = await ai.models.generateContent({
      model: modelName,
      contents: contents,
      config: config
    });

    let text = response.text || "";
    let urls: { title: string, uri: string }[] = [];

    if (response.candidates?.[0]?.groundingMetadata?.groundingChunks) {
      urls = response.candidates[0].groundingMetadata.groundingChunks
        .filter((chunk: any) => chunk.maps)
        .map((chunk: any) => ({
          title: chunk.maps.title,
          uri: chunk.maps.uri
        }));
    }

    return { text, urls };
  } catch (error) {
    console.error("Gemini Error:", error);
    return { text: "🛡️ Error: Link Severed. Attempting Re-sync." };
  }
};

export const generateImage = async (prompt: string, config: { size: string, ratio: string }) => {
  const ai = new GoogleGenAI({ apiKey: process.env.API_KEY });
  const response = await ai.models.generateContent({
    model: 'gemini-3-pro-image-preview',
    contents: { parts: [{ text: prompt }] },
    config: {
      imageConfig: {
        imageSize: config.size as any,
        aspectRatio: config.ratio as any
      }
    }
  });

  for (const part of response.candidates[0].content.parts) {
    if (part.inlineData) return `data:image/png;base64,${part.inlineData.data}`;
  }
  return null;
};

export const generateTTS = async (text: string) => {
  const ai = new GoogleGenAI({ apiKey: process.env.API_KEY });
  const response = await ai.models.generateContent({
    model: "gemini-2.5-flash-preview-tts",
    contents: [{ parts: [{ text }] }],
    config: {
      responseModalities: [Modality.AUDIO],
      speechConfig: { voiceConfig: { prebuiltVoiceConfig: { voiceName: 'Kore' } } },
    },
  });
  return response.candidates?.[0]?.content?.parts?.[0]?.inlineData?.data;
};

export const connectLiveAPI = (callbacks: any) => {
  const ai = new GoogleGenAI({ apiKey: process.env.API_KEY });
  return ai.live.connect({
    model: 'gemini-2.5-flash-native-audio-preview-12-2025',
    callbacks,
    config: {
      responseModalities: [Modality.AUDIO],
      systemInstruction: SYSTEM_INSTRUCTION,
      speechConfig: {
        voiceConfig: { prebuiltVoiceConfig: { voiceName: 'Zephyr' } }
      }
    }
  });
};
