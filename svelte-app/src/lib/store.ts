import { writable, type Writable } from 'svelte/store';
import { MyMap } from '$lib/my-map';

export let selected_day: Writable<number> = writable();
export let selected_month: Writable<number> = writable();
export let solution_key: Writable<[number, number]> = writable();
export let hints: Writable<number> = writable();
export let solution: Writable<number> = writable();
export let selected_solution: any = writable();
export let solution_set: any = writable();
export let solution_map: Writable<MyMap> = writable(new MyMap());
