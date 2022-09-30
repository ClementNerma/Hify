<script lang="ts">
  import QueueGalleryTrack from '../../molecules/QueueGalleryTrack/QueueGalleryTrack.svelte'
  import NavigableRow from '../../navigable/headless/NavigableRow/NavigableRow.svelte'
  import { queuePosition, readablePlayQueue } from '../../stores/play-queue'

  const COLUMNS = 8
</script>

{#if $readablePlayQueue}
  <NavigableRow>
    <div class="queue-gallery">
      {#each $readablePlayQueue.tracks as track (track.idInQueue)}
        <QueueGalleryTrack
          isCurrent={$queuePosition === $readablePlayQueue.tracks.indexOf(track)}
          position={$readablePlayQueue.tracks.indexOf(track)}
          columns={COLUMNS}
          {track}
        />
      {/each}
    </div>
  </NavigableRow>
{/if}

<style>
  .queue-gallery {
    display: flex;
    flex-direction: row;
    overflow-x: auto;
  }
</style>
