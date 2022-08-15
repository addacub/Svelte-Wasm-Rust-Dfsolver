import adapter from '@sveltejs/adapter-static';
import preprocess from 'svelte-preprocess';
import wasmPack from 'vite-plugin-wasm-pack';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	// Consult https://github.com/sveltejs/svelte-preprocess
	// for more information about preprocessors
	preprocess: preprocess(),

	prerender: {
		default: true,
		enabled: true,
	},

	kit: {
		adapter: adapter(),

		browser: {
			hydrate: true,
			router: true
		},

		// Override http methods in the Todo forms
		methodOverride: {
			allowed: ['PATCH', 'DELETE']
		},

		vite: () => ({
			plugins: [wasmPack(['../wasm-dfsolver'], [])],
		})
	}
};

export default config;
