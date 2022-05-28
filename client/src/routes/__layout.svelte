<script>
  import Theme from "$lib/Theme.svelte";
</script>

<Theme>
  <slot />
</Theme>

<style>
  :global(body) {
    margin: 0;
  }
</style>
