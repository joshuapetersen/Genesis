import React, { useEffect, useRef } from 'react';
import { File } from '../types';

interface PreviewProps {
  files: File[];
  isOpen: boolean;
  onClose: () => void;
}

export const Preview: React.FC<PreviewProps> = ({ files, isOpen, onClose }) => {
  const iframeRef = useRef<HTMLIFrameElement>(null);

  useEffect(() => {
    if (!isOpen || !iframeRef.current) return;

    // Construct the preview content
    // We assume index.html is the entry. If not found, show error.
    const indexFile = files.find(f => f.name === 'index.html');
    
    if (!indexFile) {
        const doc = iframeRef.current.contentDocument;
        if(doc) {
            doc.body.innerHTML = '<div style="color:red; padding:20px; font-family:sans-serif;">Error: index.html not found</div>';
        }
        return;
    }

    let content = indexFile.content;

    // Naive injection of CSS and JS for preview without server
    // Replace <link rel="stylesheet" href="..."> with <style>...</style>
    const cssFiles = files.filter(f => f.name.endsWith('.css'));
    cssFiles.forEach(css => {
        // Replace link tags or append to head
        // For simplicity, we just append all CSS to head in this mock env
        content = content.replace('</head>', `<style data-filename="${css.name}">${css.content}</style></head>`);
    });

    // Replace <script src="..."> with inline script
    // This handles relative paths simply by matching names
    const jsFiles = files.filter(f => f.name.endsWith('.js'));
    jsFiles.forEach(js => {
         const scriptTagRegex = new RegExp(`<script[^>]*src=["']${js.name}["'][^>]*><\/script>`, 'g');
         content = content.replace(scriptTagRegex, `<script>${js.content}</script>`);
    });

    const doc = iframeRef.current.contentDocument;
    if (doc) {
        doc.open();
        doc.write(content);
        doc.close();
    }

  }, [files, isOpen]);

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 bg-black/50 z-40 flex justify-center items-center p-8 backdrop-blur-sm">
      <div className="bg-white w-full h-full max-w-6xl rounded-lg shadow-2xl flex flex-col overflow-hidden relative">
        <div className="bg-gray-100 border-b border-gray-200 p-2 flex justify-between items-center">
            <span className="text-gray-600 text-sm font-medium px-2">Live Preview</span>
            <button onClick={onClose} className="text-gray-500 hover:text-gray-800">
                <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/></svg>
            </button>
        </div>
        <iframe 
            ref={iframeRef}
            title="Preview"
            className="flex-1 w-full bg-white"
            sandbox="allow-scripts allow-modals"
        />
      </div>
    </div>
  );
};
