<script lang="ts">
	import { onMount } from "svelte";

  export let wW: number;
  let viewBoxW: number;
  export let wH: number;
  let viewBoxH: number;
  const politicalPeople = {
    "democrat": ["Joe Biden", "Stacey Abrams", "Eric Adams", "Michael Bennet", "Andy Beshear", "Cory Booker", "Sherrod Brown", "Pete Buttigieg", "Hillary Clinton", "Roy Cooper", "Andrew Cuomo", "Kamala Harris", "Jay Inslee", "Joe Kennedy", "Ro Khanna", "Amy Klobuchar", "Mitch Landrieu", "Michelle Lujan Grisham", "Joe Manchin", "Chris Murphy", "Phil Murphy", "Gavin Newsom", "Alexandria Ocasio-Cortez", "J.B. Pritzker", "Gina Raimondo", "Nina Turner", "Elizabeth Warren", "Gretchen Whitmer", "Bernie Sanders", "Joe Sanberg", "Oprah Winfrey", "Andrew Yang", "Michelle Obama"],
    "republican": ["Greg Abbott", "Liz Cheney", "Chris Christie", "Bob Corker", "Tom Cotton", "Daniel Crenshaw", "Ted Cruz", "Ron DeSantis", "Doug Ducey", "Mike DeWine", "Joni Ernst", "Larry Elder", "Josh Hawley", "Adam Kinzinger", "Mike Lee", "Kristi Noem", "Rand Paul", "Mike Pence", "Mike Pompeo", "Mitt Romney", "Marco Rubio", "Ben Sasse", "Rick Scott", "Tim Scott", "Elise Stefanik", "Chris Sununu", "Glenn Youngkin", "Tucker Carlson", "Candace Owens", "Donald Trump Jr.", "Ivanka Trump", "Larry Hogan", "Mike Pompeo"]
  }
  const {democrat, republican} = politicalPeople;
  $: people = republican;

  onMount(()=> {
    viewBoxW = wW*.9
    viewBoxH = wH*.60
  })
</script>

<svelte:window bind:innerWidth={wW} bind:innerHeight={wH} />


  <h1>
    Politics... WTF?!?
  </h1>
<div class="svg">

  <svg xmlns="http://www.w3.org/2000/svg"
      viewBox="0 0 {viewBoxW} {viewBoxH}"
      onload="makeDraggable(evt)">
    
    <style>
      .static {
        cursor: not-allowed;
      }
      .draggable {
        cursor: move;
      }
    </style>
    
    <script type="text/javascript"><![CDATA[
      function makeDraggable(evt) {
        var svg = evt.target;
        svg.addEventListener('mousedown', startDrag);
        svg.addEventListener('mousemove', drag);
        svg.addEventListener('mouseup', endDrag);
        svg.addEventListener('mouseleave', endDrag);
        svg.addEventListener('touchstart', startDrag);
        svg.addEventListener('touchmove', drag);
        svg.addEventListener('touchend', endDrag);
        svg.addEventListener('touchleave', endDrag);
        svg.addEventListener('touchcancel', endDrag);
        var selectedElement, offset, transform,
            bbox, minX, maxX, minY, maxY, confined;
        var boundaryX1 = 10.5;
        var boundaryX2 = 30;
        var boundaryY1 = 2.2;
        var boundaryY2 = 19.2;
        function getMousePosition(evt) {
          var CTM = svg.getScreenCTM();
          if (evt.touches) { evt = evt.touches[0]; }
          return {
            x: (evt.clientX - CTM.e) / CTM.a,
            y: (evt.clientY - CTM.f) / CTM.d
          };
        }
        function startDrag(evt) {
          if (evt.target.classList.contains('draggable')) {
            selectedElement = evt.target;
            offset = getMousePosition(evt);
            // Make sure the first transform on the element is a translate transform
            var transforms = selectedElement.transform.baseVal;
            if (transforms.length === 0 || transforms.getItem(0).type !== SVGTransform.SVG_TRANSFORM_TRANSLATE) {
              // Create an transform that translates by (0, 0)
              var translate = svg.createSVGTransform();
              translate.setTranslate(0, 0);
              selectedElement.transform.baseVal.insertItemBefore(translate, 0);
            }
            // Get initial translation
            transform = transforms.getItem(0);
            offset.x -= transform.matrix.e;
            offset.y -= transform.matrix.f;
  
            confined = evt.target.classList.contains('confine');
            if (confined) {
                bbox = selectedElement.getBBox();
                minX = boundaryX1 - bbox.x;
                maxX = boundaryX2 - bbox.x - bbox.width;
                minY = boundaryY1 - bbox.y;
                maxY = boundaryY2 - bbox.y - bbox.height;
            }
          }
        }
        function drag(evt) {
          if (selectedElement) {
            evt.preventDefault();
            var coord = getMousePosition(evt);
            var dx = coord.x - offset.x;
            var dy = coord.y - offset.y;
            if (confined) {
                if (dx < minX) { dx = minX; }
                else if (dx > maxX) { dx = maxX; }
                if (dy < minY) { dy = minY; }
                else if (dy > maxY) { dy = maxY; }
            }
            transform.setTranslate(dx, dy);
            console.log(offset.x, offset.y, dx, dy)
          }
          
        }
        function endDrag(evt) {
          selectedElement = false;
        }
      }
    ]]> </script>
    
    {#each people as person, i}
      <text class="draggable"
        x={i*(viewBoxW*.025) < (viewBoxW*.95) && i%2==0 ? i*(viewBoxW*.025) : (i*(viewBoxW*.025) < (viewBoxW*.95) && i%2==0 ? i*(viewBoxW*.025) : i*(viewBoxW*.025))} 
        y="{i%2==0 ? (
          i%16==0 ? (viewBoxH-2) 
          : (i%12==0 ? (viewBoxH-20) 
          : (i%10==0 ? (viewBoxH-280) 
          : (i%8==0 ? (viewBoxH-40) 
          : (i%6==0 ? (viewBoxH-60) 
          : i%4==0 ? (viewBoxH-80) 
          : (viewBoxH-100))))) 
        )
          : i%3==0 ? (viewBoxH-120) 
          : i%5==0 ? (viewBoxH-140) 
          : i%7==0 ? (viewBoxH-160) 
          : i%11==0 ? (viewBoxH-180) 
          : i%13==0 ? (viewBoxH-200) 
          : i%17==0 ? (viewBoxH-220) 
          : i%19==0 ? (viewBoxH-240) 
          : i%23==0 ? (viewBoxH-260) 
          : i%29==0 ? (viewBoxH-240) 
          : i%31==0 ? (viewBoxH-260)
          : (viewBoxH-240)}"
        text-anchor="left" fill="white" font-size="{viewBoxW > viewBoxH ? viewBoxW/60 : viewBoxW/45}px" alignment-baseline="middle">{person}</text>
    {/each}
  </svg>
</div>


<style>
</style>