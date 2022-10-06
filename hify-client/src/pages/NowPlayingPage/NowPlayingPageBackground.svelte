<script lang="ts">
  import { ArtRgb, AudioTrackFragment } from '../../graphql/generated'

  export let track: AudioTrackFragment | null
  export let dim = false

  function changeBrightness(color: ArtRgb, times: number): ArtRgb {
    return {
      r: Math.round(color.r * times),
      g: Math.round(color.g * times),
      b: Math.round(color.b * times),
    }
  }

  function colorToRGB(color: ArtRgb): string {
    return `rgb(${color.r}, ${color.g}, ${color.b})`
  }

  function computeBackground(track: AudioTrackFragment): string {
    const color = track?.metadata.tags.album.art?.dominantColor ?? { r: 0, g: 0, b: 0 }

    const centerColor = changeBrightness(color, 0.9)
    const extColor = changeBrightness(color, 0.6)

    return `radial-gradient(circle, ${colorToRGB(centerColor)} 0%, ${colorToRGB(extColor)} 100%)`
  }

  $: background = track ? computeBackground(track) : 'black'
</script>

<div class="background" style:background class:dim />

<style>
  .background {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    z-index: -1;

    filter: brightness(0.8);

    transition: filter linear 1s;
  }

  .background.dim {
    filter: brightness(0.3);
  }
</style>
