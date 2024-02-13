<script lang="ts">
  import { afterUpdate } from 'svelte'

  import { AudioTrackFragment } from '@graphql/generated'
  import SimpleNavigableItem from '@navigable/headless/SimpleNavigableItem/SimpleNavigableItem.svelte'
  import { enqueue, moveTrackPositionInQueue, playTrackFromCurrentQueue, removeFromQueue } from '@stores/play-queue'
  import { bind } from '@globals/utils'
  import Card from '@molecules/Card/Card.svelte'
  import { ContextMenuOption, showContextMenu } from '@navigable/ui/molecules/ContextMenu/ContextMenu'
  import { ctxMenuOptions } from '@globals/context-menu-items'
  import { RequestFocus } from '@navigable/navigation'

  export let track: AudioTrackFragment
  export let position: number
  export let totalTracks: number
  export let isCurrent: boolean
  export let columns: number
  export let onNavigate: (newPosition: number) => void

  // let wasCurrent = isCurrent
  //
  // afterUpdate(() => {
  //   if (!wasCurrent && isCurrent) {
  //     wrapper.scrollIntoView({ block: 'nearest', inline: 'nearest', behavior: 'smooth' })
  //   }
  //
  //   wasCurrent = isCurrent
  // })

  function computeContextMenuOptions(): ContextMenuOption[] {
    const options = [ctxMenuOptions.goToAlbum(track.metadata.tags.album.id)]

    if (!isCurrent) {
      options.push({
        label: 'Remove from queue',
        onPress() {
          removeFromQueue(position)
        },
      })
    }

    if (position > 0) {
      options.push({
        label: 'Move left',
        onPress() {
          moveTrackPositionInQueue(position, position - 1)
        },
      })
    }

    if (position < totalTracks - 1) {
      options.push({
        label: 'Move right',
        onPress() {
          moveTrackPositionInQueue(position, position + 1)
        },
      })
    }

    options.push({
      label: 'Play after current track',
      onPress() {
        enqueue([track], 'next')
      },
    })

    return options
  }

  let wrapper: HTMLDivElement
  let _requestFocus: RequestFocus

  export const requestFocus = () => _requestFocus()
</script>

<div class="track" style="--column-size: {`${100 / columns}%`}" class:isCurrent bind:this={wrapper}>
  <SimpleNavigableItem
    onPress={bind(position, (position) => playTrackFromCurrentQueue(position))}
    onLongPress={() => showContextMenu(computeContextMenuOptions())}
    hasFocusPriority={isCurrent}
    fullHeight
    onLeft={() => onNavigate(Math.max(position - 1, 0))}
    onRight={() => {
      onNavigate(Math.min(position + 1, totalTracks - 1))
    }}
    bind:requestFocus={_requestFocus}
  >
    <Card title={track.metadata.tags.title} subtitle={null} boxSize={80} art={track.metadata.tags.album.art} />
  </SimpleNavigableItem>
</div>

<style>
  .track {
    text-align: center;
    min-width: var(--column-size);
    width: var(--column-size);
    max-width: var(--column-size);
  }

  .track.isCurrent {
    border-radius: 5px;
  }

  .track:not(.track.isCurrent) {
    opacity: 0.2;
  }
</style>
