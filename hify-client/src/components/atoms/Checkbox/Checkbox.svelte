<script lang="ts">
  import Button from '@atoms/Button/Button.svelte'

  export let checked = false
  export let onChange: ((checked: boolean) => void) | null = null

  export let fullHeight = false

  function onPress() {
    checked = !checked
    onChange?.(checked)
  }
</script>

<Button {onPress} {fullHeight}>
  <input type="checkbox" {checked} />
  <span class="label">
    <slot />
  </span>
</Button>
