import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [sveltekit()],
	server: {
		port: 1420,
		strictPort: true,
	},
	envPrefix: ['VITE_', 'TAURI_'],
	optimizeDeps: {
		include: ['@tauri-apps/api']
	},
	resolve: {
		alias: {
			'@tauri-apps/api': '@tauri-apps/api'
		}
	}
});
