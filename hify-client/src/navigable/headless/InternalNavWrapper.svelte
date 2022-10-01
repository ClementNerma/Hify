<script lang="ts">
  export let navId: string

  const startComment = `<!-- @start-nav: ${navId} -->`
  const endComment = `<!-- @end-nav: ${navId} -->`
</script>

{@html startComment}
<slot />
{@html endComment}
