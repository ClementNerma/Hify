<script lang="ts">
  import { getArtUri } from "../../globals/rest-api";
  import { AudioTrackFragment } from "../../graphql/generated";

  export let track: AudioTrackFragment | null;
  export let dim = false;

  function computeBackgroundImage(track: AudioTrackFragment | null): string {
    if (!track) {
      return "black";
    }

    const { art } = track?.metadata.tags.album;

    if (!art) {
      return "black";
    }

    return `url("${getArtUri(art.id)}")`;
  }

  $: background = computeBackgroundImage(track);
</script>

<div class="background" style="--background: {background};" class:dim />

<style>
  .background {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    z-index: -1;

    background: var(--background);
    background-position: center;
    background-repeat: no-repeat;
    background-size: cover;

    filter: blur(20px) brightness(0.4);

    transition: filter linear 0.5s;
  }

  .background.dim {
    filter: blur(20px) brightness(0.3);
  }
</style>
