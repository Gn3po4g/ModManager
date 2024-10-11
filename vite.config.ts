import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import unocss from 'unocss/vite';

const host = process.env.TAURI_DEV_HOST;

export default defineConfig({
	build: {
		chunkSizeWarningLimit: 1000,
	},
	plugins: [react(), unocss()],

	clearScreen: false,
	server: {
		port: 1420,
		strictPort: true,
		host: host || false,
		hmr: host
			? {
					protocol: 'ws',
					host,
					port: 1421,
				}
			: undefined,
		watch: {
			ignored: ['**/src-tauri/**'],
		},
	},
});
