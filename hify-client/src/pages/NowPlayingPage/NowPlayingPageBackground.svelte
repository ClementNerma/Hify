<script lang="ts">
  import { getArtUri } from "../../globals/rest-api";
  import { AudioTrackFragment } from "../../graphql/generated";
  import { globalBackground } from "../../organisms/Background/Background.svelte";

  export let track: AudioTrackFragment | null;
  export let dim = false;

  $: art = track?.metadata.tags.album?.art
  $: background = art ? `url("${getArtUri(art.id)}")` : 'transparent'
  $: backdropFilter = `blur(10px) brightness(${dim ? 0.3 : 0.5})`
</script>

<div class="background" style="--background: {background}"></div>
<div class="filter" style="--backdrop-filter: {backdropFilter}"></div>

<style>
    .background {
        position: fixed;

        top: 0;
        left: 0;
        right: 0;
        bottom: 0;

        z-index: -2;

        background: var(--background);
        background-position: center;
        background-repeat: no-repeat;
        background-size: cover;
    }

    .filter {
        position: fixed;

        top: 0;
        left: 0;
        right: 0;
        bottom: 0;

        z-index: -1;

        backdrop-filter: var(--backdrop-filter);
    }
</style>
