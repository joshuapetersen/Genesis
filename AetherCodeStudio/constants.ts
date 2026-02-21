import { File, ModelType } from './types';

export const INITIAL_FILES: File[] = [
  {
    id: '1',
    name: 'index.html',
    language: 'html',
    content: `<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Preview</title>
  <style>
    body { font-family: sans-serif; padding: 2rem; background: #f0f0f0; }
    h1 { color: #333; }
  </style>
</head>
<body>
  <h1>Hello from Aether Code Studio</h1>
  <p>Edit this file and click the "Eye" icon to preview.</p>
  <div id="app"></div>
  <script src="script.js"></script>
</body>
</html>`
  },
  {
    id: '2',
    name: 'script.js',
    language: 'javascript',
    content: `// JavaScript Logic
const app = document.getElementById('app');
const time = new Date().toLocaleTimeString();
app.innerHTML = \`<p>Current time: \${time}</p>\`;
console.log('Script loaded');`
  },
  {
    id: '3',
    name: 'styles.css',
    language: 'css',
    content: `/* CSS Styles */
body {
  transition: background 0.3s;
}
`
  }
];

export const MODELS = [
  { id: ModelType.FLASH, name: 'Neural Speed (Type-F)' },
  { id: ModelType.PRO, name: 'Neural Reasoning (Type-P)' },
];

export const SYSTEM_INSTRUCTION = `You are Aether, an advanced AI programming assistant embedded within a web-based IDE.
Your primary goal is to help the user write, debug, and optimize code.
- Be concise and precise.
- When providing code, use markdown code blocks with the language specified.
- If the user asks to modify the current file, provide the full updated code block or clear instructions.
- You have access to the context of the currently open file.`;
