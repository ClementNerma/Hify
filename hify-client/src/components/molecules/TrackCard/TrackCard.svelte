<script lang="ts">
  import NavigableTrack from '@atoms/NavigableTrack/NavigableTrack.svelte'
  import { EntryInPlaylist } from '@globals/context-menu-items'
  import { AudioTrackFragment } from '@graphql/generated'
  import Card from '@molecules/Card/Card.svelte'
  import { SimpleNavigableItemProps } from '@navigable/headless/SimpleNavigableItem/SimpleNavigableItem'

  export let track: AudioTrackFragment
  export let tracks: AudioTrackFragment[]
  export let inPlaylist: EntryInPlaylist | null = null
  export let onFocus: SimpleNavigableItemProps['onFocus'] = undefined
</script>

<NavigableTrack
  {track}
  {tracks}
  context={inPlaylist ? { context: 'playlist', entry: inPlaylist } : { context: 'normal' }}
  {onFocus}
>
  <Card title={track.metadata.tags.title} art={track.metadata.tags.album.art} />
</NavigableTrack>
