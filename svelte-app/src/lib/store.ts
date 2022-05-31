import { writable, type Writable } from 'svelte/store';

export let selected_day: Writable<number> = writable();
export let selected_month: Writable<number> = writable();
export let number_of_hints: Writable<number> = writable();
export let solution_number: Writable<number> = writable();
export let selected_solution: Writable<any> = writable();
export let solution_set: Writable<[[Object]]> = writable();
