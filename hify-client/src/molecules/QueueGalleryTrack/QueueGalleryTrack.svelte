<script lang="ts">
  import { AudioTrackFragment } from '../../graphql/generated'
  import SimpleNavigableItem from '../../navigable/SimpleNavigableItem/SimpleNavigableItem.svelte'
  import { getAlbumArtUri } from '../../rest-api'
  import { playTrackFromCurrentQueue } from '../../stores/play-queue'

  export let track: AudioTrackFragment
  export let position: number
  export let current: boolean
  export let columns: number
</script>

<div class="track" style="--column-size: {`${100 / columns}%`}" class:current>
  <SimpleNavigableItem
    {position}
    onPress={() => playTrackFromCurrentQueue(position)}
    hasFocusPriority={current}
    style="display: block;"
  >
    <div>
      <img class="album-art" width={80} height={80} src={getAlbumArtUri(track.metadata.tags.album.id)} alt="" />
      <div class="title experimental-line-limiter">{track.metadata.tags.title}</div>
    </div>
  </SimpleNavigableItem>
</div>

<style>
  .track {
    text-align: center;
    min-width: var(--column-size);
    width: var(--column-size);
    max-width: var(--column-size);
  }

  .title {
    font-weight: bold;
    vertical-align: middle;
    overflow: hidden;
  }

  .track.current {
    font-weight: bold;
    border-radius: 5px;
  }

  .track:not(.current) {
    opacity: 0.2;
  }

  /* TODO: remove experimental stuff */
  .experimental-line-limiter {
    text-overflow: ellipsis;
    display: -webkit-box;
    -webkit-line-clamp: 2;
    line-clamp: 2;
    -webkit-box-orient: vertical;
  }
</style>
