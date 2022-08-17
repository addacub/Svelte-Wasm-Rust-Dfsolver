<script lang="ts">
	import { spring } from 'svelte/motion';
	import type { Writable } from 'svelte/store';
	import { solution_number } from './store';

	export let min: number = 1;
	export let max: number = 1;
	let count: Writable<number> = solution_number;

	const displayed_count = spring();
	$: displayed_count.set($count);
	$: offset = modulo($displayed_count, 1);

	function modulo(n: number, m: number) {
		// handle negative numbers
		return ((n % m) + m) % m;
	}

	function decrement() {
		if ($count > min) {
			count.set($count - 1);
		}
	}

	function increment() {
		if ($count < max) {
			count.set($count + 1);
		}
	}
</script>

<div class="counter">
	<button on:click={decrement} aria-label="Decrease the counter by one">
		<svg aria-hidden="true" viewBox="0 0 1 1">
			<path d="M0,0.5 L1,0.5" />
		</svg>
	</button>

	<div class="counter-viewport">
		<div class="counter-digits" style="transform: translate(0, {100 * offset}%)">
			<strong class="hidden" aria-hidden="true">{Math.floor($displayed_count + 1)}</strong>
			<strong>{Math.floor($displayed_count)}</strong>
		</div>
	</div>

	<button on:click={increment} aria-label="Increase the counter by one">
		<svg aria-hidden="true" viewBox="0 0 1 1">
			<path d="M0,0.5 L1,0.5 M0.5,0 L0.5,1" />
		</svg>
	</button>
</div>

<style>
	.counter {
		display: flex;
		margin: 1rem 0;
	}

	.counter button {
		width: 1em;
		padding: 0;
		display: flex;
		align-items: center;
		justify-content: center;
		border: 0;
		background-color: #c8adc0;
		touch-action: manipulation;
		font-size: 2rem;
		border-radius: 5%;
	}

	.counter button:hover {
		background-color: hsl(318, 20%, 50%);
	}

	.counter button:active {
		transform: translateY(4px);
	}

	svg {
		width: 25%;
		height: 25%;
	}

	path {
		vector-effect: non-scaling-stroke;
		stroke-width: 2px;
		stroke: black;
	}

	.counter-viewport {
		width: 3.5em;
		height: 2em;
		overflow: hidden;
		text-align: center;
		position: relative;
	}

	.counter-viewport strong {
		position: absolute;
		display: flex;
		width: 100%;
		height: 100%;
		font-weight: 400;
		color: var(--text-color);
		font-size: 1.5rem;
		align-items: center;
		justify-content: center;
	}

	.counter-digits {
		position: absolute;
		width: 100%;
		height: 100%;
	}

	.hidden {
		top: -100%;
		user-select: none;
	}
</style>
