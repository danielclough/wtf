<script lang="ts">
    export let current;
    export let i;
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
    <button class="{answer === button ? "selected" : "not_selected"}" value={button} on:click={()=> answer = button}>
        {button}
    </button>
{/each}

<hr>
