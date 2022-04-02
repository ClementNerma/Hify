<script lang="ts">
  import { API_SERVER_URL } from '../../apollo-client'
  import { playAudio } from '../../store'

  import { AsyncAlbumTracks } from './AlbumTracks.generated'

  import NavigableList from '../../navigable/NavigableList/NavigableList.svelte'
  import SimpleNavigableItem from '../../navigable/SimpleNavigableItem/SimpleNavigableItem.svelte'

  export let albumId: string

  const albumTracks = AsyncAlbumTracks({
    variables: {
      albumId,
    },
  }).then((tracks) => {
    const album = tracks.data.album

    if (!album) {
      alert("ERROR: Failed to fetch album's data")
      return []
    }

    return album.tracks
  })

  let focusedTrackId: string | null
</script>

<table>
  <tbody>
    <NavigableList>
      {#await albumTracks}
        <SimpleNavigableItem>
          <tr>
            <td />
            <td><em>Loading...</em></td>
          </tr>
        </SimpleNavigableItem>
      {:then tracks}
        {#each tracks as track}
          <SimpleNavigableItem
            onPress={() => playAudio(`${API_SERVER_URL}/stream/${track.id}`)}
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
      {:catch e}
        <SimpleNavigableItem>
          <tr>
            <td />
            <td><strong>Failed: {e.message}</strong></td>
          </tr>
        </SimpleNavigableItem>
      {/await}
    </NavigableList>
  </tbody>
</table>

<style>
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
