import { defineConfig } from 'vitest/config';
import { svelte } from '@sveltejs/vite-plugin-svelte'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [svelte()],
  base: '/',
  build: {
    sourcemap: true,
    rollupOptions: {
      input: {
        status: './status.html'
      }
    }
  },
  resolve: process.env.VITEST ? {
    conditions: ['browser']
  } : undefined,
  server: {
    proxy: {
      '/util': {
        target: 'http://localhost:5000/util',
        changeOrigin: true,
        rewrite: (path) => path.replace(/^\/util/, '')
      }
    }
  }
});
