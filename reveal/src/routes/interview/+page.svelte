<script lang="ts">
    import Slides from "$lib/components/slides.svelte";
    import interview from "$lib/data/interview";
</script>

<Slides>
    {#if interview.version === "Verbal v1"}
        {#each interview.questions as section,i}
            {#if typeof section[i] === 'string'}
                <section>
                    {section[i]}
                </section>
            {:else}
                <section>
                    {section[i]}
                </section>
            {/if}
        {/each}
    {/if}
</Slides>