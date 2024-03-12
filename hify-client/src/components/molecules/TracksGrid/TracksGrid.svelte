<script lang="ts">
import type { EntryInPlaylist } from '@globals/context-menu-items'
import type { AudioTrackFragment } from '@graphql/generated'
import TrackCard from '@molecules/TrackCard/TrackCard.svelte'
import Grid from '@navigable/ui/organisms/Grid/Grid.svelte'
import { GRID_TRACKS_PER_ROW } from '@root/constants'

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
