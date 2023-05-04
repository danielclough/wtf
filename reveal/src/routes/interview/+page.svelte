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
                {#each section[i][1] as subSection,j}
                    <section>
                        {subSection[i]}
                    </section>
                {/each}
            {:else}
                {#each section[i] as subSection,j}
                    <section>
                        {subSection[i]}
                    </section>
                {/each}
            {/if}
        {/each}
    {/if}
</Slides>