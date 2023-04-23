<script lang="ts">
    export let wW: number;
    let viewBoxW = 0;
    export let wH: number;
    let viewBoxH = 0;
    export let data: any;

    export let topic: string;
    export let subject: any;

    let ranking: string[] = [];
    $: ranking = ranking;

    $: checkDesktop = viewBoxW > viewBoxH;
    $: desktopFontSize = viewBoxW / 80;
    $: mobileFontSize = viewBoxW / 30;
    $: calcFontHeight = checkDesktop ? desktopFontSize : mobileFontSize;
    onMount(()=> {
        viewBoxW = wW*.9
        viewBoxH = wH*.80
    })

    import {shuffle} from "$lib/utils/shuffle"
	import { onMount } from "svelte";
	import { calcX, calcY } from "./utils";
</script>

<svelte:window bind:innerWidth={wW} bind:innerHeight={wH} />

<svg xmlns="http://www.w3.org/2000/svg"
    viewBox="0 0 {viewBoxW} {viewBoxH}"
    width="100%" height="100%"
    onload="makeDraggable(evt)"
    id="dragBox">

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
            let svg = evt.target;
            svg.addEventListener('contextmenu', delTarget);
            svg.addEventListener('mousedown', mouseDown);
            svg.addEventListener('mousemove', drag);
            svg.addEventListener('mouseup', endDrag);
            svg.addEventListener('mouseleave', endDrag);
            svg.addEventListener('touchstart', startDrag);
            svg.addEventListener('touchmove', drag);
            svg.addEventListener('touchend', endDrag);
            svg.addEventListener('touchleave', endDrag);
            svg.addEventListener('touchcancel', endDrag);

            let selectedElement, offset, transform,
                bbox, minX, maxX, minY, maxY, confined;
            let boundaryX1 = 10.5;
            let boundaryX2 = 30;
            let boundaryY1 = 2.2;
            let boundaryY2 = 19.2;

            function getMousePosition(evt) {
                let CTM = svg.getScreenCTM();
                if (evt.touches) { evt = evt.touches[0]; }
                return {
                x: (evt.clientX - CTM.e) / CTM.a,
                y: (evt.clientY - CTM.f) / CTM.d
                };
            }

            function mouseDown(evt) {
                if (evt.button == 0) {
                    startDrag(evt)
                }
            }

            function delTarget(evt) {
                evt.preventDefault()
                // console.log(evt)
                if (evt.explicitOriginalTarget.nodeName !== "svg") {
                    evt.target.remove()
                }
                return false
            }

            function startDrag(evt) {
                if (evt.target.classList.contains('draggable')) {
                selectedElement = evt.target;
                offset = getMousePosition(evt);
                // Make sure the first transform on the element is a translate transform
                let transforms = selectedElement.transform.baseVal;
                if (transforms.length === 0 || transforms.getItem(0).type !== SVGTransform.SVG_TRANSFORM_TRANSLATE) {
                    // Create an transform that translates by (0, 0)
                    let translate = svg.createSVGTransform();
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
                    let coord = getMousePosition(evt);
                    let dx = coord.x - offset.x;
                    let dy = coord.y - offset.y;
                    if (confined) {
                        if (dx < minX) { dx = minX; }
                        else if (dx > maxX) { dx = maxX; }
                        if (dy < minY) { dy = minY; }
                        else if (dy > maxY) { dy = maxY; }
                    }
                    transform.setTranslate(dx, dy);

                    let text = selectedElement.id
                    selectedElement.innerHTML = `${-(50-coord.y).toFixed() >0 ? -(50-coord.y).toFixed() : 1} - ${text}`
                }
                
            }
            function endDrag(evt) {
                selectedElement = false;
            }
        }
    ]]> </script>
    <!-- {#each data as point, i} -->
    {#each shuffle(data) as point, i}
        <text class="draggable" id="{point}"
        x={calcX(i, viewBoxW)}
        y={calcY(i, viewBoxH, calcFontHeight)}
        text-anchor="left" fill="white" font-size="{checkDesktop
            ? desktopFontSize
            : mobileFontSize}px" alignment-baseline="middle">{point}</text>
    {/each}
</svg>
<form action="?/submit" method="POST">
    <input class="hidden" type="text" id="answers" name="answers" value={[topic, ...ranking]} />
    <input class="hidden" type="text" id="subject" name="subject" value={subject.title}>
    <button type="submit" style="float:right;font-size:1rem;padding:1rem;" on:click={()=> {
        let svg = document.getElementById("dragBox")
        let children = svg?.children
        if (children != undefined) {
            for (let child of children) {
                if (child.nodeName == "text") {
                    let content  = child.innerHTML;
                    let split = content.split(" - ")
                    ranking = [...ranking, split.join(' ')]
                }
            }
        }
    }}>
        Done
    </button>
</form>