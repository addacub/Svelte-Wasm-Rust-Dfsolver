<script context="module" lang="ts">
	export const prerender = true;
</script>

<script lang="ts">
	import Canvas from '$lib/game/Canvas.svelte';
	import Piece1 from '$lib/game/pieces/Piece1.svelte';

	let height: number;
	let width: number;
	const SCALE_FACTOR: number = 0.9;
</script>

<svelte:head>
	<title>A Puzzle A Day</title>
	<meta name="description" content="A Puzzle A Day Interactive Game" />
</svelte:head>

<svelte:window bind:innerHeight={height} bind:innerWidth={width} />

<section class="row">
	<p>This is where the game will live</p>
	<p>{height * SCALE_FACTOR} {width * SCALE_FACTOR}</p>
</section>

<Canvas height={height * SCALE_FACTOR} width={width * SCALE_FACTOR}>
	<Piece1 hsl={[149, 88, 41]} />
</Canvas>

<style>
</style>
