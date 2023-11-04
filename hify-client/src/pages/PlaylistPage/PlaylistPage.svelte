<script lang="ts">
  import Checkbox from '../../atoms/Checkbox/Checkbox.svelte'
  import LoadingIndicator from '../../atoms/LoadingIndicator/LoadingIndicator.svelte'
  import { AsyncPlaylistPage, AudioTrackFragment } from '../../graphql/generated'
  import PlaylistGridView from './PlaylistGridView.svelte'
  import PlaylistListView from './PlaylistListView.svelte'

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

    currentPageInfo = res.data.playlist.tracks.pageInfo
    tracks = [...tracks, ...res.data.playlist.tracks.nodes]

    return res.data.playlist
  }

  export let playlistId: string

  const playlist = feedMore().then((playlist) => playlist!)

  let tracks: AudioTrackFragment[] = []
  let gridView = false
</script>

{#await playlist}
  <LoadingIndicator />
{:then playlist}
  <h2>{playlist.name}</h2>
  <Checkbox bind:checked={gridView}>Enable grid view</Checkbox>

  {#if gridView}
    <PlaylistGridView {feedMore} {tracks} />
  {:else}
    <PlaylistListView {feedMore} {tracks} />
  {/if}
{:catch e}
  <h2>Failed: {e.message}</h2>
{/await}
