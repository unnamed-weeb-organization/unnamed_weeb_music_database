export default defineNuxtPlugin((app) => {
	const lastTheme = () => localStorage.getItem("theme") ?? "default";
	const lastMode = () => localStorage.getItem("mode") ?? "light";

	const classList = document.documentElement.classList;

	classList.add(lastTheme(), lastMode());

	const themes: ThemeManager = {
		currentMode() {
			return lastMode() as ThemeMode;
		},
		currentTheme() {
			return lastTheme();
		},
		changeMode(themeMode: ThemeMode) {
			if (classList.replace(lastMode(), themeMode))
				localStorage.setItem("mode", themeMode);
		},
		changeTheme(themeName) {
			if (classList.replace(lastTheme(), themeName))
				localStorage.setItem("theme", themeName);
		},
	};

	return {
		provide: {
			themeManager: themes,
		},
	};
});

type ThemeMode = "dark" | "light";
export interface ThemeManager {
	currentMode: () => ThemeMode;
	currentTheme: () => string;
	changeMode: (themeMode: ThemeMode) => void;
	changeTheme: (themeName: string) => void;
}
