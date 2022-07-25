<script context="module" lang="ts">
	export const prerender = true;
</script>

<script lang="ts">
	import {
		colour_classes,
		selected_solution,
		solution_set,
		selected_day,
		selected_month,
		number_of_hints,
		day_solved_for,
		month_solved_for,
		hide_text,
		is_valid_solution
	} from '$lib/solver/store';
	import {} from '$lib/solver/store';
	import { get_board_index } from '$lib/solver/utils';

	let months: String[] = [
		'Jan',
		'Feb',
		'Mar',
		'Apr',
		'May',
		'Jun',
		'Jul',
		'Aug',
		'Sep',
		'Oct',
		'Nov',
		'Dec'
	];

	// When the index number of a button is placed in one
	// of the colour group arrays, a random colour class
	// associated with the colour group is added
	// to the button to change its background colour
	let button_colour_groups: number[][] = [[], [], [], [], [], [], [], []];
	let covered_buttons: number[] = [];
	let website_colors: String[] = ['#19232D', '#EDD3C4', '#C8ADC0', '#748067', '#3E7CB1'];

	function select_month(event: MouseEvent): void {
		let event_target: EventTarget | null = event.target;

		if (event_target == null) {
			return;
		}

		let pressed_button: HTMLElement = <HTMLElement>event_target;
		selected_month.set(Number(pressed_button.id) + 1);
	}

	function select_day(event: MouseEvent): void {
		let event_target: EventTarget | null = event.target;

		if (event_target == null) {
			return;
		}

		let pressed_button: Element = <Element>event_target;
		selected_day.set(Number(pressed_button.id) - months.length + 1);
	}

	// Reactive declarations: whenever the selected day or month changes
	// update board is called.
	$: $selected_day, update_board();
	$: $selected_month, update_board();

	// Will clear board if day and month selected is not what the current solution is for
	function update_board(): void {
		if ($month_solved_for != $selected_month || $day_solved_for != $selected_day) {
			is_valid_solution.set(false);
		} else {
			is_valid_solution.set(true);
		}

		show_pieces();
	}

	// Clears the board
	function clear_peices(): void {
		button_colour_groups.forEach((button_colour_group) => (button_colour_group.length = 0));
		covered_buttons.length = 0;
		// Assignment required for svelte to know vairable has changed and to be updated
		button_colour_groups = button_colour_groups;
		covered_buttons = covered_buttons;
	}

	// Reactive declarations: whenever the value of selected_solution or number of hints changes
	// the show_pieces() function is called
	$: $selected_solution, show_pieces();
	$: $number_of_hints, show_pieces();

	// Adds pieces to the board (selected randomly), based on number of hints the user wants displayed.
	function show_pieces(): void {
		// Remove current assigned colours (i.e. remove pieces)
		clear_peices();

		// Guard checks
		if ($solution_set.length == 0) {
			return;
		}

		if (!$is_valid_solution) {
			return;
		}

		// Loop through number of hints and assign to a colour group
		for (let hint = 0; hint < $number_of_hints; hint++) {
			let piece: {
				name: String;
				board_position: [number, number];
				orientation: { data: number[]; shape: { cols: number; rows: number } };
			} = $selected_solution[hint];

			let starting_ID = get_board_index(piece.board_position);

			piece.orientation.data.forEach(function (value: number, index: number) {
				if (value == 1) {
					let button_ID: number =
						starting_ID +
						(index % piece.orientation.shape.cols) +
						Math.trunc(index / piece.orientation.shape.cols) * 7;

					button_colour_groups[hint].push(button_ID);
					covered_buttons.push(button_ID);
				}
			});
		}

		// Assignment required for svelte to know vairable has changed and to be updated
		button_colour_groups = button_colour_groups;
		covered_buttons = covered_buttons;
		reactive_flag = true;
	}

	// Purposely uses assignment to trigger function calls
	let reactive_flag: boolean = true;

	function get_colour(board_position: number, reactive_flag: boolean): string {
		for (let i: number = 0; i < button_colour_groups.length; i++) {
			if (button_colour_groups[i].includes(board_position)) {
				console.log('Inside get colour function');
				return $colour_classes[i];
			}
		}

		return '';
	}
