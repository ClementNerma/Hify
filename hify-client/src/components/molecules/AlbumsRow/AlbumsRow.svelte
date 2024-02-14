<script lang="ts">
  import Row from '@navigable/ui/molecules/Row/Row.svelte'
  import { AlbumCardFragment } from '@graphql/generated'
  import AlbumCard from '@molecules/AlbumCard/AlbumCard.svelte'

  export let albums: AlbumCardFragment[]

  const INITIAL_ITEMS = 10

  let displaying = INITIAL_ITEMS

  function reached(index: number) {
    if (index >= displaying - 2) {
      displaying += INITIAL_ITEMS
    }
  }
</script>

<Row>
  {#each albums.slice(0, displaying) as album, i (album.id)}
    <AlbumCard {album} onFocus={() => reached(i)} />
  {/each}
</Row>
