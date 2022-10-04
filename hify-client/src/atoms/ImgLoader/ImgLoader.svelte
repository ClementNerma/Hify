<script lang="ts">
  import { writable } from 'svelte/store'
  import { getArtUri } from '../../globals/rest-api'
  import { ProgressiveImgFragment } from '../../graphql/generated'
  import { createBlurHashImageSrc } from './ImgLoader'

  export let art: ProgressiveImgFragment | null | undefined

  const loaded = writable(false)

  const fullImgUrl = art ? getArtUri(art.id) : 'about:blank'
  const blurImgUrl = art ? createBlurHashImageSrc(art) : 'about:blank'

  const img = new Image()
  img.src = fullImgUrl
  img.addEventListener('load', () => loaded.set(true))
</script>

{#if $loaded}
  <slot src={fullImgUrl} />
{:else if art}
  <slot src={blurImgUrl} />
{:else}
  <slot src="about:blank" />
{/if}
