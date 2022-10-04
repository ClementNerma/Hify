<script lang="ts">
  import { afterUpdate } from 'svelte'

  import { getParentNavigable, Props, setChildrenNavigable } from '../../navigation'
  import InternalNavWrapper from '../InternalNavWrapper.svelte'
  import { NavigableList } from './NavigableList'

  export let hasFocusPriority: Props<NavigableList>['hasFocusPriority'] = null

  const list = new NavigableList(getParentNavigable(), { hasFocusPriority })

  setChildrenNavigable(list)

  afterUpdate(() => list.updateProps({ hasFocusPriority }))

  export const requestFocus = () => list.requestFocus()
</script>

<InternalNavWrapper navId={list.id}>
  <slot nav={list} />
</InternalNavWrapper>
