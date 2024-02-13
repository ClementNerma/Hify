<script lang="ts">
  import QueueGalleryTrack from '@molecules/QueueGalleryTrack/QueueGalleryTrack.svelte'
  import NavigableRow from '@navigable/headless/NavigableRow/NavigableRow.svelte'
  import { NavigableRow as Nav } from '@navigable/headless/NavigableRow/NavigableRow'
  import { Props, RequestFocus } from '@navigable/navigation'
  import { queuePosition, readablePlayQueue } from '@stores/play-queue'
  import { afterUpdate } from 'svelte'
  import { bind } from '@globals/utils'

  const COLUMNS = 7

  export let onFocusChangeCallback: Props<Nav>['onFocusChangeCallback'] = null

  let selected: string | null = null
  let prevSelected: string | null = null

  $: selectedTrackPosition = $readablePlayQueue.tracks.findIndex((track) => track.idInQueue === selected)
  $: firstVisibleTrack = Math.max(selectedTrackPosition - Math.round((COLUMNS - 1) / 2), 0)
  $: visibleTracks = $readablePlayQueue.tracks.slice(firstVisibleTrack, firstVisibleTrack + COLUMNS)

  const requestFocusByIdInQueue: Record<string, RequestFocus> = {}

  afterUpdate(() => {
    if (selected === null) {
      return
    }

    if (selected !== prevSelected) {
      prevSelected = selected
      requestFocusByIdInQueue[selected]()
    }
  })
</script>

<NavigableRow {onFocusChangeCallback}>
  <div class="queue-gallery">
    {#each visibleTracks as track (track.idInQueue)}
      {@const position = $readablePlayQueue.tracks.indexOf(track)}

      <QueueGalleryTrack
        {position}
        isCurrent={$queuePosition === $readablePlayQueue.tracks.indexOf(track)}
        totalTracks={$readablePlayQueue.tracks.length}
        columns={COLUMNS}
        hasFocusPriority={selected === track.idInQueue}
        onNavigate={(newPos) => {
          selected = $readablePlayQueue.tracks[newPos].idInQueue
        }}
        onFocus={bind(track, (track) => {
          selected = track.idInQueue
        })}
        bind:requestFocus={requestFocusByIdInQueue[track.idInQueue]}
        {track}
      />
    {/each}
  </div>
</NavigableRow>

<style>
  .queue-gallery {
    padding: 7px 0;
    display: flex;
    flex-direction: row;
    overflow: hidden;
  }
</style>
