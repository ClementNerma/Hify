<script lang="ts">
  import QueueGalleryTrack from '../../molecules/QueueGalleryTrack/QueueGalleryTrack.svelte'
  import NavigableRow from '../../navigable/NavigableRow/NavigableRow.svelte'
  import { queuePosition, readablePlayQueue } from '../../stores/play-queue'

  const SIDE_TRACKS = 3

  $: nearTracks =
    $queuePosition === null || $queuePosition < SIDE_TRACKS
      ? $readablePlayQueue.tracks.slice(0, SIDE_TRACKS * 2 + 1)
      : $queuePosition >= $readablePlayQueue.tracks.length - SIDE_TRACKS
      ? $readablePlayQueue.tracks.slice(-SIDE_TRACKS * 2 - 1)
      : $readablePlayQueue.tracks.slice($queuePosition - SIDE_TRACKS, $queuePosition + SIDE_TRACKS + 1)
</script>

{#if $readablePlayQueue && $queuePosition !== null}
  <NavigableRow>
    <div class="queue-gallery">
      {#each nearTracks as track (track.id)}
        <QueueGalleryTrack
          current={$queuePosition === $readablePlayQueue.tracks.indexOf(track)}
          position={$readablePlayQueue.tracks.indexOf(track)}
          columns={SIDE_TRACKS * 2 + 1}
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
  }
</style>
