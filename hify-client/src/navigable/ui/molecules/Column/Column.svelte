<script lang="ts">
import NavigableList from '../../../headless/NavigableList/NavigableList.svelte'
import { NavigableList as NavigableListClass } from '../../../headless/NavigableList/NavigableList'
import { Props } from '../../../navigation'

export let hasFocusPriority: Props<NavigableListClass>['hasFocusPriority'] = null
export let trapped: Props<NavigableListClass>['trapped'] = undefined
</script>

<div class="column">
  <NavigableList let:requestFocus {hasFocusPriority} {trapped}>
    <slot {requestFocus} />
  </NavigableList>
</div>

<style>
  .column {
    display: flex;
    flex-direction: column;
    overflow-x: auto;
  }
</style>
