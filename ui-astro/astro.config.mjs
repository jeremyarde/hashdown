import { defineConfig } from 'astro/config';
import wasm from 'vite-plugin-wasm';
import topLevelAwait from "vite-plugin-top-level-await";

import react from "@astrojs/react";

// https://astro.build/config
import tailwind from "@astrojs/tailwind";

// https://astro.build/config
import svelte from "@astrojs/svelte";

// https://astro.build/config
export default defineConfig({
  output: 'server',
  integrations: [react(), tailwind(), wasm(), svelte()],
  vite: {
    plugins: [wasm(), topLevelAwait()]
  }
});