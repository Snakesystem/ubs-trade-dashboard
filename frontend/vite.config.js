import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'

// https://vite.dev/config/
export default defineConfig(({ mode }) => {
  return {
    plugins: [svelte()],
    base: mode === 'production' ? '/ubs-trade-dashboard/' : '/',
    server: {
      proxy: {
        '/api': {
          target: 'https://snakesystem-web-api-tdam.shuttle.app',
          changeOrigin: true,
          secure: false,
        }
      }
    }
  }
})
