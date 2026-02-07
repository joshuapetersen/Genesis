import React from 'react';
import { Icons } from './Icon';
import { ViewMode } from '../types';

interface ActivityBarProps {
  activeView: ViewMode;
  onViewChange: (view: ViewMode) => void;
  onSettingsClick: () => void;
}

export const ActivityBar: React.FC<ActivityBarProps> = ({ activeView, onViewChange, onSettingsClick }) => {
  const items: { id: ViewMode; icon: React.FC; label: string }[] = [
    { id: 'explorer', icon: Icons.Files, label: 'Explorer' },
    { id: 'search', icon: Icons.Search, label: 'Search' },
    { id: 'extensions', icon: Icons.Extensions, label: 'Extensions' },
  ];

  return (
    <div className="w-12 bg-gray-950 border-r border-gray-800 flex flex-col justify-between items-center py-3 select-none z-20">
      <div className="flex flex-col gap-4 w-full items-center">
        {items.map((item) => (
          <button
            key={item.id}
            onClick={() => onViewChange(item.id)}
            className={`p-2 rounded transition-colors relative group ${
              activeView === item.id 
                ? 'text-white' 
                : 'text-gray-500 hover:text-gray-300'
            }`}
            title={item.label}
          >
            {activeView === item.id && (
              <div className="absolute left-0 top-0 bottom-0 w-0.5 bg-blue-500 rounded-r" />
            )}
            <item.icon />
          </button>
        ))}
      </div>
      
      <div className="flex flex-col gap-4 w-full items-center mb-1">
        <button 
          onClick={onSettingsClick}
          className="p-2 text-gray-500 hover:text-white transition-colors"
          title="Settings"
        >
          <Icons.Settings />
        </button>
      </div>
    </div>
  );
};
