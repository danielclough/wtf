<script lang="ts">
    import Slides from "$lib/components/slides.svelte";
    import Buttons from "$lib/components/buttons.svelte";
    import intro from "$lib/data/intro";
    import Record from "$lib/components/record.svelte";
    
    $: displayOpt = false
    
    let mediaRecorder: MediaRecorder;
    const stop =()=> {
        mediaRecorder.stop()
    }
</script>

<Record bind:mediaRecorder={mediaRecorder}/>

<Slides>
    {#each intro.questions as section,i}
        <section>
            {@html section.html}
            <br>
            {#if !!section.btns}
                {#each section.btns as current}
                    <Buttons bind:displayOpt={displayOpt} {current} />
                {/each}
            {/if}
            {#if displayOpt === true }
                {@html section.opt}
            {/if}
            {#if (intro.questions.length - 1) === i}
                <a on:click={()=>stop()} data-sveltekit-reload href="/interview">
                    <button>Start</button>
                </a>
            {/if}
        </section>
    {/each}
</Slides>