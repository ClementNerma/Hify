<script lang="ts">
  import { navigate } from 'svelte-navigator'

  import { ROUTES } from '../../routes'
  import { getAlbumArtUri } from '../../rest-api'
  import { AsyncAlbumPage, AudioTrackFragment } from '../../graphql/generated'

  import NavigableList from '../../navigable/NavigableList/NavigableList.svelte'
  import NavigableRow from '../../navigable/NavigableRow/NavigableRow.svelte'
  import SimpleNavigableItem from '../../navigable/SimpleNavigableItem/SimpleNavigableItem.svelte'

  import NavigableTrack from '../../atoms/NavigableTrack/NavigableTrack.svelte'
  import { bind, hasMinimumNote } from '../../utils'
  import TrackNote from '../../atoms/TrackNote/TrackNote.svelte'
  import Checkbox from '../../atoms/Checkbox/Checkbox.svelte'

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
    return onlyShowGreatSongs ? tracks.filter((track) => hasMinimumNote(track, 80)) : tracks
  }

  let onlyShowGreatSongs = false
</script>

{#await album}
  <h2>Loading...</h2>
{:then album}
  {@const filteredTracks = filterTracks(album.tracks, onlyShowGreatSongs)}

  <NavigableList>
    <div class="album-header">
      <div class="album-art">
        <img width={150} height={150} src={getAlbumArtUri(albumId)} alt="" />
      </div>
      <div class="album-infos">
        <div class="album-name">{album.name}</div>
        <div class="album-artists">
          <NavigableRow>
            {#each album.albumArtists as albumArtist}
              <SimpleNavigableItem onPress={bind(albumArtist.id, (id) => navigate(ROUTES.artist(id)))}>
                <span class="album-artist">
                  {albumArtist.name}
                </span>
              </SimpleNavigableItem>
            {/each}
          </NavigableRow>
        </div>
        <div class="album-year">{album.year ?? '<unknown year>'}</div>
        <div class="album-genres">
          <NavigableRow>
            {#each album.genres as genre}
              <SimpleNavigableItem onPress={bind(genre.id, (id) => navigate(ROUTES.genre(id)))}>
                <span class="album-genre">
                  {genre.name}
                </span>
              </SimpleNavigableItem>
            {/each}
          </NavigableRow>
        </div>

        <Checkbox bind:checked={onlyShowGreatSongs}>Only show great songs</Checkbox>
      </div>
    </div>
  </NavigableList>

  <NavigableList>
    <table>
      <tbody>
        {#each filteredTracks as track (track.id)}
          {@const tags = track.metadata.tags}

          <NavigableTrack transparent={true} tracks={filteredTracks} goToAlbumOption={false} {track}>
            <tr>
              <td class="trackno">{tags.trackNo}</td>
              <td class="title">{tags.title}</td>
              <td class="note">
                {#if tags.note}
                  <TrackNote note={tags.note} />
                {/if}
              </td>
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
  .album-header {
    display: flex;
    flex-direction: row;
    width: 90%;
    margin-left: 5%;
  }

  .album-infos {
    padding: 10px;
    min-height: 100%;
  }

  .album-name {
    font-weight: bold;
    font-size: 2em;
  }

  .album-artist {
    font-size: 1.5rem;
  }

  .album-genre {
    font-size: 0.75rem;
    padding: 7px;
  }

  .album-year {
    font-size: 1rem;
    padding: 7px;
  }

  table {
    width: 90%;
    margin-left: 5%;
    border-collapse: collapse;
  }

  tr {
    width: 100%;
  }

  td {
    padding: 10px;
  }
</style>
