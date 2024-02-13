<script lang="ts">
  import QueueGalleryTrack from '@molecules/QueueGalleryTrack/QueueGalleryTrack.svelte'
  import NavigableRow from '@navigable/headless/NavigableRow/NavigableRow.svelte'
  import { NavigableRow as Nav } from '@navigable/headless/NavigableRow/NavigableRow'
  import { Props, RequestFocus } from '@navigable/navigation'
  import { queuePosition, readablePlayQueue } from '@stores/play-queue'
  import { afterUpdate } from 'svelte'

  const COLUMNS = 7

  export let onFocusChangeCallback: Props<Nav>['onFocusChangeCallback'] = null

  let selected = 0
  let prevSelected = 0

  $: firstDisplayedTrack = Math.max(selected - Math.round((COLUMNS - 1) / 2), 0)

  const requestFocusByPosition: Record<number, RequestFocus> = {}

  afterUpdate(() => {
    if (selected !== prevSelected) {
      prevSelected = selected
      requestFocusByPosition[selected]?.()
    }
  })
</script>

{#if $readablePlayQueue}
  {@const tracks = $readablePlayQueue.tracks.slice(firstDisplayedTrack, firstDisplayedTrack + COLUMNS)}

  <NavigableRow {onFocusChangeCallback}>
    <div class="queue-gallery">
      {#each tracks as track (track.idInQueue)}
        {@const position = $readablePlayQueue.tracks.indexOf(track)}

        <QueueGalleryTrack
          {position}
          isCurrent={$queuePosition === $readablePlayQueue.tracks.indexOf(track)}
          totalTracks={$readablePlayQueue.tracks.length}
          onNavigate={(newPos) => {
            selected = newPos
          }}
          columns={COLUMNS}
          bind:requestFocus={requestFocusByPosition[position]}
          {track}
        />
      {/each}
    </div>
  </NavigableRow>
{/if}

<style>
  .queue-gallery {
    padding: 7px 0;
    display: flex;
    flex-direction: row;
    overflow: hidden;
  }
</style>
