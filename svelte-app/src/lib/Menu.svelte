<script lang="ts">
	import {
		selected_day,
		selected_month,
		solution_key,
		hints,
		solution,
		solution_map,
		solution_set
	} from '$lib/store';
	import init, { solve } from 'wasm-dfsolver';

	let solutions: any = $solution_map.get([$selected_day, $selected_month]);

	// Set default number of hints shown
	hints.set(1);

	function solve_click(): void {
		solution_key.set([$selected_day, $selected_month]);

		// See if solutions has already been found in map
		if ($solution_map.has($solution_key)) {
			// Return cached solution
			solution_set.set($solution_map.get($solution_key));
		} else {
			// Get solution and add to hash map
			let tmp: any = init().then(() => {
				return solve($selected_day, $selected_month);
			});

			console.log(tmp);
			$solution_map.set($solution_key, tmp);
			console.log($solution_map);
		}
	}
</script>

<div class="slide-container">
	<p>Select how many pieces to show:</p>
	<input bind:value={$hints} type="range" min="0" max="8" step="1" />
	<output><b>{$hints}</b></output>
</div>

<div class="spinner-container">
	<p>Select solution to show:</p>
	<input
		type="number"
		bind:value={$solution}
		min="1"
		max={$solutions != undefined ? solutions.length : 0}
		disabled={$solutions != undefined}
	/>
	{#if $solutions}
		<p>{$solutions.length} solutions were found.</p>
	{:else}
		<p>No solutions to select from.</p>
	{/if}
</div>

<div class="button-container">
	<button
		type="button"
		on:click={solve_click}
		disabled={$selected_day == undefined || $selected_month == undefined}
	>
		Solve!
	</button>
	{#if $selected_day == undefined || !selected_month == undefined}
		<p>Select a day and month to solve for.</p>
	{/if}
</div>

<style>
	div {
		padding-top: 5rem;
	}

	input {
		display: block;
		margin: auto;
	}

	output {
		text-align: center;
		color: white;
		display: block;
		padding-top: 15px;
	}

	button {
		background-color: #c8adc0;
		width: 6em;
		height: 2em;
		font-size: xx-large;
		border-width: 5px;
	}

	button:hover {
		background-color: hsl(318, 20%, 50%);
	}
</style>
