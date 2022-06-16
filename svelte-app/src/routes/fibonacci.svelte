<script context="module" lang="ts">
	export const prerender = true;
</script>

<script lang="ts">
	import Canvas from '$lib/fibonacci/Canvas.svelte';
	import Circle from '$lib/fibonacci/Circle.svelte';
	import { tweened } from 'svelte/motion';
	import { interpolateHsl, interpolateRound } from 'd3-interpolate';

	let angle = 0.618;
	let tweenedCount = tweened(0, {
		duration: 1500,
		interpolate: (a, b) => (t) => Math.round((b - a) * t + a)
	});
	const color = interpolateHsl('#35b1bb', '#f202d2');
	const MAX_COUNT = 1500;
</script>

<svelte:head>
	<title>A Puzzle A Day</title>
	<meta name="description" content="A Puzzle A Day Game and Solver" />
</svelte:head>

<section class="row">
	<div>
		<button
			on:click={() => {
				$tweenedCount = $tweenedCount > 0 ? 0 : MAX_COUNT;
			}}>Play</button
		>
		<input min="0" step="0.001" max="1" type="number" bind:value={angle} />
	</div>
</section>

<section class="row">
	<Canvas height={1000} width={1000}>
		{#each { length: $tweenedCount } as _, index (index)}
			<!-- _angleForCircle = index * angle * 360 (deg) -->
			<!-- dist = index * 0.2 -->
			<!-- _angleInRadian = angle * Math.PI / 180 -->
			<!-- x = Math.cos(_angleForCircle) * dist -->
			<!-- y = Math.sin(_angleForCircle) * dist -->
			<Circle
				r={3}
				cx={Math.cos(index * angle * 2 * Math.PI) * index * 0.3}
				cy={Math.sin(index * angle * 2 * Math.PI) * index * 0.3}
				fill={color(index / MAX_COUNT)}
			/>
		{/each}
	</Canvas>
</section>

<style>
</style>
