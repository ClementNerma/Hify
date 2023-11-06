import { svelte } from '@sveltejs/vite-plugin-svelte'
import { defineConfig } from 'vite'
import { join } from 'path'

// Vite plugins
import wasm from 'vite-plugin-wasm'
import topLevelAwait from 'vite-plugin-top-level-await'
import dotenv from 'dotenv'

export default ({ mode }) => {
  dotenv.config({ path: `./.env.${mode}` })

  return defineConfig({
    plugins: [svelte(), wasm(), topLevelAwait()],

    resolve: {
      alias: {
        "@atoms": join(__dirname, "./src/components/atoms"),
        "@molecules": join(__dirname, "./src/components/molecules"),
        "@organisms": join(__dirname, "./src/components/organisms"),
        "@globals": join(__dirname, "./src/globals"),
        "@graphql": join(__dirname, "./src/graphql"),
        "@navigable": join(__dirname, "./src/navigable"),
        "@pages": join(__dirname, "./src/pages"),
        "@stores": join(__dirname, "./src/stores"),
        "@root": join(__dirname, "./src"),
      }
    },

    server: {
      port: 8892
    }
  })
}
