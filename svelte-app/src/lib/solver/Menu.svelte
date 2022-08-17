<script context="module" lang="ts">
	export const prerender = true;
</script>

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
		colour_classes,
		hide_text,
		is_valid_solution
	} from '$lib/solver/store';
	import { solve } from 'wasm-dfsolver';
	import { shuffle } from '$lib/solver/utils';
	import { Circle2 } from 'svelte-loading-spinners';
	import Counter from '$lib/solver/Counter.svelte';

	let processing = false;

	function solve_click(): void {
		processing = true;

		// Send to back of event loop queue to let GUI catch up with state
		setTimeout(() => {
			solution_set.set(
				<
					{
						name: String;
						board_position: [number, number];
						orientation: { data: number[]; shape: { cols: number; rows: number } };
					}[][]
				>solve($selected_day, $selected_month)
			);

			day_solved_for.set($selected_day);
			month_solved_for.set($selected_month);
			is_valid_solution.set(true);

			// Adjust solution number if larger than new solution set
			if ($solution_number > $solution_set.length) {
				solution_number.set($solution_set.length);
			}

			set_selected_solution();
			setTimeout(() => (processing = false), 0);
		}, 0);
	}

	// Reactive declaration: whenever the value of selected_solution changes,
	// the set_selected_solution() function is called
	$: $solution_number, set_selected_solution();

	function set_selected_solution(): void {
		// Guard statement
		if ($solution_set.length == 0) return;

		selected_solution.set(shuffle($solution_set[$solution_number - 1]));
		colour_classes.set(shuffle($colour_classes));
	}
</script>

<div class="content">
	<div class="row">
		<div class="column left">
			<div class="left-content">
				<p>Select how many pieces to show:</p>
				<input class="range" type="range" bind:value={$number_of_hints} min="0" max="8" step="1" />
				<output><b>{$number_of_hints}</b></output>
			</div>
		</div>

		<div class="column right">
			<div class="right-content" />
		</div>
	</div>
	<div class="row">
		<div class="column left">
			<div class="left-content">
				<p>Select solution to show:</p>
				<input
					class="range"
					type="range"
					bind:value={$solution_number}
					min="1"
					max={$solution_set.length}
					step="1"
					disabled={$solution_set.length == 0}
					on:change={set_selected_solution}
				/>
				{#if $solution_set.length > 0}
					<p>{$solution_set.length} solutions were found.</p>
				{:else}
					<p>No solutions to select from.</p>
				{/if}
			</div>
		</div>
		<div class="column right">
			<div class="right-content">
				<Counter max={$solution_set.length} />
			</div>
		</div>
	</div>
	<div class="row">
		<div class="column left ">
			<div class="left-content">
				<label for="checkbox_1">Cover dates when piece is revealed?</label>
			</div>
		</div>
		<div class="column right ">
			<div class="right-content">
				<input type="checkbox" id="checkbox_1" bind:checked={$hide_text} />
			</div>
		</div>
	</div>
	<div class="row">
		<div class="column left ">
			<div class="left-content" style="height: 70px;">
				<div class="centre padding-top">
					<button
						type="button"
						disabled={$selected_day == undefined || $selected_month == undefined || processing}
						on:click={solve_click}
					>
						Solve!
					</button>
				</div>

				{#if $selected_day == undefined || !selected_month == undefined}
					<p>Select a day and month to solve for.</p>
				{/if}
			</div>
		</div>
		<div class="column right " style="display:block">
			<div class="right-content" style="height: 70px;">
				<div class="centre loading-spinner">
					{#if processing}
						<Circle2 size="60" unit="px" />
					{/if}
				</div>
			</div>
		</div>
	</div>
</div>

<style>
	input {
		display: block;
		margin: auto;
	}

	input[type='checkbox'] {
		width: 1.5rem;
		height: 1.5rem;
	}

	input[type='range'] {
		width: 10rem;
		height: 1.5rem;
	}

	output {
		text-align: center;
		color: white;
		display: block;
		padding-top: 15px;
	}

	button {
		display: inline;
		background-color: #c8adc0;
		width: 6em;
		font-size: xx-large;
		border: 0;
		border-radius: 3px;
	}

	button:active {
		transform: translateY(4px);
	}

	button:hover {
		background-color: hsl(318, 20%, 50%);
	}

	.row {
		display: flex;
		padding-bottom: 25%;
	}

	.column {
		float: left;
		height: 100%;
		margin: auto;
	}

	.left {
		width: 70%;
	}

	.right {
		width: 30%;
		padding-left: 1rem;
	}

	.left-content {
		text-align: center;
	}

	.right-content {
		display: flex;
		float: left;
		margin: 0;
	}

	.padding-top {
		padding-top: 1rem;
	}

	.centre {
		display: flex;
		align-items: center;
		justify-content: center;
		margin: auto;
	}

	.content {
		float: left;
		width: 100%;
	}

	@media screen and (max-height: 760px) {
		.row {
			padding-bottom: 10%;
		}

		.content {
			height: 400px;
		}

		button {
			font-size: large;
		}

		input[type='range'] {
			font-size: x-small;
		}

		input[type='checkbox'] {
			width: 15px;
			height: 15px;
		}

		p {
			font-size: small;
		}

		label {
			font-size: small;
		}

		output {
			font-size: small;
		}

		.loading-spinner {
			transform: scale(0.6);
		}
	}

	@media screen and (max-width: 1200px) and (orientation: portrait) {
		.content {
			padding-top: 2rem;
		}
	}
</style>
