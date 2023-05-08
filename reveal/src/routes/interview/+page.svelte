<script lang="ts">
    import Slides from "$lib/components/slides.svelte";
    import interview from "$lib/data/interview";
</script>

<Slides>
    {#if interview.version === "Verbal v1"}
        {#each interview.questions as section}
            <section>
                    {section.subject}
                {#each section.prompts as prompt}
                <section>
                    <p>
                        {@html prompt}
                    </p>
                    </section>
                {/each}
            </section>
        {/each}
    {/if}
</Slides>