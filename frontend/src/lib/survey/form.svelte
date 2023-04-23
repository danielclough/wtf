<script lang="ts">
	import { returnColor } from "./utils";

    export let submit: any;
    export let inProgress: any;
    export let answers: any;
    export let subject: any;
    export let questionList: any;
    export let types: number[];
    export let uniqueTypes: any;
    
</script>

<form action="?/submit" method="POST">
    <!-- Answered -->
    <div class="paused" style="display:{inProgress === false || submit === true ? 'flex' : 'none'}">
        
        <h2>
            Results
        </h2>
        
        <div class="space">
            {#each uniqueTypes as type,i}
                <p>
                    {uniqueTypes[i].charAt(0).toUpperCase()+uniqueTypes[i].slice(1)}
                    <a href="https://wikipedia.org/wiki/{uniqueTypes[i]}">
                        <span class="tiny">
                            Link
                        </span>
                    </a>
                    = {(types[i]).toFixed()}
                </p>
            {/each}

            {#if submit}
                <button type="submit">Submit</button>
            {:else}
                <p class="btn" on:click={()=>inProgress=true}><underline>Continue!</underline></p>
            {/if}
        </div>
        
        
        <h3>
            Answered
        </h3>

        {#each questionList as answeredQuestion,i}
            <div class="answered">
                <div class="q">
                    {answeredQuestion}
                </div>
                <div class="a" style='{returnColor(answers[i])}'>
                    {subject.responses[answers[i]-1]}
                </div>
            </div>
        {/each}

    </div>
    <div class="hidden">
        <input type="text" id="answers" name="answers" bind:value={answers}>
        <input type="text" id="subject" name="subject" bind:value={subject.title}>
    </div>

</form>

<style>
	input {
		padding: .25rem;
		width: 90%;
		margin: 0 auto;
	}

	.answered {
		margin-bottom: 1rem;
	}
	.answered .q {
		font-size: 1.2rem;
	}

	.paused {
		flex-direction: column;
		align-content: center;
		position:fixed;
		overflow-y: auto;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background-color: rgba(0,0,0,1);
		width: 100%;
		height: 100vh;
	}

    .space {
        margin: 0 auto;
    }
    .space p {
        margin: .1rem;
    }
    .btn {
        border: 1px solid white;
        padding: .5rem;
        margin: 2rem auto;
    }
    .tiny {
        font-size: .6rem;
        position: relative;
        top: -.5rem;
    }
</style>