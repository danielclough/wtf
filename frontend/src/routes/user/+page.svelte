<script>
// @ts-nocheck

	import { page } from '$app/stores';
	import User from '$lib/user/index.svelte';
    const user = $page.data.user;

</script>

<User {user} />