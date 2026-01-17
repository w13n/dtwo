
// this file is generated — do not edit it


declare module "svelte/elements" {
	export interface HTMLAttributes<T> {
		'data-sveltekit-keepfocus'?: true | '' | 'off' | undefined | null;
		'data-sveltekit-noscroll'?: true | '' | 'off' | undefined | null;
		'data-sveltekit-preload-code'?:
			| true
			| ''
			| 'eager'
			| 'viewport'
			| 'hover'
			| 'tap'
			| 'off'
			| undefined
			| null;
		'data-sveltekit-preload-data'?: true | '' | 'hover' | 'tap' | 'off' | undefined | null;
		'data-sveltekit-reload'?: true | '' | 'off' | undefined | null;
		'data-sveltekit-replacestate'?: true | '' | 'off' | undefined | null;
	}
}

export {};


declare module "$app/types" {
	export interface AppTypes {
		RouteId(): "/" | "/settings" | "/settings/new" | "/settings/[id]" | "/settings/[id]/edit";
		RouteParams(): {
			"/settings/[id]": { id: string };
			"/settings/[id]/edit": { id: string }
		};
		LayoutParams(): {
			"/": { id?: string };
			"/settings": { id?: string };
			"/settings/new": Record<string, never>;
			"/settings/[id]": { id: string };
			"/settings/[id]/edit": { id: string }
		};
		Pathname(): "/" | "/settings" | "/settings/" | "/settings/new" | "/settings/new/" | `/settings/${string}` & {} | `/settings/${string}/` & {} | `/settings/${string}/edit` & {} | `/settings/${string}/edit/` & {};
		ResolvedPathname(): `${"" | `/${string}`}${ReturnType<AppTypes['Pathname']>}`;
		Asset(): string & {};
	}
}