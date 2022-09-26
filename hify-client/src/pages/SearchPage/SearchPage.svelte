<script lang="ts">
  import { onMount } from 'svelte'

  import { AsyncSearchPage, SearchPageQuery } from '../../graphql/generated'

  import { logInfo } from '../../stores/debugger'

  import SimpleNavigableItem from '../../navigable/SimpleNavigableItem/SimpleNavigableItem.svelte'

  import { RequestFocus } from '../../navigable/navigation'
  import TracksRow from '../../molecules/TracksRow/TracksRow.svelte'
  import AlbumsRow from '../../molecules/AlbumsRow/AlbumsRow.svelte'
  import ArtistsRow from '../../molecules/ArtistsRow/ArtistsRow.svelte'

  export let searchTerms: string = ''

  const MAX_RESULTS_PER_CATEGORY = 50

  async function onChange() {
    if (searchTerms.trim().length === 0) {
      return
    }

    logInfo(`Performing search "${searchTerms}"...`)
    const start = Date.now()

    const res = await AsyncSearchPage({
      variables: {
        limit: MAX_RESULTS_PER_CATEGORY,
        input: searchTerms,
      },
    })

    results = res.data.search

    logInfo(`Received results for search "${searchTerms}" in ${Date.now() - start} ms.`)
  }

  if (searchTerms.length > 0) {
    onChange()
  }

  let results: SearchPageQuery['search'] | null = null

  let searchField: HTMLInputElement

  let requestFocus: RequestFocus

  onMount(() => requestFocus())
</script>

<div class="search-container">
  <SimpleNavigableItem
    onFocus={() => searchField.focus()}
    onUnfocus={() => searchField?.blur()}
    transparent={true}
    bind:requestFocus
  >
    <input
      class="search"
      type="text"
      bind:this={searchField}
      bind:value={searchTerms}
      on:input={onChange}
      on:change={onChange}
    />
  </SimpleNavigableItem>
</div>

{#if results}
  <h2>Tracks ({results.tracks.length})</h2>

  <TracksRow tracks={results.tracks} />

  <h2>Albums ({results.albums.length})</h2>

  <AlbumsRow albums={results.albums} />

  <h2>Artists ({results.artists.length})</h2>

  <ArtistsRow artists={results.artists} />
{/if}

<style>
  .search-container {
    padding: 10px;
    text-align: center;
  }

  .search {
    border: none;
    border-radius: 10px;
    outline: none;
    width: 33%;
    padding: 12px;
    font-size: 1.2rem;
  }
</style>