</script>

<div class="calendar">
	<div class="month-container">
		{#each months as month, index}
			<button
				class="{index + 1 == $selected_month
					? 'pressed'
					: 'unpressed'} {button_colour_groups[0].includes(index + Math.trunc(index / 6))
					? $colour_classes[0]
					: button_colour_groups[1].includes(index + Math.trunc(index / 6))
					? $colour_classes[1]
					: button_colour_groups[2].includes(index + Math.trunc(index / 6))
					? $colour_classes[2]
					: button_colour_groups[3].includes(index + Math.trunc(index / 6))
					? $colour_classes[3]
					: button_colour_groups[4].includes(index + Math.trunc(index / 6))
					? $colour_classes[4]
					: button_colour_groups[5].includes(index + Math.trunc(index / 6))
					? $colour_classes[5]
					: button_colour_groups[6].includes(index + Math.trunc(index / 6))
					? $colour_classes[6]
					: button_colour_groups[7].includes(index + Math.trunc(index / 6))
					? $colour_classes[7]
					: ''}"
				type="button"
				on:click={select_month}
				id={index.toString()}
			>
				{covered_buttons.includes(index + Math.trunc(index / 6)) && $hide_text ? '' : month}
			</button>
		{/each}
	</div>
	<div class="day-container">
		{#each Array(31) as _, index}
			<button
				class="{index + 1 == $selected_day
					? 'pressed'
					: 'unpressed'} {button_colour_groups[0].includes(index + months.length + 2)
					? $colour_classes[0]
					: button_colour_groups[1].includes(index + months.length + 2)
					? $colour_classes[1]
					: button_colour_groups[2].includes(index + months.length + 2)
					? $colour_classes[2]
					: button_colour_groups[3].includes(index + months.length + 2)
					? $colour_classes[3]
					: button_colour_groups[4].includes(index + months.length + 2)
					? $colour_classes[4]
					: button_colour_groups[5].includes(index + months.length + 2)
					? $colour_classes[5]
					: button_colour_groups[6].includes(index + months.length + 2)
					? $colour_classes[6]
					: button_colour_groups[7].includes(index + months.length + 2)
					? $colour_classes[7]
					: ''}"
				type="button"
				on:click={select_day}
				id={(index + months.length).toString()}
			>
				{covered_buttons.includes(index + months.length + 2) && $hide_text ? '' : index + 1}
			</button>
		{/each}
	</div>
</div>

<style>
	.calendar {
		padding-top: 5rem;
	}

	.month-container {
		display: inline-grid;
		grid-template-columns: auto auto auto auto auto auto;
	}

	.day-container {
		display: grid;
		grid-template-columns: auto auto auto auto auto auto auto;
	}

	.calendar button {
		width: 7rem;
		height: 7rem;
		font-size: 2rem;
		font-family: Arial, Helvetica, sans-serif;
		border-style: solid;
		border-width: 0.2rem;
		border-radius: 5%;
	}

	.calendar button:hover {
		background-color: rgb(253, 252, 151);
	}

	.unpressed {
		background-color: #edd3c4;
		border-color: white rgb(110, 110, 110) rgb(110, 110, 110) white;
	}

	.colour-one {
		background-color: #0dc566;
	}

	.colour-two {
		background-color: #dd33ff;
	}

	.colour-three {
		background-color: #e0d900;
	}

	.colour-four {
		background-color: #5cffe9;
	}

	.colour-five {
		background-color: #ff4775;
	}

	.colour-six {
		background-color: #479dff;
	}

	.colour-seven {
		background-color: #ff1f1f;
	}

	.colour-eight {
		background-color: #97fe72;
	}

	/* Specify last so class takes precedence over the colour classes */
	.pressed {
		background-color: rgb(255, 246, 195);
		border-color: rgb(110, 110, 110) white white rgb(110, 110, 110);
	}
</style>
