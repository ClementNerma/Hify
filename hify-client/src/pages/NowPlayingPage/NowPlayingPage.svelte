<script lang="ts">
  import { getAlbumArtUri } from '../../rest-api'
  import {
    humanReadableAudioProgress,
    readableAudioPaused,
    readableAudioPlaying,
    readableAudioProgress,
    setPlayingAudioProgress,
    toggleAudioPlayback,
  } from '../../stores/audio/store'

  import { useNavigate } from 'svelte-navigator'

  import SimpleNavigableItem from '../../navigable/SimpleNavigableItem/SimpleNavigableItem.svelte'
  import NavigableRow from '../../navigable/NavigableRow/NavigableRow.svelte'
  import ProgressRange from '../../atoms/ProgressRange/ProgressRange.svelte'
  import { ROUTES } from '../../routes'

  const navigate = useNavigate()
</script>

{#if !$readableAudioPlaying}
  <div class="container">
    <h2 class="no-playing">Nothing currently playing</h2>
  </div>
{:else}
  {#await $readableAudioPlaying.trackInfos}
    <div class="container">
      <h2 class="loading">Loading track informations...</h2>
    </div>
  {:then track}
    {#if !track}
      <div class="container">
        <h2 class="no-track-infos">Playing track was not found in API :(</h2>
      </div>
    {:else if !track.metadata.tags.album}
      <div class="container">
        <h2 class="no-album-track">Tracks without album are currently unsupported in the player :(</h2>
      </div>
    {:else}
      <div class="container">
        <div class="album-art">
          <img
            width={500}
            height={500}
            src={getAlbumArtUri(track.metadata.tags.album.id)}
            alt={track.metadata.tags.album.name}
          />
        </div>
        <div class="track-infos">
          <div class="track-name">{track.metadata.tags.title ?? '<unknown title>'}</div>
          <div class="track-album-infos">
            <!-- TODO: find a fix for this check -->
            <SimpleNavigableItem
              onPress={() => track.metadata.tags.album && navigate(ROUTES.album(track.metadata.tags.album.id))}
            >
              <div class="track-album-name">
                {track.metadata.tags.album.name ?? '<unknown album>'}
                {#if track.metadata.tags.album.year}
                  <span class="track-album-year">({track.metadata.tags.album.year})</span>
                {/if}
              </div>
            </SimpleNavigableItem>
          </div>
          <div class="track-artists">
            <NavigableRow>
              {#each track.metadata.tags.album.albumArtists as albumArtist}
                <SimpleNavigableItem onPress={() => alert("TODO: go to artist's page: " + albumArtist.name)}>
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
              max={track.metadata.duration}
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
                {humanReadableAudioProgress($readableAudioProgress)}
              {:else}
                --:--:--
              {/if}
            </div>
          </div>
        </div>
      </div>
    {/if}
  {/await}
{/if}

<style>
  .container {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
  }

  .no-playing,
  .loading,
  .no-track-infos,
  .no-album-track {
    margin-top: 25%;
    text-align: center;
    font-size: 3rem;
  }

  .album-art {
    position: fixed;
    top: calc(35% - 500px / 2);
    left: calc(50% - 500px / 2);
  }

  .track-infos {
    display: flex;
    flex-direction: column;
    position: fixed;
    top: calc(35% + 500px / 2);
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
    font-size: 3rem;
  }

  .track-album-name {
    font-size: 2rem;
  }

  .track-album-year {
    vertical-align: middle;
    font-size: 1rem;
  }

  .track-artists {
    font-size: 2rem;
  }

  .player-bottom {
    position: fixed;
    bottom: 10%;
    left: calc(50% - 500px / 2);
    width: 500px;
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
