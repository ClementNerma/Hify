<script lang="ts">
  import { getAlbumArtUri } from '../../rest-api'
  import {
    humanReadableDuration,
    readableAudioPaused,
    readableAudioProgress,
    setPlayingAudioProgress,
    toggleAudioPlayback,
  } from '../../stores/audio-player'
  import { currentTrack } from '../../stores/play-queue'

  import { useNavigate } from 'svelte-navigator'

  import SimpleNavigableItem from '../../navigable/SimpleNavigableItem/SimpleNavigableItem.svelte'
  import NavigableRow from '../../navigable/NavigableRow/NavigableRow.svelte'
  import ProgressRange from '../../atoms/ProgressRange/ProgressRange.svelte'
  import { ROUTES } from '../../routes'

  const navigate = useNavigate()

  $: tags = $currentTrack && $currentTrack.metadata.tags
  $: album = $currentTrack && $currentTrack.metadata.tags.album
</script>

{#if !$currentTrack || !tags || !album}
  <div class="container">
    <h2 class="no-playing">Nothing currently playing or queue is loading</h2>
  </div>
{:else}
  <div class="container">
    <div class="album-art">
      <img width={250} height={250} src={getAlbumArtUri(album.id)} alt={album.name} />
    </div>
    <div class="track-infos">
      <div class="track-name">{tags.title ?? '<unknown title>'}</div>
      <div class="track-album-infos">
        <!-- TODO: find a fix for this check -->
        <SimpleNavigableItem onPress={() => void (album && navigate(ROUTES.album(album.id)))}>
          <div class="track-album-name">
            {album.name ?? '<unknown album>'}
            {#if album.year}
              <span class="track-album-year">({album.year})</span>
            {/if}
          </div>
        </SimpleNavigableItem>
      </div>
      <div class="track-artists">
        <NavigableRow>
          {#each album.albumArtists as albumArtist}
            <SimpleNavigableItem onPress={() => navigate(ROUTES.artist(albumArtist.id))}>
              <span class="album-artist">
                {albumArtist.name}
              </span>
            </SimpleNavigableItem>
          {/each}
        </NavigableRow>
      </div>
    </div>

    <div class="player-bottom">
      <div class="progress-range">
        <ProgressRange
          max={$currentTrack.metadata.duration}
          value={$readableAudioProgress}
          onChange={setPlayingAudioProgress}
          onPress={toggleAudioPlayback}
        />
      </div>
      <div class="progress-time">
        <div class="playback-indicator">
          {#if $readableAudioPaused === null}
            -
          {:else if $readableAudioPaused}
            ⏸️
          {:else}
            ▶️
          {/if}
        </div>
        <div class="current-time">
          {#if $readableAudioProgress}
            {humanReadableDuration($readableAudioProgress)}
          {:else}
            --:--:--
          {/if}
          / {humanReadableDuration($currentTrack.metadata.duration)}
        </div>
      </div>
    </div>
  </div>
{/if}

<style>
  .container {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
  }

  .no-playing {
    margin-top: 25%;
    text-align: center;
    font-size: 2rem;
  }

  .album-art {
    position: fixed;
    top: calc(35% - 250px / 2);
    left: calc(50% - 250px / 2);
  }

  .track-infos {
    display: flex;
    flex-direction: column;
    position: fixed;
    top: calc(35% + 250px / 2);
    padding: 10px;
    width: 100%;
    text-align: center;
  }

  .track-infos * {
    display: inline-block;
  }

  .track-infos > * {
    margin: 5px;
  }

  .track-name {
    font-size: 2rem;
  }

  .track-album-name {
    font-size: 1.5rem;
  }

  .track-album-year {
    vertical-align: middle;
    font-size: 1rem;
  }

  .track-artists {
    font-size: 1.5rem;
  }

  .player-bottom {
    position: fixed;
    bottom: 5%;
    left: 25%;
    width: 50%;
  }

  .progress-range,
  .progress-range :global(input) {
    width: 100%;
  }

  .playback-indicator {
    float: left;
  }

  .current-time {
    float: right;
  }
</style>
