<script lang="ts">
  import { afterUpdate } from 'svelte'

  import { AudioTrackFragment } from '../../graphql/generated'
  import SimpleNavigableItem from '../../navigable/headless/SimpleNavigableItem/SimpleNavigableItem.svelte'
  import { playTrackFromCurrentQueue, removeFromQueue } from '../../stores/play-queue'
  import { bind } from '../../globals/utils'
  import Card from '../Card/Card.svelte'
  import { showContextMenu } from '../../navigable/ui/molecules/ContextMenu/ContextMenu'
  import { ctxMenuOptions } from '../../globals/context-menu-items'

  export let track: AudioTrackFragment
  export let position: number
  export let isCurrent: boolean
  export let columns: number

  let wasCurrent = isCurrent

  afterUpdate(() => {
    if (!wasCurrent && isCurrent) {
      wrapper.scrollIntoView({ block: 'nearest', inline: 'nearest', behavior: 'smooth' })
    }

    wasCurrent = isCurrent
  })

  $: contextMenuOptions = [ctxMenuOptions.goToAlbum(track.metadata.tags.album.id)].concat(
    isCurrent ? [] : [{ label: 'Remove from queue', onPress: () => removeFromQueue(position) }],
  )

  let wrapper: HTMLDivElement
</script>

<div class="track" style="--column-size: {`${100 / columns}%`}" class:isCurrent bind:this={wrapper}>
  <SimpleNavigableItem
    onPress={bind(position, (position) => playTrackFromCurrentQueue(position))}
    onLongPress={() => showContextMenu(contextMenuOptions)}
    hasFocusPriority={isCurrent}
    fullHeight
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
