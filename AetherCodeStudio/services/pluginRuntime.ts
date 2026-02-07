import { Extension } from '../types';

/**
 * A safe-ish runtime to execute extension code.
 */
export const executeExtension = (extension: Extension, context: any) => {
  if (!extension.isEnabled) return;
  
  // Marketplace extensions in this demo environment are metadata-only
  // In a full implementation, this would handle WebAssembly or WebContainer execution.
  if (extension.source === 'marketplace' && !extension.code) {
      // Log for demo purposes
      // console.log(`[PluginRuntime] Loaded marketplace metadata: ${extension.name}`);
      return;
  }

  try {
    const run = new Function('api', extension.code || '');
    run(context);
    console.log(`[PluginRuntime] Executed: ${extension.name}`);
  } catch (error) {
    console.error(`[PluginRuntime] Error in ${extension.name}:`, error);
  }
};

/**
 * Generates an extension based on a query (Simulated AI Generation)
 */
export const generateExtension = (query: string): Extension => {
  const id = Date.now().toString();
  
  // This logic simulates an AI creating a script that uses the environment's own API
  let code = `
    console.log("AI Extension loaded: ${query}");
    api.registerCommand({
        id: 'cmd_${id}',
        title: 'Run ${query}',
        action: () => {
            api.showNotification('Executing ${query} logic...');
        }
    });
  `;

  if (query.toLowerCase().includes('format')) {
      code = `
        api.registerCommand({
            id: 'format_doc',
            title: 'Auto-Format Code',
            action: () => {
                const file = api.getActiveFile();
                if (file) {
                    // Simple mock formatter
                    const newContent = file.content.split('\\n').map(l => l.trim()).join('\\n');
                    api.modifyFile(file.id, newContent);
                    api.showNotification('Document formatted (Mock)');
                }
            }
        });
      `;
  } else if (query.toLowerCase().includes('theme')) {
      code = `
         api.registerCommand({
            id: 'dark_mode_plus',
            title: 'Toggle Deep Dark Mode',
            action: () => {
               document.body.style.backgroundColor = '#000000';
               api.showNotification('Deep Dark Mode Engaged');
            }
         });
      `;
  }

  return {
    id,
    name: query.charAt(0).toUpperCase() + query.slice(1) + " (Neural)",
    description: `AI-Generated logic for: ${query}`,
    version: '0.0.1',
    isEnabled: true,
    source: 'ai',
    code
  };
};
