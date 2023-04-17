<script>
// @ts-nocheck

	import epistemology from "$lib/survey/epistemology/se-v1";
	import ontology from "$lib/survey/ontology/wtf-v1";
	import ism from "$lib/survey/ism/wtf-v1";
	import Survey from "$lib/survey/index.svelte"
	import { page } from '$app/stores';
    
    const subject = $page.params.subject;
</script>

{#if subject === 'epistemology' }
	<Survey subject={epistemology} />
{:else if subject === 'ontology' }
	<Survey subject={ontology} />
{:else if subject === 'ism' }
	<Survey subject={ism} />
{/if}