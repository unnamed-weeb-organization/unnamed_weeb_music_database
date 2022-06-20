import { default_theme } from "./flavors";
import { ThemeMode } from "./types";
import type { Theme, ThemeManager, ThemePartial } from "./types";

export default defineNuxtPlugin(() => {
	const manager = new Manager();
	return {
		provide: {
			themeManager: manager,
		},
	};
});

class Manager implements ThemeManager {
	/**
	 * {@link set setter} in {@link Manager} instance should be used to change the theme
	 * instead of using this.
	 */
	private current: ThemePartial = this.getLast();
	private readonly themes: Record<Theme["id"], Theme> = {
		[default_theme.id]: default_theme,
	};

	constructor() {
		this.set(this.current.id, this.current.mode);
	}

	public getCurrent() {
		return this.current;
	}

	public get(id: Theme["id"]) {
		return this.themes[id];
	}

	/**
	 * Sets the CSS variables to the current document root and
	 * stores a {@link ThemePartial} on the {@link localStorage}
	 *
	 * NOTE:
	 *
	 * This method throws an error when the specified theme
	 * doesn't contain the selected mode.
	 */
	public set(id: Theme["id"], mode: ThemeMode) {
		const theme_colors = this.get(id).colors[mode];
		if (!theme_colors)
			throw new Error("Theme doesn't implement the selected mode.");

		Object.entries(theme_colors).forEach(([key, value]) => {
			document.documentElement.style.setProperty(
				`--${key}`,
				value.replace('"', "")
			);
		});

		localStorage.setItem("theme", JSON.stringify({ id, mode }));
		this.current = { id, mode };
	}

	/**
	 * If available, fetches a {@link ThemePartial} from {@link localStorage} else,
	 * returns the default theme.
	 */
	public getLast(): ThemePartial {
		let partial: ThemePartial;

		const last = localStorage.getItem("theme");
		if (last != null) partial = JSON.parse(last);
		partial ??= { id: default_theme.id, mode: ThemeMode.light };

		return {
			id: partial.id,
			mode: ThemeMode.dark,
		};
	}
}
