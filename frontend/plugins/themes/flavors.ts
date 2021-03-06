import { Theme, ThemeMode } from "./types";

export const default_theme: Theme = {
	id: "default.unnamed_weeb_music_database",
	name: "Default",
	author: "unnamed_weeb_music_database",
	colors: {
		[ThemeMode.light]: {
			"background": "rgb(255, 255, 255)",
			"secondary": "rgb(246, 246, 246)",
			"tertiary": "rgb(231, 231, 231)",
			"400": "rgb(141, 141, 141)",
			"300": "rgb(107, 107, 107)",
			"200": "rgb(62, 62, 62)",
			"100": "rgb(39, 39, 39)"
		},
		[ThemeMode.dark]: {
			"background": "rgb(28, 28, 28)",
			"secondary": "rgb(40, 40, 40)",
			"tertiary": "rgb(51, 51, 51)",
			"400": "rgb(141, 141, 141)",
			"300": "rgb(176, 176, 176)",
			"200": "rgb(221, 221, 221)",
			"100": "rgb(244, 244, 244)"
		}
	},
};
