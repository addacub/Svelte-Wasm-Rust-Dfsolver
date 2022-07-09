<script context="module" lang="ts">
	export const prerender = true;
</script>

<script lang="ts">
	import Canvas from '$lib/game/Canvas.svelte';
	import Piece from '$lib/game/Piece.svelte';
	import { drawScale, Point, mousePosition, scaleFactor } from '$lib/game/store';

	let height: number;
	let width: number;
	let ViewportHeight: number;
	let ViewportWidth: number;
	let h: number = 100;
	let w: number = 100;

	function determineViewportSize(): void {
		ViewportWidth = Math.max(document.documentElement.clientWidth || 0, window.innerWidth || 0);
		ViewportHeight = Math.max(document.documentElement.clientHeight || 0, window.innerHeight || 0);
	}

	function logSizes(): void {
		determineViewportSize();
		console.log(`window dimensions: height = ${height} \t width = ${width}`);
		console.log(`viewport dimensions: height = ${ViewportHeight} \t width = ${ViewportWidth}`);

		h += 100;
		w += 100;
	}
</script>

<svelte:head>
	<title>A Puzzle A Day</title>
	<meta name="description" content="A Puzzle A Day Interactive Game" />
</svelte:head>

<svelte:window bind:innerHeight={height} bind:innerWidth={width} on:click={logSizes} />

<section class="row">
	<p>This is where the game will live</p>
	<p>Canvas dimensions: height = {height * $scaleFactor}&emsp;width = {width * $scaleFactor}</p>
	<p>
		Mouse position: x = {$mousePosition.getX().toPrecision(5)}&emsp;y = {$mousePosition
			.getY()
			.toPrecision(5)}
	</p>
</section>

<Canvas height={height * $scaleFactor} width={width * $scaleFactor}>
	<Piece
		position={new Point(100, 100)}
		height={2}
		width={3}
		svgPath="h{1 * $drawScale} v{1 * $drawScale} h{1 * $drawScale} v{-1 * $drawScale} h{1 *
			$drawScale} v{2 * $drawScale} h{-3 * $drawScale} z"
		hsl={[149, 88, 41]}
	/>
	<Piece
		position={new Point(500, 500)}
		height={2}
		width={3}
		svgPath="h{3 * $drawScale} v{2 * $drawScale} h{-3 * $drawScale} z"
		hsl={[290, 100, 60]}
	/>
</Canvas>

<style>
</style>
