import babel from '@rolldown/plugin-babel'
import tailwindcss from '@tailwindcss/vite'
import react, { reactCompilerPreset } from '@vitejs/plugin-react'
import { defineConfig } from 'vite'

// https://vite.dev/config/
// oxlint-disable-next-line no-default-export
export default defineConfig({
  plugins: [
    babel({
      presets: [reactCompilerPreset()],
    }),
    react(),
    tailwindcss(),
  ],
  build: {
    rolldownOptions: {
      output: {
        codeSplitting: false,
      },
    },
  },
  server: {
    port: 8892,
  },
})
