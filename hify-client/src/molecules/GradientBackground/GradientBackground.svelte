<script context="module" lang="ts">
  import { writable } from 'svelte/store'

  export type Color = {
    r: number
    g: number
    b: number
    a?: number
  }

  export type GradientBackground = {
    startColor: Color
    endColor: Color
    colorSep: number
  }

  export const defaultColors: GradientBackground = {
    startColor: { r: 10, g: 38, b: 89 },
    endColor: { r: 8, g: 4, b: 45 },
    colorSep: 80,
  }

  export const backgroundGradient = writable<GradientBackground>(defaultColors)

  export function resetBackgroundGradient(): void {
    backgroundGradient.set(defaultColors)
  }

  export function colorToHex(color: Color): string {
    return `rgba(${color.r}, ${color.g}, ${color.b}, ${color.a ?? 1})`
  }
</script>

<div
  class="background"
  style="--start-color: {colorToHex($backgroundGradient.startColor)}; --end-color: {colorToHex(
    $backgroundGradient.endColor,
  )}; --color-sep: {$backgroundGradient.colorSep}%;"
/>

<style>
  .background {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    z-index: -1;

    background: linear-gradient(to bottom, var(--start-color) 0%, var(--end-color) var(--color-sep));
    background-attachment: fixed;

    transition: --linear-start 0.5s, --linear-end 0.5s;
  }
</style>
