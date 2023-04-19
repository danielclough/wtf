<script lang="ts">
	import { onMount } from "svelte";
  import Draggable from "./draggable.svelte";
  import Grid from "./grid.svelte";

  export let subject: any;
  export let topic: any;
  
  $: data = subject[topic];

  let desktop: boolean;
  
  export let wW: number;
  export let wH: number;
  
  onMount(()=> {
    desktop = wW > 800
  })
</script>

<svelte:window bind:innerWidth={wW} bind:innerHeight={wH} />

<div class="info" style="display:flex;justify-content:space-around;">
  <div class="info-inner">
    <h1>
      Drag from 
      <strong>
        random order
      </strong>
        to
        
      <strong>
        preferred order.
      </strong>
    </h1>
    <h2>
      <strong>
        {desktop ? "Right Click" : "Long Press"}
      </strong>
      to
      <strong>
        Delete Item.
      </strong>
    </h2>
  </div>
</div>

<div class="svg">
  <div class="left-bar">
    <p>
      Like
    </p>
    <p style="font-size:4rem;">
      ↕
    </p>
    <p>
      Dislike
    </p>
  </div>
  <Grid />
  <div class="overlayed">
    <Draggable {data} />  
  </div>
</div>

<style>
  .svg {
    display: flex;
    position: absolute;
    left: 0;
    right: 0;
    top: 10rem;
    bottom: 0;
  }
  .overlayed {
    position: absolute;
    left: 0;
    right: 0;
    top: 0;
    bottom: 0;
  }
  .left-bar {
    writing-mode: vertical-lr;
    text-orientation: upright;
    font-size: 1rem;
    display: flex;
    justify-content: space-between;
    height: 48%;
  }
  p {
    padding: 0;
    margin:0;
    line-height:2rem;
  }
  @media (min-width: 900px) {
    .left-bar {
      font-size: 1.5rem;
    }
  }
</style>