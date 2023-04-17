<script lang="ts">
    export let wW: number;
    let viewBoxW: number;
    export let wH: number;
    let viewBoxH: number;
    export let data: any;

    let desktop: boolean;

    onMount(()=> {
        viewBoxW = wW*.9
        viewBoxH = wH*.60
        desktop = viewBoxW > viewBoxH
    })

    import {shuffle} from "$lib/utils/shuffle"
	import { onMount } from "svelte";
</script>

<svelte:window bind:innerWidth={wW} bind:innerHeight={wH} />

<svg xmlns="http://www.w3.org/2000/svg"
    viewBox="0 0 {viewBoxW} {viewBoxH}"
    width="100%" height="100%"
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
    svg.addEventListener('oncontextmenu', delTarget);
    svg.addEventListener('mousedown', mouseDown);
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

    function mouseDown(evt) {
        if (evt.button == 0) {
            startDrag(evt)
        } else if (evt.explicitOriginalTarget.length > 0) {
            console.log(evt)
            delTarget(evt)
        }
        return false
    }

    function delTarget(evt) {
        evt.target.remove()
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

{#each data as point, i}
    <text class="draggable"
    x={i*(viewBoxW*.025) < (viewBoxW) && i%2==0
        ? i*(viewBoxW*.025)
        : (i*(viewBoxW*.025) < (viewBoxW*.95) && i%2==0
            ? i*(viewBoxW*.025)
            : i*(viewBoxW*.025))} 
    y="{i%2==0 ? (
        i%16==0 ? (viewBoxH-((desktop ? 30 : 20) *1))
        : (i%22==0 ? (viewBoxH-((desktop ? 30 : 20) *4)) 
        : (i%12==0 ? (viewBoxH-((desktop ? 30 : 20) *2)) 
        : (i%10==0 ? (viewBoxH-((desktop ? 30 : 20) *3))
        : (i%8==0 ? (viewBoxH-((desktop ? 30 : 20) *4))
        : (i%6==0 ? (viewBoxH-((desktop ? 30 : 20) *5))
        : i%4==0 ? (viewBoxH-((desktop ? 30 : 20) *6))
        : (viewBoxH-((desktop ? 30 : 20) *7)))))) )
    )
        : i%21==0 ? (viewBoxH-((desktop ? 30 : 20) *10))
        : i%27==0 ? (viewBoxH-((desktop ? 30 : 20) *11))
        : i%9==0 ? (viewBoxH-((desktop ? 30 : 20) *15))
        : i%3==0 ? (viewBoxH-((desktop ? 30 : 20) *8))
        : i%5==0 ? (viewBoxH-((desktop ? 30 : 20) *9))
        : i%7==0 ? (viewBoxH-((desktop ? 30 : 20) *10))
        : i%11==0 ? (viewBoxH-((desktop ? 30 : 20) *11))
        : i%13==0 ? (viewBoxH-((desktop ? 30 : 20) *12))
        : i%17==0 ? (viewBoxH-((desktop ? 30 : 20) *13))
        : i%19==0 ? (viewBoxH-((desktop ? 30 : 20) *14))
        : i%23==0 ? (viewBoxH-((desktop ? 30 : 20) *15))
        : i%29==0 ? (viewBoxH-((desktop ? 30 : 20) *14))
        : i%31==0 ? (viewBoxH-((desktop ? 30 : 20) *16))
        : (viewBoxH-((desktop ? 30 : 20) *13))}"
    text-anchor="left" fill="white" font-size="{desktop
        ? viewBoxW/50
        : viewBoxW/30}px" alignment-baseline="middle">{point}</text>
{/each}
</svg>