<script lang="ts">
  import { useNavigate } from 'svelte-navigator'

  import { getAlbumArtUri } from '../../rest-api'
  import { AlbumYearStrategy, AsyncSearchPage, SearchPageQuery } from '../../graphql/generated'

  import { ROUTES } from '../../routes'

  import { logInfo } from '../../stores/debugger'

  import NavigableRow from '../../navigable/NavigableRow/NavigableRow.svelte'
  import SimpleNavigableItem from '../../navigable/SimpleNavigableItem/SimpleNavigableItem.svelte'
  import NavigableTrack from '../../atoms/NavigableTrack/NavigableTrack.svelte'

  import Card from '../../molecules/Card/Card.svelte'
  import NonInteractiveCard from '../../molecules/Card/NonInteractiveCard.svelte'
  import { onMount } from 'svelte'
  import { CONFIG } from '../../config'
  import { bind } from '../../utils'
  import { Readable } from 'svelte/store'

  export let searchTerms: string = ''

  const MAX_RESULTS_PER_CATEGORY = 50

  const navigate = useNavigate()

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
        albumYearStrategy: CONFIG.albumYearStrategy,
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

  let requestFocus: () => void

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

  <div class="row">
    <NavigableRow>
      {#each results.tracks as track, i (track.id)}
        <NavigableTrack position={i} tracks={results.tracks} {track}>
          <NonInteractiveCard
            title={track.metadata.tags.title}
            subtitle={`${track.metadata.tags.album.name} - ${track.metadata.tags.artists
              .map((artist) => artist.name)
              .join(' / ')}`}
            pictureUrl={getAlbumArtUri(track.metadata.tags.album.id)}
          />
        </NavigableTrack>
      {/each}
    </NavigableRow>
  </div>

  <h2>Albums ({results.albums.length})</h2>

  <div class="row">
    <NavigableRow>
      {#each results.albums as album}
        <Card
          title={album.name}
          subtitle={album.albumArtists.map((artist) => artist.name).join(' / ')}
          pictureUrl={getAlbumArtUri(album.id)}
          onPress={bind(album, (album) => navigate(ROUTES.album(album.id)))}
          onLongPress={() => alert('TODO: context menu for playing options')}
        />
      {/each}
    </NavigableRow>
  </div>

  <h2>Artists ({results.artists.length})</h2>

  <div class="row">
    <NavigableRow>
      {#each results.artists as artist}
        <Card
          title={artist.name}
          subtitle=""
          pictureUrl={'TODO: get picture of first album? and if zero first participation in album?'}
          onPress={bind(artist.id, (id) => navigate(ROUTES.artist(artist.id)))}
          onLongPress={() => alert('TODO: context menu for playing options')}
        />
      {/each}
    </NavigableRow>
  </div>
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

  .row {
    display: flex;
    flex-direction: row;
    overflow-x: auto;
  }
</style>
