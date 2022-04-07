<script lang="ts">
  import SimpleNavigableItem from '../../navigable/SimpleNavigableItem/SimpleNavigableItem.svelte'

  export let max: number
  export let value: number | null
  export let onChange: (newValue: number) => void
  export let onPress: SimpleNavigableItem['$$prop_def']['onPress'] = undefined

  let input: HTMLInputElement

  function getValue(): number {
    const number = parseInt(input.value)

    if (Number.isNaN(number)) {
      throw new Error('Range value is not a number!')
    }

    return number
  }

  function onLeft() {
    onChange(Math.max(getValue() - 5, 0))
  }

  function onRight() {
    onChange(Math.min(getValue() + 5, max))
  }
</script>

<SimpleNavigableItem {onLeft} {onRight} {onPress} transparent={true}>
  <div class="container">
    <input type="range" {max} {value} bind:this={input} on:change={() => onChange(getValue())} />
  </div>
</SimpleNavigableItem>
