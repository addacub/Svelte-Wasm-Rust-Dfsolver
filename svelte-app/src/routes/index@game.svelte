<script context="module" lang="ts">
	export const prerender = true;
</script>

<script lang="ts">
	import Canvas from '$lib/game/Canvas.svelte';
	import Piece from '$lib/game/Piece.svelte';
	import { drawScale, Point, mousePosition, scaleFactor } from '$lib/game/store';

	let height: number;
	let width: number;
</script>

<svelte:head>
	<title>A Puzzle A Day</title>
	<meta name="description" content="A Puzzle A Day Interactive Game" />
</svelte:head>

<svelte:window bind:innerHeight={height} bind:innerWidth={width} />

<section>
	<p>
		This is where the game will live.&emsp; Canvas dimensions: height = {height *
			$scaleFactor}&emsp;width = {width * $scaleFactor}
		&emsp;&emsp;Mouse position: x = {$mousePosition.getX().toPrecision(5)}&emsp;y = {$mousePosition
			.getY()
			.toPrecision(5)}
	</p>
</section>

<Canvas height={height * $scaleFactor} width={width * $scaleFactor}>
	<Piece
		position={new Point(1 * $drawScale, 1 * $drawScale)}
		height={2}
		width={3}
		svgPath="h{1 * $drawScale} v{1 * $drawScale} h{1 * $drawScale} v{-1 * $drawScale} h{1 *
			$drawScale} v{2 * $drawScale} h{-3 * $drawScale} z"
		hsl={[149, 88, 41]}
	/>
	<Piece
		position={new Point(1 * $drawScale, 4 * $drawScale)}
		height={2}
		width={3}
		svgPath="h{3 * $drawScale} v{2 * $drawScale} h{-3 * $drawScale} z"
		hsl={[290, 100, 60]}
	/>
	<Piece
		position={new Point(1 * $drawScale, 7 * $drawScale)}
		height={2}
		width={3}
		svgPath="h{2 * $drawScale} v{1 * $drawScale} h{1 * $drawScale} v{1 * $drawScale} h{-3 *
			$drawScale} z"
		hsl={[58, 100, 44]}
	/>
	<Piece
		position={new Point(1 * $drawScale, 10 * $drawScale)}
		height={2}
		width={4}
		svgPath="h{4 * $drawScale} v{2 * $drawScale} h{-1 * $drawScale} v{-1 * $drawScale} h{-3 *
			$drawScale} z"
		hsl={[172, 100, 68]}
	/>
	<Piece
		position={new Point(7 * $drawScale, 1 * $drawScale)}
		height={3}
		width={3}
		svgPath="h{1 * $drawScale} v{2 * $drawScale} h{2 * $drawScale} v{1 * $drawScale} h{-3 *
			$drawScale} z"
		hsl={[345, 100, 64]}
	/>
	<Piece
		position={new Point(7 * $drawScale, 5 * $drawScale)}
		height={2}
		width={4}
		svgPath="h{3 * $drawScale} v{1 * $drawScale} h{1 * $drawScale} v{1 * $drawScale} h{-2 *
			$drawScale} v{-1 * $drawScale} h{-2 * $drawScale} z"
		hsl={[212, 100, 64]}
	/>
	<Piece
		position={new Point(7 * $drawScale, 8 * $drawScale)}
		height={2}
		width={4}
		svgPath="h{4 * $drawScale} v{1 * $drawScale} h{-2 * $drawScale} v{1 * $drawScale} h{-1 *
			$drawScale} v{-1 * $drawScale} h{-1 * $drawScale} z"
		hsl={[0, 100, 56]}
	/>
	<Piece
		position={new Point(7 * $drawScale, 11 * $drawScale)}
		height={3}
		width={3}
		svgPath="h{1 * $drawScale} v{2 * $drawScale} h{2 * $drawScale} v{1 * $drawScale} h{-3 *
			$drawScale} z"
		hsl={[104, 99, 72]}
	/>
</Canvas>

<style>
</style>
