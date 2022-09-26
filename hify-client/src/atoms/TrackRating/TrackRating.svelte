<script lang="ts">
  export let rating: number

  const ratingStr = [11, 31, 51, 71, 91].map((min) => (rating >= min ? '&starf;' : '&star;')).join('')
</script>

<span>{@html ratingStr}</span>
