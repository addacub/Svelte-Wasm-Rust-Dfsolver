<script lang="ts">
	import {
		selected_day,
		selected_month,
		day_solved_for,
		month_solved_for,
		number_of_hints,
		solution_number,
		selected_solution,
		solution_set,
		colour_classes
	} from '$lib/solver/store';
	import init, { solve } from 'wasm-dfsolver';
	import { shuffle } from '$lib/solver/utils';
	import { Circle2 } from 'svelte-loading-spinners';

	// Controls when the spinner loader is shown
	let processing = false;
	// Controls when the solve_click function can execute
	let pressed = false;

	function solve_click(): void {
		if (pressed == true) {
			console.debug('Entered guard statement: ' + processing);
			return;
		}

		processing = true;
		pressed = true;

		// let start_time = new Date().getMilliseconds();
		init().then(
			// Success result
			() => {
				solution_set.set(
					<
						{
							name: String;
							board_position: [number, number];
							orientation: { data: number[]; shape: { cols: number; rows: number } };
						}[][]
					>solve($selected_day, $selected_month)
				);
				// let duration = start_time - new Date().getMilliseconds();
				// console.debug(duration);

				day_solved_for.set($selected_day);
				month_solved_for.set($selected_month);

				// Adjust solution number if larger than new solution set
				if ($solution_number > $solution_set.length) {
					solution_number.set($solution_set.length);
				}

				set_selected_solution();
				processing = false;
				setTimeout(() => (pressed = false), 50);
			},
			// Failure result
			() => {
				console.log('A unspecified failure occured.');
				return null;
			}
		);
	}

	function set_selected_solution(): void {
		selected_solution.set(shuffle($solution_set[$solution_number - 1]));
		colour_classes.set(shuffle($colour_classes));
	}
</script>

<section class="row">
	<div class="column left">
		<div class="slide-container padded">
			<p>Select how many pieces to show:</p>
			<input bind:value={$number_of_hints} type="range" min="0" max="8" step="1" />
			<output><b>{$number_of_hints}</b></output>
		</div>
	</div>
</section>
<section class="row">
	<div class="column left">
		<div class="spinner-container padded">
			<p>Select solution to show:</p>
			<input
				type="number"
				bind:value={$solution_number}
				min="1"
				max={$solution_set != undefined ? $solution_set.length : 0}
				disabled={$solution_set == undefined}
				on:change={set_selected_solution}
			/>
			{#if $solution_set != undefined}
				<p>{$solution_set.length} solutions were found.</p>
			{:else}
				<p>No solutions to select from.</p>
			{/if}
		</div>
	</div>
</section>
<section class="row">
	<div class="column left padded">
		<button
			type="button"
			disabled={$selected_day == undefined || $selected_month == undefined || pressed}
			on:click={solve_click}
			style="display: inline; margin-right: 1rem"
		>
			Solve!
		</button>
		{#if $selected_day == undefined || !selected_month == undefined}
			<p>Select a day and month to solve for.</p>
		{/if}
	</div>
	<div class="column right padded" style="display:block">
		{#if processing}
			<Circle2 size="60" unit="px" />
		{/if}
	</div>
</section>

<style>
	div {
		float: left;
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
		border-color: white rgb(110, 110, 110) rgb(110, 110, 110) white;
	}

	button:active {
		border-color: rgb(110, 110, 110) white white rgb(110, 110, 110);
	}

	button:hover {
		background-color: hsl(318, 20%, 50%);
	}

	.padded {
		padding-top: 5rem;
	}

	.row {
		display: flex;
	}

	.column {
		float: left;
	}

	.left {
		width: 70%;
	}

	.right {
		width: 30%;
		margin-right: 6rem;
	}
</style>
