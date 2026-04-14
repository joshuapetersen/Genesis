import React, { useState } from 'react';

export default function CalculatorApp() {
  const [display, setDisplay] = useState('0');
  const [equation, setEquation] = useState('');

  const handlePress = (val: string) => {
    if (val === 'C') {
      setDisplay('0');
      setEquation('');
    } else if (val === '=') {
      try {
        // Safe eval alternative for simple math
        const result = new Function('return ' + display)();
        setEquation(display + ' =');
        setDisplay(String(result));
      } catch (e) {
        setDisplay('Error');
      }
    } else {
      setDisplay(display === '0' ? val : display + val);
    }
  };

  const buttons = [
    'C', '(', ')', '/',
    '7', '8', '9', '*',
    '4', '5', '6', '-',
    '1', '2', '3', '+',
    '0', '.', '='
  ];

  return (
    <div className="h-full w-full bg-gray-50 dark:bg-gray-900 p-4 flex flex-col">
      <div className="flex-1 bg-white dark:bg-gray-800 rounded-xl mb-4 p-4 flex flex-col items-end justify-end shadow-inner border border-gray-200 dark:border-gray-700">
        <div className="text-gray-500 dark:text-gray-400 text-sm h-6">{equation}</div>
        <div className="text-4xl font-light text-gray-900 dark:text-white truncate w-full text-right">{display}</div>
      </div>
      <div className="grid grid-cols-4 gap-2 h-2/3">
        {buttons.map((btn, i) => (
          <button
            key={i}
            onClick={() => handlePress(btn)}
            className={`rounded-xl text-lg font-medium transition-colors ${
              btn === '=' ? 'col-span-2 bg-blue-500 hover:bg-blue-600 text-white' :
              ['/', '*', '-', '+'].includes(btn) ? 'bg-gray-200 dark:bg-gray-700 hover:bg-gray-300 dark:hover:bg-gray-600 text-blue-600 dark:text-blue-400' :
              btn === 'C' ? 'bg-red-100 dark:bg-red-900/30 hover:bg-red-200 dark:hover:bg-red-900/50 text-red-600 dark:text-red-400' :
              'bg-white dark:bg-gray-800 hover:bg-gray-100 dark:hover:bg-gray-700 text-gray-900 dark:text-gray-100 shadow-sm border border-gray-200 dark:border-gray-700'
            }`}
          >
            {btn}
          </button>
        ))}
      </div>
    </div>
  );
}
