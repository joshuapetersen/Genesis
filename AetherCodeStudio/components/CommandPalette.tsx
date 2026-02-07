import React, { useState, useEffect, useRef } from 'react';
import { Command } from '../types';
import { Icons } from './Icon';

interface CommandPaletteProps {
  isOpen: boolean;
  onClose: () => void;
  commands: Command[];
}

export const CommandPalette: React.FC<CommandPaletteProps> = ({ isOpen, onClose, commands }) => {
  const [query, setQuery] = useState('');
  const [selectedIndex, setSelectedIndex] = useState(0);
  const inputRef = useRef<HTMLInputElement>(null);

  const filteredCommands = commands.filter(cmd => 
    cmd.title.toLowerCase().includes(query.toLowerCase())
  );

  useEffect(() => {
    if (isOpen) {
      setTimeout(() => inputRef.current?.focus(), 50);
      setQuery('');
      setSelectedIndex(0);
    }
  }, [isOpen]);

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (!isOpen) return;

      if (e.key === 'ArrowDown') {
        e.preventDefault();
        setSelectedIndex(i => Math.min(i + 1, filteredCommands.length - 1));
      } else if (e.key === 'ArrowUp') {
        e.preventDefault();
        setSelectedIndex(i => Math.max(i - 1, 0));
      } else if (e.key === 'Enter') {
        e.preventDefault();
        if (filteredCommands[selectedIndex]) {
          filteredCommands[selectedIndex].action();
          onClose();
        }
      } else if (e.key === 'Escape') {
        onClose();
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [isOpen, filteredCommands, selectedIndex, onClose]);

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 bg-black/40 z-50 flex justify-center pt-20" onClick={onClose}>
      <div 
        className="w-[600px] bg-gray-850 rounded-lg shadow-2xl border border-gray-700 flex flex-col max-h-[400px] overflow-hidden"
        onClick={e => e.stopPropagation()}
      >
        <div className="p-3 border-b border-gray-700 flex items-center gap-3">
          <span className="text-gray-400"><Icons.Search /></span>
          <input
            ref={inputRef}
            type="text"
            className="flex-1 bg-transparent text-white focus:outline-none text-lg placeholder-gray-500"
            placeholder="Type a command..."
            value={query}
            onChange={e => {
                setQuery(e.target.value);
                setSelectedIndex(0);
            }}
          />
        </div>
        
        <div className="flex-1 overflow-y-auto p-2">
            {filteredCommands.length === 0 ? (
                 <div className="p-4 text-center text-gray-500">No matching commands</div>
            ) : (
                filteredCommands.map((cmd, idx) => (
                    <button
                        key={cmd.id}
                        className={`w-full text-left px-4 py-3 rounded flex items-center justify-between group ${
                            idx === selectedIndex ? 'bg-blue-600 text-white' : 'text-gray-300 hover:bg-gray-800'
                        }`}
                        onClick={() => {
                            cmd.action();
                            onClose();
                        }}
                        onMouseEnter={() => setSelectedIndex(idx)}
                    >
                        <span>{cmd.title}</span>
                        {idx === selectedIndex && <Icons.Command />}
                    </button>
                ))
            )}
        </div>
      </div>
    </div>
  );
};
