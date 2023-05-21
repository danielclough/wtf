<script lang="ts">
    import Slides from "$lib/components/slides.svelte";
    import Buttons from "$lib/components/buttons.svelte";
    import interview from "$lib/data/interview";
    import Record from "$lib/components/record.svelte";
    
    $: displayOpt = false
    
    let mediaRecorder: MediaRecorder;
    const stop =()=> {
        mediaRecorder.stop()
    }
</script>


<Record bind:mediaRecorder={mediaRecorder}/>
<Slides>
    {#if interview.version === "Verbal v1"}
        {#each interview.questions as section,i}
            <section>
                {section.subject}
                {#each section.prompts as prompt}
                    <section>
                        <p>
                            {@html prompt}
                        </p>
                        {#if (interview.questions.length - 1) === i}
                            <a on:click={()=>stop()} data-sveltekit-reload href="/">
                                <button>Finished!</button>
                            </a>
                        {/if}
                    </section>
                {/each}
            </section>
        {/each}
    {/if}
</Slides>
