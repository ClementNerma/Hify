<script lang="ts">
  import { useNavigate } from 'svelte-navigator'

  import { SearchPageQuery } from '../../graphql/types'

  import SimpleNavigableItem from '../../navigable/SimpleNavigableItem/SimpleNavigableItem.svelte'

  import CardRow from '../../organisms/CardRow/CardRow.svelte'
  import { getAlbumArtUri } from '../../rest-api'
  import { ROUTES } from '../../routes'
  import { playTrack } from '../../stores/audio/store'
  import { AsyncSearchPage } from './SearchPage.generated'

  export let searchTerms: string = ''

  const MAX_RESULTS_PER_CATEGORY = 50

  const navigate = useNavigate()

  async function onChange() {
    if (searchTerms.trim().length === 0) {
      return
    }

    const res = await AsyncSearchPage({
      variables: {
        limit: MAX_RESULTS_PER_CATEGORY,
        input: searchTerms,
      },
    })

    results = res.data.search
  }

  if (searchTerms.length > 0) {
    onChange()
  }

  let results: SearchPageQuery['search'] | null = null
</script>

<div class="search-container">
  <SimpleNavigableItem>
    <input class="search" type="text" bind:value={searchTerms} on:change={onChange} />
  </SimpleNavigableItem>
</div>

{#if results}
  <h2>Tracks ({results.tracks.length})</h2>

  <CardRow
    items={results.tracks.map(({ id, metadata: { tags } }) => ({
      title: tags.title,
      subtitle: `${tags.album.name} - ${tags.artists.map((artist) => artist.name).join(' / ')}`,
      pictureUrl: getAlbumArtUri(tags.album.id),
      pictureAlt: tags.album.name,
      onPress: () => playTrack(id),
      onLongPress: () => alert('TODO: context menu for playing options'),
    }))}
  />

  <h2>Albums ({results.albums.length})</h2>

  <CardRow
    items={results.albums.map((album) => ({
      title: album.name,
      subtitle: album.albumArtists.map((artist) => artist.name).join(' / '),
      pictureUrl: getAlbumArtUri(album.id),
      pictureAlt: 'Album art',
      onPress: () => navigate(ROUTES.album(album.id)),
      onLongPress: () => alert('TODO: context menu for playing options'),
    }))}
  />

  <h2>Artists ({results.artists.length})</h2>

  <CardRow
    items={results.artists.map((artist) => ({
      title: artist.name,
      subtitle: '',
      pictureUrl: 'TODO: get picture of first album? and if zero first participation in album?',
      pictureAlt: 'Album art',
      onPress: () => navigate(ROUTES.artist(artist.id)),
      onLongPress: () => alert('TODO: context menu for playing options'),
    }))}
  />
{/if}

<style>
  .search-container {
    padding: 10px;
    text-align: center;
  }

  .search {
    border: none;
    outline: none;
    width: 33%;
    padding: 20px;
    font-size: 2rem;
  }
</style>
