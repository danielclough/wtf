<script lang="ts">
    export let current: any;
    export let displayOpt: boolean;
    let answer = "";
</script>

<hr>

{#if current.details.length === 1}
    <p>
        {current.details[0]}
    </p>
{:else}
    <p>List Opts</p>
{/if}

{#each current.text as button}
    <button
        class="{answer === button ? "selected" : "semi-opaque"}"
        value={button} 
        on:click={()=> {
            answer = button;
            if (button === "Yes") {
                displayOpt = true
            } else displayOpt = false
    }}>
        {button}
    </button>
{/each}

<hr>
