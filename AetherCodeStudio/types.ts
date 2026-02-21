export interface File {
  id: string;
  name: string;
  language: string;
  content: string;
}

export enum ModelType {
  FLASH = 'sarah-neural-v1', // Sovereign Core
  PRO = 'sarah-logic-v2',    // Advanced Logic
}

export interface ChatMessage {
  id: string;
  role: 'user' | 'model' | 'system';
  text: string;
  timestamp: number;
}

export type ExtensionSource = 'ai' | 'marketplace' | 'github';

export interface Extension {
  id: string;
  name: string;
  description: string;
  version: string;
  author?: string;
  isEnabled: boolean;
  source: ExtensionSource;
  code?: string; // Executable logic (for AI/GitHub extensions)
  downloadUrl?: string; // For marketplace extensions
  iconUrl?: string;
}

export interface Command {
  id: string;
  title: string;
  action: () => void;
}

export type ViewMode = 'explorer' | 'extensions' | 'search';

export interface AppState {
  files: File[];
  activeFileId: string | null;
  model: ModelType;
  chatHistory: ChatMessage[];
  extensions: Extension[];
  activeView: ViewMode;
  isSettingsOpen: boolean;
  isPreviewOpen: boolean;
  isCommandPaletteOpen: boolean;
}

export type FileAction =
  | { type: 'ADD_FILE'; payload: { name: string; language: string; content?: string } }
  | { type: 'UPDATE_FILE'; payload: { id: string; content: string } }
  | { type: 'DELETE_FILE'; payload: string }
  | { type: 'SET_ACTIVE_FILE'; payload: string }
  | { type: 'SET_MODEL'; payload: ModelType }
  | { type: 'ADD_MESSAGE'; payload: ChatMessage }
  | { type: 'CLEAR_MESSAGES' }
  | { type: 'TOGGLE_SETTINGS' }
  | { type: 'TOGGLE_PREVIEW' }
  | { type: 'TOGGLE_COMMAND_PALETTE' }
  | { type: 'SET_VIEW'; payload: ViewMode }
  | { type: 'INSTALL_EXTENSION'; payload: Extension }
  | { type: 'TOGGLE_EXTENSION'; payload: string };
