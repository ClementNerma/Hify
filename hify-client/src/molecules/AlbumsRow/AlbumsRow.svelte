<script lang="ts">
  import Row from '../../navigable/ui/molecules/Row/Row.svelte'
  import { AlbumCardFragment } from '../../graphql/generated'
  import AlbumCard from '../AlbumCard/AlbumCard.svelte'

  export let albums: AlbumCardFragment[]
</script>

<Row>
  {#each albums as album}
    <AlbumCard {album} />
  {/each}
</Row>
