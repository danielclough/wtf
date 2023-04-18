<script lang="ts">
	import { onMount } from "svelte";
  import Draggable from "./draggable.svelte";
  import Grid from "./grid.svelte";
  const dataObj = {
    "republican": ["Greg Abbott", "Liz Cheney", "Chris Christie", "Bob Corker", "Tom Cotton", "Daniel Crenshaw", "Ted Cruz", "Ron DeSantis", "Doug Ducey", "Mike DeWine", "Joni Ernst", "Larry Elder", "Josh Hawley", "Adam Kinzinger", "Mike Lee", "Kristi Noem", "Rand Paul", "Mike Pence", "Mike Pompeo", "Mitt Romney", "Marco Rubio", "Ben Sasse", "Rick Scott", "Tim Scott", "Elise Stefanik", "Chris Sununu", "Glenn Youngkin", "Tucker Carlson", "Candace Owens", "Donald Trump Jr.", "Ivanka Trump", "Larry Hogan", "Mike Pompeo"],
    "democrat": ["Joe Biden", "Stacey Abrams", "Eric Adams", "Michael Bennet", "Andy Beshear", "Cory Booker", "Sherrod Brown", "Pete Buttigieg", "Hillary Clinton", "Roy Cooper", "Andrew Cuomo", "Kamala Harris", "Jay Inslee", "Joe Kennedy", "Ro Khanna", "Amy Klobuchar", "Mitch Landrieu", "Michelle Lujan Grisham", "Joe Manchin", "Chris Murphy", "Phil Murphy", "Gavin Newsom", "Alexandria Ocasio-Cortez", "J.B. Pritzker", "Gina Raimondo", "Nina Turner", "Elizabeth Warren", "Gretchen Whitmer", "Bernie Sanders", "Joe Sanberg", "Oprah Winfrey", "Andrew Yang", "Michelle Obama"],
  }
  $: data = dataObj[Object.keys(dataObj)[0]];

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
  <div class="info-inner">
    <select size="{Object.keys(dataObj).length}" multiple bind:value={data}>
      {#each Object.keys(dataObj) as party}
        <option value={dataObj[party]}>
          {party}
        </option>
      {/each}
    </select>
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