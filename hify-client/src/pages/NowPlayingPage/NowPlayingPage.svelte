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
  import QueueGallery from '../../organisms/QueueGallery/QueueGallery.svelte'

  const navigate = useNavigate()

  $: tags = $currentTrack && $currentTrack.metadata.tags
  $: album = $currentTrack && $currentTrack.metadata.tags.album
</script>

{#if !$currentTrack || !tags || !album}
  <h2 class="no-playing">Nothing currently playing or queue is loading</h2>
{:else}
  <div class="album-art">
    <img width={250} height={250} src={getAlbumArtUri(album.id)} alt={album.name} />
  </div>
  <div class="track-infos">
    <div class="track-name"><span class="emoji">🎵</span> {tags.title}</div>
    <div class="track-album-infos">
      <!-- TODO: find a fix for this check -->
      <SimpleNavigableItem onPress={() => void (album && navigate(ROUTES.album(album.id)))}>
        <div class="track-album-name">
          <span class="emoji">💿</span>
          {album.name}
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
              <span class="emoji">🎤</span>
              {albumArtist.name}
            </span>
          </SimpleNavigableItem>
        {/each}
      </NavigableRow>
    </div>
  </div>

  <div class="player-bottom">
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
        {humanReadableDuration($currentTrack.metadata.duration)}
      </div>
    </div>
    <div class="progress-range">
      <ProgressRange
        max={$currentTrack.metadata.duration}
        value={$readableAudioProgress}
        onChange={setPlayingAudioProgress}
        onPress={toggleAudioPlayback}
      />
    </div>
  </div>

  <div class="gallery">
    <QueueGallery />
  </div>
{/if}

<style>
  .no-playing {
    position: fixed;
    top: 25%;
    width: 100%;
    text-align: center;
    font-size: 2rem;
  }

  .album-art {
    position: fixed;
    top: calc(35% - 250px / 2);
    left: 5%;
  }

  .track-infos {
    display: flex;
    flex-direction: column;
    position: fixed;
    top: calc(35% - 250px / 2);
    left: calc(5% + 250px);
    width: 100%;
  }

  .track-infos * {
    display: inline-block;
  }

  .track-infos > * {
    margin: 5px;
  }

  .track-infos .emoji {
    font-size: 1.5rem;
  }

  .track-name {
    font-size: 2rem;
    padding: 25px;
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

    top: calc(35% + 250px / 2 + 15px);
    left: 10%;
    right: 10%;
  }

  .progress-range,
  .progress-range :global(input) {
    width: 100%;
    color: red;
  }

  .player-time {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    padding: 0px 10px;
  }

  .gallery {
    position: fixed;
    bottom: 20px;
  }
</style>
