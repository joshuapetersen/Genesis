import React, { useState } from 'react';
import { Extension } from '../types';
import { Icons } from './Icon';
import { searchOpenVSX, fetchFromGitHub } from '../services/marketplace';

interface ExtensionsPanelProps {
  extensions: Extension[];
  onInstall: (ext: Extension) => void;
  onToggle: (id: string) => void;
  onDelete: (id: string) => void;
  onGenerateAI: (query: string) => void;
}

export const ExtensionsPanel: React.FC<ExtensionsPanelProps> = ({ 
  extensions, onInstall, onToggle, onDelete, onGenerateAI 
}) => {
  const [activeTab, setActiveTab] = useState<'marketplace' | 'ai' | 'github'>('marketplace');
  const [query, setQuery] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const [searchResults, setSearchResults] = useState<Extension[]>([]);

  const handleSearch = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!query.trim()) return;

    setIsLoading(true);

    if (activeTab === 'marketplace') {
      const results = await searchOpenVSX(query);
      setSearchResults(results);
    } else if (activeTab === 'ai') {
      onGenerateAI(query);
      setQuery('');
    } else if (activeTab === 'github') {
      const ext = await fetchFromGitHub(query);
      if (ext) {
        onInstall(ext);
        setQuery('');
      } else {
        alert('Failed to fetch from GitHub. Ensure URL is raw or public.');
      }
    }
    
    setIsLoading(false);
  };

  const handleInstallMarketplace = (ext: Extension) => {
      // In a real web container, we would handle the VSIX buffer.
      // For this architecture, we mark it installed and hypothetically "enable" it.
      // We flag it as enabled so it shows up in the installed list.
      onInstall({ ...ext, isEnabled: true });
  };

  return (
    <div className="h-full flex flex-col bg-gray-900 border-r border-gray-800 w-80">
      
      {/* Tabs */}
      <div className="flex border-b border-gray-800">
        <button 
          onClick={() => setActiveTab('marketplace')}
          className={`flex-1 py-3 text-xs font-medium border-b-2 transition-colors ${activeTab === 'marketplace' ? 'border-blue-500 text-white' : 'border-transparent text-gray-500 hover:text-gray-300'}`}
        >
          Open VSX
        </button>
        <button 
          onClick={() => setActiveTab('ai')}
          className={`flex-1 py-3 text-xs font-medium border-b-2 transition-colors ${activeTab === 'ai' ? 'border-purple-500 text-white' : 'border-transparent text-gray-500 hover:text-gray-300'}`}
        >
          Neural Gen
        </button>
        <button 
          onClick={() => setActiveTab('github')}
          className={`flex-1 py-3 text-xs font-medium border-b-2 transition-colors ${activeTab === 'github' ? 'border-green-500 text-white' : 'border-transparent text-gray-500 hover:text-gray-300'}`}
        >
          GitHub
        </button>
      </div>

      {/* Search / Input Area */}
      <div className="p-3 border-b border-gray-800 bg-gray-950">
        <form onSubmit={handleSearch} className="relative">
          <input
            type="text"
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            placeholder={
                activeTab === 'marketplace' ? "Search Open VSX Registry..." :
                activeTab === 'ai' ? "Describe tool to generate..." :
                "Paste GitHub Raw URL..."
            }
            className="w-full bg-gray-800 text-white text-sm px-2 py-1.5 pl-8 rounded border border-gray-700 focus:outline-none focus:border-blue-500 placeholder-gray-600"
          />
          <div className="absolute left-2 top-1.5 text-gray-500">
            {activeTab === 'marketplace' ? <Icons.Search /> : 
             activeTab === 'ai' ? <Icons.Cpu /> : 
             <div className="w-4 h-4 flex items-center justify-center font-bold text-[10px]">&lt;/&gt;</div>}
          </div>
        </form>
      </div>

      <div className="flex-1 overflow-y-auto">
        {isLoading && (
             <div className="p-4 text-center border-b border-gray-800 animate-pulse bg-gray-900">
                <span className="text-xs text-blue-400">Processing Request...</span>
             </div>
        )}

        {/* Marketplace Results */}
        {activeTab === 'marketplace' && searchResults.length > 0 && (
           <div className="border-b border-gray-800">
             <div className="px-3 py-2 text-[10px] font-bold text-gray-500 bg-gray-950/50 uppercase">Marketplace Results</div>
             {searchResults.map(ext => (
               <div key={ext.id} className="p-3 border-b border-gray-800 hover:bg-gray-800/30">
                 <div className="flex gap-3">
                    {ext.iconUrl ? (
                        <img src={ext.iconUrl} alt="icon" className="w-8 h-8 rounded bg-gray-800" />
                    ) : (
                        <div className="w-8 h-8 rounded bg-gray-800 flex items-center justify-center text-gray-600 text-xs">EXT</div>
                    )}
                    <div className="flex-1 min-w-0">
                        <div className="font-medium text-gray-200 text-sm truncate">{ext.name}</div>
                        <div className="text-xs text-gray-500 truncate">{ext.author}</div>
                        <p className="text-[10px] text-gray-500 line-clamp-2 mt-1">{ext.description}</p>
                    </div>
                 </div>
                 <button 
                    onClick={() => handleInstallMarketplace(ext)}
                    className="w-full mt-2 bg-blue-900/20 text-blue-400 border border-blue-900/50 hover:bg-blue-900/40 text-xs py-1 rounded transition-colors"
                 >
                    Install
                 </button>
               </div>
             ))}
           </div>
        )}

        {/* Installed Extensions */}
        <div className="px-3 py-2 text-[10px] font-bold text-gray-500 bg-gray-950/50 uppercase sticky top-0">Installed</div>
        
        {extensions.length === 0 ? (
          <div className="p-8 text-center text-gray-600 text-xs italic">
            No extensions active.
          </div>
        ) : (
          extensions.map(ext => (
            <div key={ext.id} className="p-3 border-b border-gray-800 hover:bg-gray-800/50 transition-colors group">
              <div className="flex justify-between items-start mb-1">
                <span className="font-semibold text-gray-200 text-sm">{ext.name}</span>
                <span className={`text-[9px] px-1 rounded uppercase ${
                    ext.source === 'ai' ? 'bg-purple-900 text-purple-300' :
                    ext.source === 'github' ? 'bg-green-900 text-green-300' :
                    'bg-blue-900 text-blue-300'
                }`}>
                    {ext.source}
                </span>
              </div>
              <p className="text-xs text-gray-500 line-clamp-2 mb-2">{ext.description}</p>
              
              <div className="flex gap-2 opacity-100 transition-opacity">
                <button 
                  onClick={() => onToggle(ext.id)}
                  className={`px-2 py-0.5 text-[10px] font-medium rounded border transition-colors ${
                    ext.isEnabled 
                      ? 'bg-blue-900/30 text-blue-400 border-blue-800 hover:bg-blue-900/50' 
                      : 'bg-gray-800 text-gray-400 border-gray-700 hover:bg-gray-700'
                  }`}
                >
                  {ext.isEnabled ? 'Disable' : 'Enable'}
                </button>
                <button 
                  onClick={() => onDelete(ext.id)}
                  className="px-2 py-0.5 text-[10px] font-medium rounded border border-gray-800 text-gray-500 hover:text-red-400 hover:border-red-900 bg-gray-900"
                >
                  Uninstall
                </button>
              </div>
            </div>
          ))
        )}
      </div>
    </div>
  );
};
