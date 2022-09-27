<script lang="ts">
  import QueueGalleryTrack from '../../molecules/QueueGalleryTrack/QueueGalleryTrack.svelte'
  import NavigableRow from '../../navigable/NavigableRow/NavigableRow.svelte'
  import { queuePosition, readablePlayQueue } from '../../stores/play-queue'
</script>

{#if $readablePlayQueue && $queuePosition !== null}
  <NavigableRow>
    <div class="queue-gallery">
      {#each $readablePlayQueue.tracks as track (track.idInQueue)}
        <QueueGalleryTrack
          isCurrent={$queuePosition === $readablePlayQueue.tracks.indexOf(track)}
          position={$readablePlayQueue.tracks.indexOf(track)}
          columns={7}
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
