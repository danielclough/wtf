<script>
	import { page } from '$app/stores';
</script>

<header>
	<div class="corner">
		<a href="/">
			<img src="/images/wtf-100.png" alt="?" />
		</a>
	</div>

	<nav>
		<ul>
			<li aria-current={$page.url.pathname === '/' ? 'page' : undefined}>
				<a href="/">Home</a>
			</li>
			<li aria-current={$page.url.pathname === '/about' ? 'page' : undefined}>
				<a href="/about">About</a>
			</li>
		</ul>
	</nav>

	<div class="corner">
		<a href="https://github.com/danielclough/wtf">
			<img src="/images/github.svg" alt="GitHub" />
		</a>
	</div>
</header>