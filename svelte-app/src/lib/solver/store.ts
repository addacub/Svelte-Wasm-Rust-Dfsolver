import { writable, type Writable } from 'svelte/store';

export let selected_day: Writable<number> = writable();
export let selected_month: Writable<number> = writable();
export let day_solved_for: Writable<number> = writable();
export let month_solved_for: Writable<number> = writable();
export let number_of_hints: Writable<number> = writable(1);
export let solution_number: Writable<number> = writable(1);
export let selected_solution: Writable<{name: String, board_position: [number, number], orientation: {data: number[], shape: {cols: number, rows: number}} }[]> = writable([]);
export let solution_set: Writable<{name: String, board_position: [number, number], orientation: {data: number[], shape: {cols: number, rows: number}} }[][]> = writable([]);
export let hide_text: Writable<boolean> = writable(false);

export let colour_classes: Writable<string[]> = writable([
    'colour-one',
    'colour-two',
    'colour-three',
    'colour-four',
    'colour-five',
    'colour-six',
    'colour-seven',
    'colour-eight'
]);
