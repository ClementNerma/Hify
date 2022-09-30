<script lang="ts">
  import { afterUpdate } from 'svelte'

  import { navigate } from 'svelte-navigator'
  import { AudioTrackFragment } from '../../graphql/generated'
  import SimpleNavigableItem from '../../navigable/SimpleNavigableItem/SimpleNavigableItem.svelte'
  import { contextMenuStore } from '../../pages/Template/TplContextMenu.svelte'
  import { getAlbumArtUri } from '../../rest-api'
  import { ROUTES } from '../../routes'
  import { playTrackFromCurrentQueue, removeFromQueue } from '../../stores/play-queue'
  import { bind } from '../../utils'
  import Card from '../Card/Card.svelte'
  import { ContextMenuOption, showContextMenu } from '../ContextMenu/ContextMenu.svelte'

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

  function buildContextMenuOptions(): ContextMenuOption[] {
    const options = [{ label: 'Go to album', onPress: () => navigate(ROUTES.album(track.metadata.tags.album.id)) }]

    if (!isCurrent) {
      options.push({ label: 'Remove from queue', onPress: () => removeFromQueue(position) })
    }

    return options
  }

  let wrapper: HTMLDivElement
</script>

<div class="track" style="--column-size: {`${100 / columns}%`}" class:isCurrent bind:this={wrapper}>
  <SimpleNavigableItem
    {position}
    onPress={bind(position, (position) => playTrackFromCurrentQueue(position))}
    onLongPress={() => showContextMenu(contextMenuStore, buildContextMenuOptions())}
    hasFocusPriority={isCurrent}
    fullHeight
  >
    <Card
      title={track.metadata.tags.title}
      subtitle={null}
      boxSize={80}
      pictureUrl={getAlbumArtUri(track.metadata.tags.album.id)}
    />
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
