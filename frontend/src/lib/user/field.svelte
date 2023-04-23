<script lang="ts">
    export let field: string[];
    $: editing = false;
    $: change = '';
</script>


{#if editing}
    <div>
        {field[0]}:
        <input bind:value={change} >
        <button type="submit">
            Save
        </button>
    </div>
{:else}
    <div>
        <p>
            {field[0]}:
            <br>
            <span>
                {field[1]}
            </span>
        </p>
    </div>
{/if}

<button on:click={()=>{editing = !editing}}>
    {editing ? 'Nevermind' : 'Edit'}
</button>
<hr>


<style>
	p {
		font-weight: 700;
	}
	span {
		font-weight: 300;
	}
	button {
		float: right;
        margin-left: .5rem;
	}
    input {
        margin-bottom: .5rem;
    }
</style>