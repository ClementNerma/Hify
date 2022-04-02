<script lang="ts">
  import SimpleNavigableItem from '../../layout/SimpleNavigableItem/SimpleNavigableItem.svelte'

  export let pictureUrl: string
  export let pictureAlt: string
  export let title: string
  export let onPress: () => void
  export let onLongPress: (() => void) | undefined = undefined
  export let subtitle: string
  export let onSubtitleClick: () => void

  let focused = false
</script>

<SimpleNavigableItem
  onFocusChange={(has) => {
    focused = has
  }}
  {onPress}
  {onLongPress}
>
  <div class="card {focused ? 'focused' : ''}" on:click={onPress}>
    <img width={250} height={250} src={pictureUrl} alt={pictureAlt} />
    <div class="title">{title}</div>
    <div class="subtitle" on:click|stopPropagation={onSubtitleClick}>{subtitle}</div>
  </div>
</SimpleNavigableItem>

<style>
  .card {
    text-align: center;
    padding: 10px;
  }

  .card.focused {
    border: 3px solid pink;
    background-color: pink;
    border-radius: 15px;
    padding: 7px;
  }

  .card:hover {
    cursor: pointer;
  }

  .title {
    font-weight: bold;
  }
</style>
