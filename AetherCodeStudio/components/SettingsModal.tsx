import React from 'react';
import { ModelType } from '../types';
import { MODELS } from '../constants';
import { Icons } from './Icon';

interface SettingsModalProps {
  isOpen: boolean;
  onClose: () => void;
  model: ModelType;
  setModel: (model: ModelType) => void;
}

export const SettingsModal: React.FC<SettingsModalProps> = ({ 
  isOpen, onClose, model, setModel 
}) => {

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 bg-black bg-opacity-70 flex items-center justify-center z-50 p-4">
      <div className="bg-gray-850 border border-gray-700 rounded-lg w-full max-w-md shadow-2xl">
        <div className="flex justify-between items-center p-4 border-b border-gray-700">
          <h2 className="text-xl font-semibold text-white flex items-center gap-2">
            <Icons.Settings /> System Configuration
          </h2>
          <button onClick={onClose} className="text-gray-400 hover:text-white">
            <Icons.X />
          </button>
        </div>
        
        <div className="p-6 space-y-6">
          <div className="space-y-2">
            <label className="block text-sm font-medium text-gray-300">
              Intelligence Core
            </label>
            <div className="grid gap-2">
              {MODELS.map((m) => (
                <button
                  key={m.id}
                  onClick={() => setModel(m.id)}
                  className={`flex items-center justify-between p-3 rounded border text-sm transition-all ${
                    model === m.id 
                      ? 'bg-blue-900/30 border-blue-500 text-blue-200' 
                      : 'bg-gray-950 border-gray-700 text-gray-400 hover:border-gray-500'
                  }`}
                >
                  <span className="font-medium">{m.name}</span>
                  {model === m.id && <div className="w-2 h-2 rounded-full bg-blue-500"></div>}
                </button>
              ))}
            </div>
            <p className="text-xs text-gray-500 pt-2">
              Select the active neural processing unit for code generation.
            </p>
          </div>

        </div>

        <div className="p-4 border-t border-gray-700 flex justify-end">
          <button 
            onClick={onClose}
            className="bg-blue-600 hover:bg-blue-500 text-white px-4 py-2 rounded font-medium transition-colors"
          >
            Close
          </button>
        </div>
      </div>
    </div>
  );
};
