<script lang="ts">
  import SimpleNavigableItem from '../../navigable/SimpleNavigableItem/SimpleNavigableItem.svelte'

  export let max: number
  export let value: number | null
  export let onChange: (newValue: number) => void

  let input: HTMLInputElement

  function getValue(): number {
    const number = parseInt(input.value)

    if (Number.isNaN(number)) {
      throw new Error('Range value is not a number!')
    }

    return number
  }

  function onLeft() {
    onChange(Math.max(getValue() - 1, 0))
  }

  function onRight() {
    onChange(Math.min(getValue() + 1, max))
  }
</script>

<SimpleNavigableItem {onLeft} {onRight}>
  <div class="container">
    <input type="range" {max} {value} bind:this={input} on:change={() => onChange(getValue())} />
  </div>
</SimpleNavigableItem>

<style>
  .container {
    padding: 5px;
  }
</style>
