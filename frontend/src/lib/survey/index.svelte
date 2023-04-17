<script lang="ts">
	import type { Survey } from "./types";

	export let subject: Survey;

	let [img, alt] = subject.image;
	let answers = Array.from({length: 24});
	let topic = 0;
	$: inProgress = false;
	$: username = "username"
	$: email = "email"
	$: question = 0;
	$: currentTopic = subject.questions[topic].topic;
	$: currentQuestion = subject.questions[topic].questions;
	$: submit = false;

	const nextQuestion = (r: number) => {
		if (inProgress === false) inProgress = true
		answers[currentQuestion[question].id-1] = r+1
		if (subject.questions[topic].questions.length-1 === question) {
			if (topic === subject.questions.length-1) {
				submit = true
			} else {
				topic++
				question=0
			}
		} else question ++
	}
</script>

<svelte:head>
	<title>Home</title>
	<meta name="description" content="SE Survey" />
</svelte:head>

<section>
	<!-- in progress -->
	<div style="display:{inProgress === true ? 'block' : 'none'}">
		<p>
			{username}: {email}
			<button class="edit" on:click={()=>inProgress=false}>Edit</button>
		</p>
	</div>
	<br />

	<form action="https://formspree.io/f/mwkjwogv" method="POST">
		<!-- start -->
		<div class="signup" style="display:{inProgress === true ? 'none' : 'flex'}">
			<label for="username">Username</label>
			<br />
			<input type="text" id="username" name="username" on:focus|once={()=>username = ""} bind:value={username}>
			<br />
			<label for="username">Email</label>
			<br />
			<input type="email" id="email" name="email" on:focus|once={() => email = ""} bind:value={email}>
			<p on:click={()=>inProgress=true}><underline>Get started!</underline></p>
		</div>
		<div class="hidden">
			<input type="text" id="answers" name="answers" bind:value={answers}>
			<input type="text" id="subject" name="subject" bind:value={subject.title}>
		</div>
		
		<button style="display:{submit !== true ? 'none' : 'block'}" type="submit">Submit</button>
	</form>

	<div class="questionArea" style="display:{submit === true ? 'none' : 'block'}">
		<h1>
			{currentTopic}
		</h1>
		<h2>
			{currentQuestion[question].text}
		</h2>
		<div>
			{#each subject.responses as r, i }
				<button class="submit" on:click={() => nextQuestion(i)}>
					{r}			
				</button>
			{/each}
		</div>
	</div>

	<hr />
	<div class="img-and-copyright" style="font-size:{inProgress !== true ? '1rem' : '.6rem'};height: {inProgress !== true ? '6rem' : '3rem'};">
		<img src={img} alt={alt} />
		<small>{subject.copyright.title} ©{subject.copyright.date} {subject.copyright.rights}
			<br>
			<a style="font-size:{inProgress !== true ? '1rem' : '.6rem'};" href={subject.copyright.cite[1]||subject.copyright.alt[1]}>{subject.copyright.cite[0]||subject.copyright.alt[0]}</a></small>
	</div>

</section>

<style>
	section {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		flex: 0.6;
	}

	img {
		max-width: 50%;
		max-height: 100%;
	}

	input {
		padding: .25rem;
		width: 90%;
		margin: 0 auto;
	}
	h1 {
		width: 100%;
	}
	h2 {
		font-size: 2rem;
	}

	.hidden {
		display: none;
	}

	.img-and-copyright {
		display: flex;
		align-items: center;
		margin-top: 1rem;
	}

	.signup {
		flex-direction: column;
		justify-content: center;
		align-content: center;
		position:fixed;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background-color: rgba(0,0,0,0.8);
		width: 100%;
	}
	.submit {
		font-size: 1.5rem;
		padding: .5rem;
		margin: .25rem;
	}
	.edit {
		font-size: .8rem;
		margin: .5rem;
		opacity: .5;
	}
	.edit:hover {
		opacity: 1;
	}
	underline {
		text-decoration: underline;
	}
</style>