<script lang="ts">
  import Button from '../Button/Button.svelte'

  export let checked = false
  export let onChange: ((checked: boolean) => void) | null = null

  function onPress() {
    checked = !checked
    onChange?.(checked)
  }
</script>

<Button {onPress}>
  <input type="checkbox" {checked} />
  <slot />
</Button>
