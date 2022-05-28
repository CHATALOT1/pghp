<script>
  import { themeColours, themeIsDark } from "./stores/themeSettings";

  $: colours = $themeColours;
  $: themeCss = `--primary-colour: ${colours.primary};\
  --secondary-colour: ${colours.secondary}; --text-colour: ${colours.text};\
  --header-colour: ${colours.header};`;
</script>

<div id="theme-container" style={themeCss}>
  <slot />
</div>
