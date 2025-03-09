import { getContext, setContext } from "svelte";
import { writable, type Writable } from "svelte/store";

type SidebarState = "expanded" | "collapsed";
type SidebarContext = {
	state: SidebarState;
	isMobile: boolean;
	openMobile: boolean;
	setOpenMobile: (value: boolean) => void;
	toggle: () => void;
};

const SIDEBAR_CONTEXT_KEY = Symbol("sidebar");

export function setSidebarContext(value: SidebarContext) {
	setContext(SIDEBAR_CONTEXT_KEY, value);
}

export function useSidebar(): SidebarContext {
	return getContext<SidebarContext>(SIDEBAR_CONTEXT_KEY);
}

export function createSidebarContext(): SidebarContext {
	const state = writable<SidebarState>("expanded");
	const isMobile = writable(false);
	const openMobile = writable(false);

	return {
		get state() {
			return $state;
		},
		get isMobile() {
			return $isMobile;
		},
		get openMobile() {
			return $openMobile;
		},
		setOpenMobile(value: boolean) {
			openMobile.set(value);
		},
		toggle() {
			state.update((s) => (s === "expanded" ? "collapsed" : "expanded"));
		},
	};
}
