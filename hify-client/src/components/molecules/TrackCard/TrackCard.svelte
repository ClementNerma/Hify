<script lang="ts">
  import NavigableTrack, { NavigableTrackInPlaylist } from '@atoms/NavigableTrack/NavigableTrack.svelte'
  import { AudioTrackFragment } from '@graphql/generated'
  import Card from '@molecules/Card/Card.svelte'
  import { SimpleNavigableItemProps } from '@navigable/headless/SimpleNavigableItem/SimpleNavigableItem'

  export let track: AudioTrackFragment
  export let tracks: AudioTrackFragment[]
  export let inPlaylist: NavigableTrackInPlaylist | null = null
  export let onFocus: SimpleNavigableItemProps['onFocus'] = undefined
</script>

<NavigableTrack {track} {tracks} {inPlaylist} {onFocus}>
  <Card title={track.metadata.tags.title} art={track.metadata.tags.album.art} />
</NavigableTrack>
