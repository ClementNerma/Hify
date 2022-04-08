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
  import { bind } from '../../utils'

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
    <img width={250} height={250} src={getAlbumArtUri(album.id)} alt="" />
  </div>
  <div class="track-infos">
    <SimpleNavigableItem onPress={bind(tags, (tags) => void navigate(ROUTES.searchTerms(tags.title)))}>
      <div class="track-info">🎵 {tags.title}</div>
    </SimpleNavigableItem>
    <SimpleNavigableItem onPress={bind(album, (album) => void navigate(ROUTES.album(album.id)))}>
      <div class="track-info">💿 {album.name}</div>
    </SimpleNavigableItem>
    <div class="artists">
      <NavigableRow>
        {#each album.albumArtists as albumArtist}
          <SimpleNavigableItem onPress={bind(albumArtist.id, (id) => navigate(ROUTES.artist(id)))}>
            <span class="track-info">🎤 {albumArtist.name}</span>
          </SimpleNavigableItem>
        {/each}
      </NavigableRow>
    </div>
    {#if tags.date}
      <div item-like-style>
        <div class="track-info">
          🕒 {tags.date.year}{tags.date.month ? `-${tags.date.month}` : ''}{tags.date.day
            ? `${tags.date.month ? '' : '??'}-${tags.date.day}`
            : ''}
        </div>
      </div>
    {/if}
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
    top: calc(35% - 250px / 2 + 33px);
    left: calc(5% + 250px + 10px);
    font-size: 1.2rem;
  }

  .track-info {
    display: inline-block;
    padding: 5px;
  }

  .artists {
    display: flex;
    flex-direction: row;
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
    width: calc(100% - 5px);
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
    left: 5%;
    right: 5%;
    bottom: 20px;
  }
</style>
