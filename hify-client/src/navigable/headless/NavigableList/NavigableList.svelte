<script lang="ts">
  import { afterUpdate } from 'svelte'

  import { getParentNavigable, Props, setChildrenNavigable } from '../../navigation'
  import InternalNavWrapper from '../InternalNavWrapper.svelte'
  import { NavigableList } from './NavigableList'

  export let hasFocusPriority: Props<NavigableList>['hasFocusPriority'] = null
  export let trapped: Props<NavigableList>['trapped'] = undefined

  const list = new NavigableList(getParentNavigable(), { hasFocusPriority, trapped })

  setChildrenNavigable(list)

  afterUpdate(() => list.updateProps({ hasFocusPriority, trapped }))

  export const requestFocus = () => list.requestFocus()
</script>

<InternalNavWrapper navId={list.id}>
  <slot nav={list} />
</InternalNavWrapper>
