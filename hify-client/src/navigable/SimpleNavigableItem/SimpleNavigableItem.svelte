<script lang="ts">
  import { SimpleNavigableItem, SimpleNavigableItemProps } from './SimpleNavigableItem'
  import { getParentNavigable } from '../navigation'

  export let onPress: SimpleNavigableItemProps['onPress'] = undefined
  export let onLongPress: SimpleNavigableItemProps['onLongPress'] = undefined
  export let onFocusChange: SimpleNavigableItemProps['onFocusChange'] = undefined
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
