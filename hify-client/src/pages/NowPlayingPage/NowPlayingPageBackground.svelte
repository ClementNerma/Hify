<script lang="ts">
  import { createBlurHashImageSrc } from "../../globals/blurhash-decoder";
  import { AudioTrackFragment } from "../../graphql/generated";

  export let track: AudioTrackFragment | null;
  export let dim = false;

  function computeBackground(track: AudioTrackFragment | null): string {
    if (!track) {
      return "black";
    }

    const { art } = track?.metadata.tags.album;

    if (!art) {
      return "black";
    }

    const blurHashSrc = createBlurHashImageSrc(
      art,
      window.innerWidth,
      window.innerHeight
    );

    return `url("${blurHashSrc}")`;
  }

  $: background = computeBackground(track);
</script>

<div class="background" style:background class:dim />

<svelte:window on:resize={() => (background = computeBackground(track))} />

<style>
  .background {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    z-index: -1;

    filter: brightness(0.5);

    transition: filter linear 0.5s;
  }

  .background.dim {
    filter: brightness(0.3);
  }
</style>
