<script lang="ts">
  import Checkbox from '@atoms/Checkbox/Checkbox.svelte'
  import LoadingIndicator from '@atoms/LoadingIndicator/LoadingIndicator.svelte'
  import { AsyncPlaylistPage, PlaylistPageQuery } from '@graphql/generated'
  import PlaylistListView from './PlaylistListView.svelte'
  import TracksGrid from '@organisms/TracksGrid/TracksGrid.svelte'
  import type { EntryInPlaylist } from '@globals/context-menu-items'

  const TRACKS_BULK = 50

  let currentPageInfo: { endCursor?: string | null; hasNextPage: boolean } | null = null

  const feedMore = async () => {
    if (currentPageInfo?.hasNextPage === false) {
      return
    }

    const res = await AsyncPlaylistPage({
      variables: {
        playlistId,
        tracksPagination: {
          after: currentPageInfo?.endCursor,
          first: TRACKS_BULK,
        },
      },
    })

    currentPageInfo = res.data.playlist.entries.pageInfo
    playlistEntries = [...playlistEntries, ...res.data.playlist.entries.nodes]

    return res.data.playlist
  }

  export let playlistId: string

  const playlist = feedMore().then((playlist) => playlist!)

  let playlistEntries: PlaylistPageQuery['playlist']['entries']['nodes'] = []
  let gridView = false

  const inPlaylist: Omit<EntryInPlaylist, 'trackEntry'> = {
    playlistId,
    allEntries: playlistEntries,
  }

  $: tracks = playlistEntries.map((entry) => entry.track)
</script>

{#await playlist}
  <LoadingIndicator />
{:then playlist}
  <h2>{playlist.name}</h2>
  <Checkbox bind:checked={gridView}>Enable grid view</Checkbox>

  {#if gridView}
    <TracksGrid {tracks} {inPlaylist} {feedMore} />
  {:else}
    <PlaylistListView {tracks} {inPlaylist} {feedMore} />
  {/if}
{:catch e}
  <h2>Failed: {e.message}</h2>
{/await}
