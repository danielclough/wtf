<script lang="ts">
    import Slides from "$lib/components/slides.svelte";
    import Buttons from "$lib/components/buttons.svelte";
    import intro from "$lib/data/intro";
</script>

<Slides>
    {#each intro as section,i}
        <section>
            {section.md}
            {#if !!section.btns}
                {#each section.btns as current,i}
                    <Buttons {current} {i} />
                {/each}
            {/if}
            {#if (intro.length - 1) === i}
                <a href="/interview">
                    <button>Start</button>
                </a>
            {/if}
        </section>
    {/each}
</Slides>