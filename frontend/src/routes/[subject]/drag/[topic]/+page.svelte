<script>
// @ts-nocheck

	import Drag from "$lib/drag/index.svelte"
	import political from "$lib/drag/political/wtf-v1";

	import { page } from '$app/stores';
    const subject = $page.params.subject;
    const topic = $page.params.topic;
</script>

{#if subject === 'political' }
	<Drag subject={political} topic={topic} />
{/if}
