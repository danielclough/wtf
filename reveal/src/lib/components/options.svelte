<script lang="ts">
    export let optionCurrent;
    export let i;
    let answerResponse = "";
</script>

{#if optionCurrent.details.length === 1}
<p>
    {optionCurrent.details[0]}
</p>
{:else}
<p>List Opts</p>
{/if}

<select bind:value={answerResponse}>
{#each optionCurrent.html as option}
    <option value={i+option}>
        {option}
    </option>
{/each}
</select>