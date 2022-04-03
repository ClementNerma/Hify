<script lang="ts">
  import { AsyncHomePage } from './HomePage.generated'

  import AlbumsPage from '../AlbumsPage/AlbumsPage.svelte'

  const indexInfos = AsyncHomePage({ variables: {} }).then((res) => res.data.indexInfos)
</script>

{#await indexInfos}
  <h1>Loading...</h1>
{:then indexInfos}
  <h1>Index fingerprint: <strong><em>{indexInfos.fingerprint}</em></strong></h1>

  <h2>Albums ({indexInfos.albumsCount})</h2>

  <AlbumsPage />
{:catch e}
  <h1>Failed to load homepage: {e.message}</h1>
{/await}
