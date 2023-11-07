<script lang="ts">
  import NavigableTrack, { NavigableTrackInPlaylist } from '@atoms/NavigableTrack/NavigableTrack.svelte'
  import { AudioTrackFragment } from '@graphql/generated'
  import Card from '@molecules/Card/Card.svelte'

  export let track: AudioTrackFragment
  export let tracks: AudioTrackFragment[]
  export let inPlaylist: NavigableTrackInPlaylist | null = null
</script>

<NavigableTrack {track} {tracks} {inPlaylist}>
  <Card title={track.metadata.tags.title} art={track.metadata.tags.album.art} />
</NavigableTrack>
