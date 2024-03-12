<script lang="ts">
import { AudioTrackFragment } from '@graphql/generated'
import Grid from '@navigable/ui/organisms/Grid/Grid.svelte'
import { GRID_TRACKS_PER_ROW } from '@root/constants'
import TrackCard from '@molecules/TrackCard/TrackCard.svelte'
import { EntryInPlaylist } from '@globals/context-menu-items'

export let tracks: AudioTrackFragment[]
export let inPlaylist: Omit<EntryInPlaylist, 'trackEntry'> | null = null
export let feedMore: (() => void) | undefined = undefined
</script>

<Grid columns={GRID_TRACKS_PER_ROW} lazyLoader={feedMore}>
  {#each tracks as track, i}
    {@const trackInPlaylist = inPlaylist ? { ...inPlaylist, trackEntry: inPlaylist.allEntries[i] } : null}

    <TrackCard {track} {tracks} inPlaylist={trackInPlaylist} />
  {/each}
</Grid>
