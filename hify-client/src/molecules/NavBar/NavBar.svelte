<script context="module" lang="ts">
  export type Tab = {
    label: string
    uri: string
  }
</script>

<script lang="ts">
  import { useLocation, navigate } from 'svelte-navigator'

  import NavigableRow from '../../navigable/NavigableRow/NavigableRow.svelte'
  import { RequestFocus } from '../../navigable/navigation'
  import SimpleNavigableItem from '../../navigable/SimpleNavigableItem/SimpleNavigableItem.svelte'

  const location = useLocation()

  export let tabs: Tab[]
  export let tabsFocusRequest = new Array<RequestFocus>(tabs.length)

  export const requestFocus = (): void => {
    for (let i = 0; i < tabs.length; i++) {
      if ($location.pathname === tabs[i].uri) {
        tabsFocusRequest[i]()
        return
      }
    }

    if (tabsFocusRequest.length) {
      tabsFocusRequest[0]()
    }
  }

  let isFocused: boolean
</script>

<div class="container" class:isFocused>
  <NavigableRow>
    {#each tabs as tab, i}
      <SimpleNavigableItem
        onPress={() => navigate(tab.uri)}
        hasFocusPriority={$location.pathname === tab.uri}
        onFocus={() => {
          window.scrollTo({ top: 0, left: 0, behavior: 'smooth' })
          isFocused = true
        }}
        onUnfocus={() => {
          isFocused = false
        }}
        bind:requestFocus={tabsFocusRequest[i]}
      >
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
    margin-bottom: 10px;
  }

  .container:not(.isFocused) {
    opacity: 0.2;
    transition: opacity linear 0.8s;
    transition-delay: 0.2s;
  }

  .tab {
    padding: 0 25px;
    font-size: 1rem;
  }
</style>
