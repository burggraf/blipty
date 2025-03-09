import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
export default {
	kit: {
		adapter: adapter({
			fallback: 'index.html'
		}),
		prerender: {
			handleMissingId: 'ignore'
		}
	},
	preprocess: vitePreprocess()
};
