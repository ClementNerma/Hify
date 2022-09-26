<script lang="ts">
  export let note: number

  const noteStr = [11, 31, 51, 71, 91].map((min) => (note > min ? '&starf;' : '&star;')).join('')
</script>

<span>{@html noteStr}</span>
