import { writable, type Writable } from 'svelte/store';

/** Store values for puzzle pieces used in tables */
export let table_draw_scale: Writable<number> = writable(20);
export let table_svg_stroke_width: Writable<number> = writable(0.05);
export let table_svg_padding: Writable<number> = writable(0.05);

/** Store values for puzzle piece used in animation for representing the piece */
export let maxtrix_draw_scale: Writable<number> = writable(50);
export let matrix_svg_stroke_width: Writable<number> = writable(0.05);
export let matrix_svg_padding: Writable<number> = writable(0.05);

/** Store values for board and puzzle pieces */
export let board_draw_scale: Writable<number> = writable(60);
export let board_svg_stroke_width: Writable<number> = writable(0.05);
export let board_svg_padding: Writable<number> = writable(0.05);