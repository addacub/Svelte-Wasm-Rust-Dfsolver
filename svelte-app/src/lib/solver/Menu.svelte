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

	function set_selected_solution(): void {
		selected_solution.set(shuffle($solution_set[$solution_number - 1]));
		colour_classes.set(shuffle($colour_classes));
	}
</script>

<div class="content">
	<section class="row">
		<div class="column left">
			<div class="slide-container">
				<p>Select how many pieces to show:</p>
				<input bind:value={$number_of_hints} type="range" min="0" max="8" step="1" />
				<output><b>{$number_of_hints}</b></output>
			</div>
		</div>
	</section>
	<section class="row">
		<div class="column left">
			<div class="spinner-container ">
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
		<div class="column left ">
			<label for="checkbox_1">Cover dates when piece is revealed?</label>
		</div>
		<div class="column right ">
			<input type="checkbox" id="checkbox_1" bind:checked={$hide_text} />
		</div>
	</section>
	<section class="row">
		<div class="column left ">
			<button
				type="button"
				disabled={$selected_day == undefined || $selected_month == undefined || processing}
				on:click={solve_click}
			>
				Solve!
			</button>
			{#if $selected_day == undefined || !selected_month == undefined}
				<p>Select a day and month to solve for.</p>
			{/if}
		</div>
		<div class="column right " style="display:block">
			{#if processing}
				<Circle2 size="60" unit="px" />
			{/if}
		</div>
	</section>
</div>

<style>
	div {
		float: left;
	}

	input {
		display: block;
		margin: auto;
	}

	input[type='checkbox'] {
		position: relative;
		top: 50%;
		width: 20px;
		height: 20px;
	}

	output {
		text-align: center;
		color: white;
		display: block;
		padding-top: 15px;
	}

	button {
		display: inline;
		margin-right: 1rem;
		background-color: #c8adc0;
		width: 6em;
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

	.row {
		display: flex;
		padding-bottom: 25%;
	}

	.column {
		float: left;
	}

	.left {
		width: 70%;
	}

	.right {
		width: 30%;
	}

	.content {
		height: fit-content;
	}

	@media screen and (max-height: 760px) and (orientation: landscape) {
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
			width: fit-content;
			height: fit-content;
			font-size: x-small;
		}

		input[type='number'] {
			width: fit-content;
			height: fit-content;
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
	}

	@media screen and (max-width: 1200px) and (orientation: portrait) {
		.content {
			padding-top: 2rem;
		}
	}
</style>
