<script context="module" lang="ts">
  export type Tab = {
    label: string
    uri: string
  }
</script>

<script lang="ts">
  import { useLocation, useNavigate } from 'svelte-navigator'

  import NavigableRow from '../../navigable/NavigableRow/NavigableRow.svelte'
  import SimpleNavigableItem from '../../navigable/SimpleNavigableItem/SimpleNavigableItem.svelte'

  const navigate = useNavigate()
  const location = useLocation()

  export let tabs: Tab[]
</script>

<div class="container">
  <NavigableRow>
    {#each tabs as tab}
      <SimpleNavigableItem onPress={() => navigate(tab.uri)} hasFocusPriority={$location.pathname === tab.uri}>
        <div class="tab">{tab.label}</div>
      </SimpleNavigableItem>
    {/each}
  </NavigableRow>
</div>

<style>
  .container {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: center;
  }

  .tab {
    margin: 5px 25px;
    font-size: 1rem;
  }
</style>
