<script lang="ts">
	import type { Survey } from "./types";
	import { onMount } from "svelte";
	import Form from "./form.svelte";
	import QuestionArea from "./questionArea.svelte";

	export let subject: Survey;

	let [img, alt] = subject.image;

	// collect number of questions and types
	$: nOfQuestions = 0;
	let nOfTypes = 0;
	let uniqueTypes: string[] = [];
	let answers: number[] = [];
	let questionList: string[] = [];
	let types: number[] = [];
	$: types = types;
	onMount(()=>{
		for (let questionsBlock of subject.questions) {
			nOfQuestions += questionsBlock.questions.length
			for (let qBlock of questionsBlock.questions) {
				nOfTypes += qBlock.type.length
				for (let t of qBlock.type) {
					if (!uniqueTypes.includes(t)) uniqueTypes.push(t)
				}
			}
		}
		// answers is an array of numbers the length of the nOfQuestions
		answers = Array.from({length: nOfQuestions})
		// types is an array of strings the length of the nOfTypes
		types = Array.from({length: uniqueTypes.length}, ()=>0)
	})

	// handle Questions
	let topic = 0;
	$: inProgress = true;
	$: question = 0;
	$: currentTopic = subject.questions[topic].topic;
	$: currentQuestion = subject.questions[topic].questions;
	$: submit = false;

</script>

<svelte:head>
	<title>Home</title>
	<meta name="description" content="SE Survey" />
</svelte:head>

<section>
	<Form bind:uniqueTypes={uniqueTypes} bind:types={types} bind:submit={submit} bind:inProgress={inProgress} bind:answers={answers} bind:subject={subject} bind:questionList={questionList} />
	<QuestionArea bind:uniqueTypes={uniqueTypes} bind:types={types} bind:answers={answers} bind:questionList={questionList} bind:topic={topic}  bind:subject={subject} bind:submit={submit} bind:inProgress={inProgress} bind:currentTopic={currentTopic} bind:question={question} bind:nOfQuestions={nOfQuestions} bind:currentQuestion={currentQuestion} />
</section>

<hr />
	
<div class="img-and-copyright">
	<img src={img} alt={alt} />
	<a href={subject.copyright.cite[1]||subject.copyright.alt[1]}>
		{subject.copyright.cite[0]||subject.copyright.alt[0]}
	</a>
	<br>
	<small>
		{subject.copyright.title} ©{subject.copyright.date} {subject.copyright.rights}
	</small>
</div>

<style>
	img {
		max-width: 20%;
		max-height: 100%;
		float: left;
	}

	.img-and-copyright {
		margin-top: 1rem;
		line-height: .8rem;
	}
	a {
		line-height: .8rem;
		font-size: .8rem;
	}
	small {
		line-height: .6rem;
		font-size: .6rem;
	}
</style>