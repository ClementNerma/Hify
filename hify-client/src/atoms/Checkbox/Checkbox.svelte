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
  <div class="container">
    <input type="checkbox" {checked} />
    <span class="label">
      <slot />
    </span>
  </div>
</Button>

<style>
  .container {
    display: flex;
    align-items: center;
  }

  .label {
    margin-left: 5px;
  }
</style>
