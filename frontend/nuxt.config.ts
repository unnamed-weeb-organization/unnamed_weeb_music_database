import { defineNuxtConfig } from "nuxt";

// https://v3.nuxtjs.org/api/configuration/nuxt.config
export default defineNuxtConfig({
	// I like being sane.
	ssr: false,
	typescript: {
		strict: true,
	},
	modules: ["@pinia/nuxt", "nuxt-windicss"],
	windicss: {
		analyze: {
			analysis: { interpretUtilities: true },
			server: { port: 3001 },
		},
	},
});
