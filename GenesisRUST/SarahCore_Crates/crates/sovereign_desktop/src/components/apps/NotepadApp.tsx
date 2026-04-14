import React, { useState } from 'react';

export default function NotepadApp() {
  const [text, setText] = useState('Welcome to Notepad!\n\nStart typing here...');

  return (
    <div className="h-full w-full bg-white dark:bg-gray-900 flex flex-col">
      <div className="h-8 bg-gray-100 dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700 flex items-center px-2 text-xs text-gray-600 dark:text-gray-400 gap-4">
        <button className="hover:bg-gray-200 dark:hover:bg-gray-700 px-2 py-1 rounded">File</button>
        <button className="hover:bg-gray-200 dark:hover:bg-gray-700 px-2 py-1 rounded">Edit</button>
        <button className="hover:bg-gray-200 dark:hover:bg-gray-700 px-2 py-1 rounded">View</button>
      </div>
      <textarea
        value={text}
        onChange={(e) => setText(e.target.value)}
        className="flex-1 w-full p-4 resize-none outline-none bg-transparent text-gray-800 dark:text-gray-200 font-sans"
        spellCheck={false}
      />
    </div>
  );
}
