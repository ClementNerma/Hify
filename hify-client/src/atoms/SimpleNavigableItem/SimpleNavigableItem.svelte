<script lang="ts">
  import { getParentNavigable } from '../../others/navigation'

  import { SimpleNavigableItem, SimpleNavigableItemProps } from './SimpleNavigableItem'

  export let onPress: SimpleNavigableItemProps['onPress']
  export let onLongPress: SimpleNavigableItemProps['onLongPress'] = undefined
  export let onFocusChange: SimpleNavigableItemProps['onFocusChange']
  export let onBack: SimpleNavigableItemProps['onBack'] = undefined

  const nav = getParentNavigable()

  const item = new SimpleNavigableItem(nav, {
    onPress,
    onLongPress,
    onFocusChange,
    onBack,
  })

  nav.append(item)
</script>

<slot />
