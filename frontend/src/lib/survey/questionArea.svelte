<script lang="ts">
	import { returnColor } from "./utils";

    export let submit: any;
    export let inProgress: any;
    export let currentTopic: any;
    export let question: any;
    export let nOfQuestions: number;
    export let currentQuestion: any;
    export let subject: any;
    export let answers: any;
    export let questionList: any;
    export let topic: any;
    export let uniqueTypes: any;
    export let types: any;

    const handleQuestion = (r: number) => {
        let score = r + 1;
        answers[currentQuestion[question].id - 1] = score;
        let weight = score -3;
        
        questionList[currentQuestion[question].id - 1] = currentQuestion[question].text;
        uniqueTypes.forEach((t: string, i: number) => {
            if (currentQuestion[question].type.includes(t)) {
                types[i] += weight
            }
        });

        if (subject.questions[topic].questions.length - 1 === question) {
            if (topic === subject.questions.length - 1) {
                submit = true;
            } else {
                topic++;
                question = 0;
            }
        } else question++;
    };

</script>


<div class="questionArea" style="display:{(submit == true || inProgress == false) ? 'none' : 'block'}">
    <h1>
        {currentTopic}
    </h1>
    <span>
        {question} / {nOfQuestions}
    </span>
    <h2>
        {currentQuestion[question].text}
    </h2>
    <div>
        {#each subject.responses as r, i }
            <button style={returnColor(r)} on:click={() => handleQuestion(i)}>
                {r}			
            </button>
        {/each}
    </div>
    <ul>
        {#if currentQuestion[question].type.length > 0}
            {#each currentQuestion[question].type as t}
                <li>
                    {t}
                </li>
            {/each}
        {/if}
    </ul>

    <p on:click={()=>inProgress=false}><underline>Pause</underline></p>
</div>

<style>
	ul {
		display: flex;
		flex-direction: row;
	}
	li {
		list-style: "#";
		padding-right: 1rem;
		font-size: .8rem;
	}
	button {
		font-size: 1.5rem;
		padding: .5rem;
		margin: .25rem;
	}
	underline {
		text-decoration: underline;
	}
</style>