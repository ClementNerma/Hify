<script context="module" lang="ts">
export type Tab = {
	label: string
	uri: string
	subMenu?: TabDropdownItem[]
}

export type TabDropdownItem = Omit<Tab, 'subMenu'>
</script>

<script lang="ts">
  import { useLocation, navigate } from 'svelte-navigator'

  import NavigableRow from '@navigable/headless/NavigableRow/NavigableRow.svelte'
  import { RequestFocus } from '@navigable/navigation'
  import SimpleNavigableItem from '@navigable/headless/SimpleNavigableItem/SimpleNavigableItem.svelte'
  import ItemStyleLayer from '@navigable/headless/SimpleNavigableItem/ItemStyleLayer.svelte'
  import { onMount } from 'svelte'
  import { showContextMenu } from '@navigable/ui/molecules/ContextMenu/ContextMenu'

  const location = useLocation()

  export let tabs: Tab[]
  export let tabsFocusRequest = new Array<RequestFocus>(tabs.length)

  onMount(() =>
    location.subscribe(() => {
      const index = tabs.findIndex((tab) => tab.uri === $location.pathname)

      // Fallback to first tab if needed
      tabsFocusRequest[index === -1 ? 0 : index]()
    }),
  )

  function showSubMenu(subMenu: TabDropdownItem[]) {
    showContextMenu(
      subMenu.map(({ label, uri }) => ({
        label,
        onPress: () => navigate(uri),
      })),
    )
  }

  let isFocused: boolean
</script>

<div class="container" class:isFocused>
  <NavigableRow>
    {#each tabs as tab, i}
      <SimpleNavigableItem
        onPress={() => navigate(tab.uri)}
        onLongPress={() => tab.subMenu && showSubMenu(tab.subMenu)}
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
        <ItemStyleLayer>
          <div class="tab">
            {tab.label}

            {#if tab.subMenu}
              <span class="dropdown">▽</span>
            {/if}
          </div>
        </ItemStyleLayer>
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

  .dropdown {
    font-size: 8px;
  }
</style>
