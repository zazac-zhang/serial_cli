import { defineConfig } from 'vite';
import solidPlugin from 'vite-plugin-solid';

export default defineConfig({
  plugins: [solidPlugin()],
  server: {
    port: 1420,
    strictPort: true,
  },
  clearScreen: false,
  build: {
    target: 'esnext',
    polyfillDynamicImport: false,
  },
});
