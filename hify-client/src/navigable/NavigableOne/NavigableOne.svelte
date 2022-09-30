<script lang="ts">
  import { afterUpdate, onDestroy } from 'svelte'

  import { getParentNavigable, Props, setChildrenNavigable } from '../navigation'
  import { NavigableOne } from './NavigableOne'

  export let position: Props<NavigableOne>['position'] = null
  export let hasFocusPriority: Props<NavigableOne>['hasFocusPriority'] = null
  export let onFocusChangeCallback: Props<NavigableOne>['onFocusChangeCallback'] = null

  const oneProps = (): Props<NavigableOne> => ({
    position,
    hasFocusPriority,
    onFocusChangeCallback,
  })

  const nav = getParentNavigable()
  const one = new NavigableOne(nav, oneProps())

  nav.append(one)

  setChildrenNavigable(one)

  afterUpdate(() => nav.updateProps(oneProps()))

  onDestroy(() => nav.remove(one))

  export const requestFocus = () => one.requestFocus()
</script>

<slot nav={one} />
