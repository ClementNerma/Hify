<script lang="ts">
  import { AudioTrackFragment } from '../../graphql/generated'
  import { SimpleNavigableItemProps } from '../../navigable/headless/SimpleNavigableItem/SimpleNavigableItem'
  import { drawComputedTrackWaveForm, fetchAudioBuffer, generateTrackWaveform } from '../../stores/waveform'
  import SimpleNavigableItem from '../../navigable/headless/SimpleNavigableItem/SimpleNavigableItem.svelte'
  import { afterUpdate, beforeUpdate } from 'svelte'

  export let track: AudioTrackFragment

  export let width: string
  export let height: string
  export let progress: number

  let canvas: HTMLCanvasElement
  let loading = true

  let prevTrack = track.id

  beforeUpdate(() => {
    if (track.id !== prevTrack) {
      prevTrack = track.id
      loading = true
    }
  })

  // Whenever track changes, we fetch the audio buffer
  $: audioBuffer = fetchAudioBuffer(track)

  // Whenever the audio buffer changes, we generate thea new waveform
  $: waveForm = audioBuffer.then((data) =>
    canvas?.offsetWidth ? generateTrackWaveform(data.slice(0), Math.round(canvas.offsetWidth / 3)) : null
  )

  // Whenever the wave form OR the progress changes, we draw it on the canvas
  $: waveForm.then((waveForm) => {
    if (waveForm) {
      drawComputedTrackWaveForm(canvas, waveForm, progress)
      loading = false
    }
  })
</script>

{#if loading}
  <p class="loading-msg" style:width style:height>Loading waveform...</p>
{/if}

<canvas bind:this={canvas} class:loading style:width style:height />

<style>
  canvas {
    width: 100%;
    height: 75px;
  }

  .loading-msg {
    text-align: center;
    font-size: 25px;
    font-weight: bold;
    padding: 0;
    margin: 0;
  }

  .loading {
    position: absolute;
    z-index: -999;
    visibility: hidden;
  }
</style>
