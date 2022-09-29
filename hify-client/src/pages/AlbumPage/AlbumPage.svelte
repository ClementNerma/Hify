<script lang="ts">
  import { navigate } from 'svelte-navigator'

  import { ROUTES } from '../../routes'
  import { getAlbumArtUri } from '../../rest-api'
  import { AlbumInfos, AsyncAlbumPage, AudioTrackFragment } from '../../graphql/generated'

  import NavigableList from '../../navigable/NavigableList/NavigableList.svelte'
  import NavigableRow from '../../navigable/NavigableRow/NavigableRow.svelte'
  import SimpleNavigableItem from '../../navigable/SimpleNavigableItem/SimpleNavigableItem.svelte'

  import NavigableTrack from '../../atoms/NavigableTrack/NavigableTrack.svelte'
  import { bind, dedup, filterMap, hasMinimumRating, isDefined } from '../../utils'
  import TrackRating from '../../atoms/TrackRating/TrackRating.svelte'
  import Checkbox from '../../atoms/Checkbox/Checkbox.svelte'
  import Button from '../../atoms/Button/Button.svelte'
  import Emoji from '../../atoms/Emoji/Emoji.svelte'
  import { queueAsNext } from '../../stores/play-queue'
  import Row from '../../atoms/Row/Row.svelte'
  import { humanReadableDuration } from '../../stores/audio-player'

  export let albumId: string

  function getAlbumInfos(filteredTracks: AudioTrackFragment[]) {
    const discs = dedup(filterMap(filteredTracks, (track) => track.metadata.tags.disc)).map((num) => ({
      number: num.toString(),
      tracks: filteredTracks.filter((track) => track.metadata.tags.disc === num),
    }))

    const tracksWithoutDisc = filteredTracks.filter((track) => !isDefined(track.metadata.tags.disc))

    if (tracksWithoutDisc.length > 0) {
      discs.unshift({ number: '?', tracks: tracksWithoutDisc })
    }

    return {
      totalDuration: filteredTracks.map((track) => track.metadata.duration).reduce((a, x) => a + x),
      discs,
    }
  }

  const album = AsyncAlbumPage({
    variables: {
      albumId,
    },
  }).then((res) => {
    const album = res.data.album

    if (!album) {
      throw new Error("ERROR: Failed to fetch album's data")
    }

    return album
  })

  function filterTracks(tracks: AudioTrackFragment[], onlyShowGreatSongs: boolean): AudioTrackFragment[] {
    return onlyShowGreatSongs ? tracks.filter((track) => hasMinimumRating(track, 80)) : tracks
  }

  let onlyShowGreatSongs = false
</script>

{#await album}
  <h2>Loading...</h2>
{:then album}
  {@const filteredTracks = filterTracks(album.tracks, onlyShowGreatSongs)}
  {@const { totalDuration, discs } = getAlbumInfos(filteredTracks)}

  <div class="container">
    <NavigableList>
      <div class="header">
        <img class="art" width={192} height={192} src={getAlbumArtUri(albumId)} alt="" />

        <div class="infos">
          <div class="name">
            {album.name}
          </div>

          {#if album.year}
            <div>
              <span class="year" data-item-like-style>🕒 {album.year ?? '?'}</span>
            </div>
          {/if}

          <div class="artists">
            <NavigableRow>
              {#each album.albumArtists as albumArtist}
                <SimpleNavigableItem onPress={bind(albumArtist.id, (id) => navigate(ROUTES.artist(id)))}>
                  <span class="artist">
                    🎤 {albumArtist.name}
                  </span>
                </SimpleNavigableItem>
              {/each}
            </NavigableRow>
          </div>

          <div class="genres">
            <NavigableRow>
              {#each album.genres as genre}
                <SimpleNavigableItem onPress={bind(genre.id, (id) => navigate(ROUTES.genre(id)))}>
                  <span class="genre">
                    🎵 {genre.name}
                  </span>
                </SimpleNavigableItem>
              {/each}
            </NavigableRow>
          </div>

          <div class="length" data-item-like-style>
            ⌛ {humanReadableDuration(totalDuration)} /
            {filteredTracks.length} track{filteredTracks.length > 1 ? 's' : ''}
            {#if discs.length > 1}/ {discs.length}
              discs{/if}
          </div>

          <Row>
            <Checkbox bind:checked={onlyShowGreatSongs} fullHeight>Only show great songs</Checkbox>
            <Button onPress={() => queueAsNext(filteredTracks)} fullHeight><Emoji>▶️</Emoji> Play next</Button>
          </Row>
        </div>
      </div>
    </NavigableList>

    <NavigableList>
      {#each discs as disc (disc.number)}
        {#if discs.length > 1}
          <h2>Disc {disc.number}</h2>
        {/if}

        <NavigableList>
          <table>
            <tbody>
              {#each disc.tracks as track, i (track.id)}
                {@const tags = track.metadata.tags}

                <NavigableTrack
                  position={filteredTracks.indexOf(track)}
                  tracks={filteredTracks}
                  goToAlbumOption={false}
                  display="transparent"
                  {track}
                >
                  <tr class:notFirst={i !== 0}>
                    <td class="trackno">{tags.trackNo}</td>
                    <td class="title">{tags.title}</td>
                    <td class="rating">
                      {#if tags.rating}
                        <TrackRating rating={tags.rating} />
                      {/if}
                    </td>
                    <td class="duration">{humanReadableDuration(track.metadata.duration)}</td>
                  </tr>
                </NavigableTrack>
              {/each}
            </tbody>
          </table>
        </NavigableList>
      {/each}
    </NavigableList>
  </div>
{:catch e}
  <h2>Failed: {e.message}</h2>
{/await}

<style>
  .container {
    margin-top: 10px;
    margin-left: 15%;
    width: 70%;
  }

  .header {
    display: flex;
    flex-direction: row;
  }

  .infos {
    display: flex;
    flex-direction: column;
    margin-top: 10px;
    margin-left: 10px;
    gap: 10px;
  }

  .infos .name {
    font-size: 2rem;
    font-weight: bold;
  }

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

  td.title {
    width: 100%;
  }

  td.duration {
    text-align: right;
  }
</style>
