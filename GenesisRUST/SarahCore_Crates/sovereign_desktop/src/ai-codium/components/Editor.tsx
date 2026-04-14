import React from 'react';
import Editor, { Monaco } from '@monaco-editor/react';

interface CodeEditorProps {
  value: string;
  onChange: (value: string | undefined) => void;
  language?: string;
}

export default function CodeEditor({ value, onChange, language = 'typescript' }: CodeEditorProps) {
  const handleEditorDidMount = (editor: any, monaco: Monaco) => {
    // Custom theme configuration if needed
    monaco.editor.defineTheme('vs-dark-custom', {
      base: 'vs-dark',
      inherit: true,
      rules: [],
      colors: {
        'editor.background': '#1e1e1e',
      }
    });
    monaco.editor.setTheme('vs-dark-custom');
  };

  return (
    <div className="h-full w-full overflow-hidden">
      <Editor
        height="100%"
        defaultLanguage={language}
        defaultValue={value}
        value={value}
        onChange={onChange}
        onMount={handleEditorDidMount}
        theme="vs-dark"
        options={{
          minimap: { enabled: true },
          fontSize: 14,
          fontFamily: "'JetBrains Mono', 'Fira Code', monospace",
          scrollBeyondLastLine: false,
          automaticLayout: true,
          padding: { top: 16 },
          lineNumbers: 'on',
          glyphMargin: true,
          folding: true,
          lineDecorationsWidth: 10,
          lineNumbersMinChars: 3,
        }}
      />
    </div>
  );
}
