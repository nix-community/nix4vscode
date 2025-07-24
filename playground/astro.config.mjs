// @ts-check

import react from '@astrojs/react';

import tailwindcss from '@tailwindcss/vite';
import { defineConfig } from 'astro/config';

// https://astro.build/config
export default defineConfig({
  site: 'https://nix-community.github.io',
  base: '/nix4vscode',
  vite: {
    plugins: [tailwindcss()],
  },

  integrations: [react()],
});
