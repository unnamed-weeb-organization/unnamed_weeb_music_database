import { defineConfig } from "vite-plugin-windicss";

export default defineConfig({
	theme: {
		colors: {
			background: "var(--background)",
			secondary: "var(--secondary)",
			tertiary: "var(--tertiary)",
			400: "var(--400)",
			300: "var(--300)",
			200: "var(--200)",
			100: "var(--100)",
		},
	},
});
