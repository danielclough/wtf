<script lang="ts">
	import { onMount } from "svelte";
    import { start, stop } from "./record";

    // MediaRecorder is stopped from parent
    export let mediaRecorder: MediaRecorder;

    onMount(async ()=>mediaRecorder = await start(mediaRecorder))

</script>

<audio hidden id="audio" controls></audio>
{#if mediaRecorder?.state === "recording"}
    <button on:click={async ()=> mediaRecorder = await stop(mediaRecorder)} >Stop 🎙️</button>
{:else}
    <button on:click={async ()=>mediaRecorder = await start(mediaRecorder)} >Start 🎙️</button>
{/if}

<style>
    button {
        font-size: 1.9rem;
        font-weight: 900;
        color: darkred;
        background-color: rgba(0, 0, 0, 0.3);
        line-height: 2rem;
        position: absolute;
        top: 0;
        left: 0;
        z-index: 100;
    }
</style>