<script lang="ts">
    import Slides from "$lib/components/slides.svelte";
    import Buttons from "$lib/components/buttons.svelte";
    import intro from "$lib/data/intro";
    $: displayHtml = false
</script>

<Slides>
    {#each intro as section,i}
        <section>
            {section.text}
            <br>
            {#if !!section.btns}
                {#each section.btns as current}
                    <Buttons bind:displayHtml={displayHtml} {current} />
                {/each}
            {/if}
            {#if displayHtml === true }
                {@html section.html}
            {/if}
            {#if (intro.length - 1) === i}
                <a data-sveltekit-reload href="/interview">
                    <button>Start</button>
                </a>
            {/if}
        </section>
    {/each}
</Slides>