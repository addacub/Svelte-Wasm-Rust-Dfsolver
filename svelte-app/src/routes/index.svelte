<script context="module" lang="ts">
	export const prerender = true;
</script>

<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { Shadow } from 'svelte-loading-spinners';

	onMount(() => {
		goto('/solver', { replaceState: true });
	});
</script>

<svelte:head>
	<title>A Puzzle A Day</title>
	<meta name="description" content="A Puzzle A Day Game and Solver" />
</svelte:head>

<div class="loading_spinner centred">
	<Shadow size="60" unit="px" />
</div>

<style>
	.centred {
		/* Center vertically and horizontally */
		display: flex;
		align-items: center;
	}

	.loading_spinner {
		font-family: arial;
		font-size: 24px;
		margin: 0 auto;
		padding-top: 145%;
		padding-bottom: 10%;
	}
</style>
