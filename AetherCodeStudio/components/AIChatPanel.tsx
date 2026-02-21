
import React, { useState, useRef, useEffect } from 'react';
import { ChatMessage, ModelType } from '../types';
import { Icons } from './Icon';
import { streamLocalSarahResponse } from '../services/sarahService';

interface AIChatPanelProps {
    messages: ChatMessage[];
    onSendMessage: (text: string, role: 'user' | 'model') => void;
    model: ModelType;
    currentFileContent: string;
}

export const AIChatPanel: React.FC<AIChatPanelProps> = ({
    messages, onSendMessage, model, currentFileContent
}) => {
    const [input, setInput] = useState('');
    const [isLoading, setIsLoading] = useState(false);
    const messagesEndRef = useRef<HTMLDivElement>(null);

    const scrollToBottom = () => {
        messagesEndRef.current?.scrollIntoView({ behavior: "smooth" });
    };

    useEffect(() => {
        scrollToBottom();
    }, [messages, isLoading]);

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();
        if (!input.trim() || isLoading) return;

        const userText = input;
        setInput('');
        onSendMessage(userText, 'user');
        setIsLoading(true);

        try {
            // Map history for context
            const history = messages.map(m => ({
                role: m.role,
                text: m.text
            }));
            history.push({ role: 'user', text: userText });

            const responseText = await streamLocalSarahResponse(
                model,
                history,
                currentFileContent,
                (chunk) => {
                    // Streaming placeholder if needed
                }
            );

            onSendMessage(responseText, 'model');

        } catch (error) {
            console.error(error);
            const errMessage = error instanceof Error ? error.message : "Unknown Error";
            // Ensure specific resonance error is displayed if offline
            onSendMessage(`Error: Neural Link Unstable (${errMessage})`, 'model');
        } finally {
            setIsLoading(false);
        }
    };

    return (
        <div className="h-full flex flex-col bg-gray-900 border-l border-gray-800 w-96">
            {/* Header */}
            <div className="p-3 border-b border-gray-800 flex justify-between items-center bg-gray-950">
                <div className="flex items-center gap-2 text-gray-200 font-semibold text-sm">
                    <Icons.Cpu />
                    <span>System Core</span>
                </div>
                <div className="flex items-center gap-2">
                    <span className="text-xs text-gray-500 uppercase px-2 py-0.5 border border-gray-800 rounded">
                        {model === ModelType.FLASH ? 'Type-F' : 'Type-P'}
                    </span>
                </div>
            </div>

            {/* Messages */}
            <div className="flex-1 overflow-y-auto p-4 space-y-4">
                {messages.length === 0 && (
                    <div className="text-center text-gray-500 mt-10 text-sm">
                        <p>System Online.</p>
                        <p className="text-xs mt-2 opacity-60">Awaiting instructions.</p>
                    </div>
                )}

                {messages.map(msg => (
                    <div key={msg.id} className={`flex flex-col ${msg.role === 'user' ? 'items-end' : 'items-start'}`}>
                        <div className={`max-w-[90%] rounded-lg p-3 text-sm whitespace-pre-wrap ${msg.role === 'user'
                                ? 'bg-blue-600 text-white'
                                : 'bg-gray-800 text-gray-200 border border-gray-700'
                            }`}>
                            {msg.text}
                        </div>
                        <span className="text-[10px] text-gray-600 mt-1 uppercase">{msg.role}</span>
                    </div>
                ))}

                {isLoading && (
                    <div className="flex justify-start">
                        <div className="bg-gray-800 border border-gray-700 rounded-lg p-3 text-sm text-gray-400 animate-pulse">
                            Processing...
                        </div>
                    </div>
                )}
                <div ref={messagesEndRef} />
            </div>

            {/* Input */}
            <form onSubmit={handleSubmit} className="p-3 bg-gray-950 border-t border-gray-800">
                <div className="relative">
                    <input
                        type="text"
                        value={input}
                        onChange={(e) => setInput(e.target.value)}
                        placeholder="Execute command..."
                        className="w-full bg-gray-900 border border-gray-700 rounded-md py-2 pl-3 pr-10 text-sm text-white focus:outline-none focus:border-blue-500 placeholder-gray-600"
                        disabled={isLoading}
                    />
                    <button
                        type="submit"
                        disabled={isLoading || !input.trim()}
                        className="absolute right-2 top-1.5 text-blue-500 hover:text-blue-400 disabled:opacity-50 disabled:cursor-not-allowed"
                    >
                        <Icons.Send />
                    </button>
                </div>
            </form>
        </div>
    );
};
