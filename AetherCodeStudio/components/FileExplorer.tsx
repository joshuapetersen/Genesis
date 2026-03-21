import React, { useState, useEffect } from 'react';
import { Icons } from './Icon';
import { fsService } from '../services/fileSystem';

interface FileExplorerProps {
  activeFileId: string | null; // Currently open file path
  onFileSelect: (path: string) => void;
  rootPath: string;
  setRootPath: (path: string) => void;
}

interface FSItem {
  name: string;
  path: string;
  is_dir: boolean;
  size: number;
}

export const FileExplorer: React.FC<FileExplorerProps> = ({
  activeFileId, onFileSelect, rootPath, setRootPath
}) => {
  const [items, setItems] = useState<FSItem[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const loadDir = async (path: string) => {
    setIsLoading(true);
    setError(null);
    try {
      const res = await fsService.listFiles(path);
      setItems(res.items);
      setRootPath(res.current_path); // Normalize path
    } catch (err) {
      console.error(err);
      setError("Failed to load directory");
    } finally {
      setIsLoading(false);
    }
  };

  useEffect(() => {
    loadDir(rootPath);
  }, [rootPath]);

  const handleItemClick = (item: FSItem) => {
    if (item.is_dir) {
      loadDir(item.path);
    } else {
      onFileSelect(item.path);
    }
  };

  const handleUp = () => {
    // Simple parent directory logic
    const parent = rootPath.split('\\').slice(0, -1).join('\\') || "C:\\";
    loadDir(parent);
  };

  return (
    <div className="h-full flex flex-col bg-gray-950 border-r border-gray-800 w-64">
      <div className="p-3 border-b border-gray-800 flex justify-between items-center gap-2">
        <span className="text-xs font-bold text-gray-400 uppercase tracking-wider truncate" title={rootPath}>
          {rootPath.split('\\').pop() || rootPath}
        </span>
        <button
          onClick={handleUp}
          className="text-gray-500 hover:text-white"
          title="Up Level"
        >
          <Icons.CornerUpLeft size={14} />
        </button>
      </div>

      <div className="flex-1 overflow-y-auto p-2 space-y-1">
        {isLoading && <div className="text-xs text-center text-gray-600 animate-pulse">Scanning Sector...</div>}
        {error && <div className="text-xs text-red-500 p-2 border border-red-900 rounded bg-red-900/10">{error}</div>}

        {!isLoading && items.map(item => (
          <div
            key={item.path}
            className={`group flex items-center gap-2 px-2 py-1.5 rounded cursor-pointer transition-colors text-sm ${activeFileId === item.path ? 'bg-gray-800 text-blue-400' : 'text-gray-400 hover:bg-gray-800/50 hover:text-gray-200'
              }`}
            onClick={() => handleItemClick(item)}
          >
            <div className="flex-shrink-0">
              {item.is_dir ? <Icons.Folder className="text-yellow-600" /> : <Icons.File />}
            </div>
            <span className="truncate">{item.name}</span>
          </div>
        ))}

        {!isLoading && items.length === 0 && (
          <div className="text-center text-xs text-gray-700 mt-4">Empty Sector</div>
        )}
      </div>

      <div className="p-2 border-t border-gray-800 text-[10px] text-gray-600 truncate px-4">
        {rootPath}
      </div>
    </div>
  );
};
