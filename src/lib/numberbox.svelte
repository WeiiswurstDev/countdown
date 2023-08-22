<script lang="ts">
  export let number: number;
  export let description: String | null;
</script>

<div class="outline-blue numberbox">
  {#if description}
    <span class="muted center">{description}</span>
  {/if}
  <h1 class="center">{number}</h1>
</div>

<style>
  .numberbox:hover {
    background: var(--teal);
    transition-duration: 0.1s;
    padding: 10px;
    margin: 0px;
  }

  .numberbox {
    margin: 5px;
    min-width: 75px;
  }
</style>
