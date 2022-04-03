<script lang="ts">
  import { playAudio } from '../../store'
  import { getAlbumArtUri, getStreamUri } from '../../rest-api'

  import { AsyncAlbumPage } from './AlbumPage.generated'
  import { AlbumYearStrategy } from '../../graphql/types'

  import NavigableList from '../../navigable/NavigableList/NavigableList.svelte'
  import NavigableRow from '../../navigable/NavigableRow/NavigableRow.svelte'
  import SimpleNavigableItem from '../../navigable/SimpleNavigableItem/SimpleNavigableItem.svelte'

  export let albumId: string

  const album = AsyncAlbumPage({
    variables: {
      albumId,
      strategy: AlbumYearStrategy.IdenticalOnly,
    },
  }).then((res) => {
    const album = res.data.album

    if (!album) {
      throw new Error("ERROR: Failed to fetch album's data")
    }

    return album
  })

  let focusedArtistId: string | null
  let focusedTrackId: string | null
</script>

{#await album}
  <h2>Loading...</h2>
{:then album}
  <NavigableRow>
    <div class="album-header">
      <div class="album-art">
        <img width={250} height={250} src={getAlbumArtUri(albumId)} alt={album.name} />
      </div>
      <div class="album-infos">
        <div class="album-name">{album.name}</div>
        <div class="album-artists">
          <NavigableRow>
            {#each album.albumArtists as albumArtist}
              <SimpleNavigableItem
                onPress={() => alert("TODO: go to artist's page: " + albumArtist.name)}
                onFocusChange={(has) => {
                  focusedArtistId = has ? albumArtist.id : null
                }}
              >
                <span class="album-artist {focusedArtistId === albumArtist.id ? 'focused' : ''}">
                  {albumArtist.name}
                </span>
              </SimpleNavigableItem>
            {/each}
          </NavigableRow>
        </div>
        <div class="album-year">{album.year ?? '<unknown year>'}</div>
        <!-- TODO: navigable genres to search from -->
        <div class="album-genres">{album.genres.join(', ')}</div>
      </div>
    </div>
  </NavigableRow>

  <NavigableList>
    <table>
      <tbody>
        {#each album.tracks as track}
          <SimpleNavigableItem
            onPress={() => playAudio(getStreamUri(track.id))}
            onFocusChange={(has) => {
              focusedTrackId = has ? track.id : null
            }}
          >
            <tr class={track.id === focusedTrackId ? 'active' : ''}>
              <td class="play">🔊</td>
              <td class="trackno">{track.metadata.tags.trackNo}</td>
              <td class="title">{track.metadata.tags.title}</td>
            </tr>
          </SimpleNavigableItem>
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
  }

  .album-infos {
    padding: 10px;
    min-height: 100%;
  }

  .album-name {
    font-weight: bold;
    font-size: 3em;
  }

  .album-artist {
    font-size: 2em;
    padding: 7px;
  }

  .album-artist.focused {
    border: 5px solid pink;
    padding: 2px;
    border-radius: 10px;
  }

  .album-year {
    font-size: 1.5em;
    padding: 7px;
  }

  table {
    width: 90%;
    margin-left: 5%;
    border-collapse: collapse;
    table-layout: fixed;
  }

  tr {
    background-color: #e3e3e3;
  }

  tr.active {
    background-color: #c9c9c9;
  }

  td {
    padding: 10px;
  }

  td.play {
    width: 2%;
    text-align: center;
    cursor: pointer;
  }

  td.trackno {
    width: 5%;
  }
</style>
