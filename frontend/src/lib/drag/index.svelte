<script lang="ts">
	import { onMount } from "svelte";
  import Draggable from "./draggable.svelte";
  import Grid from "./grid.svelte";
  const politicalPeople = {
    "democrat": ["Joe Biden", "Stacey Abrams", "Eric Adams", "Michael Bennet", "Andy Beshear", "Cory Booker", "Sherrod Brown", "Pete Buttigieg", "Hillary Clinton", "Roy Cooper", "Andrew Cuomo", "Kamala Harris", "Jay Inslee", "Joe Kennedy", "Ro Khanna", "Amy Klobuchar", "Mitch Landrieu", "Michelle Lujan Grisham", "Joe Manchin", "Chris Murphy", "Phil Murphy", "Gavin Newsom", "Alexandria Ocasio-Cortez", "J.B. Pritzker", "Gina Raimondo", "Nina Turner", "Elizabeth Warren", "Gretchen Whitmer", "Bernie Sanders", "Joe Sanberg", "Oprah Winfrey", "Andrew Yang", "Michelle Obama"],
    "republican": ["Greg Abbott", "Liz Cheney", "Chris Christie", "Bob Corker", "Tom Cotton", "Daniel Crenshaw", "Ted Cruz", "Ron DeSantis", "Doug Ducey", "Mike DeWine", "Joni Ernst", "Larry Elder", "Josh Hawley", "Adam Kinzinger", "Mike Lee", "Kristi Noem", "Rand Paul", "Mike Pence", "Mike Pompeo", "Mitt Romney", "Marco Rubio", "Ben Sasse", "Rick Scott", "Tim Scott", "Elise Stefanik", "Chris Sununu", "Glenn Youngkin", "Tucker Carlson", "Candace Owens", "Donald Trump Jr.", "Ivanka Trump", "Larry Hogan", "Mike Pompeo"]
  }
  const {democrat, republican} = politicalPeople;
  $: data = republican;



  export let wW: number;
  let viewBoxW: number;
  export let wH: number;
  let viewBoxH: number;

  onMount(()=> {
    viewBoxW = wW*.9
    viewBoxH = wH*.60
  })
</script>

<svelte:window bind:innerWidth={wW} bind:innerHeight={wH} />

<div class="info">
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
      Right Click
    </strong>
    to
    <strong>
      Delete Item.
    </strong>
  </h2>
</div>

<div class="svg">
  <Grid />
  <div class="overlayed">
  </div>
  <div class="overlayed">
    <Draggable {viewBoxW} {viewBoxH} {data} />
  </div>
</div>



<style>
  .svg {
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
</style>