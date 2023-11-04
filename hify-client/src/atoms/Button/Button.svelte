<script lang="ts">
  import { Props, RequestFocus } from '../../navigable/navigation'
  import { SimpleNavigableItem as Nav } from '../../navigable/headless/SimpleNavigableItem/SimpleNavigableItem'
  import SimpleNavigableItem from '../../navigable/headless/SimpleNavigableItem/SimpleNavigableItem.svelte'

  export let onPress: () => void
  export let disabled = false
  export let fullHeight = false

  export let hasFocusPriority: Props<Nav>['hasFocusPriority'] = null
  export let onLongPress: Props<Nav>['onLongPress'] = undefined
  export let onFocus: Props<Nav>['onFocus'] = undefined
  export let onUnfocus: Props<Nav>['onFocus'] = undefined

  export const requestFocus: RequestFocus = () => _requestFocus()

  let _requestFocus: RequestFocus
</script>

<SimpleNavigableItem
  {onPress}
  {onLongPress}
  {disabled}
  noPadding
  notRounded
  marginRight={10}
  {hasFocusPriority}
  {onFocus}
  {onUnfocus}
  bind:requestFocus={_requestFocus}
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
