<script lang="ts">
  import Row from '../../atoms/Row/Row.svelte'
  import { ArtistCardFragment } from '../../graphql/generated'
  import ArtistCard from '../ArtistCard/ArtistCard.svelte'

  export let artists: ArtistCardFragment[]
</script>

<Row>
  {#each artists as artist (artist.id)}
    <ArtistCard {artist} />
  {/each}
</Row>
