<script lang="ts">
  import { lowerBrightness, multiplyBrightness } from '../../globals/colors'
  import { ArtRgb, AudioTrackFragment } from '../../graphql/generated'

  export let track: AudioTrackFragment | null
  export let dim = false

  function colorToRGB(color: ArtRgb): string {
    return `rgb(${color.r}, ${color.g}, ${color.b})`
  }

  function computeBackground(track: AudioTrackFragment): string {
    // Here we take the album cover's dominant color (defaulting to black)
    const color = track?.metadata.tags.album.art?.dominantColor ?? { r: 0, g: 0, b: 0 }

    // Then we darken it a little to create a radial gradient around the cover
    // The color is computed so the perceived brightness (by the human eye) is always equal,
    //   no matter what the original color was. This works by darkening the color until
    //   it reaches a certain level of brightness (here, 10 on a scale of 100).
    const centerColor = lowerBrightness(color, 30)

    // And this is the external color which will go on the side
    // The radial gradient goes from the color above (at its center)
    //   up to this color, at the exterior.
    const extColor = lowerBrightness(color, 15)

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

    transition: filter linear 0.5s;
  }

  .background.dim {
    filter: brightness(0.3);
  }
</style>
