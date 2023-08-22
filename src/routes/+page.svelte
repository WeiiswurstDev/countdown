<script lang="ts">
  import { generateGame, type Game } from "$lib";
  import Numberbox from "$lib/numberbox.svelte";
  import { invoke } from "@tauri-apps/api";

  let game: Game = generateGame(0);
  let solution = "";

  function newGame(numLarges: number) {
    game = generateGame(numLarges);
  }

  async function solveGame() {
    solution = await invoke("solve_game", game);
  }
</script>

<div class="center spaced">
  <Numberbox description="Target" number={game?.target ?? -69} />
</div>
<div class="center spaced">
  {#each game.inputs as input}
    <Numberbox description="Input" number={input} />
  {/each}
</div>
<div class="spaced flex">
  <div class="left-header outline-blue">
    <h2 class="center">Generate</h2>
  </div>
  <div class="right-header outline-blue grow p-0">
    {#each [0, 1, 2, 3, 4] as numLarges}
      <div
        class="button grow center p-5"
        role="button"
        tabindex={0}
        on:click={() => newGame(numLarges)}
      >
        <h1 class="center">{numLarges}</h1>
        <span class="center">Larges</span>
      </div>
    {/each}
  </div>
</div>

<div class="spaced flex">
  <div class="left-header outline-blue">
    <h2 class="center">Import</h2>
  </div>
  <div class="right-header outline-blue grow flex">
    <div class="row grow">
      <label for="target">Target</label><br />
      <input id="target" type="number" name="target" bind:value={game.target} />
    </div>
    {#each [0, 1, 2, 3, 4, 5] as index}
      <div class="row grow">
        <label for="input-{index}">Input {index}</label><br />
        <input
          id="input-{index}"
          type="number"
          name="input-{index}"
          min="1"
          bind:value={game.inputs[index]}
        />
      </div>
    {/each}
  </div>
</div>

<div class="spaced flex">
  <div
    class="left-header outline-blue button"
    on:click={solveGame}
    role="button"
    tabindex={0}
  >
    <h2 class="center">Solve</h2>
  </div>
  <div class="right-header outline-blue grow center">
    {#if solution}
      {solution}
    {:else}
      Click the button on the left to solve.
    {/if}
  </div>
</div>

<style>
  .left-header {
    display: inline-block;
    border-top-right-radius: 0px;
    border-bottom-right-radius: 0px;
    min-width: 120px;
    opacity: 0.7;
  }

  .right-header {
    border-top-left-radius: 0px;
    border-bottom-left-radius: 0px;
    border-left: 0px;
    opacity: 0.5;
    overflow: hidden;
    flex-grow: 0;
  }

  .spaced:hover .right-header {
    opacity: 1;
    transition-duration: 150ms;
    display: flex;
    flex-grow: 1;
  }

  .spaced:hover .left-header {
    opacity: 1;
    transition-duration: 150ms;
  }

  input {
    background: #0000;
    border: 0px;
    color: white;
    display: inline;
    font-size: 2em;
    max-width: 75px;
  }

  label {
    display: inline;
  }
</style>
