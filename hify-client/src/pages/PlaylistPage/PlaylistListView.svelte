<script lang="ts">
import NavigableTrack from '@atoms/NavigableTrack/NavigableTrack.svelte'
import TrackRating from '@atoms/TrackRating/TrackRating.svelte'
import { EntryInPlaylist } from '@globals/context-menu-items'
import { AudioTrackFragment } from '@graphql/generated'
import { NavigableListProps } from '@navigable/headless/NavigableList/NavigableList'
import NavigableList from '@navigable/headless/NavigableList/NavigableList.svelte'
import { humanReadableDuration } from '@stores/audio-player'

export let tracks: AudioTrackFragment[] = []
export let inPlaylist: Omit<EntryInPlaylist, 'trackEntry'>
export let feedMore: NavigableListProps['lazyLoader']
</script>

<NavigableList lazyLoader={feedMore}>
  <table>
    <tbody>
      {#each tracks as track, i (track.id)}
        {@const tags = track.metadata.tags}
        <NavigableTrack
          {track}
          {tracks}
          context={{ context: 'playlist', entry: { ...inPlaylist, trackEntry: inPlaylist.allEntries[i] } }}
          display="transparent"
        >
          <tr class:notFirst={i !== 0}>
            <td class="trackno">{tags.trackNo}</td>
            <td class="title">🎵 {tags.title}</td>
            <td class="album">💿 {tags.album.name}</td>
            <td class="artists">
              {tags.artists
                .slice(0, 3)
                .map((artist) => `🎤 ${artist.name}`)
                .join(', ')}

              {tags.artists.length > 3 ? '...' : ''}</td
            >
            <td class="rating">
              {#if track.computedRating}
                <TrackRating rating={track.computedRating} />
              {/if}
            </td>
            <td class="duration">{humanReadableDuration(track.metadata.duration)}</td>
          </tr>
        </NavigableTrack>
      {/each}
    </tbody>
  </table>
</NavigableList>

<style>
  table {
    margin-top: 10px;
    width: 100%;
    border-collapse: collapse;
  }

  tr {
    width: 100%;
  }

  tr.notFirst {
    border-top: 1px solid rgb(50, 50, 50);
  }

  td {
    padding: 10px;
  }

  td.duration {
    text-align: right;
  }
</style>
