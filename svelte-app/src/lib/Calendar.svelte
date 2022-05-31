<script lang="ts">
	import { selected_day, selected_month } from '$lib/store';

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

	let piece_colors: String[] = [
		'#F06543',
		'#D0BCD5',
		'#6B8F71',
		'#FFE1A8',
		'#9A4C95',
		'#B2EF9B',
		'#FF85A3',
		'#BEDCFE',
		'#E5C2C0'
	];

	let website_colors: String[] = ['#19232D', '#EDD3C4', '#C8ADC0', '#748067', '#3E7CB1'];

	function select_month(event: MouseEvent): void {
		let event_target: EventTarget | null = event.target;

		if (event_target == null) {
			return;
		}

		let pressed_button: HTMLElement = <HTMLElement>event_target;
		selected_month.set(Number(pressed_button.id) + 1);
		console.log(selected_month);
	}

	function select_day(event: MouseEvent): void {
		let event_target: EventTarget | null = event.target;

		if (event_target == null) {
			return;
		}

		let pressed_button: Element = <Element>event_target;
		selected_day.set(Number(pressed_button.id) - months.length + 1);
	}
</script>

<div class="calendar">
	<div class="month-container">
		{#each months as month, index}
			<button
				class={index + 1 == $selected_month ? 'pressed' : 'unpressed'}
				type="button"
				id={index.toString()}
				on:click={select_month}
			>
				{month}
			</button>
		{/each}
	</div>
	<div class="day-container">
		{#each Array(31) as _, index}
			<button
				class={index + 1 == $selected_day ? 'pressed' : 'unpressed'}
				type="button"
				id={(index + months.length).toString()}
				on:click={select_day}
			>
				{index + 1}
			</button>
		{/each}
	</div>
</div>
<p>
	The month selected is: {$selected_month != undefined
		? months[$selected_month - 1]
		: 'None selected'}
</p>
<br />
<p>
	The day selected is: {$selected_day != undefined ? $selected_day : 'None selected'}
</p>

<!-- <svg width="300" height="200" display="inline-block">
	<path d="m0,0 h300 v200 h-300 z" fill="#F06543" />
</svg>
<svg width="300" height="200" display="inline-block">
	<path d="m0,0 h100 v100 h100 v-100 h100 v200 h-300 z" fill="#D0BCD5" />
</svg>
<svg width="300" height="300">
	<path d="m0,0 h200 v200 h100 v100 h-200 v-200 h-100 z" fill="#6B8F71" />
</svg>
<svg width="300" height="200">
	<path d="m0,0 h200 v100 h100 v100 h-300 z" fill="#FFE1A8" />
</svg>
<svg width="400" height="200">
	<path d="m0,0 h400 v200 h-100 v-100 h-300 z" fill="#9A4C95" />
</svg>
<svg width="300" height="300">
	<path d="m0,0 h100 v200 h200 v100 h-300 z" fill="#B2EF9B" />
</svg>
<svg width="400" height="200">
	<path d="m0,0 h300 v100 h100 v100 h-200 v-100 h-200 z" fill="#FF85A3" />
</svg>
<svg width="400" height="200">
	<path d="m0,0 h400 v100 h-200 v100 h-100 v-100 h-100 z" fill="#BEDCFE" />
</svg>
<svg width="300" height="300">
	<path d="m0,0 h100 v200 h200 v100 h-300 z" fill="#E5C2C0" />
</svg> -->
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
		background-color: gold;
	}

	.unpressed {
		background-color: #edd3c4;
		border-color: white rgb(110, 110, 110) rgb(110, 110, 110) white;
	}

	.pressed {
		background-color: rgb(255, 246, 195);
		border-color: rgb(110, 110, 110) white white rgb(110, 110, 110);
	}
</style>
