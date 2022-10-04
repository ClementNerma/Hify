<script lang="ts">
  /**
   * Note: As both BlurHash decoding and Canvas painting + encoding is slow (both taking about 2 ms each)
   *       this component's features were removed, until a more performant way is found
   *
   *       The component's properties are left for compatibility reasons and future-proofing.
   */

  import { getArtUri } from '../../globals/rest-api'
  import { ProgressiveImgFragment } from '../../graphql/generated'

  export let art: ProgressiveImgFragment | null | undefined
  export const quick: boolean | null = null
</script>

{#if art}
  <slot src={getArtUri(art.id)} />
{:else}
  <slot src="about:blank" />
{/if}
