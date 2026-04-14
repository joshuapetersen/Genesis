import React, { useState } from 'react';
import { Search, ArrowLeft, ArrowRight, RotateCw } from 'lucide-react';

export default function BrowserApp() {
  const [url, setUrl] = useState('https://example.com');
  const [inputUrl, setInputUrl] = useState('https://example.com');

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    let finalUrl = inputUrl;
    if (!finalUrl.startsWith('http://') && !finalUrl.startsWith('https://')) {
      finalUrl = 'https://' + finalUrl;
    }
    setUrl(finalUrl);
    setInputUrl(finalUrl);
  };

  return (
    <div className="h-full w-full bg-white dark:bg-gray-900 flex flex-col">
      <div className="h-12 bg-gray-100 dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700 flex items-center px-2 gap-2">
        <div className="flex items-center gap-1">
          <button className="p-1.5 hover:bg-gray-200 dark:hover:bg-gray-700 rounded-md text-gray-600 dark:text-gray-400">
            <ArrowLeft size={16} />
          </button>
          <button className="p-1.5 hover:bg-gray-200 dark:hover:bg-gray-700 rounded-md text-gray-600 dark:text-gray-400">
            <ArrowRight size={16} />
          </button>
          <button className="p-1.5 hover:bg-gray-200 dark:hover:bg-gray-700 rounded-md text-gray-600 dark:text-gray-400" onClick={() => setUrl(url)}>
            <RotateCw size={16} />
          </button>
        </div>
        
        <form onSubmit={handleSubmit} className="flex-1 flex items-center bg-white dark:bg-gray-900 rounded-full px-3 py-1.5 border border-gray-300 dark:border-gray-600 focus-within:ring-2 focus-within:ring-blue-500 focus-within:border-transparent">
          <Search size={14} className="text-gray-400 mr-2" />
          <input
            type="text"
            value={inputUrl}
            onChange={(e) => setInputUrl(e.target.value)}
            className="flex-1 bg-transparent outline-none text-sm text-gray-800 dark:text-gray-200"
            placeholder="Search or enter web address"
          />
        </form>
      </div>
      
      <div className="flex-1 bg-white">
        <iframe
          src={url}
          className="w-full h-full border-none"
          title="Browser"
          sandbox="allow-same-origin allow-scripts allow-popups allow-forms"
        />
      </div>
    </div>
  );
}
