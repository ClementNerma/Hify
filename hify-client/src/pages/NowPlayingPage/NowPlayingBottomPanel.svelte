<script lang="ts">
  import { navigate } from 'svelte-navigator'
  import { bind, formatDate } from '@globals/utils'

  import {
    humanReadableDuration,
    readableAudioPaused,
    readableAudioProgress,
    setPlayingAudioProgress,
    toggleAudioPlayback,
  } from '@stores/audio-player'
  import { playTrackFromCurrentQueue, queuePosition, readablePlayQueue } from '@stores/play-queue'

  import { AudioTrackFragment } from '@graphql/generated'

  import SimpleNavigableItem from '@navigable/headless/SimpleNavigableItem/SimpleNavigableItem.svelte'
  import NavigableRow from '@navigable/headless/NavigableRow/NavigableRow.svelte'
  import { ROUTES } from '@root/routes'
  import Column from '@navigable/ui/molecules/Column/Column.svelte'
  import ModifiableTrackRating from '@atoms/ModifiableTrackRating/ModifiableTrackRating.svelte'
  import ProgressRange from '@atoms/ProgressRange/ProgressRange.svelte'
  // import TrackWaveForm from '@atoms/TrackWaveForm/TrackWaveForm.svelte'
  import ProgressiveRow from '@molecules/ProgressiveRow/ProgressiveRow.svelte'
  import Card from '@molecules/Card/Card.svelte'
  import { showContextMenu } from '@navigable/ui/molecules/ContextMenu/ContextMenu'
  import { ctxMenuOptions } from '@globals/context-menu-items'

  export let currentTrack: AudioTrackFragment | false
  let isQueueFocused = false

  // function toggleWaveForm() {
  //   showWaveform = !showWaveform
  // }

  // let showWaveform = false

  function showTrackCtxMenu(track: AudioTrackFragment, position: number) {
    showContextMenu(
      ctxMenuOptions.forTrack(
        track,
        { fromMixId: null },
        {
          context: 'queue',
          isCurrent: $queuePosition === position,
          position,
          totalTracks: $readablePlayQueue.tracks.length,
        },
      ),
    )
  }

  function setQueueFocused(isFocused: boolean) {
    isQueueFocused = isFocused
  }
</script>

<div class="player-bottom" class:isQueueFocused class:noCurrentTrack={!currentTrack}>
  <Column>
    <Column>
      {#if currentTrack}
        {@const tags = currentTrack.metadata.tags}
        {@const album = tags.album}
        {@const artists = tags.artists.length > 0 ? tags.artists : album.albumArtists}

        <!-- {#if showWaveform}
          <TrackWaveForm
            track={currentTrack}
            progress={($readableAudioProgress ?? 0) / currentTrack.metadata.duration}
            width="100%"
            height="50px"
          />
        {/if} -->

        <div class="buttons">
          <NavigableRow>
            <SimpleNavigableItem onPress={bind(tags, (tags) => void navigate(ROUTES.searchTerms(tags.title)))}>
              <div class="track-info">🎵 {tags.title}</div>
            </SimpleNavigableItem>
            <SimpleNavigableItem onPress={bind(album, (album) => void navigate(ROUTES.album(album.id)))}>
              <div class="track-info">💿 {album.name}</div>
            </SimpleNavigableItem>
            {#if tags.date}
              <SimpleNavigableItem justForStyle>
                <div class="track-info">🕒 {formatDate(tags.date)}</div>
              </SimpleNavigableItem>
            {/if}

            {#each artists as artist}
              <SimpleNavigableItem onPress={bind(artist.id, (id) => navigate(ROUTES.artist(id)))}>
                <div class="track-info">🎤 {artist.name}</div>
              </SimpleNavigableItem>
            {/each}

            <!-- TODO: This works perfectly, but waiting for performance improvements
                       before displaying it again -->
            <!-- <SimpleNavigableItem onPress={toggleWaveForm}>
              <div class="track-action">∿ Waveform</div>
            </SimpleNavigableItem> -->

            <ModifiableTrackRating track={currentTrack} />
          </NavigableRow>
        </div>

        <div class="player-time">
          <div class="track-progress">
            {#if $readableAudioProgress !== null}
              {humanReadableDuration($readableAudioProgress)}
            {:else}
              --:--
            {/if}
            {#if $readableAudioPaused}
              ⏸️
            {/if}
          </div>
          <div class="track-duration">
            {humanReadableDuration(currentTrack.metadata.duration)}
          </div>
        </div>

        <div class="progress-range">
          <ProgressRange
            max={currentTrack.metadata.duration}
            value={$readableAudioProgress}
            onChange={setPlayingAudioProgress}
            onPress={toggleAudioPlayback}
            directionalAmount={30}
          />
        </div>
      {/if}
    </Column>

    <div class="play-queue-gallery">
      <Column>
        <ProgressiveRow
          items={$readablePlayQueue.tracks}
          idProp="idInQueue"
          initialPosition={$queuePosition ?? 0}
          onItemPress={(_, pos) => playTrackFromCurrentQueue(pos)}
          onItemLongPress={showTrackCtxMenu}
          onFocusChange={setQueueFocused}
          let:item={track}
          let:position
        >
          <Card
            title={track.metadata.tags.title}
            subtitle={null}
            boxSize={80}
            art={track.metadata.tags.album.art}
            opacity={$queuePosition === position ? 1 : 0.2}
          />
        </ProgressiveRow>
      </Column>
    </div>
  </Column>
</div>

<style>
  .player-bottom {
    position: fixed;

    left: 0;
    right: 0;
    bottom: -100px;

    padding-left: 5%;
    padding-right: 5%;
    padding-bottom: 1%;

    background-image: linear-gradient(to bottom, rgba(255, 0, 0, 0), rgba(30, 30, 30, 1));

    transition: bottom 0.3s;
  }

  .player-bottom.isQueueFocused,
  .player-bottom.noCurrentTrack {
    bottom: 0px;

    transition: bottom 0.3s;
  }

  .buttons {
    display: flex;
    flex-direction: row;
    align-items: end;
    font-size: 1.2rem;
  }

  .track-info {
    padding: 5px;
    align-self: stretch;
  }

  .progress-range,
  .progress-range :global(input) {
    width: calc(100% - 5px);
  }

  .player-time {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    padding: 0px 10px;
    transition: all 1s;
  }
</style>
