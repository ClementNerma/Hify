import { svelte } from '@sveltejs/vite-plugin-svelte'
import { defineConfig } from 'vite'
import wasm from 'vite-plugin-wasm'
import topLevelAwait from 'vite-plugin-top-level-await'
import dotenv from 'dotenv'

export default ({ mode }) => {
  dotenv.config({ path: `./.env.${mode}` })

  return defineConfig({
    plugins: [svelte(), wasm(), topLevelAwait()],

    server: {
      port: 8892
    }
  })
}
