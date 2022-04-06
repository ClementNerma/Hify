<script lang="ts">
  import { AudioTrackFragment } from '../../graphql/generated'
  import SimpleNavigableItem from '../../navigable/SimpleNavigableItem/SimpleNavigableItem.svelte'
  import { getAlbumArtUri } from '../../rest-api'
  import { playTrackFromCurrentQueue } from '../../stores/play-queue'

  export let track: AudioTrackFragment
  export let position: () => number
  export let current: boolean
  export let columns: number
</script>

<SimpleNavigableItem {position} onPress={() => playTrackFromCurrentQueue(position)} hasFocusPriority={() => current}>
  <div class="track" style="--column-size: {`${100 / columns}%`}" class:current>
    <img
      class="album-art"
      width={80}
      height={80}
      src={getAlbumArtUri(track.metadata.tags.album.id)}
      alt={track.metadata.tags.album.name}
    />

    <div class="title">{track.metadata.tags.title}</div>
  </div>
</SimpleNavigableItem>

<style>
  .track {
    text-align: center;
    max-width: var(--column-size);
  }

  /* TODO: remove experimental stuff */
  .title {
    font-weight: bold;
    vertical-align: middle;
    overflow: hidden;
    text-overflow: ellipsis;
    display: -webkit-box;
    -webkit-line-clamp: 2;
    line-clamp: 2;
    -webkit-box-orient: vertical;
  }

  .track.current {
    font-weight: bold;
    border-radius: 5px;
  }

  .track:not(.current) {
    opacity: 0.2;
  }
</style>
