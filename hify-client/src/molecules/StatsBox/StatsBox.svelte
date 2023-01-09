<script lang="ts">
  import { AsyncGenerateStats } from "../../graphql/generated";
  import { humanReadableDuration } from "../../stores/audio-player";

  const statsReq = AsyncGenerateStats({
    variables: {},
    fetchPolicy: "no-cache",
  });

  $: stats = statsReq.then((stats) => stats.data.generateStats);
</script>

{#await stats}
  <strong>Loading...</strong>
{:then stats}
  <table>
    <tr>
      <td>Total number of tracks</td>
      <td><strong>{stats.tracksCount}</strong></td>
    </tr>
    <tr>
      <td>Total number of albums</td>
      <td><strong>{stats.albumsCount}</strong></td>
    </tr>
    <tr>
      <td>Total number of album artists</td>
      <td><strong>{stats.albumArtistsCount}</strong></td>
    </tr>
    <tr>
      <td>Total number of artists</td>
      <td><strong>{stats.artistsCount}</strong></td>
    </tr>
    <tr>
      <td>Number of listened tracks</td>
      <td><strong>{stats.totalTracksListened}</strong></td>
    </tr>
    <tr>
      <td>Total listening duration</td>
      <td><strong>{humanReadableDuration(stats.totalListeningTime)}</strong></td
      >
    </tr>
  </table>
{/await}
