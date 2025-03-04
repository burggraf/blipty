import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [sveltekit()],
	server: {
		host: '0.0.0.0',
		port: 1420,
		strictPort: true,
	},
	envPrefix: ['VITE_', 'TAURI_', 'ANDROID_'],
	optimizeDeps: {
		include: ['@tauri-apps/api']
	},
	build: {
		// Optimize for Android TV
		target: 'es2015',
		minify: 'terser',
		terserOptions: {
			compress: {
				drop_console: false
			}
		}
	}
});
