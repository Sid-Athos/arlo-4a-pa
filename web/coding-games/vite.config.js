import { defineConfig } from 'vite';
import solidPlugin from 'vite-plugin-solid';
import suidPlugin from "@suid/vite-plugin";

export default defineConfig({
  optimizeDeps: {
    // Add both @codemirror/state and @codemirror/view to included deps to optimize
    include: ['@codemirror/state', '@codemirror/view'],
  },
  plugins: [solidPlugin(),suidPlugin()],
  server: {
    port: 3000,
    host: true,
    strictPort: true
  },
  build: {
    target: 'esnext',
  },
});
