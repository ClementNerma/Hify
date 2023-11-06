<script lang="ts">
  import Row from '@navigable/ui/molecules/Row/Row.svelte'
  import { ArtistCardFragment } from '@graphql/generated'
  import ArtistCard from '@molecules/ArtistCard/ArtistCard.svelte'

  export let artists: ArtistCardFragment[]
</script>

<Row>
  {#each artists as artist}
    <ArtistCard {artist} />
  {/each}
</Row>
