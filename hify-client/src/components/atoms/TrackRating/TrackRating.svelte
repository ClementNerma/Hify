<script lang="ts">
export let rating: number

const ratingStr = [2, 4, 6, 8, 10].map((min) => (rating >= min ? '&starf;' : '&star;')).join('')
</script>

<span>{@html ratingStr}</span>
