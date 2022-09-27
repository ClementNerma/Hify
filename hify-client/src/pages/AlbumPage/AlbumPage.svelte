<script lang="ts">
  import { navigate } from 'svelte-navigator'

  import { ROUTES } from '../../routes'
  import { getAlbumArtUri } from '../../rest-api'
  import { AsyncAlbumPage, AudioTrackFragment } from '../../graphql/generated'

  import NavigableList from '../../navigable/NavigableList/NavigableList.svelte'
  import NavigableRow from '../../navigable/NavigableRow/NavigableRow.svelte'
  import SimpleNavigableItem from '../../navigable/SimpleNavigableItem/SimpleNavigableItem.svelte'

  import NavigableTrack from '../../atoms/NavigableTrack/NavigableTrack.svelte'
  import { bind, hasMinimumRating } from '../../utils'
  import TrackRating from '../../atoms/TrackRating/TrackRating.svelte'
  import Checkbox from '../../atoms/Checkbox/Checkbox.svelte'
  import Button from '../../atoms/Button/Button.svelte'
  import Emoji from '../../atoms/Emoji/Emoji.svelte'
  import { queueAsNext } from '../../stores/play-queue'
  import Row from '../../atoms/Row/Row.svelte'

  export let albumId: string

  const album = AsyncAlbumPage({
    variables: {
      albumId,
    },
  }).then((res) => {
    const album = res.data.album

    if (!album) {
      throw new Error("ERROR: Failed to fetch album's data")
    }

    return album
  })

  function filterTracks(tracks: AudioTrackFragment[], onlyShowGreatSongs: boolean): AudioTrackFragment[] {
    return onlyShowGreatSongs ? tracks.filter((track) => hasMinimumRating(track, 80)) : tracks
  }

  let onlyShowGreatSongs = false
</script>

{#await album}
  <h2>Loading...</h2>
{:then album}
  {@const filteredTracks = filterTracks(album.tracks, onlyShowGreatSongs)}

  <div class="container">
    <NavigableList>
      <div class="header">
        <img class="art" width={256} height={256} src={getAlbumArtUri(albumId)} alt="" />

        <div class="infos">
          <div class="name">
            {album.name}
          </div>

          {#if album.year}
            <div>
              <span class="year" data-item-like-style>🕒 {album.year ?? '?'}</span>
            </div>
          {/if}

          <div class="artists">
            <NavigableRow>
              {#each album.albumArtists as albumArtist}
                <SimpleNavigableItem onPress={bind(albumArtist.id, (id) => navigate(ROUTES.artist(id)))}>
                  <span class="artist">
                    🎤 {albumArtist.name}
                  </span>
                </SimpleNavigableItem>
              {/each}
            </NavigableRow>
          </div>

          <div class="genres">
            <NavigableRow>
              {#each album.genres as genre}
                <SimpleNavigableItem onPress={bind(genre.id, (id) => navigate(ROUTES.genre(id)))}>
                  <span class="genre">
                    🎵 {genre.name}
                  </span>
                </SimpleNavigableItem>
              {/each}
            </NavigableRow>
          </div>

          <Row>
            <Checkbox bind:checked={onlyShowGreatSongs} fullHeight>Only show great songs</Checkbox>
            <Button onPress={() => queueAsNext(filteredTracks)} fullHeight><Emoji>▶️</Emoji> Play next</Button>
          </Row>
        </div>
      </div>
    </NavigableList>

    <NavigableList>
      <table>
        <tbody>
          {#each filteredTracks as track, i (track.id)}
            {@const tags = track.metadata.tags}

            <NavigableTrack position={tags.trackNo} transparent tracks={filteredTracks} goToAlbumOption={false} {track}>
              <tr class={i === 0 ? '' : 'not-first'}>
                <td class="trackno">{tags.trackNo}</td>
                <td class="title">{tags.title}</td>
                <td class="rating">
                  {#if tags.rating}
                    <TrackRating rating={tags.rating} />
                  {/if}
                </td>
              </tr>
            </NavigableTrack>
          {/each}
        </tbody>
      </table>
    </NavigableList>
  </div>
{:catch e}
  <h2>Failed: {e.message}</h2>
{/await}

<style>
  .container {
    margin-left: 10%;
    width: 80%;
  }

  .header {
    display: flex;
    flex-direction: row;
  }

  .infos {
    display: flex;
    flex-direction: column;
    margin-top: 10px;
    margin-left: 10px;
    gap: 10px;
  }

  .infos .name {
    font-size: 2rem;
    font-weight: bold;
  }

  table {
    width: 100%;
    border-collapse: collapse;
  }

  tr {
    width: 100%;
  }

  tr.not-first {
    border-top: 1px solid rgb(50, 50, 50);
  }

  td {
    padding: 10px;
  }

  td.title {
    width: 100%;
  }
</style>
