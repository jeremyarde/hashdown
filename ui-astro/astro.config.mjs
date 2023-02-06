import { defineConfig } from 'astro/config';
import wasm from 'vite-plugin-wasm';
import topLevelAwait from "vite-plugin-top-level-await";

// https://astro.build/config
import react from "@astrojs/react";

// https://astro.build/config
import tailwind from "@astrojs/tailwind";

// https://astro.build/config
export default defineConfig({
  integrations: [react(), tailwind(),
  wasm()
  ],
  vite: {
    plugins: [wasm(), topLevelAwait()],
  }
});