<script lang="ts">
  import { SimpleNavigableItemProps } from '../../navigable/SimpleNavigableItem/SimpleNavigableItem'
  import SimpleNavigableItem from '../../navigable/SimpleNavigableItem/SimpleNavigableItem.svelte'

  export let onPress: () => void
  export let disabled = false
  export let fullHeight = false

  export let hasFocusPriority: SimpleNavigableItemProps['hasFocusPriority'] = null
  export let onFocus: SimpleNavigableItemProps['onFocus'] = undefined
  export let onUnfocus: SimpleNavigableItemProps['onFocus'] = undefined
</script>

<SimpleNavigableItem
  {onPress}
  {disabled}
  noPadding
  notRounded
  marginRight={10}
  {hasFocusPriority}
  {onFocus}
  {onUnfocus}
>
  <div class="button" class:fullHeight class:disabled>
    <slot />
  </div>
</SimpleNavigableItem>

<style>
  .button {
    display: flex;
    align-items: center;

    border: 1px solid white;

    width: fit-content;
    padding: 5px;
  }

  .button.fullHeight {
    /* TODO: investigate why 50% is required instead of 100% */
    height: calc(50% + 2px);
  }

  .button.disabled {
    opacity: 0.5;
  }
</style>
