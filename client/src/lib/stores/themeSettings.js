import { onMount } from "svelte";
import { writable, derived } from "svelte/store";
import themes from "./themes.json";

function createThemeIsDark() {
  const { subscribe, set, update } = writable(true, (set) =>
    // Set to system theme once subscriber mounted
    onMount(() => set(window.matchMedia("(prefers-color-scheme: dark)").matches))
  ); // Dark theme default

  return {
    subscribe,
    set,
    toggle: () => update((value) => !value)
  };
}

export let themeIsDark = createThemeIsDark();

export let themeColours = derived(themeIsDark, ($themeIsDark) =>
  $themeIsDark ? themes["dark"] : themes["light"]
);

// TODO: Expand this. See https://github.com/CHATALOT1/pghp/issues/9
