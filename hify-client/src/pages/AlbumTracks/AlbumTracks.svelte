<script lang="ts">
  import { API_SERVER_URL } from '../../apollo-client'
  import { playAudio } from '../../store'

  import { AsyncAlbumTracks } from './AlbumTracks.generated'

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
</script>

<table>
  <tbody>
    {#await albumTracks}
      <tr>
        <td />
        <td><em>Loading...</em></td>
      </tr>
    {:then tracks}
      {#each tracks as track}
        <tr>
          <td class="play" on:click={() => playAudio(`${API_SERVER_URL}/stream/${track.id}`)}>🔊</td>
          <td class="trackno">{track.metadata.tags.trackNo}</td>
          <td class="title">{track.metadata.tags.title}</td>
        </tr>
      {/each}
    {:catch e}
      <tr>
        <td />
        <td><strong>Failed: {e.message}</strong></td>
      </tr>
    {/await}
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

  tr:hover {
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
