<script lang="ts">
    export let viewBoxW: Number;
    export let viewBoxH: Number;
    export let data: any;
  import {shuffle} from "$lib/utils/shuffle"
</script>

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

{#each shuffle(data) as point, i}
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
    text-anchor="left" fill="white" font-size="{viewBoxW > viewBoxH ? viewBoxW/60 : viewBoxW/45}px" alignment-baseline="middle">{point}</text>
{/each}
</svg>