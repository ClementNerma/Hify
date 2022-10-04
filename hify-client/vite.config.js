import { svelte } from '@sveltejs/vite-plugin-svelte'
import { defineConfig } from 'vite'
import wasm from 'vite-plugin-wasm'
import topLevelAwait from 'vite-plugin-top-level-await'

export default ({ mode }) => {
  const dotenv = require('dotenv')
  dotenv.config({ path: `./.env.${mode}` })

  return defineConfig({
    plugins: [svelte(), wasm(), topLevelAwait()],

    /*server: {
      port: process.env['VITE_PORT'] ?? 8892,
      host: !!process.env['VITE_HOST']
    }*/
  })
}
