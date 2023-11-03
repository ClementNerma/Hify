<script lang="ts">
  import LoadingIndicator from '../../atoms/LoadingIndicator/LoadingIndicator.svelte'
  import NavigableTrack from '../../atoms/NavigableTrack/NavigableTrack.svelte'
  import TrackRating from '../../atoms/TrackRating/TrackRating.svelte'
  import { AsyncPlaylistPage, AudioTrackFragment, PlaylistPageQuery } from '../../graphql/generated'
  import NavigableList from '../../navigable/headless/NavigableList/NavigableList.svelte'
  import { humanReadableDuration } from '../../stores/audio-player'

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
</script>

{#await playlist}
  <LoadingIndicator />
{:then playlist}
  <h2>{playlist.name}</h2>

  <NavigableList lazyLoader={feedMore}>
    <table>
      <tbody>
        {#each playlist.tracks.nodes as track, i (track.id)}
          {@const tags = track.metadata.tags}

          <NavigableTrack {tracks} goToAlbumOption={false} display="transparent" {track}>
            <tr class:notFirst={i !== 0}>
              <td class="trackno">{tags.trackNo}</td>
              <td class="title">🎵 {tags.title}</td>
              <td class="album">💿 {tags.album.name}</td>
              <td class="artists">
                {tags.artists
                  .slice(0, 3)
                  .map((artist) => `🎤 ${artist.name}`)
                  .join(', ')}

                {tags.artists.length > 3 ? '...' : ''}</td
              >
              <td class="rating">
                {#if track.computedRating}
                  <TrackRating rating={track.computedRating} />
                {/if}
              </td>
              <td class="duration">{humanReadableDuration(track.metadata.duration)}</td>
            </tr>
          </NavigableTrack>
        {/each}
      </tbody>
    </table>
  </NavigableList>
{:catch e}
  <h2>Failed: {e.message}</h2>
{/await}

<style>
  table {
    margin-top: 10px;
    width: 100%;
    border-collapse: collapse;
  }

  tr {
    width: 100%;
  }

  tr.notFirst {
    border-top: 1px solid rgb(50, 50, 50);
  }

  td {
    padding: 10px;
  }

  td.duration {
    text-align: right;
  }
</style>
