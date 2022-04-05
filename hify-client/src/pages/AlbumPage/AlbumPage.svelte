<script lang="ts">
  import { getAlbumArtUri } from '../../rest-api'
  import { playTrack } from '../../stores/audio/store'

  import { AlbumYearStrategy } from '../../graphql/types'
  import { AsyncAlbumPage } from './AlbumPage.generated'

  import NavigableList from '../../navigable/NavigableList/NavigableList.svelte'
  import NavigableRow from '../../navigable/NavigableRow/NavigableRow.svelte'
  import SimpleNavigableItem from '../../navigable/SimpleNavigableItem/SimpleNavigableItem.svelte'
  import { navigate } from 'svelte-navigator'
  import { ROUTES } from '../../routes'

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
</script>

{#await album}
  <h2>Loading...</h2>
{:then album}
  <NavigableList>
    <div class="album-header">
      <div class="album-art">
        <img width={250} height={250} src={getAlbumArtUri(albumId)} alt={album.name} />
      </div>
      <div class="album-infos">
        <div class="album-name">{album.name}</div>
        <div class="album-artists">
          <NavigableRow>
            {#each album.albumArtists as albumArtist}
              <SimpleNavigableItem onPress={() => navigate(ROUTES.artist(albumArtist.id))}>
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
              <SimpleNavigableItem onPress={() => alert("TODO: go to genre's page: " + genre)}>
                <span class="album-genre">
                  {genre}
                </span>
              </SimpleNavigableItem>
            {/each}
          </NavigableRow>
        </div>
      </div>
    </div>
  </NavigableList>

  <NavigableList>
    <table>
      <tbody>
        {#each album.tracks as track}
          <SimpleNavigableItem onPress={() => playTrack(track.id)}>
            <tr>
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

  .album-genre {
    font-size: 1em;
    padding: 7px;
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

  /* tr.active {
    background-color: #c9c9c9;
  } */

  td {
    padding: 10px;
  }

  td.play {
    width: 2%;
    text-align: center;
  }

  td.trackno {
    width: 5%;
  }
</style>
