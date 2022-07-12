import { defineNuxtConfig } from "nuxt";

// https://v3.nuxtjs.org/api/configuration/nuxt.config
export default defineNuxtConfig({
	ssr: true,
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
