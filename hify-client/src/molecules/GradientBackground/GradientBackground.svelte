<script context="module" lang="ts">
  import { writable } from 'svelte/store'

  export type Color = {
    r: number
    g: number
    b: number
    a?: number
  }

  export type VerticalGradient = {
    startColor: Color
    endColor: Color
    colorSep: number
  }

  export type RadialGradient = {
    centerColor: Color
    exteriorColor: Color
    colorSep: number
  }

  export function setVerticalGradient(gradient: VerticalGradient): void {
    backgroundGradient.set(
      `linear-gradient(to bottom, ${colorToRGBA(gradient.startColor)} 0%, ${colorToRGBA(gradient.endColor)} ${
        gradient.colorSep
      }%)`,
    )
  }

  export function setRadialGradient(gradient: RadialGradient): void {
    backgroundGradient.set(
      `radial-gradient(circle, ${colorToRGBA(gradient.centerColor)} 0%, ${colorToRGBA(gradient.exteriorColor)} ${
        gradient.colorSep
      }%)`,
    )
  }

  export function resetBackgroundGradient(): void {
    setVerticalGradient({
      startColor: { r: 10, g: 38, b: 89 },
      endColor: { r: 8, g: 4, b: 45 },
      colorSep: 80,
    })
  }

  function colorToRGBA(color: Color): string {
    return `rgba(${color.r}, ${color.g}, ${color.b}, ${color.a ?? 1})`
  }

  const backgroundGradient = writable<string>('')

  resetBackgroundGradient()
</script>

<div class="background" style="--gradient: {$backgroundGradient};" />

<style>
  .background {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    z-index: -1;

    background: var(--gradient);
    background-attachment: fixed;

    transition: --linear-start 0.5s, --linear-end 0.5s;
  }
</style>
