<script lang="ts">
import { getArtUri } from '@globals/rest-api'
import type { ProgressiveImgFragment } from '@graphql/generated'

export let art: ProgressiveImgFragment | null | undefined
</script>

{#if art}
  <slot src={getArtUri(art.id)} />
{:else}
  <slot src="about:blank" />
{/if}
