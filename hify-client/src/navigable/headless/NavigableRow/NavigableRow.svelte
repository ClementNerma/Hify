<script lang="ts">
  import { afterUpdate, onDestroy } from 'svelte'

  import { getParentNavigable, Props, setChildrenNavigable } from '../../navigation'
  import { NavigableRow } from './NavigableRow'

  export let position: Props<NavigableRow>['position'] = null
  export let hasFocusPriority: Props<NavigableRow>['hasFocusPriority'] = null

  const rowProps = (): Props<NavigableRow> => ({
    position,
    hasFocusPriority,
  })

  const nav = getParentNavigable()
  const row = new NavigableRow(nav, rowProps())

  nav.append(row)

  setChildrenNavigable(row)

  afterUpdate(() => row.updateProps(rowProps()))

  onDestroy(() => nav.remove(row))

  export const requestFocus = () => row.requestFocus()
</script>

<slot nav={row} />
