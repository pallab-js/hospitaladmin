import { getContext, setContext } from "svelte";

const TABS_KEY = Symbol("tabs");

export function setTabsContext(context: { value: string }) {
  setContext(TABS_KEY, context);
}

export function getTabsContext(): { value: string } {
  return getContext(TABS_KEY);
}
