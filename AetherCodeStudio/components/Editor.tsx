import React from 'react';

interface EditorProps {
  content: string;
  language: string;
  onChange: (value: string) => void;
  fileName: string;
}

export const Editor: React.FC<EditorProps> = ({ content, language, onChange, fileName }) => {
  const lineCount = content.split('\n').length;

  return (
    <div className="h-full flex flex-col bg-gray-900 relative">
      {/* Tab Header */}
      <div className="bg-gray-950 border-b border-gray-800 flex">
        <div className="px-4 py-2 bg-gray-900 text-blue-400 text-sm border-t-2 border-blue-500 flex items-center gap-2 font-medium">
          {fileName}
          <span className="text-xs text-gray-600 uppercase ml-2 px-1 border border-gray-700 rounded">{language}</span>
        </div>
      </div>

      <div className="flex-1 relative overflow-hidden flex">
        {/* Line Numbers */}
        <div className="w-12 bg-gray-950 text-gray-600 text-right pr-2 pt-4 select-none font-mono text-sm leading-6 border-r border-gray-800 hidden md:block">
          {Array.from({ length: Math.max(lineCount, 1) }).map((_, i) => (
            <div key={i}>{i + 1}</div>
          ))}
        </div>

        {/* Text Area */}
        <textarea
          value={content}
          onChange={(e) => onChange(e.target.value)}
          spellCheck={false}
          className="flex-1 bg-transparent text-gray-200 font-mono text-sm p-4 leading-6 resize-none focus:outline-none w-full h-full"
          style={{ tabSize: 2 }}
        />
      </div>
    </div>
  );
};
