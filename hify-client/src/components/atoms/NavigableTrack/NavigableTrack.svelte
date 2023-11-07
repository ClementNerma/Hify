<script context="module" lang="ts">
  export type NavigableTrackInPlaylist = {
    playlistId: string
    trackEntry: PlaylistEntryFragment
    allEntries: PlaylistEntryFragment[]
  }
</script>

<script lang="ts">
  import { navigate } from 'svelte-navigator'
  import { AudioTrackFragment, EditPlaylist, MixParams, PlaylistEntryFragment } from '@graphql/generated'
  import { ContextMenuOption, showContextMenu } from '@navigable/ui/molecules/ContextMenu/ContextMenu'

  import SimpleNavigableItem from '@navigable/headless/SimpleNavigableItem/SimpleNavigableItem.svelte'
  import { ItemDisplay } from '@navigable/headless/SimpleNavigableItem/SimpleNavigableItem.svelte'
  import { ROUTES } from '@root/routes'
  import { enqueue, playTrackFromNewQueue } from '@stores/play-queue'

  export let tracks: AudioTrackFragment[]
  export let track: AudioTrackFragment
  export let inPlaylist: NavigableTrackInPlaylist | null = null
  export let goToAlbumOption = true
  export let display: ItemDisplay = null
  export let fromMixParams: MixParams | null = null

  function play() {
    playTrackFromNewQueue(tracks, tracks.indexOf(track), fromMixParams)
    navigate(ROUTES.nowPlaying)
  }

  function buildContextMenu(): ContextMenuOption[] {
    const options: ContextMenuOption[] = []

    if (inPlaylist) {
      const { playlistId, trackEntry, allEntries } = inPlaylist
      const position = allEntries.findIndex((entry) => entry.id === trackEntry.id)

      // TODO: when modifying, refresh parent components
      options.push(
        {
          label: 'Move up',
          onPress: async () => {
            await EditPlaylist({
              variables: {
                playlistId,
                action: {
                  move: { entries: [trackEntry.id], putAfter: position === 0 ? null : allEntries[position - 1].id },
                },
              },
            })
          },
        },
        {
          label: 'Move down',
          onPress: async () => {
            await EditPlaylist({
              variables: {
                playlistId,
                action: {
                  move: { entries: [trackEntry.id], putAfter: allEntries[position].id },
                },
              },
            })
          },
        },
        {
          label: 'Remove from playlist',
          onPress: async () => {
            await EditPlaylist({
              variables: {
                playlistId,
                action: {
                  remove: {
                    entries: [trackEntry.id],
                  },
                },
              },
            })
          },
        }
      )
    }

    if (goToAlbumOption) {
      options.push({ label: 'Go to album', onPress: () => navigate(ROUTES.album(track.metadata.tags.album.id)) })
    }

    options.push(
      { label: 'Play next', onPress: () => enqueue([track], 'next') },
      { label: 'Play last', onPress: () => enqueue([track], 'end') },
      { label: 'Play alone', onPress: () => playTrackFromNewQueue([track], 0, fromMixParams) }
    )

    return options
  }
</script>

<SimpleNavigableItem
  let:item
  let:focused
  onPress={play}
  onLongPress={() => showContextMenu(buildContextMenu())}
  {display}
  fullHeight
>
  <slot {item} {focused} />
</SimpleNavigableItem>
