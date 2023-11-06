<script lang="ts">
  import { SimpleNavigableItemProps } from '@navigable/headless/SimpleNavigableItem/SimpleNavigableItem'
  import SimpleNavigableItem from '@navigable/headless/SimpleNavigableItem/SimpleNavigableItem.svelte'

  export let max: number
  export let value: number | null
  export let onChange: (newValue: number) => void
  export let directionalAmount: number
  export let onPress: SimpleNavigableItemProps['onPress'] = undefined

  let input: HTMLInputElement

  function getValue(): number {
    const number = parseInt(input.value)

    if (Number.isNaN(number)) {
      throw new Error('Range value is not a number!')
    }

    return number
  }

  function onLeft() {
    onChange(Math.max(getValue() - directionalAmount, 0))
  }

  function onRight() {
    onChange(Math.min(getValue() + directionalAmount, max))
  }
</script>

<SimpleNavigableItem {onLeft} {onRight} {onPress} display="block">
  <div class="container">
    <input type="range" {max} {value} bind:this={input} on:change={() => onChange(getValue())} />
  </div>
</SimpleNavigableItem>
