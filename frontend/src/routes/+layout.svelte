<script>
	import Header from './Header.svelte';
	import './styles.css';
  import { page } from '$app/stores'
</script>  

<div class="app">
	<Header />

	<main>
		<slot />
	</main>

	<footer>
		<hr />
		<p><a href="https://github.com/danielclough/wtf">WTF?</a> CopyLeft (AGPL) 2023 </p>
	</footer>
</div>
