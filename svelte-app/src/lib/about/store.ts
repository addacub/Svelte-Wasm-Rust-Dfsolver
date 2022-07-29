import { writable, type Writable } from 'svelte/store';

export let table_draw_scale: Writable<number> = writable(20);
export let maxtrix_draw_scale: Writable<number> = writable(50);
export let matrix_svg_padding: Writable<number> = writable(0.05);
export let table_svg_padding: Writable<number> = writable(0.05);
export let matrix_duration: Writable<number> = writable(8);