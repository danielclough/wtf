<script>
// @ts-nocheck

	import Drag from "$lib/drag/index.svelte"
	import politics from "$lib/drag/politics/wtf-v1";

	import { page } from '$app/stores';
    const subject = $page.params.subject;
    const topic = $page.params.topic;
</script>

{#if subject === 'politics' }
	<Drag subject={politics} topic={topic} />
{/if}
