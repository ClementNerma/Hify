import path from 'node:path'
import babel from '@rolldown/plugin-babel'
import tailwindcss from '@tailwindcss/vite'
import react from '@vitejs/plugin-react'
import { defineConfig } from 'vite'

// https://vite.dev/config/
// oxlint-disable-next-line no-default-export
export default defineConfig({
  plugins: [
    react(),
    babel({
      plugins: ['babel-plugin-react-compiler'],
    }),
    tailwindcss(),
  ],
  server: {
    allowedHosts: ['yoga'],
    port: 8892,
  },
})
