<script>
	// @ts-ignore
	import { onMount } from "svelte";
    onMount(() => {
        // @ts-ignore
        Reveal.initialize({
            dependencies: [
                { src: 'plugin/markdown/marked.js' },
                { src: 'plugin/markdown/markdown.js' },
                { src: 'plugin/notes/notes.js', async: true },
                // @ts-ignore
                { src: 'plugin/highlight/highlight.js', async: true, callback: function () { hljs.initHighlightingOnLoad(); } }
            ]
        });
    })
</script>
<div class="reveal">
    <div class="slides">
        <slot />
    </div>
</div>