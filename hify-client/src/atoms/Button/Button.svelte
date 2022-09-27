<script lang="ts">
  import SimpleNavigableItem from '../../navigable/SimpleNavigableItem/SimpleNavigableItem.svelte'

  export let onPress: () => void
  export let disabled = false
  export let fullHeight = false
</script>

<SimpleNavigableItem transparent {onPress} {disabled}>
  <span class="for-margin">
    <span class="container {fullHeight ? 'full-height' : ''}">
      <div class="button {fullHeight ? 'full-height' : ''} {disabled ? 'disabled' : ''}">
        <slot />
      </div>
    </span>
  </span>
</SimpleNavigableItem>

<style>
  .for-margin {
    margin: 5px;
  }

  .container {
    display: block;
    height: 100%;
  }

  .button {
    display: flex;
    align-items: center;

    width: fit-content;
    padding: 5px;
    border: 1px solid white;
  }

  .button.full-height {
    /* TODO: investigate why 50% is required instead of 100% */
    height: calc(50% + 4px);
  }

  .button.disabled {
    opacity: 0.5;
  }
</style>
