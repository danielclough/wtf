<script lang="ts">
	import Field from "./field.svelte";

    export let user: string[];
</script>
{#each Object.entries(user) as field}
    <Field bind:field={field} />
{/each}
