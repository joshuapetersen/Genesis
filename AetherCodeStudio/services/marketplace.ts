import { Extension } from '../types';

const OPEN_VSX_API = 'https://open-vsx.org/api';

export const searchOpenVSX = async (query: string): Promise<Extension[]> => {
  try {
    const response = await fetch(`${OPEN_VSX_API}/-/search?query=${encodeURIComponent(query)}&size=10`);
    if (!response.ok) throw new Error('Marketplace unreachable');
    
    const data = await response.json();
    
    return data.objects.map((obj: any) => ({
      id: `${obj.package.namespace}.${obj.package.name}`,
      name: obj.package.displayName || obj.package.name,
      description: obj.package.description,
      version: obj.package.version,
      author: obj.package.publisher.username,
      isEnabled: false,
      source: 'marketplace',
      downloadUrl: obj.package.version ? `${OPEN_VSX_API}/${obj.package.namespace}/${obj.package.name}/${obj.package.version}/file.vsix` : undefined,
      iconUrl: obj.package.iconUrl
    }));
  } catch (error) {
    console.error('Open VSX Search Error:', error);
    return [];
  }
};

export const fetchFromGitHub = async (url: string): Promise<Extension | null> => {
  try {
    // Convert github.com blob URLs to raw.githubusercontent.com
    let rawUrl = url;
    if (url.includes('github.com') && url.includes('/blob/')) {
      rawUrl = url.replace('github.com', 'raw.githubusercontent.com').replace('/blob/', '/');
    }

    const response = await fetch(rawUrl);
    if (!response.ok) throw new Error('Failed to fetch GitHub resource');
    
    const code = await response.text();
    const fileName = url.split('/').pop() || 'github-extension';

    return {
      id: `gh-${Date.now()}`,
      name: fileName,
      description: `Imported from ${url}`,
      version: '1.0.0',
      isEnabled: true,
      source: 'github',
      code: code
    };
  } catch (error) {
    console.error('GitHub Fetch Error:', error);
    return null;
  }
};
