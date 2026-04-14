export interface FileNode {
  name: string;
  language: string;
  content: string;
  isOpen?: boolean;
}

export interface ChatMessage {
  role: 'user' | 'assistant';
  content: string;
  timestamp: number;
}
