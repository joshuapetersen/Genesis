import tailwindcss from '@tailwindcss/vite';
import react from '@vitejs/plugin-react';
import path from 'path';
import {defineConfig} from 'vite';

// Strips crossorigin from HTML — not needed for local file:// loading
function electronHtmlFix() {
  return {
    name: 'electron-html-fix',
    transformIndexHtml(html: string) {
      return html.replace(/ crossorigin/g, '');
    }
  };
}

export default defineConfig(() => {
  return {
    base: './',
    plugins: [react(), tailwindcss(), electronHtmlFix()],
    build: {
      modulePreload: false,
      rollupOptions: {
        output: {
          format: 'es',
          inlineDynamicImports: true,
          entryFileNames: 'assets/[name]-[hash].js',
        }
      }
    },
    define: {},
    resolve: {
      alias: {
        '@': path.resolve(__dirname, '.'),
      },
    },
    server: {
      hmr: process.env.DISABLE_HMR !== 'true',
    },
  };
});
