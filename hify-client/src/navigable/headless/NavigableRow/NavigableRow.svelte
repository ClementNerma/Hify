<script lang="ts">
  import { afterUpdate } from 'svelte'

  import { getParentNavigable, Props, setChildrenNavigable } from '../../navigation'
  import InternalNavWrapper from '../InternalNavWrapper.svelte'
  import { NavigableRow } from './NavigableRow'

  export let hasFocusPriority: Props<NavigableRow>['hasFocusPriority'] = null
  export let onFocusChangeCallback: Props<NavigableRow>['onFocusChangeCallback'] | null = null

  const rowProps = (): Props<NavigableRow> => ({
    hasFocusPriority,
    onFocusChangeCallback,
  })

  const row = new NavigableRow(getParentNavigable(), rowProps())

  setChildrenNavigable(row)

  afterUpdate(() => row.updateProps(rowProps()))

  export const requestFocus = () => row.requestFocus()
</script>

<InternalNavWrapper navId={row.id}>
  <slot nav={row} />
</InternalNavWrapper>
