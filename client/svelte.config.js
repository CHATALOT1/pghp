import adapter from '@sveltejs/adapter-static';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	kit: {
		adapter: adapter(),
		// trailingSlash: 'always', // TODO: Test to see if this is wanted
		prerender: { default: true }
	}
};

export default config;
