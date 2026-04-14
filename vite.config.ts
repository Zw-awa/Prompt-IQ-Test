import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

const tauriHost = process.env.TAURI_DEV_HOST

export default defineConfig({
  plugins: [react()],
  clearScreen: false,
  envPrefix: ['VITE_', 'TAURI_'],
  server: {
    port: 1420,
    strictPort: true,
    host: tauriHost || undefined,
    hmr: tauriHost
      ? {
          protocol: 'ws',
          host: tauriHost,
          port: 1421,
        }
      : undefined,
  },
  preview: {
    port: 1420,
    strictPort: true,
  },
})
