export enum ThemeMode {
	"light",
	"dark",
}

export type ThemeKeys = "100" | "200" | "300" | "400" | "background" | "secondary" | "tertiary";
export type ThemeColorRecord = Record<ThemeKeys, string>;
export type Theme = {
	id: string;
	name: string;
	author: string; // TODO: Backend add users; set userids as author
	colors: Partial<Record<ThemeMode, ThemeColorRecord>>;
};

export interface ThemePartial {
	id: Theme["id"];
	mode: ThemeMode;
}

export interface ThemeManager {
	getCurrent: () => ThemePartial;
	get: (id: Theme["id"]) => Theme;
	set: (id: Theme["id"], mode: ThemeMode) => void;
	getLast: () => ThemePartial;
}
