<script lang="ts">
  import { getContext } from 'svelte'
  import { asNavigator, NavigableItemProps, NAVIGATION_CTX } from '../handlers/nav'

  export let onFocusChange: NavigableItemProps['onFocusChange']
  export let onPress: NavigableItemProps['onPress']
  export let onLongPress: NavigableItemProps['onLongPress'] = undefined
  export let onBack: NavigableItemProps['onBack'] = undefined

  const ctx = asNavigator(getContext(NAVIGATION_CTX))

  ctx.register({
    type: 'item',
    props: {
      parent: ctx,
      onFocusChange,
      onPress,
      onLongPress,
      onBack,
    },
  })
</script>

<slot />
